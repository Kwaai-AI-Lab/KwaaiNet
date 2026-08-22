#!/bin/bash
# Watch map.kwaai.ai for silent crawler failures.
#
# On 2026-08-21 the map showed zero nodes while the network was entirely
# healthy: `shard chain` had 9 servers and every node was landing `32/32
# stored` at both bootstraps. The map's crawler had failed silently and
# recovered on a later tick, leaving no trace.
#
# The tell was `update_duration`. A real crawl takes ~2.9 s; the failing one
# returned in 0.0128 s — roughly 220x too fast to have queried anything, with
# `model_reports: []`. Crucially `bootstrap_states` still read "online"
# throughout, because the liveness check succeeded while record enumeration
# died. So "no nodes" is indistinguishable from a real outage unless you catch
# the duration.
#
# Every poll appends to the JSONL log. Only state *transitions* and anomalies
# go to stdout, so this can run under a monitor without becoming noise.
#
# Usage: map_watch.sh [interval_secs]     (default 60, matching update_period)

set -uo pipefail

URL="https://map.kwaai.ai/api/v1/state"
LOG="${MAP_WATCH_LOG:-$HOME/.kwaainet/logs/map-watch.jsonl}"
INTERVAL="${1:-60}"

# A crawl this fast cannot have queried the DHT. Real ones take seconds.
FAST_CRAWL_SECS=0.1

mkdir -p "$(dirname "$LOG")"

prev_state=""

while true; do
    now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    body=$(curl -s -m 30 -w '\n%{http_code}' "$URL" 2>/dev/null)
    http=$(printf '%s' "$body" | tail -1)
    json=$(printf '%s' "$body" | sed '$d')

    read -r state detail <<<"$(
        HTTP="$http" NOW="$now" FAST="$FAST_CRAWL_SECS" python3 -c '
import json, os, sys

http = os.environ["HTTP"]
fast = float(os.environ["FAST"])
raw = sys.stdin.read()

if http != "200":
    shown = http if http else "none"
    print("unreachable http=" + shown)
    sys.exit()
try:
    d = json.loads(raw)
except Exception:
    print("unparseable body-was-not-json")
    sys.exit()

reports = d.get("model_reports") or []
dur = float(d.get("update_duration") or 0)
boots = d.get("bootstrap_states") or []
servers = sum(len(m.get("server_rows") or m.get("servers") or []) for m in reports)

# A crawl too fast to have queried anything is broken regardless of what it
# still reports. Observed 2026-08-22: duration collapsed to 0.0656 s four
# minutes before the map emptied, while it still claimed 9 servers — the count
# then bled 9 -> 7 -> 6 -> 4 -> 0 as each failed cycle expired more entries.
# Keying only on "no models" missed that entire window, so `degraded` exists to
# catch the leading edge rather than the aftermath.
if dur < fast:
    state = "crawl-failed" if not reports else "degraded"
elif not reports:
    state = "empty"
else:
    state = "ok"

boots_s = ",".join(boots) if boots else "none"
print("%s models=%d servers=%d dur=%.4f boots=%s"
      % (state, len(reports), servers, dur, boots_s))
' <<<"$json"
    )"

    printf '{"ts":"%s","state":"%s","detail":"%s"}\n' "$now" "$state" "$detail" >>"$LOG"

    # Emit only on change, so a healthy map is silent.
    if [ "$state" != "$prev_state" ]; then
        case "$state" in
            ok)
                [ -n "$prev_state" ] && echo "$now RECOVERED — $detail"
                ;;
            crawl-failed)
                echo "$now MAP CRAWL FAILED — $detail (network may be fine; check 'kwaainet shard chain')"
                ;;
            degraded)
                echo "$now MAP CRAWL DEGRADED — $detail (crawl too fast to be real; node count will likely bleed to zero)"
                ;;
            empty)
                echo "$now MAP EMPTY but crawl ran — $detail (could be a real outage)"
                ;;
            unreachable|unparseable)
                echo "$now MAP $state — $detail"
                ;;
        esac
        prev_state="$state"
    fi

    sleep "$INTERVAL"
done
