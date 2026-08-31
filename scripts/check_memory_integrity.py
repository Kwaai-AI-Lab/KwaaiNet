#!/usr/bin/env python3
"""Check that CLAUDE.md and other memory files still describe the code.

Why this exists, and why the pai-seed growth cycle alone does not cover it:

A Personal AI's artifacts describe its Principal — vocabulary and priorities that
change only when the Principal changes, so a closing reflection catches drift.
A repository's CLAUDE.md files describe *code*, which changes without anyone
opening the document. Between 2026-05-23 and 2026-08-26 half of ours went
54-94 days without an edit while the code they describe kept moving, and
`kwaai-p2p/CLAUDE.md` still points at `src/network.rs` and `src/hivemind.rs`,
deleted in a refactor. No amount of reflection finds that; a compiler does.

Reflection handles accretion (too much, never pruned). Verification handles
staleness (claims that were true and no longer are). We need both.

PLANS ARE CHECKED DIFFERENTLY. `projects/*/plans/*.md` are covered too, because
that is where the same failure recurred: on 2026-08-31 `PerKBOntology-plan.md`
still said "not yet started" after the branch it plans had merged, and seven of
its `file.rs:LINE` pointers had drifted. But a plan is a claim about the
*future*, so an unresolvable path is often a file the plan proposes to create,
not a broken reference. Applying the CLAUDE.md rules to plans unmodified
produced 42 errors, almost all of them legitimate proposals. So:

  * for plans, unresolvable file/function refs are WARNINGS, not errors;
  * a `file.rs:N` whose named symbol has moved is a WARNING with the corrected
    line printed, and `--fix` rewrites them. It is deliberately not a gate:
    line numbers are branch-relative, so inserting ten lines near the top of
    graph.rs stales every pointer to it in docs that PR never opened. Erroring
    there would make most Rust PRs carry unrelated doc churn, which is how a
    check gets switched off. Past-EOF stays an ERROR — nothing but real rot
    causes it;
  * a pointer that lands *inside* the named function's body is left alone. Docs
    write "Round 2 gap-fill in `retrieve_iterative()` (`iterative.rs:222`)" to
    mean line 222, not the signature; "correcting" that to the definition line
    destroys the only thing the pointer carried;
  * a plan claiming "not yet started/run" while the code it describes has since
    moved is a WARNING — the exact shape of the 2026-08-31 miss;
  * age-drift is not reported for plans at all. A results document is supposed
    to be old; warning about it is the noise that teaches people to skim.

Exit non-zero when a file reference does not resolve, so CI can gate it.

Usage: scripts/check_memory_integrity.py [--fix | --self-test]
       --fix rewrites stale `file.rs:LINE` pointers to where the symbol now is.
"""
import os, re, subprocess, sys, time

ROOT = subprocess.run(["git", "rev-parse", "--show-toplevel"],
                      capture_output=True, text=True).stdout.strip()
STALE_DAYS = 60  # a doc untouched this long while its code moved is a warning
ATTR_SLACK = 2   # a line ref may sit on the attribute/doc line above a definition

# Placeholders in "how to add a thing" templates, not real claims.
PLACEHOLDER = re.compile(r"^(x_|foo|bar|example|your_|<)", re.I)

def tracked(pattern):
    return subprocess.run(["git", "ls-files", pattern], cwd=ROOT,
                          capture_output=True, text=True).stdout.split()

def last_commit_epoch(path):
    out = subprocess.run(["git", "log", "-1", "--format=%ct", "--", path],
                         cwd=ROOT, capture_output=True, text=True).stdout.strip()
    return int(out) if out else None

# Which source tree each memory file is a claim about.
def described_source(md):
    if md.startswith("core/crates/"):
        return os.path.join(os.path.dirname(md), "src")
    m = re.match(r"projects/kwaai-(\w+)/", md)
    if m:
        return {"knowledge": "core/crates/kwaai-rag/src",
                "network":   "core/crates/kwaai-p2p/src",
                "compute":   "core/crates/kwaai-inference/src",
                "storage":   "core/crates/kwaai-storage/src",
                "trust":     "core/crates/kwaai-trust/src",
                "platform":  "core/crates/kwaai-cli/src"}.get(m.group(1))
    return "core/crates"

