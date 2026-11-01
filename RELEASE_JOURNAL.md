# Journal des releases — API_template

<!-- release-journal-state {"dev": "64e49ae9f38148544dd05f2bd270bacc10e88eef", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-01 12:59 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `64e49ae` | 2026-11-01 12:59 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés

- Les traces OpenTelemetry sont exportées en OTLP/HTTP dès que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. `LOG_FORMAT=json` produit des journaux JSON, une ligne par événement. Une span est créée pour chaque requête, y compris celles refusées par `JwtMiddleware` (401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement

- Le journal des requêtes HTTP n’affiche plus la query string ni le `Referer`. Il ne contient que l’adresse client, la méthode, le chemin sans query string, le statut, la taille et la durée (`middleware::Logger::default()` est remplacé par `request_logger()`). Avant, la ligne de requête complète était journalisée. Le diff de `request_log.rs` est tronqué dans la source, donc ce point repose sur le `main.rs` et la description de la PR. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections

- Image d’exécution des APIs : `gcr.io/distroless/cc-debian13:nonroot`, builder et image de développement sur Debian 13 (trixie). Avant : `cc-debian12:nonroot`, dont `libssl3` n’était pas corrigée (CVE-2026-84782). Maintenant : image corrigée, sans vulnérabilité HIGH/CRITICAL corrigeable selon Trivy. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique

- Dépendances et images : digests Rust 1.99 (bookworm puis trixie), Node 26 pour le service `gdpr-marker` de `docker-compose-gdpr-marker.yml`, corrections RustSec de `cargo audit` (zstd, ctutils, rand, spin ; RUSTSEC-2026-0387 ignorée dans `.cargo/audit.toml`), actix-web 4.18.1, tokio 1.54.1, log 0.4.35, utoipa-swagger-ui 10.0.2. ([#126](https://github.com/mairie360/API_template/pull/126), [#125](https://github.com/mairie360/API_template/pull/125), [#121](https://github.com/mairie360/API_template/pull/121), [#109](https://github.com/mairie360/API_template/pull/109))
- CI, tests et documentation : mairie360/CICD passe de v4.3.0 à v4.12.1, checklist RGPD sur les PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)), test du marqueur RGPD sur les logs (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66)), test Dockerfile sur Debian 13, tests de télémétrie et documentation du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés

- Les traces OpenTelemetry sont exportées en OTLP/HTTP dès que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. `LOG_FORMAT=json` produit des journaux JSON, une ligne par événement. Une span est créée pour chaque requête, y compris celles refusées par `JwtMiddleware` (401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement

- Le journal des requêtes HTTP n’affiche plus la query string ni le `Referer`. Il ne contient que l’adresse client, la méthode, le chemin sans query string, le statut, la taille et la durée (`middleware::Logger::default()` est remplacé par `request_logger()`). Avant, la ligne de requête complète était journalisée. Le diff de `request_log.rs` est tronqué dans la source, donc ce point repose sur le `main.rs` et la description de la PR. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections

- Image d’exécution des APIs : `gcr.io/distroless/cc-debian13:nonroot`, builder et image de développement sur Debian 13 (trixie). Avant : `cc-debian12:nonroot`, dont `libssl3` n’était pas corrigée (CVE-2026-84782). Maintenant : image corrigée, sans vulnérabilité HIGH/CRITICAL corrigeable selon Trivy. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique

- Dépendances et images : digests Rust 1.99 (bookworm puis trixie), Node 26 pour le service `gdpr-marker` de `docker-compose-gdpr-marker.yml`, corrections RustSec de `cargo audit` (zstd, ctutils, rand, spin ; RUSTSEC-2026-0387 ignorée dans `.cargo/audit.toml`), actix-web 4.18.1, tokio 1.54.1, log 0.4.35, utoipa-swagger-ui 10.0.2. ([#126](https://github.com/mairie360/API_template/pull/126), [#125](https://github.com/mairie360/API_template/pull/125), [#121](https://github.com/mairie360/API_template/pull/121), [#109](https://github.com/mairie360/API_template/pull/109))
- CI, tests et documentation : mairie360/CICD passe de v4.3.0 à v4.12.1, checklist RGPD sur les PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)), test du marqueur RGPD sur les logs (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66)), test Dockerfile sur Debian 13, tests de télémétrie et documentation du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique

- CI et dépendances sans effet fonctionnel visible : mairie360/CICD passe de v4.3.0 à v4.3.2, les secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` sont transmis au workflow réutilisable pour le journal de release (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)), puis tokio 1.54.0 et utoipa-swagger-ui 10.0.2 ([#96](https://github.com/mairie360/API_template/pull/96), [#99](https://github.com/mairie360/API_template/pull/99)).
- Ces commits n’ajoutent aucun changement fonctionnel visible pour les agents ou les appelants de l’API.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
