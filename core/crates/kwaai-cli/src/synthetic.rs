//! Synthetic shard: stands in for compute so block routing can be tested
//! without model weights.
//!
//! A synthetic server answers on the ordinary `/kwaai/inference/1.0.0` path.
//! Its tensors keep their real shape and byte size, but carry a [`Packet`]: a
//! header every hop validates and extends, and the originator checks end to
//! end. Built only with `--features synthetic`.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};

use crate::block_rpc::{bytes_to_token_ids, InferenceRequest, PayloadType};

/// Every synthetic model name starts with this, so its DHT records can never
/// share a prefix with a real model's.
pub const MODEL_PREFIX: &str = "kwaai-synthetic/";

const MAGIC: [u8; 8] = *b"KWSYN1\0\0";
const VERSION: u16 = 1;
/// No `next_token` yet: the packet has not reached the last block.
const NO_TOKEN: u32 = u32::MAX;
/// Hop flag: this server had no session for a step past prefill — the
/// synthetic twin of a lost KV cache.
pub const FLAG_KV_RESET: u8 = 1;
/// `prompt_len` of a session that began past prefill: nothing to check.
const PROMPT_LEN_UNKNOWN: u32 = u32::MAX;
/// Same idle eviction as the real shards.
const SESSION_IDLE: Duration = Duration::from_secs(600);

/// `shard.synthetic` in config.yaml: present means `shard serve` serves
/// synthetic blocks of this model shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SyntheticConfig {
    pub total_blocks: u32,
    pub hidden_dim: u32,
    pub vocab: u32,
    /// Simulated compute per block, so timing comparisons mean something.
    pub delay_ms_per_block: u32,
}

impl Default for SyntheticConfig {
    fn default() -> Self {
        Self {
            total_blocks: 32,
            hidden_dim: 4096,
            vocab: 32000,
            delay_ms_per_block: 0,
        }
    }
}

impl SyntheticConfig {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.total_blocks > 0,
            "shard.synthetic.total_blocks must be > 0"
        );
        ensure!(
            self.hidden_dim >= 1024,
            "shard.synthetic.hidden_dim must be >= 1024 so the packet fits a one-token tensor"
        );
        ensure!(self.vocab >= 1024, "shard.synthetic.vocab must be >= 1024");
        Ok(())
    }
}

pub fn is_synthetic_model(model: &str) -> bool {
    model.starts_with(MODEL_PREFIX)
}

/// The shape `shard serve` should fake, if any: the config block, else the
/// defaults when `--synthetic` was typed.
pub fn serve_config(
    flag: bool,
    cfg: &crate::config::KwaaiNetConfig,
) -> Result<Option<SyntheticConfig>> {
    let sc = match (&cfg.shard.synthetic, flag) {
        (Some(c), _) => c.clone(),
        (None, true) => SyntheticConfig::default(),
        (None, false) => return Ok(None),
    };
    sc.validate()?;
    ensure!(
        is_synthetic_model(&cfg.model),
        "synthetic serving needs `model:` to start with {MODEL_PREFIX} (got {}), \
         so its DHT records never mix with a real model's",
        cfg.model
    );
    Ok(Some(sc))
}

// ── Packet ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HopRecord {
    pub peer_id: Vec<u8>,
    pub start_block: u32,
    pub end_block: u32,
    pub fwd_us: u32,
    pub flags: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub session_id: u64,
    pub seq_pos: u32,
    pub seq_len: u32,
    /// Originating tokens, as the first hop remembered them from prefill.
    pub prompt_len: u32,
    /// FNV-1a of the token ids the first hop received this step.
    pub token_digest: u64,
    pub next_token: Option<u32>,
    pub hops: Vec<HopRecord>,
}

pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

fn digest_ids(ids: &[u32]) -> u64 {
    let bytes: Vec<u8> = ids.iter().flat_map(|i| i.to_le_bytes()).collect();
    fnv1a(&bytes)
}

