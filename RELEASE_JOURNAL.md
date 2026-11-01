# Journal des releases — API_template

<!-- release-journal-state {"dev": "6e511b8493192e6713b378c91b02452fc9d60446", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-01 16:53 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `6e511b8` | 2026-11-01 16:53 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les traces OpenTelemetry sont exportées en OTLP/HTTP uniquement si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini ; `OTEL_SDK_DISABLED=true` les coupe. Chaque requête a un span racine nommé `<MÉTHODE> <route>`, y compris les requêtes refusées par le JWT, qui reprend le `traceparent` entrant. Les logs démarrent via `init_telemetry` : texte coloré par défaut, JSON avec `LOG_FORMAT=json`, niveau fixé par `RUST_LOG` (défaut `info`). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les journaux de requêtes n'affichent plus la query string ni le `Referer` : la ligne de requête est remplacée par la méthode et le chemin seuls, avec l'adresse client, le statut, la taille et la durée. Les erreurs de `mairie360_api_lib` décrivent la base par SQLSTATE, contrainte, table et colonne, sans le message Postgres ni `DETAIL`, et les messages serde ne citent plus la valeur reçue. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66) ; diff tronqué, `src/request_log.rs` non visible en entier)

#### Corrections
- Image d'exécution : avant, `gcr.io/distroless/cc-debian12` avec un `libssl3` non corrigé (CVE-2026-84782) ; maintenant `cc-debian13:nonroot`, avec le builder et `development.Dockerfile` sur Debian 13 (trixie). (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Alertes RustSec de `cargo audit` : avant, crypto-bigint, ctutils, zstd, rand et spin signalés ; maintenant les versions corrigées sont visées dans `Cargo.lock` (selon la description, le lockfile n'est pas dans le diff fourni) et RUSTSEC-2026-0387 est ignorée avec motif dans `.cargo/audit.toml`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))

#### Technique
- Contrôle RGPD sur chaque PR : `gdpr-pr.yml` appelle le workflow `gdpr-pr-cicd.yml` de CICD (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)).
- Mises à jour de mairie360/CICD de v4.5.0 à v4.12.2 (`cicd.yml`, `gdpr-pr.yml`) : [#119](https://github.com/mairie360/API_template/pull/119), [#127](https://github.com/mairie360/API_template/pull/127), [#128](https://github.com/mairie360/API_template/pull/128).
- Digests des images `rust:1.99` mis à jour par Renovate (#102, #103, #104, #107, #108, #111, #112, #115, #116, #117, #118), avant le passage à Debian 13.
- Dépendances Rust : actix-web 4.18.0 et 4.18.1 ([#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114)), tokio 1.54.0 et 1.54.1 ([#96](https://github.com/mairie360/API_template/pull/96), [#109](https://github.com/mairie360/API_template/pull/109)), utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)), log 0.4.35 en dev-dependency ([#120](https://github.com/mairie360/API_template/pull/120)), mairie360_api_lib 4.2.1 ([#122](https://github.com/mairie360/API_template/pull/122), diff vide dans le jeu fourni).
- Image Node 24 → 26 dans le stack `docker-compose-gdpr-marker.yml` ([#121](https://github.com/mairie360/API_template/pull/121)).
- Tests et outillage MAIR-290 : `gdpr-marker.yaml`, `gdpr_marker_test.sh`, `docker-compose-gdpr-marker.yml`, `tests/telemetry_test.rs`, test `mair_741_every_image_runs_on_debian_13`, et documentation des pools Postgres dans `CLAUDE.md` (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les traces OpenTelemetry sont exportées en OTLP/HTTP uniquement si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini ; `OTEL_SDK_DISABLED=true` les coupe. Chaque requête a un span racine nommé `<MÉTHODE> <route>`, y compris les requêtes refusées par le JWT, qui reprend le `traceparent` entrant. Les logs démarrent via `init_telemetry` : texte coloré par défaut, JSON avec `LOG_FORMAT=json`, niveau fixé par `RUST_LOG` (défaut `info`). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les journaux de requêtes n'affichent plus la query string ni le `Referer` : la ligne de requête est remplacée par la méthode et le chemin seuls, avec l'adresse client, le statut, la taille et la durée. Les erreurs de `mairie360_api_lib` décrivent la base par SQLSTATE, contrainte, table et colonne, sans le message Postgres ni `DETAIL`, et les messages serde ne citent plus la valeur reçue. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66) ; diff tronqué, `src/request_log.rs` non visible en entier)

#### Corrections
- Image d'exécution : avant, `gcr.io/distroless/cc-debian12` avec un `libssl3` non corrigé (CVE-2026-84782) ; maintenant `cc-debian13:nonroot`, avec le builder et `development.Dockerfile` sur Debian 13 (trixie). (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Alertes RustSec de `cargo audit` : avant, crypto-bigint, ctutils, zstd, rand et spin signalés ; maintenant les versions corrigées sont visées dans `Cargo.lock` (selon la description, le lockfile n'est pas dans le diff fourni) et RUSTSEC-2026-0387 est ignorée avec motif dans `.cargo/audit.toml`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))

#### Technique
- Contrôle RGPD sur chaque PR : `gdpr-pr.yml` appelle le workflow `gdpr-pr-cicd.yml` de CICD (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)).
- Mises à jour de mairie360/CICD de v4.4.0 à v4.12.2 : [#100](https://github.com/mairie360/API_template/pull/100), [#101](https://github.com/mairie360/API_template/pull/101), [#105](https://github.com/mairie360/API_template/pull/105), [#110](https://github.com/mairie360/API_template/pull/110), [#119](https://github.com/mairie360/API_template/pull/119), [#127](https://github.com/mairie360/API_template/pull/127), [#128](https://github.com/mairie360/API_template/pull/128).
- Digests des images `rust:1.99` mis à jour par Renovate (#102, #103, #104, #107, #108, #111, #112, #115, #116, #117, #118), avant le passage à Debian 13.
- Dépendances Rust : actix-web 4.18.0 et 4.18.1 ([#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114)), tokio 1.54.1 ([#109](https://github.com/mairie360/API_template/pull/109)), log 0.4.35 en dev-dependency ([#120](https://github.com/mairie360/API_template/pull/120)), mairie360_api_lib 4.2.1 ([#122](https://github.com/mairie360/API_template/pull/122), diff vide dans le jeu fourni).
- Image Node 24 → 26 dans le stack `docker-compose-gdpr-marker.yml` ([#121](https://github.com/mairie360/API_template/pull/121)).
- Tests et outillage MAIR-290 : `gdpr-marker.yaml`, `gdpr_marker_test.sh`, `docker-compose-gdpr-marker.yml`, `tests/telemetry_test.rs`, test `mair_741_every_image_runs_on_debian_13`, et documentation des pools Postgres dans `CLAUDE.md` (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Mises à jour de mairie360/CICD v4.3.0, v4.3.1 et v4.3.2 ([#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)).
- Journal des releases (MAIR-571) : `cicd.yml` transmet explicitement `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow reusable, sans effet sur l'API ([#94](https://github.com/mairie360/API_template/pull/94)).
- Dépendances Rust : utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)) et tokio 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)), diffs vides dans le jeu fourni.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
