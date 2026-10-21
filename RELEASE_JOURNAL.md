# Journal des releases — API_template

<!-- release-journal-state {"dev": "7be1696cac1c3fd89d17195012c0dc4347d6f480", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-10-21 15:41 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `7be1696` | 2026-10-21 15:41 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les logs passent par le format commun à toutes les APIs : texte coloré par défaut (`RUST_LOG`, défaut `info`), une ligne JSON par événement avec `LOG_FORMAT=json`. Les variables sont documentées, commentées, dans `docker-compose.yml`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export des traces en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini ; `OTEL_SDK_DISABLED=true` le coupe, et `OTEL_SERVICE_NAME` vaut `template-api` par défaut. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Chaque requête produit un span racine nommé `<METHOD> <route>`, y compris celles refusées par `JwtMiddleware` (span avec statut `401`). Il continue le `traceparent` entrant et n'exporte ni la query string ni l'adresse client. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Le format des logs change : la mise en place `tracing_subscriber::fmt()` de `main.rs` est remplacée par `init_telemetry("template-api")` de la lib, qui fournit le texte coloré ou le JSON. Les logs de requêtes suivent ce format. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Mise en place de la télémétrie : `mairie360_api_lib` passe à `features = ["otel"]`, `tracing-subscriber` devient dev-dependency, et `tests/telemetry_test.rs` ajoute deux tests (span continuant le `traceparent`, span d'une requête `401`). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- `CLAUDE.md` documente les variables `DB_POOL_MIN_CONNECTIONS`, `DB_POOL_MAX_CONNECTIONS` et `DB_POOL_IDLE_TIMEOUT_SECS` (défauts 1, 10 et 600), lues par la lib 4.2.0. Aucun code ne change dans le template. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Workflow réutilisable `mairie360/CICD` remonté de `v4.2.2` à `v4.4.3` par Renovate (#95, #97, #98, #100, #101, #105), et `cicd.yml` transmet désormais `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).
- Images Rust épinglées par digest mises à jour (`Dockerfile` et `development.Dockerfile`) : #102, #103, #104.
- Dépendances mises à jour : `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)) et `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96), diff absent dans le journal).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les logs passent par le format commun à toutes les APIs : texte coloré par défaut, une ligne JSON par événement avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export des traces en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` n'est pas défini. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Chaque requête produit un span racine `<METHOD> <route>`, y compris les requêtes refusées par `JwtMiddleware` (`401`). Les query strings et adresses client ne sont pas exportées. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Le format des logs change : `init_telemetry("template-api")` de la lib remplace la mise en place `tracing_subscriber::fmt()` de `main.rs`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Télémétrie : `features = ["otel"]` sur `mairie360_api_lib`, `tracing-subscriber` passe en dev-dependency, et ajout de `tests/telemetry_test.rs`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- `CLAUDE.md` documente les variables `DB_POOL_*` lues par la lib 4.2.0 (documentation seule). (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Workflow `mairie360/CICD` passé de `v4.3.2` à `v4.4.3` (Renovate, #100, #101, #105).
- Images Rust épinglées par digest mises à jour dans `Dockerfile` et `development.Dockerfile` (#102, #103, #104).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Workflow `mairie360/CICD` passé de `v4.2.2` à `v4.3.2` (Renovate, #95, #97, #98) ; `cicd.yml` transmet désormais `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).
- Dépendances mises à jour : `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)) et `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96), diff absent dans le journal).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-21 15:41 UTC | dev | — | `7be1696` | [run](https://github.com/mairie360/API_template/actions/runs/39626092728) |
| 2026-10-21 15:38 UTC | dev | — | `0c38529` | [run](https://github.com/mairie360/API_template/actions/runs/39625313777) |
| 2026-10-20 18:52 UTC | staging | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-20 18:46 UTC | dev | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-19 20:29 UTC | staging | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
| 2026-10-19 20:23 UTC | dev | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
<!-- release-journal-history:end -->
