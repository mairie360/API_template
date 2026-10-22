# Journal des releases — API_template

<!-- release-journal-state {"dev": "48c4db04baccb15eba7deff03c7ad08a82a482a6", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-10-22 09:56 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `48c4db0` | 2026-10-22 09:56 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés

- Export des traces OpenTelemetry en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini ; `OTEL_SDK_DISABLED=true` le coupe. Chaque requête produit un span nommé `<METHOD> <route>` qui continue un `traceparent` entrant. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Format de logs JSON (une ligne par événement) avec `LOG_FORMAT=json` ; sans cette variable, texte coloré. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement

- Les logs passent désormais par `init_telemetry` de la lib : le niveau reste piloté par `RUST_LOG` (défaut `info`), mais le format et le span de requête masqué des logs changent. Les requêtes refusées par `JwtMiddleware` (401) ont maintenant un span. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique

- Dépendances : tokio passe de 1.54.0 à 1.54.1 (revert d'un changement sur les timers, selon la PR), `utoipa-swagger-ui` en 10.0.2, images Rust de base épinglées par digest (Dockerfile et development.Dockerfile), CICD de v4.2.2 à v4.5.0 (workflow et tests d'intégration). ([#96](https://github.com/mairie360/API_template/pull/96), [#109](https://github.com/mairie360/API_template/pull/109), [#110](https://github.com/mairie360/API_template/pull/110))
- Télémétrie : `tracing-subscriber` passe en dev-dependency, tests `telemetry_test.rs` ajoutés ; documentation du pool Postgres dans CLAUDE.md (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)). Secrets de release transmis au workflow CICD (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés

- Export des traces OpenTelemetry en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini ; `OTEL_SDK_DISABLED=true` le coupe. Chaque requête produit un span nommé `<METHOD> <route>` qui continue un `traceparent` entrant. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Format de logs JSON (une ligne par événement) avec `LOG_FORMAT=json` ; sans cette variable, texte coloré. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement

- Les logs passent désormais par `init_telemetry` de la lib : le niveau reste piloté par `RUST_LOG` (défaut `info`), mais le format et le span de requête masqué des logs changent. Les requêtes refusées par `JwtMiddleware` (401) ont maintenant un span. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique

- Dépendances et CI : tokio 1.54.1, images Rust épinglées par digest (Dockerfile et development.Dockerfile), CICD de v4.4.0 à v4.5.0. ([#109](https://github.com/mairie360/API_template/pull/109), [#110](https://github.com/mairie360/API_template/pull/110))
- Télémétrie : `tracing-subscriber` passe en dev-dependency, tests `telemetry_test.rs` ajoutés ; documentation du pool Postgres dans CLAUDE.md (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique

- Dépendances et CI : `utoipa-swagger-ui` en 10.0.2, tokio 1.54.0, CICD de v4.3.1 à v4.3.2 puis v4.4.3 ; ajout des secrets de release au workflow CICD (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)). ([#84055a3](https://github.com/mairie360/API_template/pull/99), [#96](https://github.com/mairie360/API_template/pull/96), [#98](https://github.com/mairie360/API_template/pull/98), [#105](https://github.com/mairie360/API_template/pull/105))
- Aucun changement fonctionnel côté API : seules des mises à jour de dépendances et de configuration de pipeline.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
