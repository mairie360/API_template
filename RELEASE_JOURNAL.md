# Journal des releases — API_template

<!-- release-journal-state {"dev": "53d33085df693daff9cbd9b71d3de2fdea21e60b", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-10-22 17:44 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `53d3308` | 2026-10-22 17:44 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les logs passent par la télémétrie partagée de `mairie360_api_lib` (`init_telemetry`) : texte coloré par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export OpenTelemetry des traces en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini ; `OTEL_SDK_DISABLED=true` le coupe. Un exporteur en erreur est journalisé sans arrêter l'API. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Chaque requête produit un span racine nommé `<MÉTHODE> <route>`, y compris les requêtes refusées par `JwtMiddleware` (401), et reprend le `traceparent` entrant. Le query string et l'adresse client ne sont pas exportés. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les logs ne sont plus produits par `tracing_subscriber::fmt` mais par le format partagé de la lib ; `RUST_LOG` reste pris en compte (défaut `info`). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Images Docker épinglées par digest mises à jour (`Dockerfile` et `development.Dockerfile`, `rust:1.99` slim et bookworm) : quatre PR Renovate, sans changement fonctionnel. ([#102](https://github.com/mairie360/API_template/pull/102), [#103](https://github.com/mairie360/API_template/pull/103), [#104](https://github.com/mairie360/API_template/pull/104), [#107](https://github.com/mairie360/API_template/pull/107), [#108](https://github.com/mairie360/API_template/pull/108), [#111](https://github.com/mairie360/API_template/pull/111), [#112](https://github.com/mairie360/API_template/pull/112))
- Workflow `mairie360/CICD` passé de v4.2.2 à v4.5.0 en plusieurs PR Renovate, et transmission des secrets du journal de release (`ANTHROPIC_API_KEY`, `RELEASE_APP_ID`, `RELEASE_APP_PK`) : CI uniquement. ([#94](https://github.com/mairie360/API_template/pull/94), [#95](https://github.com/mairie360/API_template/pull/95), [#110](https://github.com/mairie360/API_template/pull/110))
- `tracing-subscriber` passe en dev-dependency ; ajout de `tests/telemetry_test.rs` (2 tests : span d'une requête continuant le `traceparent`, span d'un 401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Documentation des variables `DB_POOL_MIN_CONNECTIONS`, `DB_POOL_MAX_CONNECTIONS` et `DB_POOL_IDLE_TIMEOUT_SECS` dans `CLAUDE.md`, sans changement de code : la lib 4.2.0 les lit déjà. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Mises à jour de dépendances sans diff visible : tokio 1.54.0 et 1.54.1, utoipa-swagger-ui 10.0.2.

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les logs passent par la télémétrie partagée de `mairie360_api_lib` (`init_telemetry`) : texte coloré par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export OpenTelemetry des traces en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini ; `OTEL_SDK_DISABLED=true` le coupe. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Chaque requête produit un span racine, y compris les requêtes refusées par `JwtMiddleware` (401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les logs ne sont plus produits par `tracing_subscriber::fmt` mais par le format partagé de la lib ; `RUST_LOG` reste pris en compte. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Images Docker épinglées par digest mises à jour (`Dockerfile` et `development.Dockerfile`) : sept PR Renovate, sans changement fonctionnel.
- Workflow `mairie360/CICD` passé de v4.4.0 à v4.5.0 en plusieurs PR Renovate : CI uniquement. ([#100](https://github.com/mairie360/API_template/pull/100), [#101](https://github.com/mairie360/API_template/pull/101), [#105](https://github.com/mairie360/API_template/pull/105), [#110](https://github.com/mairie360/API_template/pull/110))
- Documentation des variables `DB_POOL_*` dans `CLAUDE.md`, sans changement de code. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Ajout de `tests/telemetry_test.rs` et passage de `tracing-subscriber` en dev-dependency. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Mise à jour de dépendance Rust sans diff visible : tokio 1.54.1 (le 1.54.0 est dans la plage staging, pas ici).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Workflow `mairie360/CICD` passé de v4.3.0 à v4.3.2 en trois PR Renovate, et transmission des secrets du journal de release (`ANTHROPIC_API_KEY`, `RELEASE_APP_ID`, `RELEASE_APP_PK`) : CI uniquement, sans effet sur l'API. ([#94](https://github.com/mairie360/API_template/pull/94), [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98))
- Mises à jour de dépendances sans diff visible : utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)) et tokio 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
