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

Exit non-zero when a file reference does not resolve, so CI can gate it.

Usage: scripts/check_memory_integrity.py [--fix-hints]
"""
import os, re, subprocess, sys, time

ROOT = subprocess.run(["git", "rev-parse", "--show-toplevel"],
                      capture_output=True, text=True).stdout.strip()
STALE_DAYS = 60  # a doc untouched this long while its code moved is a warning

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

def check(md):
    """Return (errors, warnings) for one memory file."""
    errs, warns = [], []
    txt = open(os.path.join(ROOT, md)).read()

    # Sections about a *different* repository are not claims about this one.
    # kwaai-storage documents changes needed in the PHE repo under its own
    # heading; flagging those as broken would train people to ignore the check.
    txt = strip_foreign_sections(txt)

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
            errs.append(f"file reference does not resolve: `{ref}`")
        elif "/" in ref and not any(
                r.endswith(ref) for r in found.splitlines()):
            errs.append(f"`{ref}` — no file at that path (moved? found "
                        f"{found.splitlines()[0]})")

    # 2. Every `fn name()` claim must exist.
    for fn in sorted(set(re.findall(r"`([a-z_][a-z0-9_]{4,})\(\)`", txt))):
        if PLACEHOLDER.match(fn):
            continue
        rc = subprocess.run(["grep", "-rq", f"fn {fn}", "core/crates",
                             "--include=*.rs"], cwd=ROOT).returncode
        if rc != 0:
            errs.append(f"function reference does not resolve: `{fn}()`")

    # 3. Drift: the doc has not been touched while its subject kept moving.
    src = described_source(md)
    if src and os.path.isdir(os.path.join(ROOT, src)):
        md_t, src_t = last_commit_epoch(md), last_commit_epoch(src)
        if md_t and src_t:
            days = (src_t - md_t) / 86400
            if days > STALE_DAYS:
                warns.append(f"{days:.0f} days of drift — {src} changed "
                             f"{time.strftime('%Y-%m-%d', time.localtime(src_t))}, "
                             f"this doc last {time.strftime('%Y-%m-%d', time.localtime(md_t))}")
    return errs, warns

def main():
    files = tracked("*CLAUDE.md") + tracked("projects/*/GLOSSARY.md")
    n_err = n_warn = 0
    print(f"Checking {len(files)} memory files\n")
    for md in sorted(files):
        errs, warns = check(md)
        if not errs and not warns:
            continue
        print(f"  {md}")
        for e in errs:
            print(f"    ERROR   {e}")
        for w in warns:
            print(f"    drift   {w}")
        n_err += len(errs); n_warn += len(warns)
    print(f"\n{n_err} broken reference(s), {n_warn} drift warning(s) "
          f"across {len(files)} files")
    if n_warn and not n_err:
        print("\nDrift is a prompt to re-read, not a failure. Reflect on whether the "
              "doc still says something true, then touch it or fix it.")
    return 1 if n_err else 0

if __name__ == "__main__":
    sys.exit(main())
