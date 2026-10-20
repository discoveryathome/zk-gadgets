#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export CARGO_HOME="$PWD/.cargo-home"
export RAYON_NUM_THREADS=1
cargo run --release --locked -- "${1:-replay}"
python3 scripts/test_verifier.py
python3 scripts/verify.py --results "${1:-replay}" --output "${1:-replay}/verification.json"
python3 scripts/summarize.py --results "${1:-replay}"
