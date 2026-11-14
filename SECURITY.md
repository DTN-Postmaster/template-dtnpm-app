# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of this project seriously and adhere to the European Cyber Resilience Act (CRA) vulnerability handling principles.

If you believe you have found a security vulnerability:

1. **Do not open a public issue.**
2. Send an email to `security@dtn-postmaster.org` (or contact the maintainers via a private GitHub Security Advisory).
3. Include detailed steps to reproduce, affected versions, and any relevant logs or traces.
4. Maintainers will acknowledge your report within 48 hours and provide updates on triage and mitigation.

## Security Practices

- **Zero Panic / Zero Unwrap**: Production code is strictly free of panics and unwraps.
- **Secure By Default**: Strict cryptographic signature validation and restricted network permissions are enabled by default.
- **VEX & Supply Chain**: Dependency vulnerabilities are audited with `cargo-audit` and `cargo-deny`, with formal VEX documentation where applicable.
