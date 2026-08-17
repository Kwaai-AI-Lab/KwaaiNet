# Native p2p: an extra round trip on every accepted call

Measured against `main` at `8ad4680` on 2026-08-16, from `rezas-mini-2` (macOS,
relay-only inbound) to the live fleet. Every figure here is reproducible with
`kwaainet p2p probe` in two commands; see §5.

**The finding in one line: the native stack pays two network round trips for an
accepted unary call where p2pd pays one, which makes remote inference ~57%
slower.** Everything else about the native path measured better.

---

## 1. Why this matters now

The 0.6 release is scoped as *"drop p2pd, go native"*, which means flipping
`native_p2p` from `false` to `true` in `config.rs:671` and removing the Go
daemon from the build. That flag is the difference between this being a defect
some opted-in users could hit and a defect **every** user gets on auto-update.

Remote inference is the visible casualty: **478 → 750 ms/token**. Nothing else in
the cutover moves in the wrong direction.

---

## 2. The measurements

Same machine, same peer (metro-win), same protocols, same relay. The **only**
variable is `native_p2p`.

| call | p2pd | native |
|---|---|---|
| refused protocol (unimplemented) | 158.9 ms | 157.7 ms |
| **accepted protocol (`/kwaai/p2p/hello/1.0.0`)** | **159.4 ms** | **320.1 ms** |
| 256KB payload, refused | 158 ms | 158.6 ms |
| local socket write | 0.0-0.1 ms | 0.0-0.1 ms |
| remote inference, 14-token decode | **478 ms/token** | **750 ms/token** |

Medians, n=5-7 per cell, spreads tight (±5 ms).

Three explanations are ruled out by the table itself:

- **Not compute.** The remote peer is identical in both rows; only the local
  stack changed.
- **Not bandwidth.** A 256KB payload costs the same as one byte, on both stacks.
- **Not local IPC.** The write to the control socket is 0.1 ms on both.

For context on what a healthy number looks like: a minimal request should cost
exactly **one** round trip, and on p2pd it does — 159 ms accepted, against a bare
TCP handshake to the same host measured independently at 155-160 ms. The native
accepted call is almost exactly double that.

---

## 3. Why a refusal costs the same on both — and why that hid the bug

This is the diagnostic detail, not a footnote.

A **refused** protocol costs 1 RTT on *both* stacks (158 ms either way). A refusal
is answered on the first flight: the peer says "I don't speak that" and there is
never a second exchange to pay for. An **accepted** protocol is where the two
diverge, because only an accepted call reaches the step where native waits.

The consequence for tooling is sharp. `kwaainet p2p probe` defaults to an
unimplemented protocol precisely so it needs no cooperation from the peer — and
that default reports a clean, healthy **158 ms on native**. A probe run in its
default mode would have signed off on this cutover. It took probing an
*accepted* protocol to see the problem at all.

The probe's own documented assumption — that payload size and handler dispatch
make no difference to call cost — was verified true on p2pd and is **false on
native**. That assumption is now wrong in the one place it mattered.

---

## 4. Mechanism — hypothesis, not established fact

Every measurement is consistent with **optimistic protocol negotiation being
absent on the native path**:

- **go-libp2p** sends the multistream-select proposal *and* the payload together.
  The peer confirms the protocol and returns the response in the same flight.
  **1 RTT.**
- **rust-libp2p** appears to propose, wait for confirmation, *then* send the
  payload and wait again. **2 RTT.**

That predicts exactly what we see: doubling on accepted calls, no change on
refusals, and no sensitivity to payload size.

**This has not been confirmed in code.** It was inferred from timings; nobody has
read the native unary send path to verify it. Confirming it — or finding the real
cause — is the first task, because the fix depends entirely on which it is. Note
also that this repo carries a **patched `multistream-select`**
(`patches/multistream-select`, 0.13.0 with protocol-name validation relaxed for
hivemind's slash-less names), which sits directly on this path and should be
ruled in or out.

---

## 5. Reproducing it

Two commands. Equal medians = healthy; accepted ≈ 2× refused = this bug.

```bash
# refused protocol — 1 RTT on any healthy stack
kwaainet p2p probe --peer <peer-id>

# accepted protocol — should cost the same, and does not on native
kwaainet p2p probe --peer <peer-id> --proto /kwaai/p2p/hello/1.0.0
```

Flip `kwaainet config set native_p2p true|false` and restart the daemon between
runs. `p2p probe` ships in #99.

---

## 6. What native did better

Recorded so this document is not mistaken for an argument against the migration.
It is one fixable defect in an otherwise good cutover.

- **NAT traversal is substantially better.** 16 live connections, **15 direct and
  1 relayed**, against p2pd's 3 direct / 3 relayed on the same machine minutes
  earlier. The native stack is finding direct paths that p2pd was not.
- **Circuit addresses are correct**, carrying both the relay and the destination
  peer ID — the §3.1 fix from the security analysis, visible in `p2p info`.
- **Identity survives the switch.** Same peer ID before and after, no re-announce
  churn.
- **No Go daemon spawns**, which is the point of the exercise.
- **DHT reads work**: `shard chain` returns 12 servers, 32/32 blocks.
- **The Tailscale interface is advertised** as a direct address, which p2pd did
  not do.

---

## 7. Recommendation

**Do not flip the default until this is understood.** The cutover is otherwise in
good shape, and the remaining work is small — the default flip itself, removing
the p2pd build and bundling (`core/Cargo.toml:246-248`,
`kwaai-p2p-daemon/build.rs`, `release.yml`), and a decision on the validator gap
(§2.1 of the security analysis, which the default flip makes universal).

Shipping the extra round trip would make 0.6 a release that removes a Go
dependency and, in exchange, halves remote inference throughput for everyone. The
diagnosis is cheap and the fix may be a configuration flag on the negotiation
layer. It is worth doing first.
