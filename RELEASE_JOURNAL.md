# Journal des releases — API_template

<!-- release-journal-state {"dev": "6e6f5bbac2c4d7546921378ba065f04ba26cb9fe", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-02 00:56 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `6e6f5bb` | 2026-11-02 00:56 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les requêtes sont tracées en OpenTelemetry : chaque requête, y compris celle refusée par le JWT (401), produit un span nommé `<MÉTHODE> <route>` qui continue le `traceparent` entrant. L'export OTLP/HTTP n'a lieu que si `OTEL_EXPORTER_OTLP_ENDPOINT` est défini, et `LOG_FORMAT=json` donne une ligne JSON par événement de log. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les journaux de requêtes ne contiennent plus la query string ni le `Referer` : ils donnent l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée. Avant, `Logger::default()` écrivait la ligne de requête complète. Les erreurs de base de données dans les logs sont décrites par SQLSTATE, contrainte, table et colonne, sans le message Postgres ni `DETAIL` (d'après la description de la PR, le diff étant tronqué). (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution : elle était `gcr.io/distroless/cc-debian12:nonroot`, dont `libssl3` n'est pas corrigée (CVE-2026-84782). Elle est maintenant `cc-debian13:nonroot`, et le builder ainsi que `development.Dockerfile` passent sur Debian 13 (trixie). (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Dépendances Cargo : `zstd`, `zstd-safe`, `zstd-sys`, `ctutils`, `rand` et `spin` mis à jour, et RUSTSEC-2026-0387 ignorée dans `.cargo/audit.toml` (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)).
- Garde-fous CI et tests : checklist RGPD sur les PR (`gdpr-pr.yml`, MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)), test du marqueur RGPD sur les journaux (MAIR-290), test `mair_741_every_image_runs_on_debian_13` (MAIR-741), test `telemetry_test.rs` (MAIR-503) et documentation du pool Postgres dans CLAUDE.md (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).
- `mairie360_api_lib` passe de 4.2.1 à 4.3.0 ([#122](https://github.com/mairie360/API_template/pull/122), [#130](https://github.com/mairie360/API_template/pull/130)). La PR #130 n'a pas de diff visible ; sa description cite MAIR-680 (remplacement du secret JWT HS256 partagé), à vérifier avant la prod.
- `mairie360/CICD` passe jusqu'à v5.2.0 en plusieurs étapes ([#100](https://github.com/mairie360/API_template/pull/100), [#105](https://github.com/mairie360/API_template/pull/105), [#110](https://github.com/mairie360/API_template/pull/110), [#119](https://github.com/mairie360/API_template/pull/119), [#127](https://github.com/mairie360/API_template/pull/127), [#128](https://github.com/mairie360/API_template/pull/128), [#129](https://github.com/mairie360/API_template/pull/129), [#131](https://github.com/mairie360/API_template/pull/131)). #131 est un passage majeur, dont seul l'appel de workflow est visible dans le diff.
- Renovate sans diff visible : tokio 1.54.x ([#96](https://github.com/mairie360/API_template/pull/96), [#109](https://github.com/mairie360/API_template/pull/109)), actix-web 4.18.x ([#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114)), log 0.4.35 ([#120](https://github.com/mairie360/API_template/pull/120)), utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)). Node 26 pour le stack `gdpr-marker`, hors production ([#121](https://github.com/mairie360/API_template/pull/121)).
- Digests des images `rust:1.99-slim-bookworm` et `rust:1.99-bookworm` mis à jour ([#102](https://github.com/mairie360/API_template/pull/102) à [#118](https://github.com/mairie360/API_template/pull/118)).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les requêtes sont tracées en OpenTelemetry : chaque requête, y compris celle refusée par le JWT (401), produit un span nommé `<MÉTHODE> <route>` qui continue le `traceparent` entrant. L'export OTLP/HTTP n'a lieu que si `OTEL_EXPORTER_OTLP_ENDPOINT` est défini, et `LOG_FORMAT=json` donne une ligne JSON par événement de log. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les journaux de requêtes ne contiennent plus la query string ni le `Referer` : ils donnent l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée. Avant, `Logger::default()` écrivait la ligne de requête complète. Les erreurs de base de données dans les logs sont décrites par SQLSTATE, contrainte, table et colonne, sans le message Postgres ni `DETAIL` (d'après la description de la PR, le diff étant tronqué). (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution : elle était `gcr.io/distroless/cc-debian12:nonroot`, dont `libssl3` n'est pas corrigée (CVE-2026-84782). Elle est maintenant `cc-debian13:nonroot`, et le builder ainsi que `development.Dockerfile` passent sur Debian 13 (trixie). (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Dépendances Cargo : `zstd`, `zstd-safe`, `zstd-sys`, `ctutils`, `rand` et `spin` mis à jour, et RUSTSEC-2026-0387 ignorée dans `.cargo/audit.toml` (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)).
- Garde-fous CI et tests : checklist RGPD sur les PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)), test du marqueur RGPD sur les journaux (MAIR-290), test `mair_741_every_image_runs_on_debian_13` (MAIR-741), test `telemetry_test.rs` (MAIR-503) et documentation du pool Postgres dans CLAUDE.md (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).
- `mairie360_api_lib` passe à 4.3.0 ([#130](https://github.com/mairie360/API_template/pull/130)) sans diff visible dans la PR ; sa description cite MAIR-680 (remplacement du secret JWT HS256 partagé), à vérifier avant la prod.
- `mairie360/CICD` passe de v4.4.0 à v5.2.0 ([#100](https://github.com/mairie360/API_template/pull/100) à [#131](https://github.com/mairie360/API_template/pull/131)), #131 étant un passage majeur.
- Renovate sans diff visible : tokio 1.54.1 ([#109](https://github.com/mairie360/API_template/pull/109)), actix-web 4.18.x ([#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114)), log 0.4.35 ([#120](https://github.com/mairie360/API_template/pull/120)), utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)). Node 26 pour le stack `gdpr-marker`, hors production ([#121](https://github.com/mairie360/API_template/pull/121)).
- Digests des images Rust mis à jour ([#102](https://github.com/mairie360/API_template/pull/102) à [#118](https://github.com/mairie360/API_template/pull/118)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Renovate sur `mairie360/CICD` : passage de v4.3.0 à v4.3.2 ([#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)), sans effet sur l'API.
- Renovate sans diff visible : tokio 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)).
- Le workflow `.github/workflows/cicd.yml` transmet `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow réutilisable CICD pour l'étape de journal des releases (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)). Pas d'effet sur l'API elle-même.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-02 00:56 UTC | dev | — | `6e6f5bb` | [run](https://github.com/mairie360/API_template/actions/runs/41369273127) |
| 2026-11-01 21:03 UTC | dev | — | `501e431` | [run](https://github.com/mairie360/API_template/actions/runs/41348977901) |
| 2026-11-01 20:56 UTC | dev | — | `fb815c6` | [run](https://github.com/mairie360/API_template/actions/runs/41348871571) |
| 2026-11-01 16:53 UTC | dev | — | `6e511b8` | [run](https://github.com/mairie360/API_template/actions/runs/41328037677) |
| 2026-11-01 12:59 UTC | dev | — | `64e49ae` | [run](https://github.com/mairie360/API_template/actions/runs/41308126451) |
| 2026-10-22 17:44 UTC | dev | — | `53d3308` | [run](https://github.com/mairie360/API_template/actions/runs/39823567624) |
| 2026-10-22 17:39 UTC | dev | — | `8cc13de` | [run](https://github.com/mairie360/API_template/actions/runs/39823502623) |
| 2026-10-22 09:56 UTC | dev | — | `48c4db0` | [run](https://github.com/mairie360/API_template/actions/runs/39757025776) |
| 2026-10-22 05:58 UTC | dev | — | `fb59f4b` | [run](https://github.com/mairie360/API_template/actions/runs/39729696870) |
| 2026-10-22 02:37 UTC | dev | — | `cde04e4` | [run](https://github.com/mairie360/API_template/actions/runs/39711278450) |
| 2026-10-22 02:31 UTC | dev | — | `d05c66f` | [run](https://github.com/mairie360/API_template/actions/runs/39711246658) |
| 2026-10-21 15:41 UTC | dev | — | `7be1696` | [run](https://github.com/mairie360/API_template/actions/runs/39626092728) |
| 2026-10-21 15:38 UTC | dev | — | `0c38529` | [run](https://github.com/mairie360/API_template/actions/runs/39625313777) |
| 2026-10-20 18:52 UTC | staging | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-20 18:46 UTC | dev | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-19 20:29 UTC | staging | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
| 2026-10-19 20:23 UTC | dev | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
<!-- release-journal-history:end -->
