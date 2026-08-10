# Native-p2p bake-off: acceptance requirements

Two independent implementations replace the Go `p2pd` child process with an
in-process libp2p stack. This document fixes how they are compared **before the
second one arrives**, because the cheapest moment to be honest about a benchmark
is while you still do not know which way it will point.

Status: thresholds proposed 2026-08-10, **not yet ratified**. They become binding
once both authors have reviewed them — see §7.

---

## 1. The one rule that matters

**Performance does not decide this.** Correctness and observability do.

That is not a platitude, it is what the data says. Running the real workload —
a RAG graph build over p2p — the two stacks came out at 94.0 s and 99.3 s for
identical work, a gap fully explained by one arm extracting 19% more entities.
The workload is inference-bound; the transport is a rounding error on a
multi-second LLM call. A bake-off scored on that number would have been a coin
flip dressed up as evidence.

Meanwhile the thing that actually separated implementations was a **silent
correctness failure**: relay-only peers were invisible to one stack while the
control reached them fine. Nothing logged an error. Every aggregate looked
healthy. The first diagnosis blamed the remote peer, and was wrong.

So: performance is reported and can *veto* a candidate that is catastrophically
slow, but it never picks the winner.

## 2. Always run three arms

| arm | role |
|---|---|
| `p2pd` | **control** — the Go stack, the reference for "what is possible on this network right now" |
| candidate A | implementation under test |
| candidate B | implementation under test |

The control is not a competitor, it is the measuring stick. "Candidate A reaches
68% of the fleet" means nothing — the fleet has permanently unreachable peers.
"Candidate A reaches everything the control reaches, candidate B misses the
relay-only peers" is a decision.

Run all three **on one host, at one time**, distinguished only by
`KWAAINET_SOCKET` / `KWAAINET_HOME` / port. Sequential runs are not acceptable:
fleet composition, remote GPU load and NAT state all drift over tens of minutes.

`bakeoff.sh` interleaves at the innermost loop — every arm makes the same call to
the same peer back to back — and rotates arm order per round so no arm is
permanently first and therefore always paying the cold-path cost.

## 3. Acceptance criteria

### Veto conditions — any one fails the candidate

| # | Condition | Threshold | Why |
|---|---|---|---|
| V1 | **Peer visibility** vs control | misses **0** peers the control sees in steady state | The silent-degradation check. A stack that sees a smaller network looks healthy on every other metric. |
| V2 | **Reachability** of a peer the control reaches reliably | control ≥90% and candidate <50% on any peer ⇒ fail | Catches the relay-only class of bug. |
| V3 | **Memory growth** | second-half RSS slope ≤ **+5 MB/hour** over ≥60 min | Distinguishes warm-up from a leak. Reference: a clean run plateaus after ~17 min and slopes **−1.2 MB/h**. |
| V4 | **FD growth** | ≤ **+20** over the run | |
| V5 | **Survival** | node process alive for the whole soak | |
| V6 | **Fleet transparency** | same peer ID as on p2pd, `announceable=true`, relay reservations confirmed, no `p2pd` process left running | The rest of the fleet must not be able to tell. |
| V7 | **Windows control socket** | `control socket listening addr=/ip4/127.0.0.1/tcp/5005`, and `p2p peers list` returns rows from a separate client process | Never exercised by any test to date. |
| V8 | **Catastrophic latency** | >5× the control at the median on a peer both reach **directly** | The only way perf can fail a candidate. |

### Reported, never decisive

- Per-call latency by peer and payload size (64 B → 1 MiB), direct and relayed.
- Path quality: direct vs relay connection counts.
- Throughput under the standing load.
- RAG graph build wall-clock (expected to be indistinguishable — see §1).

### Judged by people, not by the script

These decide the bake-off when both candidates pass the vetoes, and no harness
can measure them:

1. **Hand-rolled surface area.** How much libp2p behaviour is reimplemented
   rather than used. The current stack hand-rolls AutoRelay because rust-libp2p
   has none; a candidate that avoids that carries a durable maintenance
   advantage no benchmark will show.
