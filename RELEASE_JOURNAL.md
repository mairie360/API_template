# Journal des releases — API_template

<!-- release-journal-state {"dev": "cde04e4bdf6213dd344eba5714c88770d99a0148", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-10-22 02:37 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `cde04e4` | 2026-10-22 02:37 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement

- Les logs passent par le format partagé de `mairie360_api_lib` (`init_telemetry`) au lieu d'un `tracing_subscriber::fmt()` local : texte coloré sur stdout par défaut, une ligne JSON par événement avec `LOG_FORMAT=json`, niveau toujours piloté par `RUST_LOG` (défaut `info`). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export des traces OTLP/HTTP opt-in : rien n'est envoyé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini ; `OTEL_SDK_DISABLED=true` le coupe de force. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Chaque requête produit un span (`TracingLogger`, en plus de `middleware::Logger`), y compris une requête refusée par `JwtMiddleware` (401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique

- Logs et télémétrie : `mairie360_api_lib` avec la feature `otel`, `tracing-subscriber` passe en dev-dependency, nouveau `tests/telemetry_test.rs` (2 tests), documentation dans `CLAUDE.md` et `docker-compose.yml`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Pool Postgres : seule la documentation `CLAUDE.md` change, la lib 4.2.0 lit déjà `DB_POOL_*`. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- CI : `mairie360/CICD` passe de v4.2.2 à v4.4.3 (#95, #97, #98, #100, #101, #105). Le workflow reçoit `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal des releases. (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94))
- Images Rust épinglées par digest mises à jour par Renovate (#102, #103, #104, #107, #108) : `Dockerfile` sur `54e39a2`, `development.Dockerfile` sur `edaa2a8`.
- Dépendances : tokio 1.53.3 → 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et utoipa-swagger-ui 10.0.1 → 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)). Diffs absents, non vérifiés.

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement

- Les logs passent par le format partagé de `mairie360_api_lib` (`init_telemetry`) au lieu d'un `tracing_subscriber::fmt()` local : texte coloré sur stdout par défaut, une ligne JSON par événement avec `LOG_FORMAT=json`, niveau toujours piloté par `RUST_LOG` (défaut `info`). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export des traces OTLP/HTTP opt-in : rien n'est envoyé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini ; `OTEL_SDK_DISABLED=true` le coupe de force. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Chaque requête produit un span (`TracingLogger`, en plus de `middleware::Logger`), y compris une requête refusée par `JwtMiddleware` (401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique

- Logs et télémétrie : `mairie360_api_lib` avec la feature `otel`, `tracing-subscriber` passe en dev-dependency, nouveau `tests/telemetry_test.rs` (2 tests), documentation dans `CLAUDE.md` et `docker-compose.yml`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Pool Postgres : seule la documentation `CLAUDE.md` change, la lib 4.2.0 lit déjà `DB_POOL_*`. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- CI : `mairie360/CICD` passe de v4.3.2 à v4.4.3 (#95 hors plage, #100, #101, #105). Les images Rust épinglées par digest sont mises à jour par Renovate (#102, #103, #104, #107, #108) : `Dockerfile` sur `54e39a2`, `development.Dockerfile` sur `edaa2a8`.

## Différence staging → prod (en staging, pas encore en prod)

#### Technique

- CI : `mairie360/CICD` passe de v4.2.2 à v4.3.2 (#95, #97, #98). Le workflow reçoit `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal des releases. (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94))
- Dépendances : tokio 1.53.3 → 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et utoipa-swagger-ui 10.0.1 → 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)). Diffs absents, non vérifiés.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-22 02:37 UTC | dev | — | `cde04e4` | [run](https://github.com/mairie360/API_template/actions/runs/39711278450) |
| 2026-10-22 02:31 UTC | dev | — | `d05c66f` | [run](https://github.com/mairie360/API_template/actions/runs/39711246658) |
| 2026-10-21 15:41 UTC | dev | — | `7be1696` | [run](https://github.com/mairie360/API_template/actions/runs/39626092728) |
| 2026-10-21 15:38 UTC | dev | — | `0c38529` | [run](https://github.com/mairie360/API_template/actions/runs/39625313777) |
| 2026-10-20 18:52 UTC | staging | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-20 18:46 UTC | dev | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-19 20:29 UTC | staging | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
| 2026-10-19 20:23 UTC | dev | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
<!-- release-journal-history:end -->
