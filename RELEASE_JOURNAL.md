# Journal des releases — API_template

<!-- release-journal-state {"dev": "f2d7f123f4f3ba9fb5ae65ecbcd896077300d8a5", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-04 18:07 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `f2d7f12` | 2026-11-04 18:07 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Traces OpenTelemetry : chaque requête produit un span (route, statut, `traceparent` continué). L'export OTLP/HTTP n'a lieu que si `OTEL_EXPORTER_OTLP_ENDPOINT` ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` est défini. `LOG_FORMAT=json` passe les journaux en JSON, une ligne par événement. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- L'API ne lit plus `JWT_SECRET` pour vérifier les sessions : elle lit `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, au format PEM). Les jetons HS256 sont refusés (401) : les appelants (BFF, scripts) doivent présenter un jeton EdDSA signé par Core. Sans l'une des deux variables, l'API refuse de démarrer. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139), lib `mairie360_api_lib` 4.3.0 via [#130](https://github.com/mairie360/API_template/pull/130))
- Journaux de requêtes : la query string et le `Referer` n'apparaissent plus. Restent l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Les erreurs de la lib dans les journaux ne citent plus la valeur reçue : les erreurs base sont décrites par SQLSTATE, contrainte, table et colonne, et les messages serde ne citent plus la valeur. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution : avant, `gcr.io/distroless/cc-debian12` (libssl3 non corrigée, CVE-2026-84782) ; maintenant `cc-debian13:nonroot`, avec builder et `development.Dockerfile` sur `rust:1.99-*-trixie`. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Simulation d'usage RGPD avant prod (MAIR-497, [#137](https://github.com/mairie360/API_template/pull/137)) : nouveaux `gdpr-simulation.yaml`, `docker-compose-gdpr-simulation.yml` et `gdpr_simulation_test.sh`, sans endpoint modifié.
- CI : mairie360/CICD monté jusqu'à v5.3.1 ; contrôle RGPD des PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)) ; test `tests/mair_771_cicd_version.rs` qui vérifie que tous les workflows appellent la même release (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133)). `cicd.yml` transmet aussi `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).
- Images épinglées par digest (Dockerfile, development.Dockerfile, stacks compose) et Node 26 pour le runner GDPR : mises à jour Renovate sans effet fonctionnel.
- Dépendances Rust : tokio 1.54.0 et 1.54.1, actix-web 4.18.1, log 0.4.35 (dev), utoipa-swagger-ui 10.0.2. Audit RustSec (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)) : zstd, ctutils, rand et spin mis à jour, RUSTSEC-2026-0387 ignoré avec son motif dans `.cargo/audit.toml`.
- `CLAUDE.md` documente `DB_POOL_MIN_CONNECTIONS`, `DB_POOL_MAX_CONNECTIONS` et `DB_POOL_IDLE_TIMEOUT_SECS` (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)) : documentation seule.
- Tests ajoutés : `mair_741_every_image_runs_on_debian_13`, `tests/telemetry_test.rs`, `tests/auth_gate_test.rs` et `tests/session_tokens` (signature EdDSA), `tests/mair_680_jwt_config.rs`.

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Traces OpenTelemetry : chaque requête produit un span (route, statut, `traceparent` continué). L'export OTLP/HTTP n'a lieu que si `OTEL_EXPORTER_OTLP_ENDPOINT` ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` est défini. `LOG_FORMAT=json` passe les journaux en JSON, une ligne par événement. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- L'API ne lit plus `JWT_SECRET` pour vérifier les sessions : elle lit `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, au format PEM). Les jetons HS256 sont refusés (401) : les appelants (BFF, scripts) doivent présenter un jeton EdDSA signé par Core. Sans l'une des deux variables, l'API refuse de démarrer. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139), lib `mairie360_api_lib` 4.3.0 via [#130](https://github.com/mairie360/API_template/pull/130))
- Journaux de requêtes : la query string et le `Referer` n'apparaissent plus. Restent l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Les erreurs de la lib dans les journaux ne citent plus la valeur reçue : les erreurs base sont décrites par SQLSTATE, contrainte, table et colonne, et les messages serde ne citent plus la valeur. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution : avant, `gcr.io/distroless/cc-debian12` (libssl3 non corrigée, CVE-2026-84782) ; maintenant `cc-debian13:nonroot`, avec builder et `development.Dockerfile` sur `rust:1.99-*-trixie`. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Simulation d'usage RGPD avant prod (MAIR-497, [#137](https://github.com/mairie360/API_template/pull/137)) : nouveaux `gdpr-simulation.yaml`, `docker-compose-gdpr-simulation.yml` et `gdpr_simulation_test.sh`, sans endpoint modifié.
- CI : mairie360/CICD monté jusqu'à v5.3.1 ; contrôle RGPD des PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)) ; test `tests/mair_771_cicd_version.rs` qui vérifie que tous les workflows appellent la même release (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133)).
- Images épinglées par digest (Dockerfile, development.Dockerfile, stacks compose) et Node 26 pour le runner GDPR : mises à jour Renovate sans effet fonctionnel.
- Dépendances Rust : tokio 1.54.1, actix-web 4.18.1, log 0.4.35 (dev). Audit RustSec (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)) : zstd, ctutils, rand et spin mis à jour, RUSTSEC-2026-0387 ignoré avec son motif dans `.cargo/audit.toml`.
- `CLAUDE.md` documente `DB_POOL_MIN_CONNECTIONS`, `DB_POOL_MAX_CONNECTIONS` et `DB_POOL_IDLE_TIMEOUT_SECS` (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)) : documentation seule.
- Tests ajoutés : `mair_741_every_image_runs_on_debian_13`, `tests/telemetry_test.rs`, `tests/auth_gate_test.rs` et `tests/session_tokens` (signature EdDSA), `tests/mair_680_jwt_config.rs`.

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- CI : mairie360/CICD monté de v4.2.2 à v4.3.2 dans `cicd.yml` (journal des releases, MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94) ; [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98), [#95](https://github.com/mairie360/API_template/pull/95)). Aucun changement pour les APIs ni les appelants.
- `cicd.yml` transmet `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow réutilisable : effet sur la CI uniquement.
- Mises à jour utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)) et tokio 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) : sans changement de fichier visible dans la PR.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
