# scripts/portability/

Tools for moving a KwaaiNet node to another machine.

Read-only by default. Nothing here changes the machine it describes
unless you pass `--apply` (or `--check`, where that is the point).

```
inventory.sh       what state does this machine hold?
github-setup.sh    give this machine a GitHub identity that can push
verify.sh          did the move work?
handoff.sh         write a brief for whoever performs the next move
lib.sh             shared helpers
brief.template.md  the handoff template
```

## The rules these tools follow

**Never print a secret.** The inventory and the brief record secret
values as `set, sha256:<12 hex>` — a fingerprint, never the value. Two
inventories can be diffed to prove that the same key moved, and that diff
is safe to paste into chat. The CI job
`scripts-on-empty-machine` asserts this, and it also asserts that neither
file contains anything shaped like a private key.

**Never mutate in a check mode.** `inventory.sh` is read-only.
`github-setup.sh --check` must not create an SSH key, install `gh`, or
touch `~/.ssh` — CI asserts that `~/.ssh` is byte-identical afterwards.

**Fail honestly.** `verify.sh` exits non-zero on a machine with no node,
no identity and no remote, and names each missing piece. A verifier that
passes everywhere is decoration.

**Pin the SSH key.** OpenSSH expands `~` through the **passwd database**,
not through `$HOME`, so an unpinned `git push` probe on a machine with
several keys reports success for whichever key SSH picked first — which may
not be the key the migration moved. Both `verify.sh` and
`github-setup.sh` pin the key explicitly
(`-o IdentitiesOnly=yes -i <key>`), and fall back to
`-o IdentityFile=/dev/null` when there is no key at all, so the probe
fails honestly instead of finding someone else's.

## Bash 3.2

macOS ships bash 3.2 as `/bin/bash`. These scripts run on the machine
you are moving *to*, which is the one you have not set up yet, so they
avoid anything newer: no associative arrays, no `mapfile`, no
`${var,,}`, no `local -n`.

## inventory.sh

```bash
bash scripts/portability/inventory.sh              # text report
bash scripts/portability/inventory.sh --json       # flat JSON, dotted keys
bash scripts/portability/inventory.sh --out FILE
bash scripts/portability/inventory.sh --repo PATH
```

Records: the machine, the toolchain, the repository (including
uncommitted paths, stashes and branches that exist only locally), every
`KWAAINET_HOME` and its identity fingerprint, the systemd user units and
their hard-coded paths, the listening ports, the GitHub access state, and
the hivv coupling.

## github-setup.sh

```bash
bash scripts/portability/github-setup.sh --check    # report only
bash scripts/portability/github-setup.sh --apply    # make it so
```

Handles the four things that make "it works on the old box" true:

- an SSH key that exists but is **not registered** with GitHub
- no `git config user.name/user.email`, so the first commit is authored
  by `user@hostname` and rejected
- no `gh` CLI, so registering the key needs a browser on another machine
- remotes named differently from the convention the org expects

`--apply` may generate a key, install `gh`, run `gh auth login`,
register the key, set the git identity, and set the remotes. It prints the
public key, which is public by definition, so it can also be registered by
hand.

Override the remote targets with `--upstream OWNER/REPO` and
`--fork OWNER/REPO`, or the `KWAAI_UPSTREAM_REPO` and
`KWAAI_FORK_REPO` environment variables.

## verify.sh

```bash
bash scripts/portability/verify.sh
bash scripts/portability/verify.sh --expect-node
bash scripts/portability/verify.sh --expect-shard-api
```

The acceptance test for a move. Checks the checkout, the build, the
node's identity and configuration, the services, GitHub push access, and
the runtime.

## handoff.sh

```bash
bash scripts/portability/handoff.sh --out BRIEF.md
```

Writes a self-contained brief for the person — or the agent — performing
the move. The idea is the same one this repository already applies to
code review: **the context that wrote a thing carries its assumptions
into reading it.** Someone who did not build this node does not know
which files are load-bearing, which are secrets, or which failure is
silent. So the machine that *has* the context writes the brief, and the
brief travels.

The brief is rendered from `brief.template.md`. Edit that file to change
what a brief contains; the placeholders are `{{UPPER_SNAKE}}`.
