# Journal des releases — API_template

<!-- release-journal-state {"dev": "8322db87f4399c077c73d3aed244176c366c39df", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-03 15:12 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `8322db8` | 2026-11-03 15:12 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement
- Les tokens HS256 ne sont plus acceptés sans clé publique : l'API lit désormais `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, PEM) et n'a plus besoin de `JWT_SECRET`. Sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`, le démarrage échoue. Tant que `JWT_SECRET` est défini, les tokens HS256 signés avec lui restent acceptés (transition) ; sans ce secret, tout token HS256 reçoit un `401`. Un appelant doit présenter un token EdDSA signé par Core_API. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Format des journaux : texte coloré par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`. Chaque requête, y compris celle refusée par le JWT, a désormais un span (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79)).

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, désactivé par défaut : il ne s'active que si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. `OTEL_SDK_DISABLED=true` le coupe, et un exporteur en échec est seulement journalisé sans arrêter l'API. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Corrections
- Données personnelles dans les journaux : avant, les requêtes journalisaient la query string (`%r` de `Logger::default()`), et les erreurs recopiaient le message Postgres, son `DETAIL` ou la valeur citée par serde. Maintenant, les requêtes ne journalisent que l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée, et les erreurs sont décrites par type et contexte (SQLSTATE, contrainte, table, colonne). (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Technique
- Mise à jour de `mairie360_api_lib` en 4.2.0 puis 4.3.0 ; `Cargo.toml` et la configuration de test passent à la clé publique (`tests/auth_gate_test.rs` signe comme Core, nouveau `tests/mair_680_jwt_config.rs`), et `test_secrets.sh` génère une paire de clés Ed25519 par exécution. Les stacks de test exigent `JWT_PUBLIC_KEY` et ne contiennent plus de token committé. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- CICD monté en v5.2.2 dans `cicd.yml`, `gdpr-pr.yml` (et promotion), avec un test `tests/mair_771_cicd_version.rs` qui vérifie que tous les workflows appellent la même version. (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133))
- Image d'exécution passée en distroless Debian 13 et builder en `rust:1.99-slim-trixie`, avec test `mair_741_every_image_runs_on_debian_13`. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Alertes RustSec corrigées (zstd, ctutils, rand, spin), et ignore documenté de RUSTSEC-2026-0387 dans `.cargo/audit.toml`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Workflow `gdpr-pr.yml` : checklist RGPD sur les PR. (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70))
- Test de marqueur RGPD (`gdpr_marker_test.sh`, `docker-compose-gdpr-marker.yml`, `gdpr-marker.yaml`) et module `request_log`. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Documentation des variables du pool Postgres (`DB_POOL_*`) dans `CLAUDE.md`, sans changement de code. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Mises à jour Renovate : images épinglées par digest (postgres, redis, nginx, node, k6, curl, newman, ZAP, rust), Node 26 pour le marqueur RGPD, `actix-web` 4.18.1, `tokio` 1.54.1, `log` 0.4.35, CICD jusqu'à v5.2.2.

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- Les tokens HS256 ne sont plus acceptés sans clé publique : l'API lit désormais `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, PEM) et n'a plus besoin de `JWT_SECRET`. Sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`, le démarrage échoue. Tant que `JWT_SECRET` est défini, les tokens HS256 signés avec lui restent acceptés (transition) ; sans ce secret, tout token HS256 reçoit un `401`. Un appelant doit présenter un token EdDSA signé par Core_API. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Format des journaux : texte coloré par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`. Chaque requête, y compris celle refusée par le JWT, a désormais un span (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79)).

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, désactivé par défaut : il ne s'active que si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. `OTEL_SDK_DISABLED=true` le coupe, et un exporteur en échec est seulement journalisé sans arrêter l'API. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Corrections
- Données personnelles dans les journaux : avant, les requêtes journalisaient la query string (`%r` de `Logger::default()`), et les erreurs recopiaient le message Postgres, son `DETAIL` ou la valeur citée par serde. Maintenant, les requêtes ne journalisent que l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée, et les erreurs sont décrites par type et contexte. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Technique
- Mise à jour de `mairie360_api_lib` en 4.2.0 puis 4.3.0, tests JWT réécrits pour signer comme Core et génération d'une paire de clés Ed25519 par exécution (`test_secrets.sh`). (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- CICD monté en v5.2.2 dans `cicd.yml` et `gdpr-pr.yml`, avec le test `tests/mair_771_cicd_version.rs`. (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133))
- Image d'exécution en distroless Debian 13 et builder `rust:1.99-slim-trixie`, avec un test de version Debian. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Alertes RustSec corrigées et workflow de checklist RGPD sur les PR. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125) ; MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70))
- Documentation des variables `DB_POOL_*` dans `CLAUDE.md`. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Mises à jour Renovate : images épinglées par digest, Node 26 pour le marqueur RGPD, `actix-web` 4.18.1, `tokio` 1.54.1, `log` 0.4.35, `utoipa-swagger-ui` 10.0.2, CICD jusqu'à v5.2.2.

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Mise à jour de la version CICD dans `cicd.yml` (v4.3.0 à v4.3.2) et de `gdpr-pr.yml`, avec le journal des releases rédigé par Claude côté CI (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).
- Transmission à `cicd.yml` des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal des releases (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).
- Mises à jour Renovate : `utoipa-swagger-ui` 10.0.2 et `tokio` 1.54.0, sans changement fonctionnel visible dans les diffs.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
