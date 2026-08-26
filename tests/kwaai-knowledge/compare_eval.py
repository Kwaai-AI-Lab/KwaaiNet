#!/usr/bin/env python3
"""Compare two eval reports per question, not just on the headline.

The 40-question set saturates near 90% and is token-overlap scored, so an
aggregate delta can hide real movement in both directions. This reports the
questions that actually changed and, separately, the subset the ontology was
built to affect.

Usage: compare_eval.py CONTROL.md ONTOLOGY.md
"""
import re, sys

def parse(path):
    txt = open(path).read()
    overall = re.search(r"Overall recall \(token-overlap\) \| ([\d.]+)% \(([\d.]+)/(\d+)\)", txt)
    rows = {}
    for m in re.finditer(r"^\| (q\d+) \| (.+?) \| (\d+)/(\d+) \((\d+)%\) \|", txt, re.M):
        rows[m.group(1)] = {"q": m.group(2).strip(), "hit": int(m.group(3)),
                            "tot": int(m.group(4)), "pct": int(m.group(5))}
    return (float(overall.group(1)) if overall else None,
            float(overall.group(2)) if overall else None,
            int(overall.group(3)) if overall else None, rows)

# Questions the ontology should plausibly reach, and why.
TARGETED = {
    "q11": "TLSA — PoliticalOrganization",
    "q13": "All Africa Convention — PoliticalOrganization",
    "q17": "Hewat — EducationalInstitution",
    "q18": "New Era Fellowship — PoliticalOrganization",
    "q19": "NEUM — PoliticalOrganization",
    "q20": "cricket — SportsClub / played_for",
    "q06": "Buitencingle — Address",
    "q10": "Kloof Nek — Address",
    "q29": "TLSA↔NEUM — affiliated_with",
    "q31": "mosque — Venue",
    "q36": "Coloured community orgs — RacialClassification",
    "q40": "boycott policy — Doctrine",
}

def main():
    ca, ch, ct, cr = parse(sys.argv[1])
    oa, oh, ot, orr = parse(sys.argv[2])
    print(f"headline recall   control {ca}%   ontology {oa}%   "
          f"delta {oa-ca:+.1f}pp" if ca and oa else "headline unavailable")
    print(f"tokens hit        {ch:.0f}/{ct}      {oh:.0f}/{ot}\n")

    moved = [(k, cr[k], orr[k]) for k in sorted(cr) if k in orr
             and cr[k]["pct"] != orr[k]["pct"]]
    ups = [x for x in moved if x[2]["pct"] > x[1]["pct"]]
    downs = [x for x in moved if x[2]["pct"] < x[1]["pct"]]
    print(f"{len(moved)}/{len(cr)} questions changed — {len(ups)} up, {len(downs)} down\n")
    for title, group in (("IMPROVED", ups), ("REGRESSED", downs)):
        if not group:
            continue
        print(title)
        for k, c, o in sorted(group, key=lambda x: -(x[2]["pct"] - x[1]["pct"])):
            tag = "  ←targeted" if k in TARGETED else ""
            print(f"  {k}  {c['pct']:>3}% → {o['pct']:>3}%   {c['q'][:52]}{tag}")
        print()

    print("TARGETED SUBSET — questions the ontology was built to reach")
    tc = to = n = 0
    for k, why in TARGETED.items():
        if k in cr and k in orr:
            n += 1; tc += cr[k]["pct"]; to += orr[k]["pct"]
            d = orr[k]["pct"] - cr[k]["pct"]
            mark = "  " if d == 0 else ("↑ " if d > 0 else "↓ ")
            print(f"  {mark}{k}  {cr[k]['pct']:>3}% → {orr[k]['pct']:>3}%   {why}")
    if n:
        print(f"\n  targeted mean: {tc/n:.1f}% → {to/n:.1f}%  ({to/n - tc/n:+.1f}pp)")
        rest = [k for k in cr if k not in TARGETED and k in orr]
        rc = sum(cr[k]["pct"] for k in rest)/len(rest)
        ro = sum(orr[k]["pct"] for k in rest)/len(rest)
        print(f"  the rest:      {rc:.1f}% → {ro:.1f}%  ({ro-rc:+.1f}pp)")

if __name__ == "__main__":
    main()
