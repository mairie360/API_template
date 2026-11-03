# Journal des releases — API_template

<!-- release-journal-state {"dev": "bb7b1bcd269b451eb575d85f374251394d26c00c", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-03 01:33 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `bb7b1bc` | 2026-11-03 01:33 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Journaux et traces : `LOG_FORMAT=json` émet une ligne JSON par événement. L'export OTLP/HTTP des traces OpenTelemetry n'est activé que si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. Chaque requête a un span racine `<MÉTHODE> <route>` qui continue un `traceparent` entrant. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Authentification : l'API ne valide plus les sessions avec un secret `JWT_SECRET` partagé. Elle attend `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API) et démarre sans `JWT_SECRET`. Sans `JWT_PUBLIC_KEY`, le démarrage échoue. Quand `JWT_SECRET` est absent, tout jeton HS256 est refusé (401). Tant qu'il est défini, la lib accepte encore les HS256 signés avec lui (transition). Les appelants doivent présenter des jetons EdDSA signés par Core. Les stacks de test exigent `JWT_PUBLIC_KEY` et n'utilisent plus de jeton admin figé. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Journaux des requêtes : la ligne de requête n'inclut plus la query string ni le `Referer`. Seuls l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée sont journalisés. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Technique
- Images Docker : builder, runtime et image de développement passent sur Debian 13 (`trixie`), avec distroless `cc-debian13` (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126)). Un test `mair_741_every_image_runs_on_debian_13` vérifie la release Debian de chaque image.
- Digests d'images épinglés ou mis à jour par Renovate : rust, postgres, redis, curl, k6, newman, ZAP, nginx et node 26 ([#121](https://github.com/mairie360/API_template/pull/121), [#134](https://github.com/mairie360/API_template/pull/134), [#138](https://github.com/mairie360/API_template/pull/138), [#118](https://github.com/mairie360/API_template/pull/118), [#117](https://github.com/mairie360/API_template/pull/117), [#116](https://github.com/mairie360/API_template/pull/116), [#115](https://github.com/mairie360/API_template/pull/115), [#111](https://github.com/mairie360/API_template/pull/111), [#108](https://github.com/mairie360/API_template/pull/108), [#107](https://github.com/mairie360/API_template/pull/107), [#104](https://github.com/mairie360/API_template/pull/104), [#103](https://github.com/mairie360/API_template/pull/103), [#102](https://github.com/mairie360/API_template/pull/102)).
- Workflows CICD : montée progressive de v4.3.x à v5.2.2, avec la garde `tests/mair_771_cicd_version.rs` (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133)). Ajout du workflow de checklist RGPD des PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)).
- Dépendances Rust : actix-web 4.18.x ([#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114)), tokio 1.54.x ([#96](https://github.com/mairie360/API_template/pull/96), [#109](https://github.com/mairie360/API_template/pull/109)), log 0.4.35 ([#120](https://github.com/mairie360/API_template/pull/120)), utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)), mairie360_api_lib 4.2.1 puis 4.3.0 ([#122](https://github.com/mairie360/API_template/pull/122), [#130](https://github.com/mairie360/API_template/pull/130), diff absent du journal).
- Audit RustSec (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)) : mise à jour de Cargo.lock et ignore de RUSTSEC-2026-0387 dans `.cargo/audit.toml`.
- Tests et documentation : tests de télémétrie (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79)), test du marqueur RGPD dans les journaux (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66)), documentation du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)), tests JWT (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139)).

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- Authentification : l'API ne valide plus les sessions avec un secret `JWT_SECRET` partagé. Elle attend `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API) et démarre sans `JWT_SECRET`. Sans `JWT_PUBLIC_KEY`, le démarrage échoue. Quand `JWT_SECRET` est absent, tout jeton HS256 est refusé (401). Tant qu'il est défini, la lib accepte encore les HS256 signés avec lui (transition). Les appelants doivent présenter des jetons EdDSA signés par Core. Les stacks de test exigent `JWT_PUBLIC_KEY` et n'utilisent plus de jeton admin figé. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Journaux des requêtes : la ligne de requête n'inclut plus la query string ni le `Referer`. Seuls l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée sont journalisés. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Nouveautés
- Journaux et traces : `LOG_FORMAT=json` émet une ligne JSON par événement. L'export OTLP/HTTP des traces n'est activé que si `OTEL_EXPORTER_OTLP_ENDPOINT` est défini. Chaque requête a un span racine qui continue un `traceparent` entrant. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique
- Images Docker : builder, runtime et image de développement passent sur Debian 13 (`trixie`), avec distroless `cc-debian13` (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126)).
- Digests d'images épinglés ou mis à jour par Renovate (rust, postgres, redis, curl, k6, newman, ZAP, nginx, node 26) ([#121](https://github.com/mairie360/API_template/pull/121), [#134](https://github.com/mairie360/API_template/pull/134), [#138](https://github.com/mairie360/API_template/pull/138), [#118](https://github.com/mairie360/API_template/pull/118), [#117](https://github.com/mairie360/API_template/pull/117), [#116](https://github.com/mairie360/API_template/pull/116), [#115](https://github.com/mairie360/API_template/pull/115), [#111](https://github.com/mairie360/API_template/pull/111), [#108](https://github.com/mairie360/API_template/pull/108), [#107](https://github.com/mairie360/API_template/pull/107), [#104](https://github.com/mairie360/API_template/pull/104), [#103](https://github.com/mairie360/API_template/pull/103), [#102](https://github.com/mairie360/API_template/pull/102)).
- Workflows CICD montés de v4.4.0 à v5.2.2, avec la garde `tests/mair_771_cicd_version.rs` (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133)). Ajout du workflow de checklist RGPD des PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)).
- Dépendances Rust : actix-web 4.18.x ([#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114)), tokio 1.54.1 ([#109](https://github.com/mairie360/API_template/pull/109)), log 0.4.35 ([#120](https://github.com/mairie360/API_template/pull/120)), mairie360_api_lib 4.2.1 puis 4.3.0 ([#122](https://github.com/mairie360/API_template/pull/122), [#130](https://github.com/mairie360/API_template/pull/130), diff absent du journal).
- Audit RustSec (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)) : Cargo.lock et ignore de RUSTSEC-2026-0387 dans `.cargo/audit.toml`.
- Tests et documentation : télémétrie (MAIR-503), marqueur RGPD dans les journaux (MAIR-290), pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)), tests JWT (MAIR-680).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Workflows CICD montés de v4.3.0 à v5.2.x pour les appels de release, avec transmission des secrets du journal de release (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94), [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)).
- Dépendances : tokio 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et utoipa-swagger-ui 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