FOREIGN_HEADING = re.compile(
    r"(?im)^#{1,6}[^\n]*\b(separate repo|different repo|other repo|external repo|"
    r"not this one|PHE repo|upstream repo)\b[^\n]*$")

def strip_foreign_sections(txt):
    """Drop sections whose heading says they describe another repository."""
    out, skipping, skip_level = [], False, 0
    for line in txt.splitlines(keepends=True):
        h = re.match(r"^(#{1,6})\s", line)
        if h:
            level = len(h.group(1))
            if skipping and level <= skip_level:
                skipping = False
            if FOREIGN_HEADING.match(line):
                skipping, skip_level = True, level
                continue
        if not skipping:
            out.append(line)
    return "".join(out)

def is_plan(md):
    return "/plans/" in md

# `file.rs:123` — a pointer precise enough to go stale on its own.
LINE_REF = re.compile(r"`([\w./-]+\.rs):(\d+)`")
# A backticked symbol: `foo`, `foo()`, `foo(a, b)`, `Type::method`, `Type::method(x)`.
# Path qualification matters — half these docs write `GraphStore::link_chunk(...)`,
# and an earlier version that could not match it skipped 31 of 60 refs in silence.
BT_IDENT = re.compile(r"`((?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]{3,})"
                      r"(?:\([^`]*\))?`")
# How far back on the line an anchor may sit. A pointer is written next to the
# symbol it points at; anything further is a different clause or table cell.
ANCHOR_SPAN = 60

def rust_path(ref, ident=None):
    """Resolve a doc's `some/dir/graph.rs` (or bare `graph.rs`) to one source path.

    Prefers a file whose path actually ends with what the doc wrote, so a stale
    directory is not silently validated against a same-named file elsewhere.
    A bare basename can be ambiguous — four crates have a `config.rs` — in which
    case the anchored symbol breaks the tie, since only one of them defines it.
    Still returns None when nothing distinguishes the candidates: validating a
    line number against the wrong file is worse than not checking it."""
    base = os.path.basename(ref)
    hits = [p for p in subprocess.run(["git", "ls-files", f"*{base}"], cwd=ROOT,
                                      capture_output=True, text=True).stdout.split()
            if p.endswith("/" + base) and "/src/" in p]
    if "/" in ref:
        exact = [p for p in hits if p.endswith(ref.lstrip("./"))]
        if len(exact) == 1:
            return exact[0]
    if len(hits) > 1 and ident:
        defining = [p for p in hits
                    if def_span(open(os.path.join(ROOT, p)).read().splitlines(), ident)]
        if len(defining) == 1:
            return defining[0]
    return hits[0] if len(hits) == 1 else None

DEF_ANY = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?"
                     r"(?:const|static|fn|struct|enum|trait|type|mod)\s+")

def def_span(src, ident):
    """(definition line, last line of its body) for `ident`, 1-indexed, or None.

    The body end matters: docs point *inside* a function as often as at it —
    "Round 2 gap-fill in `retrieve_iterative()` (`iterative.rs:222`)" means line
    222, not the signature. Treating that as a stale pointer to the definition
    and "correcting" it destroys the only information the pointer carried."""
    name = ident.split("::")[-1]
    pat = re.compile(DEF_ANY.pattern + re.escape(name) + r"\b")
    hits = [i for i, l in enumerate(src) if pat.match(l)]
    if len(hits) != 1:
        return None
    d = hits[0]
    for j in range(d + 1, len(src)):
        if DEF_ANY.match(src[j]) and not src[j].startswith(" " * 8):
            return d + 1, j
    return d + 1, len(src)

