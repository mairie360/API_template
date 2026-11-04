# Journal des releases — API_template

<!-- release-journal-state {"dev": "88c862136cff9f98915f455bef5d90aaafdd55c6", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-04 23:15 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `88c8621` | 2026-11-04 23:15 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement
- Sans `JWT_SECRET`, l'API refuse les jetons HS256 (401) et n'accepte que les jetons EdDSA signés par Core_API, vérifiés avec `JWT_PUBLIC_KEY`. Un déploiement doit fournir `JWT_PUBLIC_KEY` : l'API refuse de démarrer sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`. Tant que `JWT_SECRET` est défini, les HS256 signés avec lui restent acceptés (transition). La description du schéma `jwt` dans l'OpenAPI est mise à jour. Les stacks de test n'embarquent plus de jeton admin HS256. Montée de `mairie360_api_lib` en 4.3.0. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, activé seulement si `OTEL_EXPORTER_OTLP_ENDPOINT` est défini. `LOG_FORMAT=json` produit des logs en JSON, une ligne par événement. Chaque requête, y compris celles refusées par le JWT, reçoit un span portant sa route et son statut. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Corrections
- Journaux de requêtes : avant, la query string et le `Referer` étaient écrits dans les logs (`Logger::default()`). Maintenant, seuls l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée sont loggés. Les erreurs de la lib décrivent la base par SQLSTATE, contrainte, table et colonne, sans la valeur ni le `DETAIL` Postgres. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : avant, `gcr.io/distroless/cc-debian12` avec une `libssl3` non corrigée (CVE-2026-84782). Maintenant, `cc-debian13:nonroot`, avec un builder `rust:1.99-slim-trixie` et une image de développement sur trixie. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Simulation d'usage RGPD avant la prod : `gdpr-simulation.yaml`, `docker-compose-gdpr-simulation.yml`, `gdpr_simulation_test.sh`, et exclusions `.gitignore` / `.dockerignore`. (MAIR-497, [#137](https://github.com/mairie360/API_template/pull/137))
- Test du marqueur RGPD dans les journaux : `gdpr-marker.yaml`, `docker-compose-gdpr-marker.yml`, `gdpr_marker_test.sh`, et tests `request_log`. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Test `tests/mair_771_cicd_version.rs` qui vérifie que tous les workflows appellent la même release CICD (≥ v4.12.0). Workflows fixés sur CICD v5.2.2. (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133))
- Checklist RGPD sur chaque PR (`gdpr-pr.yml`, check « gdpr / GDPR checklist »). (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70))
- Correction des alertes RustSec : `zstd`, `ctutils`, `rand`, `spin` mis à jour, et `RUSTSEC-2026-0387` ignoré dans `.cargo/audit.toml`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Documentation du pool Postgres (`DB_POOL_*`) dans `CLAUDE.md`, sans changement de code. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Renovate : `mairie360/CICD` de v4.3.0 à v5.5.0 (workflows `cicd.yml` et `gdpr-pr.yml`), digests d'images épinglés (rust, postgres, redis, curl, k6, newman, ZAP, nginx), `node` 24 → 26 dans le runner du marqueur, `log` 0.4.35 en dev-dependency. Le diff de certaines PR n'est pas fourni, d'où la prudence sur leur effet exact.

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- Sans `JWT_SECRET`, l'API refuse les jetons HS256 (401) et n'accepte que les jetons EdDSA signés par Core_API, vérifiés avec `JWT_PUBLIC_KEY`. Un déploiement doit fournir `JWT_PUBLIC_KEY` : l'API refuse de démarrer sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`. Tant que `JWT_SECRET` est défini, les HS256 signés avec lui restent acceptés (transition). La description du schéma `jwt` dans l'OpenAPI est mise à jour. Les stacks de test n'embarquent plus de jeton admin HS256. Montée de `mairie360_api_lib` en 4.3.0. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, activé seulement si `OTEL_EXPORTER_OTLP_ENDPOINT` est défini. `LOG_FORMAT=json` produit des logs en JSON, une ligne par événement. Chaque requête, y compris celles refusées par le JWT, reçoit un span portant sa route et son statut. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Corrections
- Journaux de requêtes : avant, la query string et le `Referer` étaient écrits dans les logs (`Logger::default()`). Maintenant, seuls l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée sont loggés. Les erreurs de la lib décrivent la base par SQLSTATE, contrainte, table et colonne, sans la valeur ni le `DETAIL` Postgres. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : avant, `gcr.io/distroless/cc-debian12` avec une `libssl3` non corrigée (CVE-2026-84782). Maintenant, `cc-debian13:nonroot`, avec un builder `rust:1.99-slim-trixie` et une image de développement sur trixie. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Simulation d'usage RGPD avant la prod : `gdpr-simulation.yaml`, `docker-compose-gdpr-simulation.yml`, `gdpr_simulation_test.sh`, et exclusions `.gitignore` / `.dockerignore`. (MAIR-497, [#137](https://github.com/mairie360/API_template/pull/137))
- Test du marqueur RGPD dans les journaux : `gdpr-marker.yaml`, `docker-compose-gdpr-marker.yml`, `gdpr_marker_test.sh`, et tests `request_log`. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Test `tests/mair_771_cicd_version.rs` qui vérifie que tous les workflows appellent la même release CICD (≥ v4.12.0). Workflows fixés sur CICD v5.2.2. (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133))
- Checklist RGPD sur chaque PR (`gdpr-pr.yml`, check « gdpr / GDPR checklist »). (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70))
- Correction des alertes RustSec : `zstd`, `ctutils`, `rand`, `spin` mis à jour, et `RUSTSEC-2026-0387` ignoré dans `.cargo/audit.toml`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Documentation du pool Postgres (`DB_POOL_*`) dans `CLAUDE.md`, sans changement de code. (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))
- Renovate : `mairie360/CICD` de v4.4.0 à v5.5.0 (workflows `cicd.yml` et `gdpr-pr.yml`), digests d'images épinglés (rust, postgres, redis, curl, k6, newman, ZAP, nginx), `node` 24 → 26 dans le runner du marqueur, `log` 0.4.35 en dev-dependency, `actix-web` 4.18.x (diff non fourni).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Dépendances : `utoipa-swagger-ui` 10.0.1 → 10.0.2 (diff non fourni, [#99](https://github.com/mairie360/API_template/pull/99)) et `tokio` 1.53.3 → 1.54.0 (diff non fourni, [#96](https://github.com/mairie360/API_template/pull/96)).
- CI : `cicd.yml` transmet à CICD les secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal de release (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).
- CI : journal de release par repo rédigé par Claude, via `mairie360/CICD` v4.3.0 à v4.3.2 (MAIR-571, [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
