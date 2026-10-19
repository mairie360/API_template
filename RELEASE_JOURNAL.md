# Journal des releases — API_template

<!-- release-journal-state {"dev": "42583728a0be8ca290c91e5ab46bcfe0b5d76537", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "42583728a0be8ca290c91e5ab46bcfe0b5d76537", "updated": {"dev": "2026-10-19 20:23 UTC", "staging": "2026-10-19 20:29 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `4258372` | 2026-10-19 20:29 UTC |
| dev | — | `4258372` | 2026-10-19 20:23 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Technique
- Pipeline CI/CD : passage du workflow réutilisable `mairie360/CICD` de `v4.2.2` à `v4.3.1`, qui ajoute l'étape de journal des releases, et transmission des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` dans `.github/workflows/cicd.yml` pour que cette étape fonctionne. (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94), [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97))
- Mise à jour de la dépendance Rust `tokio` de `1.53.3` à `1.54.0`, sans changement de comportement de l'API. ([#96](https://github.com/mairie360/API_template/pull/96))

## Différence dev → staging (en dev, pas encore en staging)

_Rien._

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Pipeline CI/CD : passage du workflow réutilisable `mairie360/CICD` de `v4.2.2` à `v4.3.1`, qui ajoute l'étape de journal des releases, et transmission des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` dans `.github/workflows/cicd.yml` pour que cette étape fonctionne. (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94), [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97))
- Mise à jour de la dépendance Rust `tokio` de `1.53.3` à `1.54.0`, sans changement de comportement de l'API. ([#96](https://github.com/mairie360/API_template/pull/96))

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-19 20:29 UTC | staging | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
| 2026-10-19 20:23 UTC | dev | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
<!-- release-journal-history:end -->