def anchored_refs(lines, li):
    """Yield (ref_base, ref_line, ident) for line `li`, pairing each pointer with
    the nearest backticked symbol before it.

    Falls back to the previous line's trailing symbol, because these docs wrap:
    ``and `extract_entity_centric`\n(`ingestion.rs:1021`)`` put the pointer and
    its symbol on different physical lines, and a per-line rule skipped it."""
    line = lines[li]
    refs = [(m.start(), m.end(), m.group(1), int(m.group(2)))
            for m in LINE_REF.finditer(line)]
    if not refs:
        return
    idents = [(m.start(), m.group(1)) for m in BT_IDENT.finditer(line)]
    prev = []
    if li:
        p_ids = [(m.start(), m.group(1)) for m in BT_IDENT.finditer(lines[li - 1])]
        p_refs = [m.start() for m in LINE_REF.finditer(lines[li - 1])]
        # Only a symbol left dangling at the end of the previous line — after any
        # pointer of its own — belongs to a pointer that wrapped onto this one.
        if p_ids and (not p_refs or p_ids[-1][0] > p_refs[-1]):
            prev = p_ids
    used = set()
    for start, end, base, n in refs:
        near = [i for i in idents
                if i[0] < start and i[1] not in used and start - i[0] <= ANCHOR_SPAN]
        ident = near[-1][1] if near else (prev[-1][1] if prev and start <= ANCHOR_SPAN
                                          else None)
        if ident is None:
            continue
        used.add(ident)
        yield start, end, base, n, ident

def check_line_refs(txt):
    """Pointers that no longer land on what the prose says they land on.

    A warning, not an error. Line numbers are branch-relative: inserting ten
    lines near the top of graph.rs stales every pointer to it in docs that PR
    never opened, and gating CI on that means unrelated PRs carry doc churn —
    which is how a check gets switched off. `--fix` is the remedy instead.
    Past-EOF stays an error: it cannot be caused by anything but real rot."""
    errs, warns = [], []
    lines = txt.splitlines()
    for li in range(len(lines)):
        for start, end, base, n, ident in anchored_refs(lines, li):
            path = rust_path(base, ident)
            if not path:
                continue
            src = open(os.path.join(ROOT, path)).read().splitlines()
            if n > len(src):
                errs.append(f"`{base}:{n}` — past end of file ({len(src)} lines)")
                continue
            sp = def_span(src, ident)
            if not sp:
                continue
            d, body_end = sp
            # Inside the named symbol's body: a deliberate intra-function pointer.
            if d <= n <= body_end:
                continue
            if abs(d - n) > ATTR_SLACK:
                warns.append(f"`{base}:{n}` — `{ident}` is defined at {base}:{d}")
    return sorted(set(errs)), sorted(set(warns))

NOT_YET = re.compile(r"(?im)^\s*(?:\*\*)?status(?:\*\*)?\s*[:\uff1a][^\n]*?"
                     r"\bnot yet (started|run|built|implemented|done|committed|"
                     r"merged|landed)\b")

def check(md):
    """Return (errors, warnings) for one memory file."""
    errs, warns = [], []
    txt = open(os.path.join(ROOT, md)).read()

    # Sections about a *different* repository are not claims about this one.
    # kwaai-storage documents changes needed in the PHE repo under its own
    # heading; flagging those as broken would train people to ignore the check.
    txt = strip_foreign_sections(txt)

    # A plan names things it intends to build. An unresolvable path there is a
    # proposal, not a broken claim, so refs are demoted to warnings — see the
    # module docstring. The line-anchor check below stays an error for both.
    plan = is_plan(md)
    ref_out = warns if plan else errs

    # 1. Every `path/to/file.rs` in backticks must exist somewhere in the tree.
    for ref in sorted(set(re.findall(r"`([\w./-]+\.(?:rs|py|sh|md|yaml|toml))`", txt))):
        base = os.path.basename(ref)
        if PLACEHOLDER.match(base):
            continue
        direct = os.path.join(ROOT, ref)
        if os.path.exists(direct):
            continue
        found = subprocess.run(
            ["git", "ls-files", f"*{base}"], cwd=ROOT,
            capture_output=True, text=True).stdout.strip()
        if not found:
            ref_out.append(f"file reference does not resolve: `{ref}`"
                           + (" (proposed?)" if plan else ""))
        elif "/" in ref and not any(
                r.endswith(ref) for r in found.splitlines()):
            ref_out.append(f"`{ref}` — no file at that path (moved? found "
                           f"{found.splitlines()[0]})")

    # 2. Every `fn name()` claim must exist.
    for fn in sorted(set(re.findall(r"`([a-z_][a-z0-9_]{4,})\(\)`", txt))):
        if PLACEHOLDER.match(fn):
            continue
        rc = subprocess.run(["grep", "-rq", f"fn {fn}", "core/crates",
                             "--include=*.rs"], cwd=ROOT).returncode
        if rc != 0:
            ref_out.append(f"function reference does not resolve: `{fn}()`"
                           + (" (proposed?)" if plan else ""))

    # 3. `file.rs:N` pointers whose symbol has since moved within that file.
    ref_errs, ref_warns = check_line_refs(txt)
    errs += ref_errs
    warns += ref_warns

    src = described_source(md)
    src_t = last_commit_epoch(src) if src and os.path.isdir(os.path.join(ROOT, src)) else None
    md_t = last_commit_epoch(md)

    # 4. A plan still claiming "not yet started" after its subject moved on.
    #    This is what went unnoticed on 2026-08-31; age alone would not catch it,
    #    the plan was six days old.
    if plan and md_t and src_t:
        m = NOT_YET.search(txt)
        if m and src_t > md_t:
            warns.append(f"status says \"not yet {m.group(1)}\" but {src} has "
                         f"changed since — recheck whether it has landed")

    # 5. Drift: the doc has not been touched while its subject kept moving.
    #    Not for plans: a results document is meant to be old, and warning about
    #    that is the noise that trains people to skim.
    if not plan and md_t and src_t:
        days = (src_t - md_t) / 86400
        if days > STALE_DAYS:
            warns.append(f"{days:.0f} days of drift — {src} changed "
                         f"{time.strftime('%Y-%m-%d', time.localtime(src_t))}, "
                         f"this doc last {time.strftime('%Y-%m-%d', time.localtime(md_t))}")
    return errs, warns

