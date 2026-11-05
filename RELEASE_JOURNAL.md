# Journal des releases — API_template

<!-- release-journal-state {"dev": "de335234ed9b89a4da30aa6c0911e3a9747a3ec3", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-05 09:44 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `de33523` | 2026-11-05 09:44 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Export OTLP des traces, désactivé par défaut : il ne s'active que si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. `OTEL_SDK_DISABLED=true` le coupe, et un exporteur en échec est journalisé sans arrêter l'API. Chaque requête, y compris celles refusées par le JWT, produit un span portant sa route et son statut. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Format des journaux : texte coloré par défaut (niveau fixé par `RUST_LOG`, `info` par défaut), une ligne JSON par événement avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les jetons de session ne sont plus vérifiés avec `JWT_SECRET` seul. L'API lit la clé publique de Core_API dans `JWT_PUBLIC_KEY` et refuse de démarrer sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`. Un jeton HS256 est refusé (401) dès que `JWT_SECRET` n'est pas défini. Tant qu'il est défini, les jetons HS256 signés avec lui restent acceptés (transition). Les jetons attendus sont EdDSA, émis par Core (`iss`, `aud`). Les appelants qui signent encore en HS256 (scripts, Postman, ZAP, k6) doivent passer à la nouvelle clé. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- La description du schéma de sécurité `jwt` de la spec OpenAPI (`/api-docs/openapi.json`) est modifiée pour refléter EdDSA et Core_API. Le diff de `swagger.rs` étant tronqué, le détail exact n'est pas vérifié. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- La stack de développement (`docker-compose.yml`) utilise par défaut la clé publique de test de la lib, avec `JWT_ALLOW_WEAK_SECRET=true`. Ce réglage ne doit jamais aller en déploiement. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))

#### Corrections
- Les journaux de requêtes ne contiennent plus la query string ni le `Referer`. Avant : actix journalisait la ligne de requête complète, query string comprise, donc des valeurs de recherche pouvant être des données personnelles. Maintenant : adresse client, méthode, chemin sans query, statut, taille et durée. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Les erreurs de la lib ne recopient plus les valeurs reçues. Avant : message Postgres et `DETAIL` complets, messages serde citant la valeur. Maintenant : erreurs base décrites par SQLSTATE, contrainte, table et colonne, messages serde sans la valeur. Les messages renvoyés aux clients peuvent donc changer. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : `gcr.io/distroless/cc-debian12` remplacée par `cc-debian13`, parce que libssl3 de Debian 12 n'est pas corrigée (CVE-2026-84782). (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Dépendances Rust : `mairie360_api_lib` passe en 4.2.1 puis 4.3.0 (la version 4.3.0 porte MAIR-680) ; corrections des alertes RustSec (zstd, ctutils, rand, spin, ignore de RUSTSEC-2026-0387) ; `actix-web` 4.18.0 et 4.18.1, `tokio` 1.54.1, `log` 0.4.35 (dev). Les diffs de ces PR sont absents ou vides. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125) ; [#130](https://github.com/mairie360/API_template/pull/130), [#122](https://github.com/mairie360/API_template/pull/122), [#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114), [#109](https://github.com/mairie360/API_template/pull/109), [#120](https://github.com/mairie360/API_template/pull/120))
- Tracing : `tracing-subscriber` ne reste qu'en dev-dependency ; tests `telemetry_test.rs` et `dockerfile_test.rs` ajoutés ; `log` pour les tests de logs. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79) ; MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Images Docker épinglées par digest (`rust` slim et final en bookworm puis trixie, `postgres`, `redis`, `nginx`, `curlimages/curl`, `grafana/k6`, `postman/newman`, `zaproxy`), `node` passé en 26 pour le runner de marqueur RGPD. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126) ; [#121](https://github.com/mairie360/API_template/pull/121) ; [#134](https://github.com/mairie360/API_template/pull/134) ; Renovate [#102](https://github.com/mairie360/API_template/pull/102) à [#143](https://github.com/mairie360/API_template/pull/143))
- `mairie360/CICD` monté de v4.4.0 à v5.5.0 dans les workflows (`cicd.yml`, `gdpr-pr.yml`) ; tests de garde de version MAIR-771 ; checklist RGPD des PR (`gdpr-pr.yml`, MAIR-295). (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70) ; MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133) ; Renovate [#100](https://github.com/mairie360/API_template/pull/100) à [#145](https://github.com/mairie360/API_template/pull/145))
- Stacks de tests : marqueur RGPD (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66)) et simulation d'usage RGPD avant prod (MAIR-497, [#137](https://github.com/mairie360/API_template/pull/137)), avec scripts et règles `.gitignore` / `.dockerignore`. Aucun effet sur le service. Les tests d'intégration, ZAP et k6 signent leurs jetons avec une paire de clés générée à chaque exécution (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139)).
- Documentation : réglages du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)), CLAUDE.md et README pour l'authentification et les logs.

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Export OTLP des traces, désactivé par défaut : il ne s'active que si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. `OTEL_SDK_DISABLED=true` le coupe, et un exporteur en échec est journalisé sans arrêter l'API. Chaque requête, y compris celles refusées par le JWT, produit un span portant sa route et son statut. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Format des journaux : texte coloré par défaut (niveau fixé par `RUST_LOG`, `info` par défaut), une ligne JSON par événement avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les jetons de session ne sont plus vérifiés avec `JWT_SECRET` seul. L'API lit la clé publique de Core_API dans `JWT_PUBLIC_KEY` et refuse de démarrer sans `JWT_PUBLIC_KEY` ni `JWT_SECRET`. Un jeton HS256 est refusé (401) dès que `JWT_SECRET` n'est pas défini. Tant qu'il est défini, les jetons HS256 signés avec lui restent acceptés (transition). Les jetons attendus sont EdDSA, émis par Core (`iss`, `aud`). Les appelants qui signent encore en HS256 (scripts, Postman, ZAP, k6) doivent passer à la nouvelle clé. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- La description du schéma de sécurité `jwt` de la spec OpenAPI (`/api-docs/openapi.json`) est modifiée pour refléter EdDSA et Core_API. Le diff de `swagger.rs` étant tronqué, le détail exact n'est pas vérifié. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- La stack de développement (`docker-compose.yml`) utilise par défaut la clé publique de test de la lib, avec `JWT_ALLOW_WEAK_SECRET=true`. Ce réglage ne doit jamais aller en déploiement. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))

