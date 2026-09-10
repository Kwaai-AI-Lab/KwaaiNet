#!/usr/bin/env python3
"""Flag Rust items inserted into someone else's doc comment.

`///` attaches to the *next* item, so inserting a function between an existing
doc block and the function it described silently re-points that doc at the new
item and leaves the original undocumented. It compiles, clippy is silent, and
rustdoc renders the stolen comment happily on the wrong function — so nothing
in CI catches it and a reviewer only sees it by reading the line *above* an
insertion, which is the line a diff makes easiest to skip.

Five instances reached review in the September 2026 PR queue (#189, #199,
#201, #202, #203). The worst, in #202, moved `load_or_create`'s "creating it
with defaults if absent" onto `peek_log_level`, whose own doc promises the
opposite — so the merged comment asserted the reverse of what the code did.

Detection: an *added* item declaration whose preceding doc block in the new
file is *unchanged* context. A new item that brings its own doc is fine; one
that inherits a pre-existing block is not.

Usage:
    git diff -U3 origin/main...HEAD | python3 scripts/check_orphaned_docs.py
    python3 scripts/check_orphaned_docs.py --base origin/main
    python3 scripts/check_orphaned_docs.py --self-test

Exit 1 if anything is flagged (or a self-test case fails), 2 on a tooling error.

Run `--self-test` before trusting a clean result. A checker that has quietly
broken reports "no orphaned doc comments" for every diff, which is
indistinguishable from a real pass — the self-test feeds it a known positive
and known negatives so a silent break shows up as a failure instead.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from dataclasses import dataclass

# Item declarations that a doc comment can attach to. Struct fields and enum
# variants are deliberately absent: they are the thing most often inserted
# after a doc comment, and `field: Type` / `Variant,` carry no keyword, so
# including them would be all false positives.
ITEM = re.compile(
    r"""^\s*
    (?: \# \[ [^\]]* \] \s* )*          # leading attributes on the same line
    (?: pub (?:\s*\( [^)]* \))? \s+ )?  # pub, pub(crate), pub(super), ...
    (?: default \s+ )?
    (?: const \s+ )?
    (?: async \s+ )?
    (?: unsafe \s+ )?
    (?: extern \s+ "[^"]*" \s+ )?
    (?: fn | struct | enum | trait | impl | mod | type | static | union
      | macro_rules! )
    \b""",
    re.VERBOSE,
)

DOC = re.compile(r"^\s*///")
ATTR = re.compile(r"^\s*\#\[")
BLANK = re.compile(r"^\s*$")


@dataclass
class Line:
    text: str
    added: bool
    new_lineno: int | None
    removed: bool = False


@dataclass
class Finding:
    path: str
    lineno: int
    item: str
    doc: str


def parse_diff(diff: str) -> dict[str, list[list[Line]]]:
    """Group each file's hunks into new-file line sequences."""
    files: dict[str, list[list[Line]]] = {}
    path: str | None = None
    hunk: list[Line] | None = None
    new_lineno = 0

    for raw in diff.splitlines():
        if raw.startswith("+++ "):
            target = raw[4:].strip()
            path = target[2:] if target.startswith("b/") else target
            hunk = None
            continue
        if raw.startswith("@@"):
            m = re.match(r"@@ -\d+(?:,\d+)? \+(\d+)", raw)
            if not m or path is None:
                hunk = None
                continue
            new_lineno = int(m.group(1))
            hunk = []
            files.setdefault(path, []).append(hunk)
            continue
        if hunk is None or path is None:
            continue

        if raw.startswith("+"):
            hunk.append(Line(raw[1:], True, new_lineno))
            new_lineno += 1
        elif raw.startswith("-"):
            # Absent from the new file, but kept: a `-old` directly before a
            # `+new` item is an edited signature, not an insertion.
            hunk.append(Line(raw[1:], False, None, removed=True))
        elif raw.startswith(" "):
            hunk.append(Line(raw[1:], False, new_lineno))
            new_lineno += 1
        # "\ No newline at end of file" and anything else: ignore

    return files


def scan_hunk(path: str, hunk: list[Line]) -> list[Finding]:
    """An added item whose doc block above it is unchanged context."""
    out: list[Finding] = []
    for i, line in enumerate(hunk):
        if not line.added or not ITEM.match(line.text):
            continue

        # An edited signature or a rename shows up as `-old` then `+new`
        # inside one change block. That item already owned the doc block
        # above it — skip. Unified diffs put every `-` of a block before its
        # `+`s, so the removed lines to inspect are the ones directly before
        # the start of this added run.
        start = i
        while start - 1 >= 0 and hunk[start - 1].added:
            start -= 1
        r = start - 1
        edited = False
        while r >= 0 and hunk[r].removed:
            if ITEM.match(hunk[r].text):
                edited = True
                break
            r -= 1
        if edited:
            continue

        # Walk up past anything that legitimately sits between a doc comment
        # and its item — but only while it is *added*. Removed lines are not
        # in the new file and are stepped over. The first unchanged line
        # decides: a context `///` means this item was slotted into a doc
        # block that already belonged to something else.
        j = i - 1
        while j >= 0:
            prev = hunk[j]
            if prev.removed:
                j -= 1
                continue
            if not prev.added:
                break
            if DOC.match(prev.text) or ATTR.match(prev.text) or BLANK.match(prev.text):
                j -= 1
                continue
            break  # added code, not a doc/attr run — this item stands alone
        else:
            continue

        prev = hunk[j]
        if prev.added or prev.removed or not DOC.match(prev.text):
            continue

        # Report the first line of the inherited block, which is the sentence
        # that now describes the wrong item.
        k = j
        while (
            k - 1 >= 0
            and not hunk[k - 1].added
            and not hunk[k - 1].removed
            and DOC.match(hunk[k - 1].text)
        ):
            k -= 1

        out.append(
            Finding(
                path=path,
                lineno=line.new_lineno or 0,
                item=line.text.strip(),
                doc=hunk[k].text.strip(),
            )
        )
    return out


