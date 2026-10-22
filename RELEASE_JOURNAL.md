# Journal des releases — API_template

<!-- release-journal-state {"dev": "fb59f4bdec9f00a2ec2b40b1bcd8f079a6c1be99", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-10-22 05:58 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `fb59f4b` | 2026-10-22 05:58 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les logs passent par la télémétrie partagée de `mairie360_api_lib` (`init_telemetry`) : texte coloré par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`, niveau lu dans `RUST_LOG` (défaut `info`). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Chaque requête produit un span (`TracingLogger`), y compris celles refusées par `JwtMiddleware` avec un 401. Les traces ne sont exportées en OTLP/HTTP que si `OTEL_EXPORTER_OTLP_ENDPOINT` ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` est défini. La requête reprend le `traceparent` entrant et l'export ne contient ni chaîne de requête ni adresse client. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Documentation : section sur les variables `DB_POOL_*` dans `CLAUDE.md`, sans câblage dans le template (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)). Télémétrie : `tracing-subscriber` passe en dev-dependency, et `docker-compose.yml` documente `LOG_FORMAT` et `OTEL_*` en commentaire.
- Dépendances et CI sans effet visible : tokio 1.54.0 puis 1.54.1, utoipa-swagger-ui 10.0.2, mairie360/CICD de 4.2.2 à 4.4.3, images `rust:1.99` épinglées par digest, transmission des secrets de release au workflow CICD (#94).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les logs passent par la télémétrie partagée de `mairie360_api_lib` (`init_telemetry`) : texte coloré par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`, niveau lu dans `RUST_LOG` (défaut `info`). Chaque requête produit un span, y compris celles refusées par `JwtMiddleware` avec un 401. Les traces ne sont exportées en OTLP/HTTP que si `OTEL_EXPORTER_OTLP_ENDPOINT` ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` est défini. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Dépendances et CI sans effet visible : tokio 1.54.0 puis 1.54.1, mairie360/CICD de 4.3.x à 4.4.3, images `rust:1.99` (bookworm et slim-bookworm) épinglées par digest, documentation du pool Postgres dans `CLAUDE.md` (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).
- Le changement de télémétrie est déjà inclus dans cette plage : `tracing-subscriber` passe en dev-dependency et `docker-compose.yml` documente `LOG_FORMAT` et `OTEL_*` en commentaire.

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Mises à jour de dépendances et CI sans effet fonctionnel visible : utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)), tokio 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)), mairie360/CICD de 4.2.2 à 4.3.2 ([#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)).
- Le workflow `cicd.yml` transmet désormais les secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow CICD ([#94](https://github.com/mairie360/API_template/pull/94)). Cela ne touche pas l'API elle-même.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
