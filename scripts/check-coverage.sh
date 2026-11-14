#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
export PATH="${HOME}/.cargo/bin:${PATH}"

MODE="${1:---summary-only}"

case "$MODE" in
    --html)
        cargo llvm-cov --workspace --html
        echo "HTML coverage report written to target/llvm-cov/html/index.html"
        ;;
    --lcov)
        cargo llvm-cov --workspace --lcov --output-path lcov.info
        echo "LCOV report written to lcov.info"
        ;;
    --json)
        cargo llvm-cov --workspace --json --output-path coverage.json
        echo "JSON coverage report written to coverage.json"
        ;;
    --summary-only|*)
        cargo llvm-cov --workspace --summary-only
        ;;
esac
