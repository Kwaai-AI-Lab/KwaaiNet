#!/usr/bin/env python3
"""Turn a bake-off results.jsonl into a scorecard with pass/fail verdicts.

Thresholds live in THRESHOLDS below and are meant to be agreed *before* the
candidate PRs land — see BAKEOFF.md. Change them in a commit, not in the middle
of an evaluation.

Usage:  python3 scorecard.py results.jsonl [--control p2pd]
"""

import argparse
import collections
import json
import statistics as st
import sys

THRESHOLDS = {
    # VETO: a peer the control reaches reliably that a candidate almost never
    # reaches. This is the relay-only class of failure — invisible on averages.
    "veto_control_reach": 0.90,
    "veto_candidate_reach": 0.50,
    # WARN: meaningfully worse than the control without being a hard failure.
    "warn_reach_gap_pp": 10.0,
    # VETO: peers the control sees in steady state that the candidate does not.
    # Silent degradation — the candidate looks healthy on every other metric.
    "veto_missing_peers": 0,
    # VETO: sustained memory growth. Measured on the second half only, so
    # start-up warm-up does not count against an arm.
    "veto_rss_slope_mb_per_h": 5.0,
    "veto_fd_growth": 20,
    # A slope extrapolated from a few minutes is noise wearing a units label —
    # a 36s smoke run happily reports "+995 MB/h". Below this, report the
    # numbers but never veto on them.
    "min_resource_minutes": 30,
    # REPORT ONLY: latency is not allowed to decide the bake-off, but a large
    # regression against the control is worth surfacing.
    "flag_latency_ratio": 2.0,
    # Fraction of rounds treated as "steady state" for visibility/latency.
    "steady_tail": 0.5,
}


def load(path):
    rows = []
    with open(path) as fh:
        for line in fh:
            line = line.strip()
            if not line:
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError:
                pass  # a truncated final line on a killed run is not fatal
    return rows


