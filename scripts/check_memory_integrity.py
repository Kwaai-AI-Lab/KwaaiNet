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
  * a `file.rs:N` whose named symbol is *defined in that same file at another
    line* is an ERROR everywhere — no reading of that is a proposal, and the
    corrected line number is printed;
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
# Backticked identifiers on the same line, including `foo(...)` and `foo(a, b)`.
BT_IDENT = re.compile(r"`([A-Za-z_][A-Za-z0-9_]{3,})(?:\([^`]*\))?`")

def rust_path(base):
    """Resolve a bare `graph.rs` to its one source path, or None if ambiguous."""
    hits = [p for p in subprocess.run(["git", "ls-files", f"*{os.path.basename(base)}"],
                                      cwd=ROOT, capture_output=True, text=True).stdout.split()
            if p.endswith("/" + os.path.basename(base)) and "/src/" in p]
    return hits[0] if len(hits) == 1 else None

DEF = (r"(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?"
       r"(?:const|static|fn|struct|enum|trait|type|mod)\s+")

def check_line_refs(txt):
    """`file.rs:N` where the symbol named alongside it is defined elsewhere in
    that same file. Mechanically false — a proposal cannot be at a wrong line in
    a file that already defines it — so this is an error even in a plan.

    These docs write the pointer as ``ident` (`file.rs:N`)``, so each ref is
    paired with the nearest identifier *before* it rather than with every
    identifier on the line. Without that, a line naming two symbols and two
    line numbers cross-pairs them and reports both as wrong."""
    errs = []
    for line in txt.splitlines():
        refs = [(m.start(), m.group(1), int(m.group(2)))
                for m in LINE_REF.finditer(line)]
        if not refs:
            continue
        idents = [(m.start(), m.group(1)) for m in BT_IDENT.finditer(line)
                  if not m.group(1).endswith("rs")]
        for pos, base, n in refs:
            path = rust_path(base)
            if not path:
                continue
            src = open(os.path.join(ROOT, path)).read().splitlines()
            if n > len(src):
                errs.append(f"`{base}:{n}` — past end of file ({len(src)} lines)")
                continue
            before = [i for i in idents if i[0] < pos]
            if not before:
                continue
            ident = before[-1][1]
            where = [k + 1 for k, l in enumerate(src)
                     if re.match(DEF + re.escape(ident) + r"\b", l.strip())]
            # A pointer landing on the `#[derive]` or doc comment immediately
            # above a definition is aimed correctly; only real drift is reported.
            if len(where) == 1 and abs(where[0] - n) > ATTR_SLACK:
                errs.append(f"`{base}:{n}` — `{ident}` is defined at "
                            f"{base}:{where[0]}")
    return sorted(set(errs))

# A plan asserting it has not happened yet is a claim about the present, and the
# only one in a plan that keeps a shelf life. Cheap falsifier: the code the plan
# describes has been committed to since the plan was last touched.
NOT_YET = re.compile(r"(?im)^\s*(?:\*\*)?status(?:\*\*)?\s*[::][^\n]*?"
                     r"\bnot yet (started|run|built|implemented|done)\b")

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
    errs += check_line_refs(txt)

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
    makes every pointer to it stale, in docs it never touched. That is a real
    staleness — the pointer is wrong on that branch and will be wrong on main —
    but if the only remedy were hunting by hand, the check would get silenced.
    So the remedy is one command."""
    fixed = 0
    for md in files:
        path = os.path.join(ROOT, md)
        lines = open(path).read().splitlines(keepends=True)
        touched = False
        for li, line in enumerate(lines):
            refs = [(m.start(), m.end(), m.group(1), int(m.group(2)))
                    for m in LINE_REF.finditer(line)]
            if not refs:
                continue
            idents = [(m.start(), m.group(1)) for m in BT_IDENT.finditer(line)
                      if not m.group(1).endswith("rs")]
            edits = []
            for start, end, base, n in refs:
                src_path = rust_path(base)
                if not src_path:
                    continue
                src = open(os.path.join(ROOT, src_path)).read().splitlines()
                before = [i for i in idents if i[0] < start]
                if not before:
                    continue
                ident = before[-1][1]
                where = [k + 1 for k, l in enumerate(src)
                         if re.match(DEF + re.escape(ident) + r"\b", l.strip())]
                if len(where) == 1 and abs(where[0] - n) > ATTR_SLACK:
                    edits.append((start, end, f"`{base}:{where[0]}`",
                                  f"{md}: {base}:{n} -> {where[0]} ({ident})"))
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

    Each case is a bug that actually occurred while this was built."""
    import tempfile, textwrap
    global rust_path
    real_rust_path = rust_path
    fails = []

    def case(name, cond):
        if not cond:
            fails.append(name)

    with tempfile.TemporaryDirectory() as d:
        # A fake source file: `alpha` at line 5, `beta` at line 40.
        src = ["// pad"] * 60
        src[4] = "pub fn alpha() -> u8 { 0 }"
        src[39] = "pub const BETA: &[&str] = &[];"
        fake = os.path.join(d, "fake.rs")
        open(fake, "w").write("\n".join(src) + "\n")
        rust_path = lambda base: fake if os.path.basename(base) == "fake.rs" else None

        # 1. Cross-pairing: two pointers on one line, each already correct.
        #    Reported both as wrong before proximity pairing was added.
        case("cross-pairing",
             check_line_refs("`alpha` (`fake.rs:5`) and `BETA` (`fake.rs:40`)") == [])

        # 2. A stale pointer is reported, with the corrected line.
        errs = check_line_refs("`BETA` (`fake.rs:12`)")
        case("stale ref detected", len(errs) == 1 and "fake.rs:40" in errs[0])

        # 3. ATTR_SLACK: a pointer at the `#[derive]` line above a definition.
        case("attribute slack", check_line_refs("`BETA` (`fake.rs:39`)") == [])

        # 4. Past end of file is always an error.
        case("past EOF", any("past end of file" in e
                             for e in check_line_refs("`alpha` (`fake.rs:900`)")))

        # 5. No identifier before the ref: nothing to anchor to, stay silent.
        case("no anchor", check_line_refs("see `fake.rs:12` for context") == [])

        rust_path = real_rust_path

    # 6. The status claim that went stale on 2026-08-31, in both forms it took.
    case("status: not yet started",
         bool(NOT_YET.search("Status: **plan of record, not yet started.** Written 2026-08-25.")))
    case("status: not yet run",
         bool(NOT_YET.search("Status: **for review, not yet run.** Written 2026-08-25.")))
    case("status: a finished plan is not flagged",
         not NOT_YET.search("Status: **phases 0-2 landed** in #150."))

    # 7. Plans and memory files are told apart — the severity split depends on it.
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