/// The token the last block "predicts": a pure function of the step, so the
/// originator can recompute it.
pub fn expected_next_token(token_digest: u64, seq_pos: u32, vocab: u32) -> u32 {
    let mut b = token_digest.to_le_bytes().to_vec();
    b.extend_from_slice(&seq_pos.to_le_bytes());
    (fnv1a(&b) % vocab as u64) as u32
}

impl Packet {
    /// Serialise into exactly `total_len` bytes:
    /// `[header][hops][checksum u64][filler]`, the checksum covering all else.
    pub fn encode(&self, total_len: usize) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(total_len);
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&VERSION.to_le_bytes());
        out.extend_from_slice(&self.session_id.to_le_bytes());
        out.extend_from_slice(&self.seq_pos.to_le_bytes());
        out.extend_from_slice(&self.seq_len.to_le_bytes());
        out.extend_from_slice(&self.prompt_len.to_le_bytes());
        out.extend_from_slice(&self.token_digest.to_le_bytes());
        out.extend_from_slice(&self.next_token.unwrap_or(NO_TOKEN).to_le_bytes());
        let hop_count = u16::try_from(self.hops.len()).context("too many hops")?;
        out.extend_from_slice(&hop_count.to_le_bytes());
        for h in &self.hops {
            let len = u8::try_from(h.peer_id.len()).context("peer id too long")?;
            out.push(len);
            out.extend_from_slice(&h.peer_id);
            out.extend_from_slice(&h.start_block.to_le_bytes());
            out.extend_from_slice(&h.end_block.to_le_bytes());
            out.extend_from_slice(&h.fwd_us.to_le_bytes());
            out.push(h.flags);
        }
        let sum_at = out.len();
        ensure!(
            sum_at + 8 <= total_len,
            "synthetic packet ({} bytes) does not fit a {total_len}-byte tensor",
            sum_at + 8
        );
        out.extend_from_slice(&[0; 8]);
        // xorshift filler seeded by the step, so a payload is never all-constant.
        let mut x = self.session_id ^ ((self.seq_pos as u64) << 32) ^ hop_count as u64 | 1;
        while out.len() < total_len {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            out.push(x as u8);
        }
        let sum = checksum(&out, sum_at);
        out[sum_at..sum_at + 8].copy_from_slice(&sum.to_le_bytes());
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut r = Reader { b: bytes, at: 0 };
        ensure!(
            r.take(8)? == MAGIC,
            "not a synthetic packet (bad magic) — a real tensor reached a synthetic peer, or vice versa"
        );
        let version = r.u16()?;
        ensure!(
            version == VERSION,
            "synthetic packet version {version}, expected {VERSION}"
        );
        let session_id = r.u64()?;
        let seq_pos = r.u32()?;
        let seq_len = r.u32()?;
        let prompt_len = r.u32()?;
        let token_digest = r.u64()?;
        let next_token = Some(r.u32()?).filter(|t| *t != NO_TOKEN);
        let hop_count = r.u16()?;
        let mut hops = Vec::with_capacity(hop_count as usize);
        for _ in 0..hop_count {
            let len = r.take(1)?[0] as usize;
            hops.push(HopRecord {
                peer_id: r.take(len)?.to_vec(),
                start_block: r.u32()?,
                end_block: r.u32()?,
                fwd_us: r.u32()?,
                flags: r.take(1)?[0],
            });
        }
        let sum_at = r.at;
        let stored = r.u64()?;
        ensure!(
            stored == checksum(bytes, sum_at),
            "synthetic packet checksum mismatch — payload corrupted in transit"
        );
        Ok(Self {
            session_id,
            seq_pos,
            seq_len,
            prompt_len,
            token_digest,
            next_token,
            hops,
        })
    }
}

/// FNV-1a over everything but the 8 checksum bytes at `sum_at`.
fn checksum(bytes: &[u8], sum_at: usize) -> u64 {
    let mut h = fnv1a(&bytes[..sum_at]);
    for b in &bytes[sum_at + 8..] {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.at + n;
        ensure!(end <= self.b.len(), "synthetic packet truncated");
        let s = &self.b[self.at..end];
        self.at = end;
        Ok(s)
    }
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into()?))
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into()?))
    }
    fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into()?))
    }
}