def fix_line_refs(files):
    """Rewrite every `file.rs:N` that check_line_refs would flag.

    Line numbers are branch-relative: a PR that inserts lines above a definition
    stales every pointer to it, in docs it never touched. That is why the check
    is a warning rather than a gate — and why the remedy has to be one command
    rather than a manual hunt. Past-EOF refs are left alone: there is no correct
    line to move them to, and guessing one would be worse than the error."""
    fixed = 0
    for md in files:
        path = os.path.join(ROOT, md)
        lines = open(path).read().splitlines(keepends=True)
        touched = False
        for li in range(len(lines)):
            edits = []
            for start, end, base, n, ident in anchored_refs(lines, li):
                src_path = rust_path(base, ident)
                if not src_path:
                    continue
                src = open(os.path.join(ROOT, src_path)).read().splitlines()
                if n > len(src):
                    continue
                sp = def_span(src, ident)
                if not sp:
                    continue
                d, body_end = sp
                if d <= n <= body_end or abs(d - n) <= ATTR_SLACK:
                    continue
                edits.append((start, end, f"`{base}:{d}`",
                              f"{md}: {base}:{n} -> {d} ({ident})"))
            line = lines[li]
            for start, end, new, note in reversed(edits):
                line = line[:start] + new + line[end:]
                print(f"  fixed {note}")
                fixed += 1
                touched = True
            lines[li] = line
        if touched:
            open(path, "w").write("".join(lines))
    return fixed

