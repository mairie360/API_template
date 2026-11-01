# Journal des releases — API_template

<!-- release-journal-state {"dev": "fb815c6526784876416178830ab591d102908e96", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-01 20:56 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `fb815c6` | 2026-11-01 20:56 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les logs passent par la télémétrie partagée `init_telemetry` : texte coloré par défaut, une ligne JSON par événement avec `LOG_FORMAT=json`. L'export OpenTelemetry (OTLP/HTTP) reste désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` n'est pas renseigné. Chaque requête produit un span `<METHOD> <route>` qui continue le `traceparent` entrant, y compris les requêtes refusées par `JwtMiddleware` (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Corrections
- Logs de requête : avant, `middleware::Logger::default()` journalisait la query string complète (valeurs saisies par les agents, comme des noms ou des e-mails) et l'en-tête `Referer`. Maintenant, `request_logger()` ne journalise que l'adresse du client, la méthode, le chemin sans query string, le statut, la taille et la durée. Les erreurs de la lib sont décrites par type et contexte, sans la valeur reçue (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Technique
- Test de marqueur RGPD ajouté à la CI (`gdpr-marker.yaml`, `docker-compose-gdpr-marker.yml`, `gdpr_marker_test.sh`) et workflow `gdpr-pr.yml` qui vérifie la checklist RGPD des PR (MAIR-290 et MAIR-295, [#66](https://github.com/mairie360/API_template/pull/66), [#70](https://github.com/mairie360/API_template/pull/70)) ; `tests/telemetry_test.rs` ajouté, `tracing-subscriber` et `log` passés en dev-dependencies
- Image d'exécution passée en `distroless/cc-debian13` avec un builder `trixie`, test `mair_741_every_image_runs_on_debian_13` (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Alertes RustSec corrigées (zstd, ctutils, rand, spin) et RUSTSEC-2026-0387 ignorée avec justification dans `.cargo/audit.toml` (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Documentation des variables `DB_POOL_*` dans `CLAUDE.md`, sans changement de code (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Mises à jour Renovate : `mairie360/CICD` jusqu'à v4.13.0, images Rust (digests), `actix-web` 4.18.1, `tokio` 1.54.1, `log` 0.4.35, `mairie360_api_lib` 4.2.1 (dont le diff est vide dans le dépôt fourni), Node 26 pour le conteneur de marqueur RGPD ([#109](https://github.com/mairie360/API_template/pull/109), [#114](https://github.com/mairie360/API_template/pull/114), [#120](https://github.com/mairie360/API_template/pull/120), [#121](https://github.com/mairie360/API_template/pull/121), [#122](https://github.com/mairie360/API_template/pull/122))

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les logs passent par la télémétrie partagée `init_telemetry` : texte coloré par défaut, une ligne JSON par événement avec `LOG_FORMAT=json`. L'export OpenTelemetry (OTLP/HTTP) reste désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` n'est pas renseigné. Chaque requête produit un span `<METHOD> <route>` qui continue le `traceparent` entrant, y compris les requêtes refusées par `JwtMiddleware` (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Corrections
- Logs de requête : avant, `middleware::Logger::default()` journalisait la query string complète (valeurs saisies par les agents, comme des noms ou des e-mails) et l'en-tête `Referer`. Maintenant, `request_logger()` ne journalise que l'adresse du client, la méthode, le chemin sans query string, le statut, la taille et la durée. Les erreurs de la lib sont décrites par type et contexte, sans la valeur reçue (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Technique
- Test de marqueur RGPD ajouté à la CI (`gdpr-marker.yaml`, `docker-compose-gdpr-marker.yml`, `gdpr_marker_test.sh`) et workflow `gdpr-pr.yml` qui vérifie la checklist RGPD des PR (MAIR-290 et MAIR-295, [#66](https://github.com/mairie360/API_template/pull/66), [#70](https://github.com/mairie360/API_template/pull/70)) ; `tests/telemetry_test.rs` ajouté, `tracing-subscriber` et `log` passés en dev-dependencies
- Image d'exécution passée en `distroless/cc-debian13` avec un builder `trixie`, test `mair_741_every_image_runs_on_debian_13` (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Alertes RustSec corrigées (zstd, ctutils, rand, spin) et RUSTSEC-2026-0387 ignorée avec justification dans `.cargo/audit.toml` (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Documentation des variables `DB_POOL_*` dans `CLAUDE.md`, sans changement de code (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Mises à jour Renovate : `mairie360/CICD` jusqu'à v4.13.0, images Rust (digests), `actix-web` 4.18.1, `log` 0.4.35, `mairie360_api_lib` 4.2.1 (dont le diff est vide dans le dépôt fourni), Node 26 pour le conteneur de marqueur RGPD ([#120](https://github.com/mairie360/API_template/pull/120), [#121](https://github.com/mairie360/API_template/pull/121), [#122](https://github.com/mairie360/API_template/pull/122), [#114](https://github.com/mairie360/API_template/pull/114))

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Mises à jour de `utoipa-swagger-ui` 10.0.1 vers 10.0.2, de `tokio` 1.53.3 vers 1.54.0 et de `mairie360/CICD` (v4.3.0 à v4.3.2) : aucun changement visible par les appelants ([#96](https://github.com/mairie360/API_template/pull/96), [#99](https://github.com/mairie360/API_template/pull/99))
- Le workflow `cicd.yml` transmet désormais à `mairie360/CICD` les secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal des releases (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94))

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