def pct(x):
    return f"{100 * x:.0f}%"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("results")
    ap.add_argument("--control", default=None,
                    help="arm label to treat as the reference (default: from the run's meta line)")
    args = ap.parse_args()

    rows = load(args.results)
    if not rows:
        sys.exit("no usable rows")

    meta = next((r for r in rows if r.get("kind") == "meta"), {})
    arms = meta.get("arms") or sorted({r["arm"] for r in rows if "arm" in r})
    control = args.control or meta.get("control") or arms[0]
    if control not in arms:
        sys.exit(f"control arm {control!r} not present; arms are {arms}")
    candidates = [a for a in arms if a != control]

    rounds = max((r.get("round", 0) for r in rows), default=0)
    tail_from = int(rounds * (1 - THRESHOLDS["steady_tail"]))

    print(f"\nBAKE-OFF SCORECARD   control={control}  candidates={', '.join(candidates)}")
    print(f"rounds={rounds}  host={meta.get('host_os','?')}  "
          f"targets={meta.get('targets','?')}\n")

    vetoes = collections.defaultdict(list)

    # -- 1. VISIBILITY --------------------------------------------------------
    # Steady-state peer sets. A candidate that cannot see what the control sees
    # is running on a smaller network than it appears to be.
    vis = collections.defaultdict(list)
    for r in rows:
        if r.get("kind") == "visibility" and r.get("round", 0) >= tail_from:
            peers = {p for p in (r.get("peers") or "").split(",") if p}
            vis[r["arm"]].append(peers)

    # Every arm is a peer of every other arm but never of itself, so the arms'
    # own peer ids must come out of the comparison or each one looks like it is
    # "missing" all the others.
    self_peers = {p for p in (meta.get("self_peers") or []) if p and p != "unknown"}

    print("VISIBILITY  (steady state; peers each arm has a live connection to)")
    if self_peers:
        print(f"  (excluding the {len(self_peers)} arm-local peer ids from the comparison)")
    ctl_seen = (set().union(*vis[control]) if vis.get(control) else set()) - self_peers
    # count a peer as "consistently seen" if present in most steady-state samples
    ctl_consistent = {p for p in ctl_seen
                      if sum(p in s for s in vis[control]) >= max(1, len(vis[control]) // 2)}
    print(f"  {control:<14s} sees {len(ctl_consistent)} peers consistently  (reference)")
    for a in candidates:
        seen = (set().union(*vis[a]) if vis.get(a) else set()) - self_peers
        missing = ctl_consistent - seen
        extra = seen - ctl_seen
        verdict = "OK" if len(missing) <= THRESHOLDS["veto_missing_peers"] else "VETO"
        if verdict == "VETO":
            vetoes[a].append(f"cannot see {len(missing)} peer(s) the control sees: "
                             f"{', '.join(sorted(p[:16] + '…' for p in missing))}")
        print(f"  {a:<14s} sees {len(seen):3d}  missing-vs-control={len(missing):2d}  "
              f"extra={len(extra):2d}  [{verdict}]")

    # -- 2. REACHABILITY ------------------------------------------------------
    reach = collections.defaultdict(lambda: [0, 0])   # (arm,target) -> [ok,total]
    errs = collections.defaultdict(collections.Counter)
    lat = collections.defaultdict(list)
    for r in rows:
        if r.get("kind") != "rpc":
            continue
        k = (r["arm"], r["target"])
        reach[k][1] += 1
        if r["ok"]:
            reach[k][0] += 1
            lat[k].append(r["ms"])
        elif r.get("err"):
            errs[k][r["err"][:44]] += 1

    targets = sorted({t for (_, t) in reach}, key=str)
    # An arm cannot call itself, so if one arm's node is also in the target list
    # its own row is not a failure — blank it rather than scoring it 0%.
    own = dict(zip(arms, meta.get("self_peers") or []))
    tgt_peer = {r["target"]: r["peer"] for r in rows if r.get("kind") == "rpc"}
    print("\nREACHABILITY  (per-peer unary success; the control defines what is possible)")
    hdr = f"  {'peer':26s} {control[:10]:>10s}"
    for a in candidates:
        hdr += f" {a[:12]:>12s}"
    print(hdr)
    for t in targets:
        c_ok, c_n = reach.get((control, t), [0, 0])
        c_is_self = own.get(control) and tgt_peer.get(t) == own.get(control)
        c_rate = c_ok / c_n if c_n else 0.0
        line = f"  {t[:26]:26s} {('self' if c_is_self else pct(c_rate)):>10s}"
        for a in candidates:
            ok, n = reach.get((a, t), [0, 0])
            rate = ok / n if n else 0.0
            mark = ""
            if own.get(a) and tgt_peer.get(t) == own.get(a):
                line += f" {'self':>12s}"
                continue
            if c_is_self:
                # no usable reference for this peer — report, never veto
                line += f" {pct(rate):>12s}"
                continue
            if c_rate >= THRESHOLDS["veto_control_reach"] and rate < THRESHOLDS["veto_candidate_reach"]:
                mark = " !!"
                top = errs[(a, t)].most_common(1)
                vetoes[a].append(
                    f"unreachable peer {t} ({pct(rate)} vs control {pct(c_rate)})"
                    + (f" — {top[0][0]}" if top else ""))
            elif (c_rate - rate) * 100 > THRESHOLDS["warn_reach_gap_pp"]:
                mark = " ?"
            line += f" {pct(rate):>12s}{mark}"
        print(line)

    # -- 3. RESOURCES ---------------------------------------------------------
    print("\nRESOURCES  (second-half slope only, so warm-up is not charged to an arm)")
    res = collections.defaultdict(list)
    for r in rows:
        if r.get("kind") == "resource" and r.get("alive"):
            res[r["arm"]].append(r)
    for a in arms:
        s = sorted(res.get(a, []), key=lambda x: x["t"])
        if not s:
            print(f"  {a:<14s} (no samples — node_pid was '-' in the arms file?)")
            continue
        if len(s) < 4:
            print(f"  {a:<14s} ({len(s)} sample(s) — too few to say anything; "
                  f"needs a longer run)")
            continue
        died = any(r.get("alive") is False for r in rows
                   if r.get("kind") == "resource" and r.get("arm") == a)
        half = s[len(s) // 2:]
        span_min = (s[-1]["t"] - s[0]["t"]) / 60
        hours = (half[-1]["t"] - half[0]["t"]) / 3600 or 1e-9
        slope = ((half[-1]["rss_kb"] - half[0]["rss_kb"]) / 1024) / hours
        fd_growth = s[-1]["fds"] - s[0]["fds"]
        long_enough = span_min >= THRESHOLDS["min_resource_minutes"]
        bad = []
        if a != control and long_enough:
            if slope > THRESHOLDS["veto_rss_slope_mb_per_h"]:
                bad.append(f"RSS +{slope:.1f} MB/h")
            if fd_growth > THRESHOLDS["veto_fd_growth"]:
                bad.append(f"FD +{fd_growth}")
        if died:
            bad.append("process died mid-run")
        for b in bad:
            vetoes[a].append(b)
        if bad:
            status = "VETO: " + "; ".join(bad)
        elif not long_enough:
            status = f"too short to judge ({span_min:.0f}min < "
            status += f"{THRESHOLDS['min_resource_minutes']}min)"
        else:
            status = "OK"
        print(f"  {a:<14s} RSS {s[0]['rss_kb']/1024:6.1f} -> {s[-1]['rss_kb']/1024:6.1f} MB  "
              f"slope {slope:+6.1f} MB/h  FD {s[0]['fds']:3d} -> {s[-1]['fds']:3d}  "
              f"threads {s[-1]['threads']:3d}  [{status}]")

    # -- 4. LATENCY (report only) --------------------------------------------
    print("\nLATENCY  (median ms per call; REPORT ONLY — must not decide the bake-off)")
    hdr = f"  {'peer':26s} {control[:10]:>10s}"
    for a in candidates:
        hdr += f" {a[:12]:>12s}"
    print(hdr)
    for t in targets:
        c = lat.get((control, t), [])
        line = f"  {t[:26]:26s} {(f'{st.median(c):.0f}' if c else '-'):>10s}"
        for a in candidates:
            v = lat.get((a, t), [])
            cell = f"{st.median(v):.0f}" if v else "-"
            if v and c and st.median(v) > st.median(c) * THRESHOLDS["flag_latency_ratio"]:
                cell += " ~"
            line += f" {cell:>12s}"
        print(line)
    print(f"  (~ = slower than {THRESHOLDS['flag_latency_ratio']}x the control; a flag, not a failure)")

    # -- VERDICT --------------------------------------------------------------
    print("\n" + "=" * 72)
    for a in candidates:
        if vetoes[a]:
            print(f"  {a}: FAIL")
            for v in dict.fromkeys(vetoes[a]):
                print(f"      - {v}")
        else:
            print(f"  {a}: PASS (no veto condition triggered)")
    print("=" * 72)
    print("A PASS here is necessary, not sufficient. The axes this cannot measure —\n"
          "hand-rolled surface area, build portability, review burden — are in\n"
          "BAKEOFF.md and are decided by people, not by this script.\n")


if __name__ == "__main__":
    main()
