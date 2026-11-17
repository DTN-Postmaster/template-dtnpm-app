# Agent Guide — `template-dtnpm-app`

## 🎯 Mission
Ready-to-use GitHub template for creating a new standalone DTN BPv7 application or service in the **DTN Postmaster** (`dtnpm`) ecosystem. It implements a BPA service connected via gRPC or UNIX socket, secure configuration management, BPSec (RFC 9172) signing/verification, and Cyber Resilience Act (CRA) compliance.

## Documentation Language
English must be used as the language for all documentation, unless the file is suffixed with `.fr.md` or another language is explicitly requested in the prompt.

---

## 🚀 How to Instantiate This Template

To create a new project based on this template:
1. Rename the directory: `cp -r template-dtnpm-app dtnpm-app-<my-service>` (or use the GitHub "Use this template" button).
2. In `Cargo.toml`: replace `template-dtnpm-app` and `template_dtnpm_app` with the desired name (e.g., `dtnpm-app-weather`).
3. Update `README.md` and `src/lib.rs` with the description and logic of the new service.
4. Add your application logic in `src/service.rs` (`process_payload`).
5. Run `./scripts/install-hooks.sh` to enable local security checks.

---

## 🛡️ Security Standards & Requirements

### 1. Cyber Resilience Act (CRA)
- **Secure-by-Default**:
  - Strict BPSec policy by default (`VerifyPolicy::Strict`).
  - Any tolerance for unsigned packets requires explicit activation of a flag (`--allow-unauthenticated` or `allow_unauthenticated = true`).
  - No secrets on the command line: systematically use `--token-file` / `DTN_HDY_TOKEN_FILE`.
- **Supply-Chain & VEX**:
  - Automated audits with `cargo-audit` and `cargo-deny`. `not_affected` VEX declaration for `RUSTSEC-2026-0459`.
  - GitHub CI actions pinned by 40-character SHA-1 with `permissions: contents: read`.
- **Security Policy**: Documented in [SECURITY.md](SECURITY.md).

### 2. Rust Engineering Standards
- **Zero Panic / Zero Unwrap Rule**:
  - Production code 100% free of `.unwrap()`, `.expect()`, and `panic!`.
  - Strict linter: `-D warnings -D clippy::unwrap_used -D clippy::panic`.
- **Mandatory ADRs**:
  - Record every design change in `adr/` and index it in [adr/README.md](adr/README.md).

---

## ⚡ Critical Verification Commands

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --lib --bins -- -D warnings -D clippy::unwrap_used -D clippy::panic`
- `cargo test --all-targets`
- `./scripts/check-security.sh --strict`
- `./scripts/check-coverage.sh --summary-only`
