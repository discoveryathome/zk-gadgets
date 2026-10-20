#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export CARGO_HOME="${CARGO_HOME:-$PWD/.cargo-home}"
export RAYON_NUM_THREADS=1
PILOT_REPEATS=1 PILOT_SKIP_PROOFS=1 cargo run --release --locked -- replay
python3 - <<'PY'
import json
from pathlib import Path
reference=Path('results'); replay=Path('replay')
a=json.loads((reference/'benchmarks.json').read_text())
b=json.loads((replay/'benchmarks.json').read_text())
assert {(x['instance'],x['method']):x['cost'] for x in a}=={(x['instance'],x['method']):x['cost'] for x in b}
for row in a:
    assert (reference/row['matrix']).read_bytes()==(replay/row['matrix']).read_bytes(), row['matrix']
assert (reference/'poseidon.json').read_bytes()==(replay/'poseidon.json').read_bytes()
print(f'PASS: rebuilt {len(a)} identical matrices, assignments and exact cost records')
PY
