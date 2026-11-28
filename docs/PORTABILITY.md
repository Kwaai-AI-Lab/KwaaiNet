# Portability: moving a node to another machine

A `git clone` gives you the code. It does not give you the node.

This document is about the second thing. It exists because moving a
KwaaiNet node from one machine to another is a two-hour job that has
failed in the same three ways every time, and because "it worked on the
old box" is not a claim anyone can check.

---

## 1. What actually has to move

| | in git? | notes |
|---|---|---|
| the source code | **yes** | `git clone` is enough |
| the workspace's patched dependencies | fetched, not cloned | `bash core/patches/fetch-patches.sh` — checksum-pinned, needed before cargo can even parse the workspace |
| the toolchain | no | Rust 1.80+, protoc, (optionally) the CUDA toolkit |
| **the node's identity key** | **no** | `~/.kwaainet/identity.key` — **the node** |
| **the node's configuration** | **no** | `~/.kwaainet/config.yaml` — port, announce address, block range, model |
| **the services** | **no** | `~/.config/systemd/user/*.service` — absolute paths inside. The shard api unit now lives in the repo: [distrib/systemd/](distrib/systemd/README.md) |
| **the node's reputation** | **no** | `~/.kwaainet/reputation.json` — per node, does not travel |
| **the GitHub identity** | **no** | SSH key + `git config user.name/user.email` |
| the hivv node identity and token | **no** | `~/.local/share/hivv/identity/`, `hivv.env` |

The bolded rows are the ones that fail silently. A missing identity key
does not crash the node — it starts happily as a **new** node with a new
peer id, and the network treats it as a stranger.

## 2. The four commands

```bash
# What is on this machine, in a form that is safe to paste into chat?
bash scripts/portability/inventory.sh

# Give this machine a GitHub identity that can clone AND push.
bash scripts/portability/github-setup.sh --apply

# Did the move work?
bash scripts/portability/verify.sh

# Write a brief for whoever (or whatever) performs the next move.
bash scripts/portability/handoff.sh --out BRIEF.md
```

Each has a `--check` (or read-only default) mode. See
[scripts/portability/README.md](../scripts/portability/README.md).

## 3. Moving to a new machine

On the **new** machine:

```bash
# 1. Code
git clone git@github.com:OWNER/KwaaiNet.git ~/kwaai/KwaaiNet
cd ~/kwaai/KwaaiNet

# 2. The patched dependencies cargo cannot resolve on its own
bash core/patches/fetch-patches.sh

# 3. Toolchain, build, install. --check first if you want to see the plan.
bash setup.sh --check
bash setup.sh

# 4. GitHub identity — clone AND push
bash scripts/portability/github-setup.sh --apply

# 5. The node's identity and configuration, copied from the old machine
mkdir -p ~/.kwaainet && chmod 700 ~/.kwaainet
scp OLDBOX:~/.kwaainet/identity.key ~/.kwaainet/
scp OLDBOX:~/.kwaainet/config.yaml   ~/.kwaainet/
chmod 600 ~/.kwaainet/identity.key

# 6. The services
#    The unit is IN this repository — render it for this machine rather
#    than copying one from the old box:
bash distrib/systemd/install-user-unit.sh --check
bash distrib/systemd/install-user-unit.sh

# 7. Prove it
bash scripts/portability/verify.sh --expect-shard-api
```

### Prove the identity actually moved

A copy that silently produced a different key is the failure this check
exists for:

```bash
sha256sum ~/.kwaainet/identity.key | cut -c1-12
```

The value must be identical on both machines. The inventory prints it as
`set, sha256:<12 hex>` — a fingerprint, never the key itself — so you can
compare without exposing anything.

## 4. The hivv ("live help") coupling

The hivv agent and the KwaaiNet node are two halves of one thing on a
production box:

- `kwaainet-shard-api.service` serves an **OpenAI-compatible API on
  port 18090**. This is the door hivv.org knocks on. Its model is served by
  the *network*, not the box: `shard api` resolves a covering block chain
  through the libp2p hivemind DHT per request, so an answer can come from
  any node serving that model. The box contributes its own blocks
  (hivv.org, 0-4) and relays the rest.
- `llama.service` serves a local model on port 8090 for the cases where
  the network cannot.
