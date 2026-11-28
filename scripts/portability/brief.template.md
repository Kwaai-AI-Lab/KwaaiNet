# KwaaiNet migration brief

Generated {{GENERATED_AT}} from {{HOST}} ({{OS}}/{{ARCH}}).
Repository: {{REPO_NAME}} at {{BRANCH}} ({{HEAD}}).

This brief is safe to share. Every secret in it appears as a fingerprint
(`sha256:<12 hex>`), never as a value. Two briefs can be diffed to prove
that the same key moved, and the diff reveals nothing.

---

## 1. What is being moved

A KwaaiNet node, the hivv ("live help") agent that talks to it, and the
GitHub identity that pushes to the repository.

The code is the easy part: it is in git, and a clone brings it. What a
clone does NOT bring is the list in section 2. That list is the actual
work, and it is the part that fails silently when it is skipped.

## 2. State that must be transferred

### 2.1 The node's identity — CRITICAL

| item | value |
|---|---|
| KWAAINET_HOME | {{HOME_PATH}} |
| identity key | {{IDENTITY}} |
| config.yaml | {{CONFIG}} |

`identity.key` is the node. A node whose identity key does not move is a
NEW node: new peer id, empty reputation, and every peer that pinned the
old peer id loses it. Bootstrap nodes are the worst case — their peer id
is written into every other node's `initial_peers`, so a fresh identity
silently orphans the network.

Transfer it with `scp`, preserve the mode (`chmod 600`), and PROVE it
landed by comparing fingerprints:

```
sha256sum ~/.kwaainet/identity.key | cut -c1-12
```

The value must equal the fingerprint above on BOTH machines.

### 2.2 The node's configuration

`config.yaml` carries the port, the announce address, the block range and
the model. Copy it as a file — do not retype it. The keys present on the
source machine are:

```
{{CONFIG_KEYS}}
```

### 2.3 The services

Units that must exist on the new machine, with the paths they hard-code:

| unit | enabled | active | exec |
|---|---|---|---|
{{UNITS_TABLE}}

A systemd user unit with an absolute `ExecStart` path does not survive a
username change. Check every path in that table still exists on the new
machine, or the unit will start and die with a "no such file" that the
crash-loop breaker turns into a quiet "failed" state.

### 2.4 The hivv coupling

| item | value |
|---|---|
| hivv lib dir | {{HIVV_LIB}} |
| hivv share dir | {{HIVV_SHARE}} |
| hivv identity files | {{HIVV_IDENTITY}} |
| hivv.env | {{HIVV_ENV}} |
| vhost | {{HIVV_VHOST}} |
| shard API (KwaaiNet side) | {{HIVV_SHARD_API}} |

The shard API on port 18090 is the OpenAI-compatible door that hivv.org
knocks on. The hive can be fully up and still have no local answer to
give if that port is closed.

### 2.5 The GitHub identity

| item | value |
|---|---|
| ssh key | {{GH_KEY}} |
| key fingerprint | {{GH_FINGERPRINT}} |
| gh login | {{GH_LOGIN}} |
| token scopes | {{GH_SCOPES}} |
| git identity | {{GH_NAME}} <{{GH_EMAIL}}> |

A clone needs read access. A PUSH needs the SSH key registered with
GitHub AND a git identity configured, and it is the second one that is
usually forgotten: without `user.name`/`user.email` the first commit on
the new machine is authored by `user@hostname` and rejected.

## 3. The runbook

On the NEW machine:

```bash
# 1. Code
git clone git@github.com:OWNER/KwaaiNet.git ~/kwaai/KwaaiNet
cd ~/kwaai/KwaaiNet

# 2. Build dependencies that cargo cannot resolve on its own
bash core/patches/fetch-patches.sh

# 3. Toolchain, build, install
bash setup.sh

# 4. GitHub identity — clone AND push
bash scripts/portability/github-setup.sh --apply

# 5. Node identity and config (copied from the old machine)
mkdir -p ~/.kwaainet && chmod 700 ~/.kwaainet
scp OLDBOX:~/.kwaainet/identity.key ~/.kwaainet/
scp OLDBOX:~/.kwaainet/config.yaml   ~/.kwaainet/
chmod 600 ~/.kwaainet/identity.key

# 6. Services
cp OLDBOX:~/.config/systemd/user/kwaainet-*.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now kwaainet-shard-api.service

# 7. hivv state
cp -r OLDBOX:~/.local/share/hivv/identity ~/.local/share/hivv/
cp OLDBOX:~/.local/share/hivv/hivv.env ~/.local/share/hivv/

# 8. Prove it
bash scripts/portability/verify.sh --expect-shard-api
```

## 4. Acceptance criteria

The move is done when `scripts/portability/verify.sh` passes with no
failures, and when:

- [ ] the identity fingerprint on the new machine equals the one in 2.1
- [ ] `kwaainet --version` runs on the new machine
- [ ] `git push --dry-run` succeeds from the new machine
- [ ] the shard API answers `/v1/models` on port 18090
- [ ] the old machine's node is STOPPED (or is intentionally a second node)

## 5. Gotchas

1. **Two nodes, one identity.** If the old box keeps running with the same
   `identity.key`, both are the same peer id on the network. That is a
   split brain: the DHT sees one node flapping between two addresses.
   Stop the old one, or give it its own key on purpose.

2. **Two nodes, one port.** Copying `config.yaml` verbatim moves the port
   with it. If both machines are on the same LAN, the second one will fail
   to bind. Change the port — and the announce address with it.

3. **Two homes, two identities.** A box running the hivv integration has
   BOTH `~/.kwaainet` and `~/.kwaainet-hivv`, each with its own identity
   and its own reputation. Moving one and not the other leaves the hive
   with a door that opens onto a different node.

4. **Reputation does not travel.** `reputation.json` is per node. Copy it
   or accept the new node starts at zero.

5. **The bootstrap caveat.** A bootstrap's peer id is pinned into other
   nodes' `initial_peers`. Moving a bootstrap means every other node
   needs its `initial_peers` updated, or the network keeps dialling an
   address that no longer answers.

6. **macOS has no systemd.** The units become launchd plists in
   `~/Library/LaunchAgents`, and `systemctl --user` checks are skipped
   rather than failed.

7. **The repo's own memory files drift.** `CLAUDE.md` files are read by
   agents as authoritative. After a move, run
   `python3 scripts/check_memory_integrity.py` before trusting them.

---

_Regenerate this brief on the source machine with
`scripts/portability/handoff.sh --out BRIEF.md`._