def scan_diff(diff: str) -> list[Finding]:
    """Every finding in a unified diff, Rust files only. The one path both
    the real run and the self-test go through."""
    findings: list[Finding] = []
    for path, hunks in parse_diff(diff).items():
        if not path.endswith(".rs"):
            continue
        for hunk in hunks:
            findings.extend(scan_hunk(path, hunk))
    return findings


# (name, unified diff, expected finding count). The first case is the shape
# that reached review five times in September 2026 (#189's `quic` stealing
# `reloaded`'s doc); the `mod` case is #202's. The negatives are the shapes
# that must *not* fire: an edited signature, a fresh item with its own doc,
# a struct field, and a non-Rust file.
SELF_TEST_CASES: list[tuple[str, str, int]] = [
    (
        "inserted fn inherits an unchanged doc block",
        """\
diff --git a/src/config.rs b/src/config.rs
--- a/src/config.rs
+++ b/src/config.rs
@@ -1,4 +1,8 @@
 /// Re-read config.yaml before a save so writes by other processes are kept;
 /// falls back to `self` if the file cannot be read.
+/// Effective `enable_quic`: the explicit setting, else the swarm default.
+pub fn quic(&self) -> bool {
+    true
+}
+
 pub fn reloaded(&self) -> Self {
     self.clone()
 }
""",
        1,
    ),
    (
        "inserted #[cfg(test)] mod inherits an unchanged doc block",
        """\
diff --git a/src/config.rs b/src/config.rs
--- a/src/config.rs
+++ b/src/config.rs
@@ -10,3 +10,8 @@
 }

 /// `cargo test` must not be able to touch the developer's real config.
+#[cfg(test)]
+mod log_level_values {
+}
+
 #[cfg(test)]
 mod test_isolation {
""",
        1,
    ),
    (
        "edited signature keeps its own doc (must not fire)",
        """\
diff --git a/src/node.rs b/src/node.rs
--- a/src/node.rs
+++ b/src/node.rs
@@ -1,3 +1,3 @@
 /// Sync the announced block range from the live config.
-async fn refresh_server_info(server_info: &mut DHTServerInfo) {
+async fn refresh_server_info(server_info: &mut DHTServerInfo, config: &KwaaiNetConfig) {
 }
""",
        0,
    ),
    (
        "fresh item bringing its own doc (must not fire)",
        """\
diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -3,2 +3,6 @@
 }
+
+/// Its own doc.
+pub fn g() {}
+
 pub fn h() {}
""",
        0,
    ),
    (
        "struct field inserted after a field doc (must not fire)",
        """\
diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,3 +1,4 @@
 pub struct S {
     /// Doc for b.
+    pub a: u8,
     pub b: u8,
""",
        0,
    ),
    (
        "same shape in a non-Rust file is ignored (must not fire)",
        """\
diff --git a/scripts/x.py b/scripts/x.py
--- a/scripts/x.py
+++ b/scripts/x.py
@@ -1,2 +1,4 @@
 /// not rust
+/// still not rust
+pub fn looks_like_rust() {}
 pub fn also_not() {}
""",
        0,
    ),
]


def run_self_test() -> int:
    failures = 0
    print("check_orphaned_docs --self-test")
    for name, diff, expected in SELF_TEST_CASES:
        found = len(scan_diff(diff))
        ok = found == expected
        failures += 0 if ok else 1
        print(f"  {'ok  ' if ok else 'FAIL'}  {name}  (expected {expected}, found {found})")
    if failures:
        print(f"\n{failures} self-test case(s) failed: the checker is broken, not the code.")
        return 1
    print(f"\nall {len(SELF_TEST_CASES)} cases pass")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--base",
        help="diff against this ref instead of reading a diff on stdin",
    )
    ap.add_argument(
        "--self-test",
        action="store_true",
        help="run the checker against known positives and negatives, then exit",
    )
    args = ap.parse_args()

    if args.self_test:
        return run_self_test()

    if args.base:
        try:
            diff = subprocess.run(
                ["git", "diff", "-U3", f"{args.base}...HEAD"],
                capture_output=True,
                text=True,
                check=True,
            ).stdout
        except subprocess.CalledProcessError as e:
            print(f"check_orphaned_docs: git diff failed: {e.stderr.strip()}", file=sys.stderr)
            return 2
    else:
        diff = sys.stdin.read()

    if not diff.strip():
        print("check_orphaned_docs: empty diff, nothing to check")
        return 0

    findings = scan_diff(diff)

    if not findings:
        print("check_orphaned_docs: no orphaned doc comments")
        return 0

    print(f"check_orphaned_docs: {len(findings)} item(s) inserted into an existing doc block\n")
    for f in findings:
        print(f"  {f.path}:{f.lineno}")
        print(f"    inserted: {f.item}")
        print(f"    inherits: {f.doc}")
        print("    The doc block above this item documented what now follows it.")
        print("    Move the new item below that item, or give the block back.\n")
    print("`///` attaches to the next item, so the comment above now describes")
    print("the wrong function and the original is left undocumented.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