2. **Build portability.** Does it still need a Go toolchain? Does the patch
   apparatus survive on Windows without Git Bash? Both are open problems today.
3. **Failure legibility.** When it cannot reach a peer, does it say so? The bug
   found on 2026-08-10 was invisible precisely because the failure surfaced as a
   timeout rather than as "I refused this address".
4. **Review burden.** Diff size, stacking, test density, doc quality.

## 4. Equal search effort, not equal test suites

The subtlest way to get this wrong: run the same fixed suite against both, and
conclude the one with fewer findings is better. It may only be the one nobody
hunted through.

The relay-only bug was found because one fleet peer happened to be relay-only
that day. That was luck, not coverage.

Therefore:

- Budget **equal deliberate adversarial time** on each candidate, tracked in hours.
- **Cross-assign it**: each author tries to break the other's implementation, and
  specifically audits the other's address handling — the area that has produced
  every bug so far.
- Findings against either implementation get written up in the same format and
  count the same, regardless of who found them.
- Absence of found bugs is never reported as evidence of correctness.

## 5. Harness bias

`bakeoff.sh` was written against one implementation and could encode its
assumptions. Its coupling surface is deliberately narrow — it drives only
`kwaainet p2p peers send`, `p2p peers list` and `p2p info` through the control
socket, and reads `ps` / `/proc` for resources. It never touches internals.

That is fine **only if** both candidates sit behind the same `native_p2p` config
flag and the same p2pd control socket. If either does not, an adapter is needed,
and whoever writes it can tilt the result.

**Both authors must review this harness and sign off before any run counts.**
Objections to a metric are resolved by changing it in a commit *before* the run,
never by reinterpreting it after.

## 6. Run protocol

Setup — one host, three nodes, each with its own home, socket and port:

```bash
# arms.tsv:  label <TAB> socket <TAB> binary <TAB> node_pid
p2pd     default             /path/to/kwaainet          <pid>
candA    /tmp/kw-a.sock      /path/to/candA/kwaainet    <pid>
candB    /tmp/kw-b.sock      /path/to/candB/kwaainet    <pid>
```

Keep socket paths short — macOS caps unix socket paths at 104 bytes.

```bash
bash bakeoff.sh --arms arms.tsv --targets targets.tsv --rounds 240
python3 scorecard.py <out>/results.jsonl
```

Minimum for a run to count:

| | requirement |
|---|---|
| duration | ≥60 min continuous (V3 needs the second half to be meaningful) |
| targets | ≥10 live peers spanning **all three OSes**, including ≥1 relay-only peer |
| repeats | ≥3 independent runs at different times of day |
| platforms | the full matrix run on macOS, Linux **and** Windows hosts |

The relay-only peer is not optional. It is the single most informative target in
the fleet, and on 2026-08-10 the whole fleet had exactly one.

## 7. Ratification

Before the second PR lands:

- [ ] Both authors review `bakeoff.sh` and `scorecard.py` for bias
- [ ] Thresholds in §3 agreed, or amended by commit
- [ ] Target fleet agreed, including which peer is the relay-only one
- [ ] Adversarial time budget agreed and cross-assigned (§4)

After that, thresholds change only by commit with both authors' agreement, and
never while a result is pending.

## 8. Known limits of this harness

Stated plainly so nobody over-reads its output:

- **One call per process spawn.** Every sample pays CLI start-up (~10 ms,
  measured). It cancels in comparisons but inflates absolute latency, and it
  caps offered load — this measures correctness under sustained use, not peak
  throughput.
- **It cannot see inside the stack.** The 2-round-trip finding was inferred from
  latency fit plus module docs, not from a packet capture. Treat mechanism
  claims as hypotheses until someone traces them.
- **Fleet composition is uncontrolled.** Peers join and leave mid-run; that is
  why the control arm exists and why ≥3 runs are required.
- **A PASS is necessary, not sufficient.** §3's human-judged axes still decide.