def self_test():
    """Regression cover for the plan-checking behaviours, per the standing rule
    that a fix ships with a test. Synthetic fixtures only — touches no repo file.

    Every case is a bug that actually occurred while building this; the last
    five came out of the review of PR #164."""
    import tempfile
    global rust_path
    real_rust_path = rust_path
    fails = []

    def case(name, cond):
        if not cond:
            fails.append(name)

    def W(txt):   # warnings only
        return check_line_refs(txt)[1]

    def E(txt):   # errors only
        return check_line_refs(txt)[0]

    with tempfile.TemporaryDirectory() as d:
        src = ["// pad"] * 60
        src[4]  = "pub fn alpha() -> u8 { 0 }"          # line 5, body runs to 39
        src[19] = "    let parse_headers = 1;"          # line 20, inside alpha
        src[39] = "pub const BETA: &[&str] = &[];"      # line 40
        src[44] = "pub fn link_chunk(&self) {}"         # line 45
        fake = os.path.join(d, "fake.rs")
        open(fake, "w").write("\n".join(src) + "\n")
        rust_path = lambda base, ident=None: (fake if os.path.basename(base) == "fake.rs"
                                              else None)

        # 1. Two pointers on one line, each already correct: no cross-pairing.
        case("cross-pairing", W("`alpha` (`fake.rs:5`) and `BETA` (`fake.rs:40`)") == [])

        # 2. A stale pointer is reported, with the corrected line.
        w = W("`BETA` (`fake.rs:12`)")
        case("stale ref detected", len(w) == 1 and "fake.rs:40" in w[0])

        # 3. A pointer on the `#[derive]`/doc line just above a definition.
        case("attribute slack", W("`BETA` (`fake.rs:39`)") == [])

        # 4. Past end of file is an error, not a warning.
        case("past EOF", any("past end of file" in e for e in E("`alpha` (`fake.rs:900`)")))

        # 5. Nothing to anchor to: stay silent rather than guess.
        case("no anchor", W("see `fake.rs:12` for context") == [])

        # 6. A path-qualified symbol must anchor. Failing this skipped 31 of 60
        #    real refs in silence, including every `Type::method()` in the plans.
        w = W("`GraphStore::link_chunk(chunk_id)` (`fake.rs:12`)")
        case("path-qualified anchor", len(w) == 1 and "fake.rs:45" in w[0])

        # 7. A pointer *inside* the named function's body is deliberate.
        #    `--fix` used to rewrite it to the signature, destroying the only
        #    thing it carried. Two real pointers were lost that way.
        case("intra-function pointer kept",
             W("Round 2 gap-fill in `alpha()` (`fake.rs:20`)") == [])

        # 8. An identifier ending in "rs" is a symbol, not a filename. Filtering
        #    those produced false errors and corrupting rewrites.
        case("ident ending in rs",
             W("`alpha` calls `parse_headers` (`fake.rs:20`)") == [])

        # 9. An anchor across a table-cell boundary is not an anchor. The real
        #    case: PerKBOntology-plan.md anchors a `dream.rs` pointer to a
        #    `graph.rs` symbol 72 characters earlier, in a different cell.
        far = "| `alpha` | 5 | extraction prompt, plus validation notes and " \
              "other prose filling the cell | see `fake.rs:40` |"
        case("anchor distance bounded", W(far) == [])

        # 10. A pointer wrapped onto the next line still anchors.
        w = check_line_refs("and `BETA`\n(`fake.rs:12`) matters")[1]
        case("wrapped-line anchor", len(w) == 1 and "fake.rs:40" in w[0])

        rust_path = real_rust_path

    # 11. The status claims that went stale, in the forms the plans use.
    for form in ["Status: **plan of record, not yet started.** Written 2026-08-25.",
                 "Status: **for review, not yet run.** Written 2026-08-25.",
                 "**Status**: code complete, not yet committed."]:
        case(f"status: {form[:34]}", bool(NOT_YET.search(form)))
    case("status: a finished plan is not flagged",
         not NOT_YET.search("Status: **phases 0-2 landed** in #150."))

    # 12. Plans and memory files are told apart — the severity split needs it.
    case("is_plan", is_plan("projects/kwaai-knowledge/plans/X.md")
                    and not is_plan("projects/kwaai-knowledge/CLAUDE.md"))

    for f in fails:
        print(f"  FAIL  {f}")
    print(f"\nself-test: {len(fails)} failure(s)")
    return 1 if fails else 0

def main():
    files = (tracked("*CLAUDE.md") + tracked("projects/*/GLOSSARY.md")
             + tracked("projects/*/plans/*.md"))
    if "--self-test" in sys.argv:
        return self_test()
    if "--fix" in sys.argv:
        n = fix_line_refs(sorted(files))
        print(f"\n{n} line reference(s) rewritten. Re-run without --fix to verify.")
        return 0
    n_err = n_warn = 0
    print(f"Checking {len(files)} memory and plan files\n")
    for md in sorted(files):
        errs, warns = check(md)
        if not errs and not warns:
            continue
        print(f"  {md}")
        for e in errs:
            print(f"    ERROR   {e}")
        for w in warns:
            print(f"    warn    {w}")
        n_err += len(errs); n_warn += len(warns)
    print(f"\n{n_err} broken reference(s), {n_warn} warning(s) "
          f"across {len(files)} files")
    if n_warn and not n_err:
        print("\nDrift is a prompt to re-read, not a failure. Reflect on whether the "
              "doc still says something true, then touch it or fix it.")
    return 1 if n_err else 0

if __name__ == "__main__":
    sys.exit(main())
