//! Pluggable currency backends.
//!
//! Receipts record *work*; an economy decides what that work is *worth*. Keeping
//! the two apart is what lets the community A/B different currency models over
//! the same evidence: the same set of signed receipts can be settled under a
//! frequent-flyer model one month and a fixed-pool token model the next, with no
//! change to the receipt path and no re-collection of data.
//!
//! Concretely: [`Economy::settle`] re-prices from raw token counts, **not** from
//! the `credits_owed` the provider quoted. That field records what was agreed
//! bilaterally at the time; it is not the unit any particular economy has to
//! reward in.
//!
//! ## Why net, not gross
//!
//! Minting introduces an attack that pairwise credit did not have: two colluding
//! identities can trade trivial work back and forth and farm the reward. The
//! primary defence is structural — mint on **net** position per counterparty, so
//! symmetric wash trading cancels to zero before any weighting is applied. The
//! diversity factor then discounts the asymmetric case, where a ring keeps
//! pushing work one direction.
//!
//! ## What is deliberately not here
//!
//! Work kinds other than chat tokens. [`WorkClaimPayload`](crate::WorkClaimPayload)
//! is signed with *positional* msgpack, so adding a work-kind field changes the
//! signing preimage and breaks verification against already-deployed nodes.
//! Metering embeddings therefore needs a coordinated protocol version bump, not
//! an additive field. The types here are shaped to accept more kinds when that
//! happens.

use std::collections::BTreeMap;

use crate::MicroCredits;

/// One counterparty's two-way work volume, in raw units, from our point of view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterpartyWork {
    pub peer_did: String,
    /// Tokens we served *to* this peer.
    pub tokens_served: u64,
    /// Tokens this peer served *to us*.
    pub tokens_consumed: u64,
}

/// Raw work volume per counterparty — the input every economy settles from.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkLedger {
    pub entries: Vec<CounterpartyWork>,
}

impl WorkLedger {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// What a unit of work is worth. Published by the issuer; carried in a signed
/// quote at negotiation time so the two parties agree on price before any work
/// happens, regardless of which economy each is running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateCard {
    /// Bumped whenever the issuer revalues, so a receipt can be attributed to
    /// the schedule in force when it was earned.
    pub version: u32,
    pub micro_per_1k_chat_tokens: MicroCredits,
}

/// Network-wide figures an economy may need that a single node cannot know from
/// its own receipts.
#[derive(Debug, Clone, Default)]
pub struct SettlementContext {
    /// Total emission available this epoch, for fixed-pool economies.
    pub epoch_pool: MicroCredits,
    /// Sum of every member's weighted contribution this epoch. `None` when the
    /// node is settling locally and has no network view — a fixed-pool economy
    /// can then report its own share numerator but not its payout.
    pub network_weighted_total: Option<u128>,
}

/// The outcome of settling one member's receipts.
#[derive(Debug, Clone, PartialEq)]
pub struct Settlement {
    pub economy: &'static str,
    pub unit: &'static str,
    /// Value of all work we served, before netting.
    pub gross_served: MicroCredits,
    /// Value of all work we consumed.
    pub gross_consumed: MicroCredits,
    /// Sum over counterparties of `max(0, served - consumed)` — we only mint on
    /// peers we were a net provider to.
    pub net_positive: MicroCredits,
    /// Effective number of counterparties (inverse Herfindahl). 1.0 means all
    /// contribution came from a single peer.
    pub effective_counterparties: f64,
    pub diversity_factor: f64,
    /// What this member earns for the epoch.
    pub minted: MicroCredits,
    /// Present when the figure is incomplete or provisional.
    pub note: Option<String>,
}

pub trait Economy: Send + Sync {
    /// Stable identifier, announced to peers so an experiment's participants are
    /// identifiable and results are comparable.
    fn id(&self) -> &'static str;
    /// Display unit.
    fn unit(&self) -> &'static str;
    fn rate_card(&self) -> RateCard;
    /// One-time grant for a new member. Gated elsewhere by proof-of-resource —
    /// this is only the amount.
    fn welcome_grant(&self) -> MicroCredits;
    fn settle(&self, ledger: &WorkLedger, ctx: &SettlementContext) -> Settlement;
}

/// Select a backend by its identifier. Unknown ids fall back to [`NoEconomy`]
/// rather than failing: an unrecognised setting must never stop a node serving.
pub fn economy_by_id(id: &str) -> Box<dyn Economy> {
    match id {
        MilesEconomy::ID => Box::new(MilesEconomy::default()),
        AsiShadowEconomy::ID => Box::new(AsiShadowEconomy::default()),
        _ => Box::new(NoEconomy),
    }
}