- `hivv-agent.service` and `hivv-dial.service` are the agent and the
  dial shim, both Python, both pointing at `~/.local/share/hivv`.

**Moving the node without moving the hivv state leaves the hive with a
door that opens onto a different node.** Copy `~/.local/share/hivv/` too —
in particular `identity/` and `hivv.env`.

Note that a hivv-enabled box has **two** `KWAAINET_HOME` directories:
`~/.kwaainet` and `~/.kwaainet-hivv`, each with its own identity and
its own reputation. The shard API unit sets `KWAAINET_HOME` explicitly.
Moving one and not the other is a silent half-move.

The hivv.org vhost itself (`/var/www/vhosts/hivv.org`) is a separate
repository with its own CI, and it already has the "clone and run on every
platform" gate that this document argues for here.

## 5. macOS and Linux, honestly

The claim that this stack is "heavy macOS dependent" is worth separating
into two claims, because only one of them is true.

**Not macOS-bound:** the default build. The CLI's default features are
`storage, rag, ipv6`; inference's default is `cpu`. The entire
production topology — systemd units, `/var/www/vhosts`, Apache, the
shard API, the DHT, the hivv agent — is Linux. `setup.sh` detects Linux
and macOS and installs accordingly (`apt` vs `brew`).

**macOS-bound, but opt-in:** `--features mlx` serves blocks with MLX on
Apple Silicon, and `--features metal` is the Metal backend. Neither is
in the default feature set, and neither is needed by the release binaries
that the network uses.

So the honest statement is: **the build is cross-platform, and the
deployment is Linux-first.** A machine that has never run KwaaiNet can be
brought up with `git clone && bash setup.sh`, and CI proves it on every
push to `main`.

## 6. What CI guarantees

`.github/workflows/portability.yml` runs on every push to `main` and on
every PR that touches `setup.sh`, `scripts/portability/` or
`core/patches/`:

| job | what it proves |
|---|---|
| `setup-check` | `setup.sh --check` works on a bare checkout, on Linux and macOS, and **installs nothing** |
| `build-from-clean-checkout` | a fresh machine can produce a working binary |
| `scripts-on-empty-machine` | the portability scripts behave correctly on a machine with **no** KwaaiNet state: the inventory reports the absent state honestly, `verify.sh` **fails** and says why, `github-setup.sh --check` mutates nothing, and the handoff brief renders with no secret in it |

That last job is the one that earns its keep. Every portability check
that only ever runs on a fully-provisioned development box quietly stops
testing the case that matters.

## 7. Gotchas

1. **Two nodes, one identity.** If the old box keeps running with the
   same `identity.key`, both are the same peer id on the network. That is
   a split brain: the DHT sees one node flapping between two addresses.
   Stop the old one, or give it its own key on purpose.

2. **Two nodes, one port.** Copying `config.yaml` verbatim moves the port
   with it. If both machines are on the same LAN, the second one will
   fail to bind. Change the port — and the announce address with it.

3. **Reputation does not travel.** `reputation.json` is per node. Copy it
   or accept that the new node starts at zero.

4. **The bootstrap caveat.** A bootstrap's peer id is pinned into every
   other node's `initial_peers`. Moving a bootstrap means every other
   node needs its `initial_peers` updated, or the network keeps dialling
   an address that no longer answers. See `docs/BOOTSTRAP.md`.

5. **Absolute paths in units.** A systemd user unit with an absolute
   `ExecStart` path does not survive a username change. The unit will
   start and die with a "no such file" that the crash-loop breaker turns
   into a quiet "failed" state.

6. **macOS has no systemd.** The units become launchd plists in
   `~/Library/LaunchAgents`, and the systemd checks are skipped rather
   than failed.

7. **The repo's memory files drift.** `CLAUDE.md` files are read by
   agents as authoritative. After a move, run
   `python3 scripts/check_memory_integrity.py` before trusting them.

8. **Uncommitted work does not travel.** `verify.sh` reports the dirty
   paths and the local-only branches, because those are exactly the
   things a fresh clone silently leaves behind.

---

_See also: [scripts/portability/README.md](../scripts/portability/README.md),
[docs/BOOTSTRAP.md](BOOTSTRAP.md)._
