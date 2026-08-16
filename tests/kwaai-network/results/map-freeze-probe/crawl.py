import time, sys
import hivemind
from hivemind.utils.logging import use_hivemind_log_handler, get_logger
use_hivemind_log_handler("in_root_logger")
import logging; logging.getLogger().setLevel(logging.DEBUG)

BOOTSTRAPS = [
    "/dns/bootstrap-1.kwaai.ai/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc",
    "/dns/bootstrap-2.kwaai.ai/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY",
]

t0 = time.time()
print("== starting DHT client", flush=True)
dht = hivemind.DHT(initial_peers=BOOTSTRAPS, client_mode=True, start=True)
print(f"== DHT started in {time.time()-t0:.1f}s", flush=True)

for key in ["Llama-3-1-8B-Instruct.0", "_petals.models"]:
    t0 = time.time()
    print(f"== get({key!r}, latest=True)", flush=True)
    try:
        result = dht.get(key, latest=True)
        dt = time.time()-t0
        if result is None:
            print(f"== {key}: None in {dt:.1f}s", flush=True)
        else:
            value, expiration = result
            subkeys = list(value.keys()) if hasattr(value, "keys") else value
            print(f"== {key}: {len(subkeys)} entries in {dt:.1f}s: {subkeys}", flush=True)
    except Exception as e:
        import traceback; traceback.print_exc()
        print(f"== {key}: RAISED after {time.time()-t0:.1f}s: {e!r}", flush=True)

t0 = time.time()
print("== list_peers (the untimeouted call the health service makes)", flush=True)
try:
    peers = dht.run_coroutine(lambda _, node: node.p2p.list_peers())
    print(f"== list_peers: {len(peers)} peers in {time.time()-t0:.1f}s", flush=True)
    for p in peers:
        print(f"   {p.peer_id} {[str(a) for a in getattr(p, 'addrs', [])]}", flush=True)
except Exception:
    import traceback; traceback.print_exc()

import subprocess
print("== p2pd child processes:", flush=True)
subprocess.run(["ps", "ax", "-o", "pid,stat,command"], check=False)

print("== done", flush=True)
dht.shutdown()