// ── Shared arithmetic ─────────────────────────────────────────────────────────

fn value_of(tokens: u64, card: &RateCard) -> MicroCredits {
    card.micro_per_1k_chat_tokens
        .saturating_mul(tokens)
        .div_ceil(1000)
}

/// Effective counterparty count and the resulting weighting.
///
/// `n_eff` is the inverse Herfindahl index over per-peer positive net: 1.0 when
/// everything came from one peer, approaching the true count as contribution
/// spreads out. The factor `n_eff / (n_eff + 1)` is deliberately saturating
/// rather than linear — it must discount concentration without punishing a
/// genuinely small network, where having only one or two peers to serve is
/// normal and honest rather than evidence of collusion.
fn diversity(per_peer: &[MicroCredits]) -> (f64, f64) {
    let total: u128 = per_peer.iter().map(|v| *v as u128).sum();
    if total == 0 {
        return (0.0, 0.0);
    }
    let h: f64 = per_peer
        .iter()
        .map(|v| {
            let share = *v as f64 / total as f64;
            share * share
        })
        .sum();
    let n_eff = if h > 0.0 { 1.0 / h } else { 0.0 };
    (n_eff, n_eff / (n_eff + 1.0))
}

/// Per-counterparty net, valued under `card`, plus the totals.
fn net_positions(
    ledger: &WorkLedger,
    card: &RateCard,
) -> (MicroCredits, MicroCredits, Vec<MicroCredits>) {
    let mut gross_served = 0u64;
    let mut gross_consumed = 0u64;
    let mut positives = Vec::new();

    for e in &ledger.entries {
        let served = value_of(e.tokens_served, card);
        let consumed = value_of(e.tokens_consumed, card);
        gross_served = gross_served.saturating_add(served);
        gross_consumed = gross_consumed.saturating_add(consumed);
        if served > consumed {
            positives.push(served - consumed);
        }
    }
    (gross_served, gross_consumed, positives)
}

// ── Backends ──────────────────────────────────────────────────────────────────

/// No economy: work is metered and receipts are exchanged, but nothing is
/// valued. The pre-experiment behaviour, and the safe fallback.
pub struct NoEconomy;

impl Economy for NoEconomy {
    fn id(&self) -> &'static str {
        "none"
    }
    fn unit(&self) -> &'static str {
        "—"
    }
    fn rate_card(&self) -> RateCard {
        RateCard {
            version: 0,
            micro_per_1k_chat_tokens: 0,
        }
    }
    fn welcome_grant(&self) -> MicroCredits {
        0
    }
    fn settle(&self, _ledger: &WorkLedger, _ctx: &SettlementContext) -> Settlement {
        Settlement {
            economy: self.id(),
            unit: self.unit(),
            gross_served: 0,
            gross_consumed: 0,
            net_positive: 0,
            effective_counterparties: 0.0,
            diversity_factor: 0.0,
            minted: 0,
            note: Some("no economy configured".into()),
        }
    }
}

/// **Frequent-flyer miles.** Unlimited issuance at a published rate: serve work,
/// earn miles at the rate card, forever. Supply expands with usage, and the
/// issuer controls value by revaluing the card — exactly how an airline award
/// chart behaves.
pub struct MilesEconomy {
    pub card: RateCard,
    pub welcome: MicroCredits,
}

impl MilesEconomy {
    pub const ID: &'static str = "miles";
}

impl Default for MilesEconomy {
    fn default() -> Self {
        Self {
            card: RateCard {
                version: 1,
                micro_per_1k_chat_tokens: 1_000,
            },
            // 10 credits, enough to feel usable without being worth farming.
            welcome: 10 * crate::MICRO_PER_CREDIT,
        }
    }
}

impl Economy for MilesEconomy {
    fn id(&self) -> &'static str {
        Self::ID
    }
    fn unit(&self) -> &'static str {
        "miles"
    }
    fn rate_card(&self) -> RateCard {
        self.card
    }
    fn welcome_grant(&self) -> MicroCredits {
        self.welcome
    }

    fn settle(&self, ledger: &WorkLedger, _ctx: &SettlementContext) -> Settlement {
        let (gross_served, gross_consumed, positives) = net_positions(ledger, &self.card);
        let net_positive: MicroCredits = positives.iter().copied().sum();
        let (n_eff, factor) = diversity(&positives);

        Settlement {
            economy: self.id(),
            unit: self.unit(),
            gross_served,
            gross_consumed,
            net_positive,
            effective_counterparties: n_eff,
            diversity_factor: factor,
            minted: (net_positive as f64 * factor) as MicroCredits,
            note: None,
        }
    }
}

