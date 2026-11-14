# Guide de Contribution — `template-dtnpm-app`

Ce document présente les directives de contribution et standards de qualité pour les applications basées sur `template-dtnpm-app`.

---

## 1. Principes Fondamentaux

- **Zéro Panic / Zéro Unwrap** : Tout le code de production (`--lib`, `--bins`) doit être strictement exempt d'appels risquant de provoquer un arrêt brutal du processus.
- **Sécurité et Confidentialité (CRA)** :
  - Ne jamais accepter de secrets en clair dans les paramètres CLI (`/proc/<pid>/cmdline`).
  - Imposer des permissions `0600` sur tout fichier de configuration contenant des clés ou tokens.
- **Traçabilité (ADR)** :
  - Tout changement de conception doit faire l'objet d'un fichier ADR sous `adr/`.

---

## 2. Validation Avant Commit

```bash
cargo fmt --all -- --check
cargo clippy --workspace --lib --bins -- -D warnings -D clippy::unwrap_used -D clippy::panic
cargo test --all-targets
./scripts/check-security.sh --strict
```
