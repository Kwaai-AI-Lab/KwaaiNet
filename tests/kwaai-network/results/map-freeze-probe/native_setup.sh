#!/bin/bash
set -x
cd "$(dirname "$0")"
/opt/homebrew/bin/python3.10 -m venv venv
./venv/bin/pip install -q --upgrade pip
./venv/bin/pip install -q 'setuptools<81' wheel grpcio-tools 'numpy<2' 'torch==2.3.1' > native_pip.log 2>&1
./venv/bin/pip install --no-build-isolation 'hivemind==1.1.10.post2' >> native_pip.log 2>&1
status=$?
echo "PIP_EXIT=$status"
if [ $status -ne 0 ]; then tail -30 native_pip.log; exit 1; fi
SITE=./venv/lib/python3.10/site-packages
sed -i '' 's/from torch.cuda.amp.grad_scaler import OptState, _refresh_per_optimizer_state/from torch.amp.grad_scaler import OptState, _refresh_per_optimizer_state/' \
  "$SITE/hivemind/optim/grad_scaler.py"
# Swap in our native arm64 p2pd (same hivemind fork lineage).
ls -la "$SITE/hivemind/hivemind_cli/" || true
cp /Volumes/Projects/kwaaiai/KwaaiNet/core/target/debug/p2pd "$SITE/hivemind/hivemind_cli/p2pd"
chmod +x "$SITE/hivemind/hivemind_cli/p2pd"
./venv/bin/python -c 'import hivemind; print("hivemind ok", hivemind.__version__)' || exit 1
