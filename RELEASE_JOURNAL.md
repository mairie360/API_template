# Journal des releases — API_template

<!-- release-journal-state {"dev": "db37243824abf1cdb3e30e8f1ef878323ac018cb", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-05 03:54 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `db37243` | 2026-11-05 03:54 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les journaux sur stdout suivent le format commun à toutes les APIs : texte coloré, niveau par `RUST_LOG` (défaut `info`), et une ligne JSON par événement avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export OpenTelemetry des traces en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` n'est pas renseigné. Chaque requête produit un span portant sa route et son statut, qui continue un `traceparent` entrant et n'exporte ni la query string ni l'adresse client. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Routes sous `/api` : l'API valide les JWT avec la clé publique Ed25519 de Core_API (`JWT_PUBLIC_KEY`, `EdDSA`) et n'a plus besoin de `JWT_SECRET`. Avant, le secret HS256 partagé suffisait. Tant que `JWT_SECRET` est défini, les HS256 signés avec lui restent acceptés (transition). Sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`, l'API refuse de démarrer. Les appelants doivent présenter un token signé par Core. Dans les stacks de test, les tokens HS256 en dur sont supprimés et refusés. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Journaux de requête : `request_logger()` remplace `Logger::default()`. La query string et le `Referer` ne sont plus journalisés ; restent l'adresse client, la méthode, le chemin, le statut, la taille et la durée. Les erreurs de base sont décrites par SQLSTATE, contrainte, table et colonne, au lieu du message Postgres et de `DETAIL`. Les messages serde ne citent plus la valeur fautive. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution : avant, `gcr.io/distroless/cc-debian12` avec `libssl3` non corrigée (CVE-2026-84782). Maintenant, `cc-debian13:nonroot`, avec un builder `rust:1.99-slim-trixie` et l'image de dev sur Debian 13. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Simulation d'usage RGPD avant prod (personas fictifs, purge, contrôles de base, `gdpr-simulation.yaml`, `docker-compose-gdpr-simulation.yml`, `gdpr_simulation_test.sh`) : stack de test uniquement. (MAIR-497, [#137](https://github.com/mairie360/API_template/pull/137))
- Checklist RGPD sur chaque PR (`gdpr-pr.yml`) et test `tests/mair_771_cicd_version.rs` qui vérifie que tous les workflows appellent la même release CICD (≥ v4.12.0). (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70) ; MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133))
- Dépendances Rust : `zstd`, `ctutils`, `rand` et `spin` corrigés, exception RUSTSEC-2026-0387 documentée dans `.cargo/audit.toml`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Mises à jour Renovate : `mairie360_api_lib` 4.3.0 (#130, #122 pour 4.2.1), `actix-web` 4.18.1 (#114), `tokio` 1.54.1 (#109), `log` 0.4.35 (#120), `node` 26 pour le runner marker (#121).
- CICD passé de v4.3.x à v5.5.0 au fil des PR Renovate (#129, #128, #127, #131, #135, #141, #144, #145).
- Images épinglées par digest (Rust, Postgres, Redis, nginx, curl, k6, newman, ZAP, node) et tests de versions de Dockerfile. (#134, #140, #142, #143, #146, et les PR rust:1.99 de #102 à #118)
- Documentation seule : variables `DB_POOL_*` de la lib documentées dans `CLAUDE.md`. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les journaux sur stdout suivent le format commun à toutes les APIs : texte coloré, niveau par `RUST_LOG` (défaut `info`), et une ligne JSON par événement avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Export OpenTelemetry des traces en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` n'est pas renseigné. Chaque requête produit un span portant sa route et son statut, qui continue un `traceparent` entrant et n'exporte ni la query string ni l'adresse client. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Routes sous `/api` : l'API valide les JWT avec la clé publique Ed25519 de Core_API (`JWT_PUBLIC_KEY`, `EdDSA`) et n'a plus besoin de `JWT_SECRET`. Avant, le secret HS256 partagé suffisait. Tant que `JWT_SECRET` est défini, les HS256 signés avec lui restent acceptés (transition). Sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`, l'API refuse de démarrer. Les appelants doivent présenter un token signé par Core. Dans les stacks de test, les tokens HS256 en dur sont supprimés et refusés. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Journaux de requête : `request_logger()` remplace `Logger::default()`. La query string et le `Referer` ne sont plus journalisés ; restent l'adresse client, la méthode, le chemin, le statut, la taille et la durée. Les erreurs de base sont décrites par SQLSTATE, contrainte, table et colonne, au lieu du message Postgres et de `DETAIL`. Les messages serde ne citent plus la valeur fautive. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution : avant, `gcr.io/distroless/cc-debian12` avec `libssl3` non corrigée (CVE-2026-84782). Maintenant, `cc-debian13:nonroot`, avec un builder `rust:1.99-slim-trixie` et l'image de dev sur Debian 13. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Simulation d'usage RGPD avant prod (personas fictifs, purge, contrôles de base) : stack de test uniquement. (MAIR-497, [#137](https://github.com/mairie360/API_template/pull/137))
- Checklist RGPD sur chaque PR (`gdpr-pr.yml`) et test de cohérence des versions CICD. (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70) ; MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133))
- Dépendances Rust : `zstd`, `ctutils`, `rand` et `spin` corrigés, exception RUSTSEC-2026-0387 documentée. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Mises à jour Renovate : `mairie360_api_lib` 4.3.0, `actix-web` 4.18.1, `tokio` 1.54.1, `log` 0.4.35, `node` 26 pour le runner marker, CICD jusqu'à v5.5.0.
- Images épinglées par digest et tests de version des Dockerfile. (#102 à #118, #134, #138, #140, #142, #143, #146)
- Documentation seule : variables `DB_POOL_*` documentées dans `CLAUDE.md`. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Dépendances sans effet visible : `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)) et `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)).
- CICD mis à jour de v4.3.0 à v4.3.2 au fil des PR Renovate ([#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)). Le workflow de journal de release (MAIR-571) n'a pas d'effet sur l'API.
- Transmission des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow CICD dans `cicd.yml`, pour le journal de release. (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94))

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-05 03:54 UTC | dev | — | `db37243` | [run](https://github.com/mairie360/API_template/actions/runs/41963506300) |
| 2026-11-04 23:15 UTC | dev | — | `88c8621` | [run](https://github.com/mairie360/API_template/actions/runs/41931274021) |
| 2026-11-04 18:07 UTC | dev | — | `f2d7f12` | [run](https://github.com/mairie360/API_template/actions/runs/41882520253) |
| 2026-11-04 15:29 UTC | dev | — | `246f16a` | [run](https://github.com/mairie360/API_template/actions/runs/41854613759) |
| 2026-11-04 01:40 UTC | dev | — | `8a6e687` | [run](https://github.com/mairie360/API_template/actions/runs/41754209258) |
| 2026-11-04 01:39 UTC | dev | — | `c94246c` | [run](https://github.com/mairie360/API_template/actions/runs/41754246705) |
| 2026-11-03 15:14 UTC | dev | — | `51f9b72` | [run](https://github.com/mairie360/API_template/actions/runs/41660330048) |
| 2026-11-03 15:12 UTC | dev | — | `8322db8` | [run](https://github.com/mairie360/API_template/actions/runs/41660238139) |
| 2026-11-03 01:33 UTC | dev | — | `bb7b1bc` | [run](https://github.com/mairie360/API_template/actions/runs/41559252705) |
| 2026-11-03 00:08 UTC | dev | — | `7071f05` | [run](https://github.com/mairie360/API_template/actions/runs/41548546849) |
| 2026-11-03 00:05 UTC | dev | — | `2d8f9b4` | [run](https://github.com/mairie360/API_template/actions/runs/41548120063) |
| 2026-11-02 19:37 UTC | dev | — | `0d5dc99` | [run](https://github.com/mairie360/API_template/actions/runs/41508758912) |
| 2026-11-02 19:35 UTC | dev | — | `f21d81f` | [run](https://github.com/mairie360/API_template/actions/runs/41508699007) |
| 2026-11-02 00:56 UTC | dev | — | `6e6f5bb` | [run](https://github.com/mairie360/API_template/actions/runs/41369273127) |
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
