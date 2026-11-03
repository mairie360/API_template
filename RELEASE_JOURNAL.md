# Journal des releases — API_template

<!-- release-journal-state {"dev": "51f9b72ac6d0c9981ab51767891dc73e8e702862", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-03 15:14 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `51f9b72` | 2026-11-03 15:14 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement
- L'API lit désormais la clé publique Ed25519 de Core_API dans `JWT_PUBLIC_KEY` et refuse de démarrer sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`. Tant que `JWT_SECRET` est défini, les jetons HS256 signés avec lui restent acceptés (transition). Une fois ce secret retiré, un jeton HS256 reçoit `401`, et un jeton EdDSA sans l'audience de Core est refusé. Les appelants (front, BFF) doivent donc présenter des jetons signés par Core_API. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Les logs passent par la configuration commune `init_telemetry` : texte coloré sur stdout par défaut (niveau `RUST_LOG`, `info` par défaut), une ligne JSON par événement avec `LOG_FORMAT=json`. Le span racine de chaque requête n'apparaît pas dans ces logs. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Nouveautés
- Export de traces OpenTelemetry en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini ; `OTEL_SDK_DISABLED=true` le coupe. Chaque requête produit un span nommé `<METHODE> <route>` qui continue un en-tête `traceparent` entrant ; les requêtes refusées par le contrôle JWT (401) en ont un aussi. Les requêtes SQL apparaissent en événements de span, sans leurs valeurs. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Corrections
- Logs de requêtes : avant, la ligne de requête (query string comprise) et l'en-tête `Referer` étaient journalisés. Maintenant seuls l'adresse client, la méthode, le chemin sans query string, le statut, la taille et la durée sont journalisés (`request_logger()`). (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : `gcr.io/distroless/cc-debian12` remplacée par `cc-debian13` (libssl3 corrigée, CVE-2026-84782). Le builder et l'image de développement passent sur `rust:1.99-*-trixie`. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Dépendances Rust : `zstd`, `ctutils`, `rand` et `spin` mis à jour, RUSTSEC-2026-0387 ignoré dans `.cargo/audit.toml` (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)). Mises à jour de `actix-web`, `tokio`, `log`, `utoipa-swagger-ui` et `mairie360_api_lib` (4.2.1 puis 4.3.0), sans effet visible.
- CI : mises à jour de mairie360/CICD de v4.4.0 à v5.3.0 (une quinzaine de PR), de digests Docker (rust, postgres, redis, curl, k6, newman, nginx, node, zaproxy) et de Node 26 pour le gdpr-marker. Ajout de tests de cohérence (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133)) et de la checklist RGPD des PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)). Transmission de `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow réutilisable (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)). Sans effet sur l'API.
- Documentation des variables `DB_POOL_*` (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)). Mise à jour de `tokio` et `utoipa-swagger-ui` sans changement de code.

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- L'API lit désormais la clé publique Ed25519 de Core_API dans `JWT_PUBLIC_KEY` et refuse de démarrer sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`. Tant que `JWT_SECRET` est défini, les jetons HS256 signés avec lui restent acceptés (transition). Une fois ce secret retiré, un jeton HS256 reçoit `401`, et un jeton EdDSA sans l'audience de Core est refusé. Les appelants (front, BFF) doivent donc présenter des jetons signés par Core_API. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Les logs passent par la configuration commune `init_telemetry` : texte coloré sur stdout par défaut (niveau `RUST_LOG`, `info` par défaut), une ligne JSON par événement avec `LOG_FORMAT=json`. Le span racine de chaque requête n'apparaît pas dans ces logs. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Nouveautés
- Export de traces OpenTelemetry en OTLP/HTTP, désactivé tant que `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) n'est pas défini ; `OTEL_SDK_DISABLED=true` le coupe. Chaque requête produit un span nommé `<METHODE> <route>` qui continue un en-tête `traceparent` entrant ; les requêtes refusées par le contrôle JWT (401) en ont un aussi. Les requêtes SQL apparaissent en événements de span, sans leurs valeurs. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Corrections
- Logs de requêtes : avant, la ligne de requête (query string comprise) et l'en-tête `Referer` étaient journalisés. Maintenant seuls l'adresse client, la méthode, le chemin sans query string, le statut, la taille et la durée sont journalisés (`request_logger()`). (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : `gcr.io/distroless/cc-debian12` remplacée par `cc-debian13` (libssl3 corrigée, CVE-2026-84782). Le builder et l'image de développement passent sur `rust:1.99-*-trixie`. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Dépendances Rust : `zstd`, `ctutils`, `rand` et `spin` mis à jour, RUSTSEC-2026-0387 ignoré dans `.cargo/audit.toml` (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)). Mises à jour de `actix-web`, `tokio`, `log`, `utoipa-swagger-ui` et `mairie360_api_lib` (4.2.1 puis 4.3.0), sans effet visible.
- CI : mises à jour de mairie360/CICD de v4.4.0 à v5.3.0 (une dizaine de PR), de digests Docker (rust, postgres, redis, curl, k6, newman, nginx, node, zaproxy) et de Node 26 pour le gdpr-marker. Ajout de tests de cohérence (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133)) et de la checklist RGPD des PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)). Sans effet sur l'API.
- Documentation des variables `DB_POOL_*` (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)), documentation seule.

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Mises à jour de mairie360/CICD (v4.3.0 à v4.3.2), de `tokio` 1.54.0 et de `utoipa-swagger-ui` 10.0.2, sans effet visible pour les utilisateurs de l'API.
- Dans `cicd.yml`, les secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` sont transmis au workflow réutilisable (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)). Sans effet sur l'API.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
