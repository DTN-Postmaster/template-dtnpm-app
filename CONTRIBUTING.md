# Contributing Guide — `template-dtnpm-app`

This document outlines the contribution guidelines and quality standards for applications based on `template-dtnpm-app`.

---

## 1. Core Principles

- **Zero Panic / Zero Unwrap**: All production code (`--lib`, `--bins`) must be strictly free of calls that could cause an abrupt process termination.
- **Security and Privacy (CRA)**:
  - Never accept cleartext secrets in CLI parameters (`/proc/<pid>/cmdline`).
  - Enforce `0600` permissions on any configuration file containing keys or tokens.
- **Traceability (ADR)**:
  - Every design change must be documented in an ADR file under `adr/`.

---

## 2. Pre-Commit Validation

```bash
cargo fmt --all -- --check
cargo clippy --workspace --lib --bins -- -D warnings -D clippy::unwrap_used -D clippy::panic
cargo test --all-targets
./scripts/check-security.sh --strict
```
