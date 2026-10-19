# Journal des releases — API_template

<!-- release-journal-state {"dev": "42583728a0be8ca290c91e5ab46bcfe0b5d76537", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "updated": {"dev": "2026-10-19 20:23 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `9e6fe42` | — |
| dev | — | `4258372` | 2026-10-19 20:23 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Technique
- Pipeline CI/CD : passage du workflow réutilisable `mairie360/CICD` de `v4.2.2` à `v4.3.1`, qui ajoute l'étape de journal des releases, et transmission des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` dans `.github/workflows/cicd.yml`. (MAIR-571, [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#94](https://github.com/mairie360/API_template/pull/94))
- Mise à jour de la dépendance `tokio` de `1.53.3` à `1.54.0`. ([#96](https://github.com/mairie360/API_template/pull/96))

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Pipeline CI/CD : passage du workflow réutilisable `mairie360/CICD` de `v4.2.2` à `v4.3.1`, qui ajoute l'étape de journal des releases, et transmission des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` dans `.github/workflows/cicd.yml`. (MAIR-571, [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#94](https://github.com/mairie360/API_template/pull/94))
- Mise à jour de la dépendance `tokio` de `1.53.3` à `1.54.0`. ([#96](https://github.com/mairie360/API_template/pull/96))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-19 20:23 UTC | dev | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
<!-- release-journal-history:end -->
