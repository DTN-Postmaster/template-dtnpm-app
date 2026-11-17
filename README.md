# template-dtnpm-app — DTN Application / Service Template

Official GitHub template for creating a new standalone DTN BPv7 application or service within the **DTN Postmaster** (`dtnpm`) ecosystem.

## 📦 Built-in Features

- **Robust BPA Service**: Ready-to-use implementation of the `hardy_bpa::services::BpaService` trait.
- **Integrated BPSec Control**: RFC 9172 signature verification with strict policy by default and explicit override option (`--allow-unauthenticated`).
- **Cyber Resilience Act (CRA) Compliance**:
  - Secure token resolution via `--token-file` / environment variable (no secrets in `cmdline`).
  - Strict `0600` permissions recommended on configuration files.
  - VEX declaration for `cargo-audit` and `cargo-deny`.
  - GitHub CI actions pinned by 40-character SHA-1 with minimal token permissions.
- **Zero Panic / Zero Unwrap**: Production code 100% free of panics.
- **Pre-configured ADRs**: `adr/` directory ready to record design decisions.

---

## 🚀 Usage & Template Instantiation

### Step 1: Create your repository from the template
On GitHub, click **"Use this template"** or clone locally:
```bash
cp -r template-dtnpm-app dtnpm-app-my-service
cd dtnpm-app-my-service
```

### Step 2: Rename the Cargo package
In `Cargo.toml`:
```toml
[package]
name = "dtnpm-app-my-service"
description = "My custom DTN application"

[lib]
name = "dtnpm_app_my_service"

[[bin]]
name = "dtnpm-app-my-service"
```

### Step 3: Implement business logic
Edit `src/service.rs` and complete the `process_payload` method:
```rust
impl AppService {
    pub async fn process_payload(&self, id: &hardy_bpv7::bundle::Id, payload: &[u8]) {
        // Your application processing here...
    }
}
```

### Step 4: Validate the test and security suite
```bash
cargo fmt --all -- --check
cargo clippy --workspace --lib --bins -- -D warnings -D clippy::unwrap_used -D clippy::panic
cargo test --all-targets
./scripts/check-security.sh --strict
```

---

## 🛠️ CLI Commands & Options

```bash
# Launch with secure token file
dtnpm-app-my-service --token-file /etc/dtnpm/token.secret --service my-service

# Launch via local UNIX socket
dtnpm-app-my-service --socket /run/dtnpm/dtnpm.sock --service my-service

# Explicit degraded mode for unsigned testbench
dtnpm-app-my-service --allow-unauthenticated
```

---

## 📚 Documentation & References

- [AGENTS.md](AGENTS.md): Development guide for AI agents.
- [CONTRIBUTING.md](CONTRIBUTING.md): Contribution guide and quality standards.
- [SECURITY.md](SECURITY.md): Security policy and vulnerability disclosure process.
- [adr/README.md](adr/README.md): Architecture Decision Records (ADRs) index.

## 📄 License

Dual-licensed under the Apache License, Version 2.0 or the MIT License. See [LICENSE.md](LICENSE.md).
