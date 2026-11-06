# Journal des releases — API_template

<!-- release-journal-state {"dev": "a06e22bb559456525e95dc6fc55b65e211ca805c", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-06 03:00 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `a06e22b` | 2026-11-06 03:00 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

- chore(deps): update rust:1.99-trixie docker digest to bc2c4c2 (#151) ([#151](https://github.com/mairie360/API_template/pull/151))
- chore(deps): update postgres:18-alpine docker digest to 123b179 (#150) ([#150](https://github.com/mairie360/API_template/pull/150))
- chore(deps): update rust:1.99-slim-trixie docker digest to 7eab7ce (#149) ([#149](https://github.com/mairie360/API_template/pull/149))
- chore(deps): update node.js to add5c17 (#148) ([#148](https://github.com/mairie360/API_template/pull/148))
- chore(deps): update node.js to c3aea91 (#147) ([#147](https://github.com/mairie360/API_template/pull/147))
- chore(deps): update rust:1.99-slim-trixie docker digest to 59f72d5 (#146) ([#146](https://github.com/mairie360/API_template/pull/146))
- chore(deps): update dependency mairie360/cicd to v5.5.0 (#145) ([#145](https://github.com/mairie360/API_template/pull/145))
- feat: MAIR-497 GDPR - Usage simulation and AI review of logs and database before prod ([#137](https://github.com/mairie360/API_template/pull/137))
- chore(deps): update dependency mairie360/cicd to v5.3.1 (#144) ([#144](https://github.com/mairie360/API_template/pull/144))
- chore(deps): update rust:1.99-trixie docker digest to 4c4c8ec (#143) ([#143](https://github.com/mairie360/API_template/pull/143))
- chore(deps): update rust:1.99-slim-trixie docker digest to ea0999f (#142) ([#142](https://github.com/mairie360/API_template/pull/142))
- chore(deps): update dependency mairie360/cicd to v5.3.0 (#141) ([#141](https://github.com/mairie360/API_template/pull/141))
- chore(deps): update zaproxy/zap-stable docker digest to 9624669 (#140) ([#140](https://github.com/mairie360/API_template/pull/140))
- fix: MAIR-680 [Audit 23/10] Toutes les APIs et BFFs : remplacer le secret JWT HS256 partagé par 12 services (A1) ([#139](https://github.com/mairie360/API_template/pull/139))
- ci: MAIR-771 APIs, BFFs, Database : monter CICD en v5.2.2 pour activer les contrôles RGPD (#133) ([#133](https://github.com/mairie360/API_template/pull/133))
- chore(deps): update rust:1.99-slim-trixie docker digest to 9daf2b2 (#138) ([#138](https://github.com/mairie360/API_template/pull/138))
- chore(deps): update dependency mairie360/cicd to v5.2.2 (#135) ([#135](https://github.com/mairie360/API_template/pull/135))
- chore(deps): pin dependencies (#134) ([#134](https://github.com/mairie360/API_template/pull/134))
- chore(deps): update dependency mairie360/cicd to v5 (#131) ([#131](https://github.com/mairie360/API_template/pull/131))
- chore(deps): update rust crate mairie360_api_lib to v4.3.0 (#130) ([#130](https://github.com/mairie360/API_template/pull/130))
- chore(deps): update dependency mairie360/cicd to v4.13.0 (#129) ([#129](https://github.com/mairie360/API_template/pull/129))
- chore(deps): update dependency mairie360/cicd to v4.12.2 (#128) ([#128](https://github.com/mairie360/API_template/pull/128))
- fix: MAIR-741 APIs : passer l'image d'exécution en distroless Debian 13 (CVE-2026-84782) (#126) ([#126](https://github.com/mairie360/API_template/pull/126))
- chore(deps): update dependency mairie360/cicd to v4.12.1 (#127) ([#127](https://github.com/mairie360/API_template/pull/127))
- fix(deps): MAIR-740 APIs : corriger les alertes RustSec de cargo audit (crypto-bigint, ctutils, zstd) (#125) ([#125](https://github.com/mairie360/API_template/pull/125))
- chore(deps): update rust crate mairie360_api_lib to v4.2.1 (#122) ([#122](https://github.com/mairie360/API_template/pull/122))
- chore(deps): update node.js to v26 (#121) ([#121](https://github.com/mairie360/API_template/pull/121))
- chore(deps): update rust crate log to v0.4.35 (#120) ([#120](https://github.com/mairie360/API_template/pull/120))
- fix: MAIR-290 Détecter les données personnelles dans les journaux avec un test du marqueur (#66) ([#66](https://github.com/mairie360/API_template/pull/66))
- ci: MAIR-295 Ajouter une checklist RGPD aux PR et des rappels d'échéances automatiques (#70) ([#70](https://github.com/mairie360/API_template/pull/70))
- chore(deps): update dependency mairie360/cicd to v4.12.0 (#119) ([#119](https://github.com/mairie360/API_template/pull/119))
- chore(deps): update rust:1.99-bookworm docker digest to 9181b56 (#118) ([#118](https://github.com/mairie360/API_template/pull/118))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 24707e7 (#117) ([#117](https://github.com/mairie360/API_template/pull/117))
- chore(deps): update rust:1.99-bookworm docker digest to e9da6f6 (#116) ([#116](https://github.com/mairie360/API_template/pull/116))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 61e86ba (#115) ([#115](https://github.com/mairie360/API_template/pull/115))
- chore(deps): update rust crate actix-web to v4.18.1 (#114) ([#114](https://github.com/mairie360/API_template/pull/114))
- chore(deps): update rust crate actix-web to v4.18.0 (#113) ([#113](https://github.com/mairie360/API_template/pull/113))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 86234e8 (#112) ([#112](https://github.com/mairie360/API_template/pull/112))
- chore(deps): update rust:1.99-bookworm docker digest to 48b7f43 (#111) ([#111](https://github.com/mairie360/API_template/pull/111))
- chore(deps): update dependency mairie360/cicd to v4.5.0 (#110) ([#110](https://github.com/mairie360/API_template/pull/110))
- chore(deps): update rust crate tokio to v1.54.1 (#109) ([#109](https://github.com/mairie360/API_template/pull/109))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 54e39a2 (#108) ([#108](https://github.com/mairie360/API_template/pull/108))
- chore(deps): update rust:1.99-bookworm docker digest to edaa2a8 (#107) ([#107](https://github.com/mairie360/API_template/pull/107))
- docs: MAIR-565 APIs - Rendre le pool de connexions Postgres paramétrable par instance (#82) ([#82](https://github.com/mairie360/API_template/pull/82))
- feat: MAIR-503 Mutualiser logs et télémétrie OpenTelemetry dans API_lib et les brancher sur toutes les APIs (#79) ([#79](https://github.com/mairie360/API_template/pull/79))
- chore(deps): update dependency mairie360/cicd to v4.4.3 (#105) ([#105](https://github.com/mairie360/API_template/pull/105))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 16113eb (#104) ([#104](https://github.com/mairie360/API_template/pull/104))
- chore(deps): update rust:1.99-slim-bookworm docker digest to f48da6d (#103) ([#103](https://github.com/mairie360/API_template/pull/103))
- chore(deps): update rust:1.99-bookworm docker digest to 5c1a173 (#102) ([#102](https://github.com/mairie360/API_template/pull/102))
- chore(deps): update dependency mairie360/cicd to v4.4.1 (#101) ([#101](https://github.com/mairie360/API_template/pull/101))
- chore(deps): update dependency mairie360/cicd to v4.4.0 (#100) ([#100](https://github.com/mairie360/API_template/pull/100))
- chore(deps): update rust crate utoipa-swagger-ui to v10.0.2 (#99) ([#99](https://github.com/mairie360/API_template/pull/99))
- chore(deps): update dependency mairie360/cicd to v4.3.2 (#98) ([#98](https://github.com/mairie360/API_template/pull/98))
- chore(ci): MAIR-571 Forward the release-journal inputs to CICD v4.3.1 (#94) ([#94](https://github.com/mairie360/API_template/pull/94))
- chore(deps): update dependency mairie360/cicd to v4.3.1 (#97) ([#97](https://github.com/mairie360/API_template/pull/97))
- chore(deps): update rust crate tokio to v1.54.0 (#96) ([#96](https://github.com/mairie360/API_template/pull/96))
- chore(deps): update dependency mairie360/cicd to v4.3.0 (#95) ([#95](https://github.com/mairie360/API_template/pull/95))

## Différence dev → staging (en dev, pas encore en staging)

- chore(deps): update rust:1.99-trixie docker digest to bc2c4c2 (#151) ([#151](https://github.com/mairie360/API_template/pull/151))
- chore(deps): update postgres:18-alpine docker digest to 123b179 (#150) ([#150](https://github.com/mairie360/API_template/pull/150))
- chore(deps): update rust:1.99-slim-trixie docker digest to 7eab7ce (#149) ([#149](https://github.com/mairie360/API_template/pull/149))
- chore(deps): update node.js to add5c17 (#148) ([#148](https://github.com/mairie360/API_template/pull/148))
- chore(deps): update node.js to c3aea91 (#147) ([#147](https://github.com/mairie360/API_template/pull/147))
- chore(deps): update rust:1.99-slim-trixie docker digest to 59f72d5 (#146) ([#146](https://github.com/mairie360/API_template/pull/146))
- chore(deps): update dependency mairie360/cicd to v5.5.0 (#145) ([#145](https://github.com/mairie360/API_template/pull/145))
- feat: MAIR-497 GDPR - Usage simulation and AI review of logs and database before prod ([#137](https://github.com/mairie360/API_template/pull/137))
- chore(deps): update dependency mairie360/cicd to v5.3.1 (#144) ([#144](https://github.com/mairie360/API_template/pull/144))
- chore(deps): update rust:1.99-trixie docker digest to 4c4c8ec (#143) ([#143](https://github.com/mairie360/API_template/pull/143))
- chore(deps): update rust:1.99-slim-trixie docker digest to ea0999f (#142) ([#142](https://github.com/mairie360/API_template/pull/142))
- chore(deps): update dependency mairie360/cicd to v5.3.0 (#141) ([#141](https://github.com/mairie360/API_template/pull/141))
- chore(deps): update zaproxy/zap-stable docker digest to 9624669 (#140) ([#140](https://github.com/mairie360/API_template/pull/140))
- fix: MAIR-680 [Audit 23/10] Toutes les APIs et BFFs : remplacer le secret JWT HS256 partagé par 12 services (A1) ([#139](https://github.com/mairie360/API_template/pull/139))
- ci: MAIR-771 APIs, BFFs, Database : monter CICD en v5.2.2 pour activer les contrôles RGPD (#133) ([#133](https://github.com/mairie360/API_template/pull/133))
- chore(deps): update rust:1.99-slim-trixie docker digest to 9daf2b2 (#138) ([#138](https://github.com/mairie360/API_template/pull/138))
- chore(deps): update dependency mairie360/cicd to v5.2.2 (#135) ([#135](https://github.com/mairie360/API_template/pull/135))
- chore(deps): pin dependencies (#134) ([#134](https://github.com/mairie360/API_template/pull/134))
- chore(deps): update dependency mairie360/cicd to v5 (#131) ([#131](https://github.com/mairie360/API_template/pull/131))
- chore(deps): update rust crate mairie360_api_lib to v4.3.0 (#130) ([#130](https://github.com/mairie360/API_template/pull/130))
- chore(deps): update dependency mairie360/cicd to v4.13.0 (#129) ([#129](https://github.com/mairie360/API_template/pull/129))
- chore(deps): update dependency mairie360/cicd to v4.12.2 (#128) ([#128](https://github.com/mairie360/API_template/pull/128))
- fix: MAIR-741 APIs : passer l'image d'exécution en distroless Debian 13 (CVE-2026-84782) (#126) ([#126](https://github.com/mairie360/API_template/pull/126))
- chore(deps): update dependency mairie360/cicd to v4.12.1 (#127) ([#127](https://github.com/mairie360/API_template/pull/127))
- fix(deps): MAIR-740 APIs : corriger les alertes RustSec de cargo audit (crypto-bigint, ctutils, zstd) (#125) ([#125](https://github.com/mairie360/API_template/pull/125))
- chore(deps): update rust crate mairie360_api_lib to v4.2.1 (#122) ([#122](https://github.com/mairie360/API_template/pull/122))
- chore(deps): update node.js to v26 (#121) ([#121](https://github.com/mairie360/API_template/pull/121))
- chore(deps): update rust crate log to v0.4.35 (#120) ([#120](https://github.com/mairie360/API_template/pull/120))
- fix: MAIR-290 Détecter les données personnelles dans les journaux avec un test du marqueur (#66) ([#66](https://github.com/mairie360/API_template/pull/66))
- ci: MAIR-295 Ajouter une checklist RGPD aux PR et des rappels d'échéances automatiques (#70) ([#70](https://github.com/mairie360/API_template/pull/70))
- chore(deps): update dependency mairie360/cicd to v4.12.0 (#119) ([#119](https://github.com/mairie360/API_template/pull/119))
- chore(deps): update rust:1.99-bookworm docker digest to 9181b56 (#118) ([#118](https://github.com/mairie360/API_template/pull/118))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 24707e7 (#117) ([#117](https://github.com/mairie360/API_template/pull/117))
- chore(deps): update rust:1.99-bookworm docker digest to e9da6f6 (#116) ([#116](https://github.com/mairie360/API_template/pull/116))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 61e86ba (#115) ([#115](https://github.com/mairie360/API_template/pull/115))
- chore(deps): update rust crate actix-web to v4.18.1 (#114) ([#114](https://github.com/mairie360/API_template/pull/114))
- chore(deps): update rust crate actix-web to v4.18.0 (#113) ([#113](https://github.com/mairie360/API_template/pull/113))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 86234e8 (#112) ([#112](https://github.com/mairie360/API_template/pull/112))
- chore(deps): update rust:1.99-bookworm docker digest to 48b7f43 (#111) ([#111](https://github.com/mairie360/API_template/pull/111))
- chore(deps): update dependency mairie360/cicd to v4.5.0 (#110) ([#110](https://github.com/mairie360/API_template/pull/110))
- chore(deps): update rust crate tokio to v1.54.1 (#109) ([#109](https://github.com/mairie360/API_template/pull/109))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 54e39a2 (#108) ([#108](https://github.com/mairie360/API_template/pull/108))
- chore(deps): update rust:1.99-bookworm docker digest to edaa2a8 (#107) ([#107](https://github.com/mairie360/API_template/pull/107))
- docs: MAIR-565 APIs - Rendre le pool de connexions Postgres paramétrable par instance (#82) ([#82](https://github.com/mairie360/API_template/pull/82))
- feat: MAIR-503 Mutualiser logs et télémétrie OpenTelemetry dans API_lib et les brancher sur toutes les APIs (#79) ([#79](https://github.com/mairie360/API_template/pull/79))
- chore(deps): update dependency mairie360/cicd to v4.4.3 (#105) ([#105](https://github.com/mairie360/API_template/pull/105))
- chore(deps): update rust:1.99-slim-bookworm docker digest to 16113eb (#104) ([#104](https://github.com/mairie360/API_template/pull/104))
- chore(deps): update rust:1.99-slim-bookworm docker digest to f48da6d (#103) ([#103](https://github.com/mairie360/API_template/pull/103))
- chore(deps): update rust:1.99-bookworm docker digest to 5c1a173 (#102) ([#102](https://github.com/mairie360/API_template/pull/102))
- chore(deps): update dependency mairie360/cicd to v4.4.1 (#101) ([#101](https://github.com/mairie360/API_template/pull/101))
- chore(deps): update dependency mairie360/cicd to v4.4.0 (#100) ([#100](https://github.com/mairie360/API_template/pull/100))

## Différence staging → prod (en staging, pas encore en prod)

- chore(deps): update rust crate utoipa-swagger-ui to v10.0.2 (#99) ([#99](https://github.com/mairie360/API_template/pull/99))
- chore(deps): update dependency mairie360/cicd to v4.3.2 (#98) ([#98](https://github.com/mairie360/API_template/pull/98))
- chore(ci): MAIR-571 Forward the release-journal inputs to CICD v4.3.1 (#94) ([#94](https://github.com/mairie360/API_template/pull/94))
- chore(deps): update dependency mairie360/cicd to v4.3.1 (#97) ([#97](https://github.com/mairie360/API_template/pull/97))
- chore(deps): update rust crate tokio to v1.54.0 (#96) ([#96](https://github.com/mairie360/API_template/pull/96))
- chore(deps): update dependency mairie360/cicd to v4.3.0 (#95) ([#95](https://github.com/mairie360/API_template/pull/95))

> Résumé Claude indisponible pour ce passage : listes brutes des commits.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-06 03:00 UTC | dev | — | `a06e22b` | [run](https://github.com/mairie360/API_template/actions/runs/42153078722) |
| 2026-11-05 18:22 UTC | dev | — | `1470ebd` | [run](https://github.com/mairie360/API_template/actions/runs/42081747176) |
| 2026-11-05 18:19 UTC | dev | — | `6990d87` | [run](https://github.com/mairie360/API_template/actions/runs/42081690123) |
| 2026-11-05 09:44 UTC | dev | — | `de33523` | [run](https://github.com/mairie360/API_template/actions/runs/42003910852) |
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
