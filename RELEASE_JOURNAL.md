# Journal des releases — API_template

<!-- release-journal-state {"dev": "d05c66f47e46e708ae37833f89f031b8d0ee145f", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-10-22 02:31 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `d05c66f` | 2026-10-22 02:31 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les logs passent par la télémétrie partagée de `mairie360_api_lib` : texte coloré par défaut, une ligne JSON par événement avec `LOG_FORMAT=json`, et `RUST_LOG` (défaut `info`) reste le niveau. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export OTLP/HTTP des traces, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini. `OTEL_SDK_DISABLED=true` le coupe. Chaque requête, y compris celle refusée par `JwtMiddleware` avec un `401`, produit un span racine nommé `<METHOD> <route>` qui reprend le `traceparent` entrant. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Télémétrie : `main.rs` appelle `init_telemetry` et `TracingLogger`, `tracing-subscriber` passe en dev-dependency, et un nouveau `tests/telemetry_test.rs` couvre deux cas (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79)).
- Documentation : `CLAUDE.md` décrit les variables `DB_POOL_*` lues par la lib 4.2.0, sans changement de configuration dans le template (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).
- Images `rust:1.99` épinglées par digest mises à jour dans `Dockerfile` et `development.Dockerfile`, tag inchangé (Renovate, [#102](https://github.com/mairie360/API_template/pull/102), [#103](https://github.com/mairie360/API_template/pull/103), [#104](https://github.com/mairie360/API_template/pull/104), [#107](https://github.com/mairie360/API_template/pull/107)).
- CI : workflow `mairie360/CICD` passé de v4.2.2 à v4.4.3 (Renovate, [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98), [#100](https://github.com/mairie360/API_template/pull/100), [#101](https://github.com/mairie360/API_template/pull/101), [#105](https://github.com/mairie360/API_template/pull/105)). Le caller transmet désormais `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal de release de MAIR-571 ([#94](https://github.com/mairie360/API_template/pull/94)).
- Dépendances : `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)), sans diff fourni pour ces deux PR.

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les logs passent par la télémétrie partagée de `mairie360_api_lib` : texte coloré par défaut, une ligne JSON par événement avec `LOG_FORMAT=json`, et `RUST_LOG` (défaut `info`) reste le niveau. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export OTLP/HTTP des traces, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini. `OTEL_SDK_DISABLED=true` le coupe. Chaque requête, y compris celle refusée par `JwtMiddleware` avec un `401`, produit un span racine nommé `<METHOD> <route>` qui reprend le `traceparent` entrant. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Télémétrie : `main.rs` appelle `init_telemetry` et `TracingLogger`, `tracing-subscriber` passe en dev-dependency, et un nouveau `tests/telemetry_test.rs` couvre deux cas (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79)).
- Documentation : `CLAUDE.md` décrit les variables `DB_POOL_*` lues par la lib 4.2.0, sans changement de configuration dans le template (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).
- Images `rust:1.99` épinglées par digest mises à jour dans `Dockerfile` et `development.Dockerfile`, tag inchangé (Renovate, [#102](https://github.com/mairie360/API_template/pull/102), [#103](https://github.com/mairie360/API_template/pull/103), [#104](https://github.com/mairie360/API_template/pull/104), [#107](https://github.com/mairie360/API_template/pull/107)).
- CI : workflow `mairie360/CICD` passé de v4.2.2 à v4.4.1 (Renovate, [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98), [#100](https://github.com/mairie360/API_template/pull/100), [#101](https://github.com/mairie360/API_template/pull/101)). Le caller transmet désormais `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal de release de MAIR-571 ([#94](https://github.com/mairie360/API_template/pull/94)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- CI : workflow `mairie360/CICD` passé de v4.2.2 à v4.3.2 (Renovate, [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)). Le caller transmet désormais `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal de release de MAIR-571 ([#94](https://github.com/mairie360/API_template/pull/94)).
- Dépendances : `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)), sans diff fourni pour ces deux PR.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-22 02:31 UTC | dev | — | `d05c66f` | [run](https://github.com/mairie360/API_template/actions/runs/39711246658) |
| 2026-10-21 15:41 UTC | dev | — | `7be1696` | [run](https://github.com/mairie360/API_template/actions/runs/39626092728) |
| 2026-10-21 15:38 UTC | dev | — | `0c38529` | [run](https://github.com/mairie360/API_template/actions/runs/39625313777) |
| 2026-10-20 18:52 UTC | staging | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-20 18:46 UTC | dev | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-19 20:29 UTC | staging | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
| 2026-10-19 20:23 UTC | dev | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
<!-- release-journal-history:end -->
