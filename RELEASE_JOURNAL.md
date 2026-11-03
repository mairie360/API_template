# Journal des releases — API_template

<!-- release-journal-state {"dev": "2d8f9b41b15d702f13a653944155b30fb23be3a5", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-03 00:05 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `2d8f9b4` | 2026-11-03 00:05 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Export des traces OpenTelemetry, désactivé par défaut : il s'active dès que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini, et `OTEL_SDK_DISABLED=true` le coupe. Chaque requête produit un span racine `<MÉTHODE> <route>` qui reprend le `traceparent` entrant, y compris les requêtes refusées par `JwtMiddleware` (401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Format de logs partagé : texte coloré par défaut, une ligne JSON par événement avec `LOG_FORMAT=json`, niveau réglé par `RUST_LOG` (défaut `info`). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Checklist RGPD sur chaque pull request : le check « gdpr / GDPR checklist » échoue si la section « Personal data » de la description n'est pas remplie. (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70))

#### Changements de comportement
- Les journaux de requête n'affichent plus la query string ni le `Referer` : seuls l'adresse du client, la méthode, le chemin sans query, le statut, la taille et la durée sont journalisés. Un outil qui lisait ces détails dans les logs ne les retrouve plus. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution : `libssl3` de Debian 12 (distroless `cc-debian12:nonroot`) n'est pas corrigée (CVE-2026-84782). Avant : runtime `cc-debian12:nonroot`, builder `rust:1.99-slim-bookworm`, `development.Dockerfile` en `bookworm`. Maintenant : runtime `cc-debian13:nonroot`, builder `rust:1.99-slim-trixie`, dev en `rust:1.99-trixie`. Le test `mair_741_every_image_runs_on_debian_13` échoue si une image n'est pas sur Debian 13. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Fuite de données personnelles dans les journaux : avant, `middleware::Logger::default()` écrivait la ligne de requête complète, query string comprise, et le `Referer` ; les erreurs de la lib pouvaient reprendre les messages Postgres et les valeurs saisies. Maintenant, le `request_logger()` du dépôt et les erreurs décrites par leur type et leur contexte. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66) ; le diff fourni est tronqué à la fin de `src/main.rs`)

#### Technique
- Alertes RustSec corrigées par mise à jour de `Cargo.lock` (zstd 0.14.3, zstd-safe 8.2.0, zstd-sys 2.1.2, ctutils 0.4.7, rand, spin) ; RUSTSEC-2026-0387 (crypto-bigint) ignorée dans `.cargo/audit.toml` avec sa justification. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Tests et stack de vérification des logs : `tests/telemetry_test.rs`, `docker-compose-gdpr-marker.yml`, `gdpr-marker.yaml`, `gdpr_marker_test.sh`, et la dépendance de test `log`. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Images de test et de build épinglées par digest (Dockerfile, development.Dockerfile, docker-compose*.yml) ; `gdpr-marker` passe de `node:24` à `node:26-bookworm-slim`. (Renovate, [#134](https://github.com/mairie360/API_template/pull/134), [#121](https://github.com/mairie360/API_template/pull/121))
- Workflows CICD `mairie360/CICD` passés de v4.5.0 à v5.2.2 (passage en majeure v5 via [#131](https://github.com/mairie360/API_template/pull/131)), avec les digests Rust et Redis mis à jour ([#107](https://github.com/mairie360/API_template/pull/107) à [#118](https://github.com/mairie360/API_template/pull/118)).
- `mairie360_api_lib` passée de 4.2.0 à 4.2.1 ([#122](https://github.com/mairie360/API_template/pull/122)) puis 4.3.0 ([#130](https://github.com/mairie360/API_template/pull/130)) : pas de diff visible dans la PR ; la description de #130 annonce un changement du secret JWT HS256, à vérifier côté lib.
- Mises à jour sans diff visible : `log` 0.4.35, `actix-web` 4.18.1, `tokio` 1.54.1, `utoipa-swagger-ui` 10.0.2.
- Documentation : section « Logs and observability » et réglages du pool Postgres (`DB_POOL_*`) dans CLAUDE.md. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79) ; MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Export des traces OpenTelemetry, désactivé par défaut : il s'active dès que `OTEL_EXPORTER_OTLP_ENDPOINT` est défini, et `OTEL_SDK_DISABLED=true` le coupe. Chaque requête produit un span racine qui reprend le `traceparent` entrant, y compris les requêtes refusées par `JwtMiddleware` (401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Format de logs partagé : texte coloré par défaut, JSON avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Checklist RGPD sur chaque pull request, check « gdpr / GDPR checklist ». (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70))

#### Changements de comportement
- Les journaux de requête n'affichent plus la query string ni le `Referer`. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution passée à distroless Debian 13 (`cc-debian13:nonroot`) et builder en `rust:1.99-slim-trixie`, pour corriger `libssl3` non patchée de Debian 12 (CVE-2026-84782). (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Fuite de données personnelles dans les journaux : les logs ne contiennent plus les valeurs de requête ni les messages d'erreur bruts. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Technique
- Alertes RustSec corrigées dans `Cargo.lock`, RUSTSEC-2026-0387 ignorée avec justification. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Digests épinglés pour les images Docker et CICD mis à jour `mairie360/CICD` jusqu'à v4.12.2 à v5.2.0 ([#128](https://github.com/mairie360/API_template/pull/128), [#131](https://github.com/mairie360/API_template/pull/131)).
- `mairie360_api_lib` 4.2.1 puis 4.3.0 (diff non visible dans les PR), `log`, `actix-web` et `tokio` mis à jour sans changement visible.
- Tests de logs et stack `gdpr-marker` ajoutés.

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Journal des releases rédigé par Claude (MAIR-571) : `mairie360/CICD` passe de v4.3.0 à v4.3.2 ([#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)). Ce changement touche le workflow de release, sans effet sur l'API elle-même.
- Le caller CICD transmet explicitement `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow réutilisable, car un `secrets:` explicite ne les forwarde pas. (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94))
- Dépendances sans diff visible : `utoipa-swagger-ui` 10.0.1 vers 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)) et `tokio` 1.53.3 vers 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)), dont la description annonce un MSRV relevé à 1.85.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