/// **$ASI shadow.** Accounts as though rewards came from a fixed periodic
/// emission pool rather than unlimited minting: your payout is your share of the
/// network's weighted contribution, so the same work earns less as the network
/// grows.
///
/// This is the economically interesting contrast with miles — capped supply and
/// competitive dilution versus a guaranteed rate — and it is the property worth
/// A/B testing. Shadow means exactly that: **no custody, no keys, no chain, no
/// transactions.** Nothing here touches a real asset.
pub struct AsiShadowEconomy {
    pub card: RateCard,
}

impl AsiShadowEconomy {
    pub const ID: &'static str = "asi-shadow";
}

impl Default for AsiShadowEconomy {
    fn default() -> Self {
        Self {
            // The card only converts tokens into a comparable weight here; the
            // payout is a share of the pool, so the absolute rate cancels out.
            card: RateCard {
                version: 1,
                micro_per_1k_chat_tokens: 1_000,
            },
        }
    }
}

impl Economy for AsiShadowEconomy {
    fn id(&self) -> &'static str {
        Self::ID
    }
    fn unit(&self) -> &'static str {
        "ASI(shadow)"
    }
    fn rate_card(&self) -> RateCard {
        self.card
    }
    /// No welcome grant: a fixed-supply token cannot conjure one, which is
    /// itself part of what the comparison is testing.
    fn welcome_grant(&self) -> MicroCredits {
        0
    }

    fn settle(&self, ledger: &WorkLedger, ctx: &SettlementContext) -> Settlement {
        let (gross_served, gross_consumed, positives) = net_positions(ledger, &self.card);
        let net_positive: MicroCredits = positives.iter().copied().sum();
        let (n_eff, factor) = diversity(&positives);

        let weighted = (net_positive as f64 * factor) as u128;

        let (minted, note) = match ctx.network_weighted_total {
            Some(total) if total > 0 => (
                ((ctx.epoch_pool as u128).saturating_mul(weighted) / total) as MicroCredits,
                None,
            ),
            // Settling locally, with no view of what everyone else contributed.
            // Report the numerator honestly rather than inventing a payout.
            _ => (
                0,
                Some(format!(
                    "share pending — weighted contribution {weighted}, network total unknown"
                )),
            ),
        };

        Settlement {
            economy: self.id(),
            unit: self.unit(),
            gross_served,
            gross_consumed,
            net_positive,
            effective_counterparties: n_eff,
            diversity_factor: factor,
            minted,
            note,
        }
    }
}

