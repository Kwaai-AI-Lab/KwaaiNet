# distrib/systemd

systemd **user** units for running a KwaaiNet node as a service.

## Why these live in the repository

The units that run a production box live in `~/.config/systemd/user/`,
which is in no repository. Two things follow from that:

1. **A box cannot be rebuilt from its own source.** Recovering the box
   means reconstructing the unit from memory, and every reconstruction is
   slightly different.
2. **A fix to a unit stays on one box.** The crash-loop breaker, the
   `KWAAINET_HOME` override, the log path — each of those was a fix that
   had to be re-applied by hand on every machine that ran the service.

Putting the template here makes the unit part of the code, and the
installer makes the rendered copy part of the setup.

## Usage

```bash
# See what would be installed, and where
bash distrib/systemd/install-user-unit.sh --check

# Install it
bash distrib/systemd/install-user-unit.sh

# A second node with its own identity, on its own port
bash distrib/systemd/install-user-unit.sh \
    --name kwaainet-shard-api-hivv.service \
    --home ~/.kwaainet-hivv \
    --port 18090
```

The installer renders the template, creates the log directory systemd's
`append:` target needs, reloads the user manager, enables and starts the
unit, and enables **linger** so the unit survives a reboot without a login
session.

## Rules these units follow

- **User scope, not system scope.** No sudo, and no root-owned files in a
  home directory.
- **`Restart=on-failure`, not `always`.** A config error should stay
  visible instead of looping forever.
- **A crash-loop breaker.** `StartLimitIntervalSec=300` with
  `StartLimitBurst=3` turns "restart forever" into "sit in failed and say
  so". systemd's default limit (5 starts / 10 s) can never trip when
  `RestartSec >= 10`, which is exactly how a unit ends up looping every
  5 s indefinitely.
- **One port, one scope.** Never run the same port under both system and
  user scope: the loser spins forever on "address already in use".
- **The unit names its own `KWAAINET_HOME`.** A box that runs two nodes
  needs two homes, and a unit that inherits the ambient environment will
  silently pick the wrong one.

## Files

| file | what it is |
|---|---|
| `kwaainet-shard-api.service.template` | the template, with `{{...}}` placeholders |
| `install-user-unit.sh` | renders and installs it; `--check` installs nothing |
