# 0001. Architecture d'application DTN autonome et sécurité par défaut

- **Statut** : Accepté
- **Date** : 2026-11-14

## Contexte
Le développement d'applications ou services DTN satellites dans l'écosystème `dtn-postmaster` nécessite un modèle réutilisable garantissant dès l'instanciation le respect des exigences réglementaires du Cyber Resilience Act (CRA) et des règles d'ingénierie applicables (OWASP, zero panic/unwrap, BPSec).

## Décision
1. Structurer l'application autour d'une bibliothèque (`src/lib.rs`) exposant un service implémentant le trait `hardy_bpa::services::BpaService`.
2. Connecter l'application au démon BPA local via gRPC (`RemoteBpa`) ou socket UNIX protégé avec jeton Bearer passé via `--token-file`.
3. Activer la politique BPSec stricte par défaut (`VerifyPolicy::Strict`), tout en permettant une dérogation explicite (`allow_unauthenticated = true` ou `--allow-unauthenticated`) lorsque l'utilisateur le demande expressément.
4. Imposer la règle Zéro Panic et Zéro Unwrap en production (`-D clippy::unwrap_used -D clippy::panic`).

## Conséquences
- Toute nouvelle application instanciée depuis ce modèle hérite automatiquement des contrôles de sécurité et passe sans régression les audits de conformité CRA et CI.