/// Fold a set of per-peer balances into a [`WorkLedger`].
pub fn work_ledger_from(rows: impl IntoIterator<Item = (String, u64, u64)>) -> WorkLedger {
    let mut acc: BTreeMap<String, CounterpartyWork> = BTreeMap::new();
    for (peer_did, served, consumed) in rows {
        let e = acc
            .entry(peer_did.clone())
            .or_insert_with(|| CounterpartyWork {
                peer_did,
                tokens_served: 0,
                tokens_consumed: 0,
            });
        e.tokens_served = e.tokens_served.saturating_add(served);
        e.tokens_consumed = e.tokens_consumed.saturating_add(consumed);
    }
    WorkLedger {
        entries: acc.into_values().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn work(pairs: &[(&str, u64, u64)]) -> WorkLedger {
        work_ledger_from(pairs.iter().map(|(d, s, c)| ((*d).to_string(), *s, *c)))
    }

    #[test]
    fn symmetric_wash_trading_mints_nothing() {
        // The whole reason minting settles on net rather than gross: two
        // identities trading the same volume back and forth cancel completely,
        // before any weighting is even applied.
        let s = MilesEconomy::default().settle(
            &work(&[("did:peer:b", 1_000_000, 1_000_000)]),
            &SettlementContext::default(),
        );
        assert_eq!(s.net_positive, 0);
        assert_eq!(s.minted, 0);
        assert!(s.gross_served > 0, "gross should still show the volume");
    }

    #[test]
    fn a_colluding_pair_earns_less_than_a_diversely_connected_node() {
        // Same total net contribution, concentrated vs spread. This is the
        // wash-trading test the plan calls for.
        let miles = MilesEconomy::default();
        let total = 600_000u64;

        let ring = miles.settle(
            &work(&[("did:peer:b", total, 0)]),
            &SettlementContext::default(),
        );
        let diverse = miles.settle(
            &work(&[
                ("did:peer:b", total / 6, 0),
                ("did:peer:c", total / 6, 0),
                ("did:peer:d", total / 6, 0),
                ("did:peer:e", total / 6, 0),
                ("did:peer:f", total / 6, 0),
                ("did:peer:g", total / 6, 0),
            ]),
            &SettlementContext::default(),
        );

        assert_eq!(ring.net_positive, diverse.net_positive, "same work");
        assert!(
            diverse.minted > ring.minted,
            "diverse {} should beat concentrated {}",
            diverse.minted,
            ring.minted
        );
        assert!((ring.effective_counterparties - 1.0).abs() < 1e-9);
        assert!((diverse.effective_counterparties - 6.0).abs() < 1e-6);
    }

    #[test]
    fn diversity_never_zeroes_out_a_small_honest_network() {
        // A node with exactly one peer is the normal case early on. It must be
        // discounted, not punished to nothing, or the economy cannot bootstrap.
        let s = MilesEconomy::default().settle(
            &work(&[("did:peer:b", 100_000, 0)]),
            &SettlementContext::default(),
        );
        assert!((s.diversity_factor - 0.5).abs() < 1e-9);
        assert!(s.minted > 0);
    }

    #[test]
    fn diversity_increases_monotonically_with_spread() {
        let miles = MilesEconomy::default();
        let mut last = 0.0;
        for n in 1..=8u64 {
            let rows: Vec<_> = (0..n)
                .map(|i| (format!("did:peer:{i}"), 120_000 / n, 0u64))
                .collect();
            let s = miles.settle(&work_ledger_from(rows), &SettlementContext::default());
            assert!(
                s.diversity_factor > last,
                "factor should rise with {n} counterparties"
            );
            last = s.diversity_factor;
        }
    }

    #[test]
    fn only_peers_we_net_served_contribute() {
        // Being a net consumer of one peer must not cancel what we earned from
        // another — otherwise a heavy consumer could never earn at all.
        let s = MilesEconomy::default().settle(
            &work(&[("did:peer:b", 100_000, 0), ("did:peer:c", 0, 100_000)]),
            &SettlementContext::default(),
        );
        assert_eq!(s.net_positive, value_of(100_000, &s_card()));
        assert!(s.gross_consumed > 0);
    }

    fn s_card() -> RateCard {
        MilesEconomy::default().card
    }

    #[test]
    fn asi_shadow_dilutes_as_the_network_grows() {
        // The economic contrast worth testing: identical work, fixed pool, more
        // competition means a smaller payout.
        let asi = AsiShadowEconomy::default();
        let mine = work(&[("did:peer:b", 100_000, 0), ("did:peer:c", 100_000, 0)]);

        let small = asi.settle(
            &mine,
            &SettlementContext {
                epoch_pool: 1_000_000,
                network_weighted_total: Some(200_000),
            },
        );
        let large = asi.settle(
            &mine,
            &SettlementContext {
                epoch_pool: 1_000_000,
                network_weighted_total: Some(2_000_000),
            },
        );
        assert!(
            large.minted < small.minted,
            "a bigger network must dilute: {} vs {}",
            large.minted,
            small.minted
        );
    }

    #[test]
    fn asi_shadow_reports_pending_rather_than_inventing_a_payout() {
        let s = AsiShadowEconomy::default().settle(
            &work(&[("did:peer:b", 100_000, 0)]),
            &SettlementContext::default(),
        );
        assert_eq!(s.minted, 0);
        assert!(s
            .note
            .as_deref()
            .is_some_and(|n| n.contains("share pending")));
    }

    #[test]
    fn miles_grants_a_welcome_balance_and_asi_does_not() {
        assert!(MilesEconomy::default().welcome_grant() > 0);
        assert_eq!(AsiShadowEconomy::default().welcome_grant(), 0);
    }

    #[test]
    fn an_unknown_economy_id_falls_back_to_none_rather_than_failing() {
        assert_eq!(economy_by_id("miles").id(), "miles");
        assert_eq!(economy_by_id("asi-shadow").id(), "asi-shadow");
        assert_eq!(economy_by_id("wat").id(), "none");
        assert_eq!(economy_by_id("").id(), "none");
    }

    #[test]
    fn the_same_receipts_settle_differently_under_each_economy() {
        // The point of the whole abstraction: one body of evidence, two verdicts.
        let w = work(&[("did:peer:b", 100_000, 0), ("did:peer:c", 50_000, 0)]);
        let ctx = SettlementContext {
            epoch_pool: 500_000,
            network_weighted_total: Some(1_000_000),
        };
        let m = MilesEconomy::default().settle(&w, &ctx);
        let a = AsiShadowEconomy::default().settle(&w, &ctx);
        assert_ne!(m.minted, a.minted);
        assert_eq!(
            m.net_positive, a.net_positive,
            "the underlying work is identical; only its valuation differs"
        );
    }
}