#### Corrections
- Les journaux de requêtes ne contiennent plus la query string ni le `Referer`. Avant : actix journalisait la ligne de requête complète, query string comprise, donc des valeurs de recherche pouvant être des données personnelles. Maintenant : adresse client, méthode, chemin sans query, statut, taille et durée. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Les erreurs de la lib ne recopient plus les valeurs reçues. Avant : message Postgres et `DETAIL` complets, messages serde citant la valeur. Maintenant : erreurs base décrites par SQLSTATE, contrainte, table et colonne, messages serde sans la valeur. Les messages renvoyés aux clients peuvent donc changer. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : `gcr.io/distroless/cc-debian12` remplacée par `cc-debian13`, parce que libssl3 de Debian 12 n'est pas corrigée (CVE-2026-84782). (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- Dépendances Rust : `mairie360_api_lib` passe en 4.2.1 puis 4.3.0 (la version 4.3.0 porte MAIR-680) ; corrections des alertes RustSec (zstd, ctutils, rand, spin, ignore de RUSTSEC-2026-0387) ; `actix-web` 4.18.0 et 4.18.1, `tokio` 1.54.1, `log` 0.4.35 (dev). Les diffs de ces PR sont absents ou vides. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125) ; [#130](https://github.com/mairie360/API_template/pull/130), [#122](https://github.com/mairie360/API_template/pull/122), [#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114), [#109](https://github.com/mairie360/API_template/pull/109), [#120](https://github.com/mairie360/API_template/pull/120))
- Tracing : `tracing-subscriber` ne reste qu'en dev-dependency ; tests `telemetry_test.rs` et `dockerfile_test.rs` ajoutés. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79) ; MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Images Docker épinglées par digest (`rust` slim et final en bookworm puis trixie, `postgres`, `redis`, `nginx`, `curlimages/curl`, `grafana/k6`, `postman/newman`, `zaproxy`), `node` passé en 26 pour le runner de marqueur RGPD. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126) ; [#121](https://github.com/mairie360/API_template/pull/121) ; [#134](https://github.com/mairie360/API_template/pull/134) ; Renovate [#102](https://github.com/mairie360/API_template/pull/102) à [#143](https://github.com/mairie360/API_template/pull/143))
- `mairie360/CICD` monté de v4.4.0 à v5.5.0 dans les workflows (`cicd.yml`, `gdpr-pr.yml`) ; tests de garde de version MAIR-771 ; checklist RGPD des PR (`gdpr-pr.yml`, MAIR-295). (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70) ; MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133) ; Renovate [#100](https://github.com/mairie360/API_template/pull/100) à [#145](https://github.com/mairie360/API_template/pull/145))
- Stacks de tests : marqueur RGPD (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66)) et simulation d'usage RGPD avant prod (MAIR-497, [#137](https://github.com/mairie360/API_template/pull/137)), avec scripts et règles `.gitignore` / `.dockerignore`. Aucun effet sur le service. Les tests d'intégration, ZAP et k6 signent leurs jetons avec une paire de clés générée à chaque exécution (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139)).
- Documentation : réglages du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)), CLAUDE.md et README pour l'authentification et les logs.

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Workflows CI : `mairie360/CICD` passé de v4.3.0 à v4.3.2 dans `cicd.yml` et `gdpr-pr.yml`. (Renovate [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98))
- Le caller `cicd.yml` transmet au workflow réutilisable les secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal de release (MAIR-571). Aucun effet sur le service. ([#94](https://github.com/mairie360/API_template/pull/94))
- Dépendances Rust sans diff visible : `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
