# Journal des releases — API_template

<!-- release-journal-state {"dev": "0c38529990813eb3ca2e72242c8d28444907e4f4", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-10-21 15:38 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `0c38529` | 2026-10-21 15:38 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, désactivé par défaut : il ne s'active que si `OTEL_EXPORTER_OTLP_ENDPOINT` ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` est défini, et `OTEL_SDK_DISABLED=true` le coupe. Un exporteur en erreur est journalisé sans arrêter l'API. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Logs sur stdout au format commun à toutes les APIs : texte coloré par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Chaque requête produit désormais un span racine nommé `<METHOD> <route>`, y compris les requêtes refusées par `JwtMiddleware` (401). Il reprend le `traceparent` entrant. La query string et l'adresse client ne sont pas exportées. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Mutualisation de la télémétrie : `mairie360_api_lib` passe en 4.2.0 avec la feature `otel`, `tracing-subscriber` devient une dev-dependency, `main.rs` appelle `init_telemetry`, et `tests/telemetry_test.rs` et la documentation (CLAUDE.md, docker-compose.yml) sont ajoutés. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Mises à jour de dépendances et de CI sans effet fonctionnel : workflow CICD de la v4.2.2 à la v4.4.3 en plusieurs étapes (#95, #97, #98, #100, #101, #105), transmission des secrets de release au workflow CICD (#94), digests des images Rust (#102, #103, #104), `utoipa-swagger-ui` 10.0.2 (#99) et `tokio` 1.54.0 (#96).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, désactivé par défaut : il ne s'active que si `OTEL_EXPORTER_OTLP_ENDPOINT` ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` est défini, et `OTEL_SDK_DISABLED=true` le coupe. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Logs sur stdout au format commun à toutes les APIs : texte coloré par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Chaque requête produit désormais un span racine, y compris les requêtes refusées par `JwtMiddleware` (401), qui reprend le `traceparent` entrant. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Mutualisation de la télémétrie avec `mairie360_api_lib` 4.2.0 (feature `otel`) et ajout de `tests/telemetry_test.rs`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Mises à jour sans effet fonctionnel : workflow CICD de la v4.3.0 à la v4.4.0 (#95, #97, #98, #100, #101), digests des images Rust (#102, #103, #104).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Mises à jour sans effet fonctionnel : `utoipa-swagger-ui` 10.0.2 (#99), `tokio` 1.54.0 (#96), workflow CICD de la v4.2.2 à la v4.3.2 (#653588c, #298ae00, #0121050 : PR #98, #97, #95).
- Transmission au workflow CICD des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal de release (#94). Aucun effet sur le comportement de l'API.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-21 15:38 UTC | dev | — | `0c38529` | [run](https://github.com/mairie360/API_template/actions/runs/39625313777) |
| 2026-10-20 18:52 UTC | staging | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-20 18:46 UTC | dev | — | `84055a3` | [run](https://github.com/mairie360/API_template/actions/runs/39476062185) |
| 2026-10-19 20:29 UTC | staging | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
| 2026-10-19 20:23 UTC | dev | — | `4258372` | [run](https://github.com/mairie360/API_template/actions/runs/39316681288) |
<!-- release-journal-history:end -->
