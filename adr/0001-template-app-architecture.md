# 0001. Standalone DTN Application Architecture and Secure-by-Default

- **Status**: Accepted
- **Date**: 2026-11-14

## Context
Developing satellite DTN applications or services in the `dtn-postmaster` ecosystem requires a reusable template ensuring compliance from instantiation with Cyber Resilience Act (CRA) regulatory requirements and applicable engineering rules (OWASP, zero panic/unwrap, BPSec).

## Decision
1. Structure the application around a library (`src/lib.rs`) exposing a service implementing the `hardy_bpa::services::BpaService` trait.
2. Connect the application to the local BPA daemon via gRPC (`RemoteBpa`) or a protected UNIX socket with a Bearer token passed via `--token-file`.
3. Enable strict BPSec policy by default (`VerifyPolicy::Strict`), while allowing an explicit opt-in (`allow_unauthenticated = true` or `--allow-unauthenticated`) when the user explicitly requests it.
4. Enforce the Zero Panic and Zero Unwrap rule in production (`-D clippy::unwrap_used -D clippy::panic`).

## Consequences
- Every new application instantiated from this template automatically inherits security controls and passes CRA and CI compliance audits without regression.
