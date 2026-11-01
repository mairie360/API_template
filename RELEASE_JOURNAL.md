# Journal des releases — API_template

<!-- release-journal-state {"dev": "501e4314fdc198bb9aa81e1f1f2e8d60caf1214f", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-01 21:03 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `501e431` | 2026-11-01 21:03 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Télémétrie OpenTelemetry : chaque requête, y compris celles refusées par `JwtMiddleware` (401), produit un span avec sa route et son statut, et continue le `traceparent` entrant. L'export OTLP/HTTP reste désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` n'est pas défini. `LOG_FORMAT=json` donne des journaux en JSON ligne par ligne. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les journaux de requête n'affichent plus la query string ni le `Referer` : seuls l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée sont journalisés. Avant, `Logger::default()` écrivait la ligne de requête complète. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Les erreurs de `mairie360_api_lib` ne reprennent plus les valeurs reçues (SQLSTATE, contrainte, table et colonne à la place du message Postgres et de `DETAIL`). (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Mise à jour de `mairie360_api_lib` 4.2.1 vers 4.3.0. Le diff de `Cargo.lock` n'est pas fourni. Les notes de release mentionnent MAIR-680 (remplacement du secret JWT HS256 partagé), à vérifier avant la prod car l'authentification passe par la lib. (MAIR-680, [#130](https://github.com/mairie360/API_template/pull/130))

#### Corrections
- Image d'exécution des APIs : avant, `gcr.io/distroless/cc-debian12` avec `libssl3` non corrigée (CVE-2026-84782) ; maintenant `cc-debian13:nonroot`, avec un builder `rust:1.99-slim-trixie` et `development.Dockerfile` sur Debian 13. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Alertes RustSec corrigées (zstd, ctutils, rand, spin) et RUSTSEC-2026-0387 (crypto-bigint) ignorée avec sa justification dans `.cargo/audit.toml`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Mises à jour de dépendances : tokio 1.54.1, log 0.4.35, actix-web 4.18.0 puis 4.18.1, utoipa-swagger-ui 10.0.2 (sans diff visible) et image Node 26 du runner gdpr-marker. ([#109](https://github.com/mairie360/API_template/pull/109), [#120](https://github.com/mairie360/API_template/pull/120), [#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114), [#99](https://github.com/mairie360/API_template/pull/99), [#121](https://github.com/mairie360/API_template/pull/121))
- Digests Docker `rust:1.99` mis à jour une dizaine de fois par Renovate (#102 à #118).
- Workflows CI : `mairie360/CICD` passe de v4.5.0 à v4.13.0 ; ajout d'une checklist RGPD sur les PR (`gdpr-pr.yml`, MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)) ; test du marqueur RGPD sur les journaux et test de Dockerfiles (MAIR-290, MAIR-741) ; tests de télémétrie.
- Documentation : section CLAUDE.md sur le pool Postgres (`DB_POOL_*`), sans changement de code. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Télémétrie OpenTelemetry : chaque requête, y compris celles refusées par `JwtMiddleware` (401), produit un span avec sa route et son statut, et continue le `traceparent` entrant. L'export OTLP/HTTP reste désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` n'est pas défini. `LOG_FORMAT=json` donne des journaux en JSON. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les journaux de requête n'affichent plus la query string ni le `Referer` ; seuls l'adresse client, la méthode, le chemin, le statut, la taille et la durée sont journalisés. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Les erreurs de `mairie360_api_lib` ne reprennent plus les valeurs reçues (SQLSTATE, contrainte, table et colonne). (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Mise à jour de `mairie360_api_lib` 4.2.1 vers 4.3.0 ; le diff de `Cargo.lock` n'est pas fourni, et les notes de release mentionnent MAIR-680 (secret JWT HS256 partagé remplacé), à vérifier avant la prod. (MAIR-680, [#130](https://github.com/mairie360/API_template/pull/130))

#### Corrections
- Image d'exécution des APIs : avant, `cc-debian12` avec `libssl3` non corrigée (CVE-2026-84782) ; maintenant `cc-debian13:nonroot`, builder `rust:1.99-slim-trixie`. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Alertes RustSec corrigées (zstd, ctutils, rand, spin) et RUSTSEC-2026-0387 ignorée dans `.cargo/audit.toml`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Mises à jour de dépendances et d'images : tokio 1.54.1, log 0.4.35, actix-web 4.18.x, utoipa-swagger-ui 10.0.2, Node 26 pour le runner gdpr-marker, et digests Docker `rust:1.99` mis à jour par Renovate.
- CI : `mairie360/CICD` passe de v4.5.0 à v4.13.0 ; checklist RGPD sur les PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)) ; test du marqueur RGPD et tests de Dockerfiles ; tests de télémétrie.
- Documentation : section CLAUDE.md sur le pool Postgres, sans changement de code. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Aucun changement fonctionnel pour l'API. Mises à jour de `mairie360/CICD` (v4.3.0 à v4.3.2, journal des releases MAIR-571 et transmission des secrets du workflow, [#94](https://github.com/mairie360/API_template/pull/94), [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)), de tokio 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et de utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99), sans diff visible).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-01 21:03 UTC | dev | — | `501e431` | [run](https://github.com/mairie360/API_template/actions/runs/41348977901) |
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
