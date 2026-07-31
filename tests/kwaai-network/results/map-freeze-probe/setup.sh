#!/bin/bash
set -x
apt-get update -qq >/dev/null 2>&1
apt-get install -y -qq git wget procps >/dev/null 2>&1
pip install --no-cache-dir 'setuptools<81' wheel grpcio-tools > /probe/pip.log 2>&1
pip install --no-cache-dir 'numpy<2' 'torch==2.3.1' --index-url https://download.pytorch.org/whl/cpu >> /probe/pip.log 2>&1
pip install --no-cache-dir --no-build-isolation 'hivemind==1.1.10.post2' >> /probe/pip.log 2>&1
status=$?
echo "PIP_EXIT=$status"
# hivemind 1.1.10's optim module imports private names from the torch.cuda.amp
# shim that the +cpu build does not re-export; the real definitions live in
# torch.amp. The optim module is irrelevant to the DHT crawl.
sed -i 's/from torch.cuda.amp.grad_scaler import OptState, _refresh_per_optimizer_state/from torch.amp.grad_scaler import OptState, _refresh_per_optimizer_state/' \
  /usr/local/lib/python3.10/site-packages/hivemind/optim/grad_scaler.py
tail -40 /probe/pip.log
if [ $status -eq 0 ]; then
  # Go's signal-based async preemption crashes under qemu user emulation;
  # p2pd inherits this env var.
  export GODEBUG=asyncpreemptoff=1
  python -c 'import hivemind; print("hivemind ok", hivemind.__version__)' \
    && timeout 300 python /probe/crawl.py > /probe/crawl.log 2>&1
  echo "CRAWL_EXIT=$?"
  echo "── panic/fatal lines:"
  grep -nE "panic|fatal|SIGSEGV|SIGILL" /probe/crawl.log | head -10
  echo "── crawl log tail:"
  tail -50 /probe/crawl.log
fi
