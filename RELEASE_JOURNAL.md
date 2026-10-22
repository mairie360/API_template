# Journal des releases — API_template

<!-- release-journal-state {"dev": "8cc13deb1470eab0385c720c8acc5b4d9ecc46f3", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-10-22 17:39 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `8cc13de` | 2026-10-22 17:39 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les traces OpenTelemetry sont exportées en OTLP/HTTP, uniquement si `OTEL_EXPORTER_OTLP_ENDPOINT` ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` est défini. `OTEL_SDK_DISABLED=true` les coupe, et un exporteur en échec est seulement journalisé sans arrêter l'API. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Les logs peuvent être émis en JSON, une ligne par événement, avec `LOG_FORMAT=json`. Sinon ils sont en texte coloré, avec le niveau réglé par `RUST_LOG` (défaut `info`). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Chaque requête produit désormais un span racine nommé `<MÉTHODE> <route>`. Les requêtes refusées par `JwtMiddleware` (401) en ont un aussi, et le span reprend le `traceparent` entrant. Le span racine n'apparaît pas dans les logs et n'exporte ni l'adresse client ni la query string. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Le format des logs vient désormais de la lib partagée `mairie360_api_lib` (feature `otel`) au lieu d'un `tracing_subscriber::fmt` local. Le journal de requêtes actix (`middleware::Logger`) est conservé.

#### Technique
- Workflow CI `mairie360/CICD` mis à jour de v4.2.2 à v4.5.0 en plusieurs PR Renovate. L'appel transmet aussi `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal de release (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).
- Images de base `rust:1.99-bookworm` et `rust:1.99-slim-bookworm` mises à jour par digest dans `Dockerfile` et `development.Dockerfile`, sans effet sur le code de l'API.
- Dépendances : tokio 1.54.0 et 1.54.1, utoipa-swagger-ui 10.0.2. Les diffs de ces PR sont vides, donc `Cargo.lock` n'est pas visible.
- `tracing-subscriber` passe en dev-dependency et `tests/telemetry_test.rs` ajoute deux tests de span (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79)).
- Documentation : `CLAUDE.md` décrit les variables du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)) et l'observabilité (MAIR-503).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les traces OpenTelemetry sont exportées en OTLP/HTTP, uniquement si `OTEL_EXPORTER_OTLP_ENDPOINT` ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` est défini. `OTEL_SDK_DISABLED=true` les coupe. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Les logs peuvent être émis en JSON avec `LOG_FORMAT=json`. Sinon ils sont en texte coloré, niveau réglé par `RUST_LOG`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Chaque requête produit un span racine `<MÉTHODE> <route>`, y compris les requêtes refusées en 401 par `JwtMiddleware`. Le span racine n'apparaît pas dans les logs et n'exporte ni l'adresse client ni la query string. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Le format des logs vient de la lib partagée `mairie360_api_lib` au lieu d'un `tracing_subscriber::fmt` local.

#### Technique
- Workflow CI `mairie360/CICD` mis à jour jusqu'à v4.4.0, puis v4.4.1 et v4.4.3 en PR Renovate ensuite passées dans les plages suivantes.
- Images de base `rust:1.99-*` épinglées par digest mises à jour (`Dockerfile` et `development.Dockerfile`), sans effet sur le code.
- Documentation `CLAUDE.md` pour le pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)) et l'observabilité (MAIR-503).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Workflow CI `mairie360/CICD` mis à jour de v4.3.0 à v4.3.2, avec les PR Renovate correspondantes.
- Transmission des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow pour le journal de release (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).
- Dépendances : tokio 1.54.0 et utoipa-swagger-ui 10.0.2. Les diffs de ces PR sont vides, donc `Cargo.lock` n'est pas visible.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