// ── Server ───────────────────────────────────────────────────────────────────

struct Session {
    next_pos: u32,
    prompt_len: u32,
    last_used: Instant,
}

/// One synthetic block range. Its "KV cache" is just the next position each
/// session expects, which is what makes reordering and lost state visible.
pub struct SyntheticShard {
    peer_id: Vec<u8>,
    start_block: u32,
    end_block: u32,
    cfg: SyntheticConfig,
    sessions: Mutex<HashMap<u64, Session>>,
}

impl SyntheticShard {
    pub fn new(peer_id: Vec<u8>, start_block: u32, end_block: u32, cfg: SyntheticConfig) -> Self {
        Self {
            peer_id,
            start_block,
            end_block,
            cfg,
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn blocks(&self) -> (usize, usize) {
        (self.start_block as usize, self.end_block as usize)
    }
    pub fn is_first(&self) -> bool {
        self.start_block == 0
    }
    pub fn is_last(&self) -> bool {
        self.end_block >= self.cfg.total_blocks
    }

    pub fn gc_sessions(&self) {
        let mut s = self.sessions.lock().unwrap_or_else(|p| p.into_inner());
        s.retain(|_, v| v.last_used.elapsed() < SESSION_IDLE);
    }

    /// Run one step: returns `(shape, data, is_logits)` like a real forward.
    pub fn forward(&self, req: &InferenceRequest) -> Result<(Vec<u32>, Vec<u8>, bool)> {
        let started = Instant::now();
        let mut packet = match req.payload_type {
            PayloadType::TokenIds => {
                ensure!(
                    self.is_first(),
                    "TokenIds sent to a shard starting at block {}",
                    self.start_block
                );
                let ids = bytes_to_token_ids(&req.data)?;
                Packet {
                    session_id: req.session_id,
                    seq_pos: req.seq_pos,
                    seq_len: ids.len() as u32,
                    prompt_len: 0,
                    token_digest: digest_ids(&ids),
                    next_token: None,
                    hops: Vec::new(),
                }
            }
            PayloadType::HiddenStates => {
                let p = Packet::decode(&req.data)?;
                ensure!(
                    p.session_id == req.session_id,
                    "packet session {} != request session {}",
                    p.session_id,
                    req.session_id
                );
                ensure!(
                    p.seq_pos == req.seq_pos,
                    "packet seq_pos {} != request seq_pos {}",
                    p.seq_pos,
                    req.seq_pos
                );
                let want = [1, p.seq_len, self.cfg.hidden_dim];
                ensure!(
                    req.shape == want,
                    "shape {:?}, expected {want:?}",
                    req.shape
                );
                let want_len = p.seq_len as usize * self.cfg.hidden_dim as usize * 2;
                ensure!(
                    req.data.len() == want_len,
                    "payload {} bytes, expected {want_len}",
                    req.data.len()
                );
                let prev_end = p.hops.last().map(|h| h.end_block);
                ensure!(
                    prev_end == Some(self.start_block),
                    "hop gap: previous hop ended at {prev_end:?}, this shard starts at {}",
                    self.start_block
                );
                p
            }
        };

        let flags = self.advance_session(&mut packet)?;

        let delay = self.cfg.delay_ms_per_block as u64 * (self.end_block - self.start_block) as u64;
        if delay > 0 {
            std::thread::sleep(Duration::from_millis(delay));
        }
        packet.hops.push(HopRecord {
            peer_id: self.peer_id.clone(),
            start_block: self.start_block,
            end_block: self.end_block,
            fwd_us: started.elapsed().as_micros().min(u32::MAX as u128) as u32,
            flags,
        });

        if self.is_last() {
            packet.next_token = Some(expected_next_token(
                packet.token_digest,
                packet.seq_pos,
                self.cfg.vocab,
            ));
            let data = packet.encode(self.cfg.vocab as usize * 2)?;
            Ok((vec![1, 1, self.cfg.vocab], data, true))
        } else {
            let total = packet.seq_len as usize * self.cfg.hidden_dim as usize * 2;
            let data = packet.encode(total)?;
            Ok((vec![1, packet.seq_len, self.cfg.hidden_dim], data, false))
        }
    }

    /// Check the step against this session's fake KV and move it on.
    fn advance_session(&self, packet: &mut Packet) -> Result<u8> {
        let mut sessions = self.sessions.lock().unwrap_or_else(|p| p.into_inner());
        let mut flags = 0;
        let fresh = !sessions.contains_key(&packet.session_id);
        let s = sessions.entry(packet.session_id).or_insert(Session {
            next_pos: 0,
            prompt_len: 0,
            last_used: Instant::now(),
        });
        if packet.seq_pos == 0 {
            // Prefill, or a fresh prefill restarting a known id.
            s.prompt_len = packet.seq_len;
        } else if fresh {
            flags |= FLAG_KV_RESET;
            s.prompt_len = PROMPT_LEN_UNKNOWN;
        } else {
            ensure!(
                packet.seq_pos == s.next_pos,
                "out-of-order step: session {} expected seq_pos {}, got {}",
                packet.session_id,
                s.next_pos,
                packet.seq_pos
            );
        }
        s.next_pos = packet.seq_pos + packet.seq_len;
        s.last_used = Instant::now();
        if self.is_first() {
            packet.prompt_len = s.prompt_len;
        }
        Ok(flags)
    }
}

// ── Originator ───────────────────────────────────────────────────────────────

/// What the originator sent this step, to check the final packet against.
pub struct Expected<'a> {
    pub session_id: u64,
    pub seq_pos: u32,
    pub token_ids: &'a [u32],
    pub prompt_len: u32,
    pub total_blocks: u32,
    /// Every `(peer_id, start, end)` the DHT announced; each hop must be one.
    pub servers: &'a [(Vec<u8>, u32, u32)],
}

#[derive(Debug)]
pub struct Verified {
    pub next_token: u32,
    pub kv_resets: usize,
}

/// Check a final (logits) response end to end.
pub fn verify_final(shape: &[u32], data: &[u8], exp: &Expected<'_>) -> Result<Verified> {
    let p = Packet::decode(data)?;
    let [1, 1, vocab] = shape else {
        bail!("final shape {shape:?}, expected [1, 1, vocab]");
    };
    ensure!(
        data.len() == *vocab as usize * 2,
        "final payload {} bytes, expected {} for vocab {vocab}",
        data.len(),
        *vocab as usize * 2
    );
    ensure!(
        p.session_id == exp.session_id,
        "session {} != {}",
        p.session_id,
        exp.session_id
    );
    ensure!(
        p.seq_pos == exp.seq_pos,
        "seq_pos {} != {}",
        p.seq_pos,
        exp.seq_pos
    );
    ensure!(
        p.seq_len as usize == exp.token_ids.len(),
        "seq_len {} != {} tokens sent",
        p.seq_len,
        exp.token_ids.len()
    );
    ensure!(
        p.token_digest == digest_ids(exp.token_ids),
        "token digest mismatch — the first hop saw different token ids"
    );

    let mut pos = 0;
    for (i, h) in p.hops.iter().enumerate() {
        ensure!(
            h.start_block == pos,
            "hop {i} starts at block {}, expected {pos}",
            h.start_block
        );
        ensure!(
            exp.servers
                .iter()
                .any(|(id, s, e)| *id == h.peer_id && *s == h.start_block && *e == h.end_block),
            "hop {i} [{}, {}) is not a server announcing that range",
            h.start_block,
            h.end_block
        );
        pos = h.end_block;
    }
    ensure!(
        pos == exp.total_blocks,
        "hops cover [0, {pos}), expected [0, {})",
        exp.total_blocks
    );

    if p.prompt_len != PROMPT_LEN_UNKNOWN {
        ensure!(
            p.prompt_len == exp.prompt_len,
            "prompt_len {} != {}",
            p.prompt_len,
            exp.prompt_len
        );
    }
    let want = expected_next_token(p.token_digest, p.seq_pos, *vocab);
    ensure!(
        p.next_token == Some(want),
        "next_token {:?}, expected {want}",
        p.next_token
    );
    Ok(Verified {
        next_token: want,
        kv_resets: p
            .hops
            .iter()
            .filter(|h| h.flags & FLAG_KV_RESET != 0)
            .count(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_rpc::token_ids_to_bytes;

    fn cfg() -> SyntheticConfig {
        SyntheticConfig {
            total_blocks: 12,
            hidden_dim: 1024,
            vocab: 2048,
            delay_ms_per_block: 0,
        }
    }

    fn path() -> Vec<SyntheticShard> {
        [(0, 4), (4, 8), (8, 12)]
            .iter()
            .enumerate()
            .map(|(i, (s, e))| SyntheticShard::new(vec![i as u8; 4], *s, *e, cfg()))
            .collect()
    }

    fn servers() -> Vec<(Vec<u8>, u32, u32)> {
        vec![(vec![0; 4], 0, 4), (vec![1; 4], 4, 8), (vec![2; 4], 8, 12)]
    }

    /// Star-style relay: the originator copies each response into the next request.
    fn step(
        path: &[SyntheticShard],
        session: u64,
        seq_pos: u32,
        ids: &[u32],
    ) -> Result<(Vec<u32>, Vec<u8>)> {
        let (shape, data) = token_ids_to_bytes(ids);
        let mut req = InferenceRequest {
            session_id: session,
            seq_pos,
            payload_type: PayloadType::TokenIds,
            shape,
            data,
        };
        for (i, s) in path.iter().enumerate() {
            let (shape, data, is_logits) = s.forward(&req)?;
            assert_eq!(is_logits, i == path.len() - 1);
            req = InferenceRequest {
                session_id: session,
                seq_pos,
                payload_type: PayloadType::HiddenStates,
                shape,
                data,
            };
        }
        Ok((req.shape, req.data))
    }

    fn expect<'a>(
        seq_pos: u32,
        ids: &'a [u32],
        servers: &'a [(Vec<u8>, u32, u32)],
    ) -> Expected<'a> {
        Expected {
            session_id: 7,
            seq_pos,
            token_ids: ids,
            prompt_len: 5,
            total_blocks: 12,
            servers,
        }
    }

    #[test]
    fn packet_round_trips_and_pads_to_size() {
        let p = Packet {
            session_id: 1,
            seq_pos: 2,
            seq_len: 3,
            prompt_len: 4,
            token_digest: 5,
            next_token: Some(6),
            hops: vec![HopRecord {
                peer_id: vec![9; 38],
                start_block: 0,
                end_block: 8,
                fwd_us: 12,
                flags: FLAG_KV_RESET,
            }],
        };
        let bytes = p.encode(4096).unwrap();
        assert_eq!(bytes.len(), 4096);
        assert_eq!(Packet::decode(&bytes).unwrap(), p);
    }

    #[test]
    fn corruption_anywhere_is_caught() {
        let p = Packet {
            session_id: 1,
            seq_pos: 0,
            seq_len: 1,
            prompt_len: 1,
            token_digest: 0,
            next_token: None,
            hops: vec![],
        };
        let good = p.encode(2048).unwrap();
        for at in [9, 40, 1000, 2047] {
            let mut bad = good.clone();
            bad[at] ^= 0x01;
            assert!(Packet::decode(&bad).is_err(), "flip at {at} went unnoticed");
        }
        assert!(
            Packet::decode(&[0u8; 64]).is_err(),
            "a real tensor must fail on magic"
        );
    }

    #[test]
    fn a_three_hop_generation_verifies_every_step() {
        let path = path();
        let servers = servers();
        let prompt = [10, 11, 12, 13, 14];
        let (shape, data) = step(&path, 7, 0, &prompt).unwrap();
        let v = verify_final(&shape, &data, &expect(0, &prompt, &servers)).unwrap();
        assert_eq!(Packet::decode(&data).unwrap().hops.len(), 3);
        assert_eq!(v.kv_resets, 0);

        let mut seq_pos = 5;
        let mut tok = v.next_token;
        for _ in 0..4 {
            let ids = [tok];
            let (shape, data) = step(&path, 7, seq_pos, &ids).unwrap();
            let v = verify_final(&shape, &data, &expect(seq_pos, &ids, &servers)).unwrap();
            tok = v.next_token;
            seq_pos += 1;
        }
    }

    #[test]
    fn a_replayed_step_is_out_of_order() {
        let path = path();
        step(&path, 7, 0, &[1, 2, 3]).unwrap();
        step(&path, 7, 3, &[4]).unwrap();
        let err = step(&path, 7, 3, &[4]).unwrap_err();
        assert!(format!("{err:#}").contains("out-of-order"), "{err:#}");
    }

    #[test]
    fn a_fresh_server_mid_session_flags_a_kv_reset() {
        let servers = servers();
        let prompt = [1, 2, 3, 4, 5];
        step(&path(), 7, 0, &prompt).unwrap();
        // A rebuilt path: same ranges, servers that never saw the prefill.
        let rebuilt = path();
        let (shape, data) = step(&rebuilt, 7, 5, &[9]).unwrap();
        let v = verify_final(&shape, &data, &expect(5, &[9], &servers)).unwrap();
        assert_eq!(v.kv_resets, 3);
        // The next step on that path is in order and carries no stale prompt_len.
        let (shape, data) = step(&rebuilt, 7, 6, &[4]).unwrap();
        let v = verify_final(&shape, &data, &expect(6, &[4], &servers)).unwrap();
        assert_eq!(v.kv_resets, 0);
    }

    #[test]
    fn a_seq_len_the_payload_cannot_hold_is_refused() {
        let path = path();
        let (shape, data) = token_ids_to_bytes(&[1]);
        let req = InferenceRequest {
            session_id: 7,
            seq_pos: 0,
            payload_type: PayloadType::TokenIds,
            shape,
            data,
        };
        let (_, data, _) = path[0].forward(&req).unwrap();
        let mut p = Packet::decode(&data).unwrap();
        p.seq_len = 1 << 30;
        let lie = InferenceRequest {
            session_id: 7,
            seq_pos: 0,
            payload_type: PayloadType::HiddenStates,
            shape: vec![1, p.seq_len, cfg().hidden_dim],
            data: p.encode(data.len()).unwrap(),
        };
        let err = path[1].forward(&lie).unwrap_err();
        assert!(format!("{err:#}").contains("payload"), "{err:#}");
    }

    #[test]
    fn a_gap_in_the_route_is_refused_by_the_next_hop() {
        let path = path();
        let (shape, data) = token_ids_to_bytes(&[1]);
        let req = InferenceRequest {
            session_id: 7,
            seq_pos: 0,
            payload_type: PayloadType::TokenIds,
            shape,
            data,
        };
        let (shape, data, _) = path[0].forward(&req).unwrap();
        let skip = InferenceRequest {
            session_id: 7,
            seq_pos: 0,
            payload_type: PayloadType::HiddenStates,
            shape,
            data,
        };
        let err = path[2].forward(&skip).unwrap_err();
        assert!(format!("{err:#}").contains("hop gap"), "{err:#}");
    }

    #[test]
    fn verify_rejects_a_hop_nobody_announced() {
        let prompt = [1, 2, 3, 4, 5];
        let (shape, data) = step(&path(), 7, 0, &prompt).unwrap();
        let only_two = &servers()[..2];
        let err = verify_final(&shape, &data, &expect(0, &prompt, only_two)).unwrap_err();
        assert!(format!("{err:#}").contains("not a server"), "{err:#}");
    }

    #[test]
    fn verify_rejects_a_different_prompt() {
        let servers = servers();
        let (shape, data) = step(&path(), 7, 0, &[1, 2, 3, 4, 5]).unwrap();
        let err = verify_final(&shape, &data, &expect(0, &[1, 2, 3, 4, 6], &servers)).unwrap_err();
        assert!(format!("{err:#}").contains("digest"), "{err:#}");
    }
}
