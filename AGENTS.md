# Agent Guide — `template-dtnpm-app`

## 🎯 Mission
Modèle GitHub prêt à l'emploi pour créer une nouvelle application ou un nouveau service autonome DTN BPv7 dans l'écosystème **DTN Postmaster** (`dtnpm`). Il implémente un service BPA connecté via gRPC ou socket UNIX, la gestion des configurations sécurisées, la signature/vérification BPSec (RFC 9172) et la conformité Cyber Resilience Act (CRA).

---

## 🚀 Comment Instancier ce Template

Pour créer un nouveau projet basé sur ce template :
1. Renommer le dossier : `cp -r template-dtnpm-app dtnpm-app-<mon-service>` (ou utiliser le bouton GitHub "Use this template").
2. Dans `Cargo.toml` : remplacer `template-dtnpm-app` et `template_dtnpm_app` par le nom souhaité (ex: `dtnpm-app-weather`).
3. Mettre à jour `README.md` et `src/lib.rs` avec la description et logique du nouveau service.
4. Ajouter votre logique applicative dans `src/service.rs` (`process_payload`).
5. Exécuter `./scripts/install-hooks.sh` pour activer les contrôles de sécurité locaux.

---

## 🛡️ Référentiels et Exigences de Sécurité

### 1. Cyber Resilience Act (CRA)
- **Sécurité par défaut (Secure-by-Default)** :
  - Politique BPSec stricte par défaut (`VerifyPolicy::Strict`).
  - Toute tolérance de paquets non signés nécessite l'activation explicite d'un drapeau (`--allow-unauthenticated` ou `allow_unauthenticated = true`).
  - Aucun secret en ligne de commande : utiliser systématiquement `--token-file` / `DTN_HDY_TOKEN_FILE`.
- **Supply-Chain & VEX** :
  - Audits automatisés avec `cargo-audit` et `cargo-deny`. Déclaration VEX `not_affected` pour `RUSTSEC-2026-0459`.
  - Actions CI GitHub épinglées par SHA-1 de 40 caractères avec `permissions: contents: read`.
- **Politique de sécurité** : Documentée dans [SECURITY.md](SECURITY.md).

### 2. Standards d'Ingénierie Rust
- **Règle Zéro Panic / Zéro Unwrap** :
  - Code de production 100% exempt de `.unwrap()`, `.expect()` et `panic!`.
  - Linter strict : `-D warnings -D clippy::unwrap_used -D clippy::panic`.
- **ADRs Obligatoires** :
  - Consigner toute évolution de conception dans `adr/` et indexer dans [adr/README.md](adr/README.md).

---

## ⚡ Commandes Critiques de Vérification

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --lib --bins -- -D warnings -D clippy::unwrap_used -D clippy::panic`
- `cargo test --all-targets`
- `./scripts/check-security.sh --strict`
- `./scripts/check-coverage.sh --summary-only`
