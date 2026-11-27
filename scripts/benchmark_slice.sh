#!/usr/bin/env bash
# GE-0020 benchmark harness: run the frozen end-to-end slice and write a JSON evidence record.
# Usage: scripts/benchmark_slice.sh [output.json]
set -euo pipefail
OUT="${1:-engineering/reports/slice-latest.json}"
mkdir -p "$(dirname "$OUT")"
GENESIS_SLICE_REPORT="$OUT" cargo test -p genesis-core --locked --test vertical_slice -- --nocapture
echo "Benchmark evidence written to $OUT"
