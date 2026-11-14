# template-dtnpm-app — Template d'Application / Service DTN

Template GitHub officiel pour créer une nouvelle application ou un service autonome DTN BPv7 au sein de l'écosystème **DTN Postmaster** (`dtnpm`).

## 📦 Caractéristiques Embarquées

- **Service BPA Robuste** : Implémentation prête à l'emploi du trait `hardy_bpa::services::BpaService`.
- **Contrôle BPSec Intégré** : Vérification des signatures RFC 9172 avec politique stricte par défaut et option de dérogation explicite (`--allow-unauthenticated`).
- **Conformité Cyber Resilience Act (CRA)** :
  - Résolution sécurisée de jetons via `--token-file` / variable d'environnement (aucun secret dans `cmdline`).
  - Permissions strictes `0600` recommandées sur les fichiers de configuration.
  - Déclaration VEX pour `cargo-audit` et `cargo-deny`.
  - Actions GitHub CI épinglées par SHA-1 à 40 caractères avec permissions de jeton minimales.
- **Zéro Panic / Zéro Unwrap** : Code de production 100% exempt de panics.
- **ADRs Pré-configurés** : Répertoire `adr/` prêt pour consigner les décisions de conception.

---

## 🚀 Utilisation & Instanciation du Template

### Étape 1 : Créer votre dépôt à partir du template
Sur GitHub, cliquez sur **"Use this template"** ou clonez localement :
```bash
cp -r template-dtnpm-app dtnpm-app-mon-service
cd dtnpm-app-mon-service
```

### Étape 2 : Renommer le paquet Cargo
Dans `Cargo.toml` :
```toml
[package]
name = "dtnpm-app-mon-service"
description = "Mon application DTN personnalisée"

[lib]
name = "dtnpm_app_mon_service"

[[bin]]
name = "dtnpm-app-mon-service"
```

### Étape 3 : Implémenter la logique métier
Éditez `src/service.rs` et complétez la méthode `process_payload` :
```rust
impl AppService {
    pub async fn process_payload(&self, id: &hardy_bpv7::bundle::Id, payload: &[u8]) {
        // Votre traitement applicatif ici...
    }
}
```

### Étape 4 : Valider la suite de tests et de sécurité
```bash
cargo fmt --all -- --check
cargo clippy --workspace --lib --bins -- -D warnings -D clippy::unwrap_used -D clippy::panic
cargo test --all-targets
./scripts/check-security.sh --strict
```

---

## 🛠️ Commandes & Options CLI

```bash
# Lancement avec fichier de jeton sécurisé
dtnpm-app-mon-service --token-file /etc/dtnpm/token.secret --service mon-service

# Lancement via socket UNIX local
dtnpm-app-mon-service --socket /run/dtnpm/dtnpm.sock --service mon-service

# Mode dégradé explicite pour banc de test sans signature
dtnpm-app-mon-service --allow-unauthenticated
```

---

## 📚 Documentation & Références

- [AGENTS.md](AGENTS.md) : Guide de développement pour les agents d'intelligence artificielle.
- [CONTRIBUTING.md](CONTRIBUTING.md) : Guide de contribution et standards de qualité.
- [SECURITY.md](SECURITY.md) : Politique de sécurité et processus de divulgation de vulnérabilités.
- [adr/README.md](adr/README.md) : Registre des décisions d'architecture (ADRs).

## 📄 Licence

Dual-licensed under the Apache License, Version 2.0 or the MIT License. See [LICENSE.md](LICENSE.md).
