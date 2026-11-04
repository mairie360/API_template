# Journal des releases — API_template

<!-- release-journal-state {"dev": "c94246c284b53cbcdd3fd94cefe92eef5cf428de", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-04 01:39 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `c94246c` | 2026-11-04 01:39 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les traces OpenTelemetry sont exportées en OTLP/HTTP seulement si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. Chaque requête produit un span racine `<METHOD> <route>` qui continue un `traceparent` entrant. `LOG_FORMAT=json` passe les logs en JSON, une ligne par événement. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les JWT HS256 ne sont plus la voie d'authentification : l'API lit `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, PEM) et vérifie des tokens EdDSA. Sans `JWT_SECRET`, un token HS256 reçoit un 401. Tant que `JWT_SECRET` est défini, les HS256 signés avec lui restent acceptés (transition). La lib refuse de démarrer sans l'une des deux variables. Un appelant qui signe encore en HS256 (BFF, scripts) doit passer à EdDSA. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Les logs de requête n'affichent plus la query string ni le `Referer` : ils gardent l'adresse client, la méthode, le chemin, le statut, la taille et la durée. Les erreurs de la lib ne reprennent plus le message Postgres, le `DETAIL` ni la valeur reçue (SQLSTATE, contrainte, table, colonne à la place). (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution : avant, le runtime était `gcr.io/distroless/cc-debian12:nonroot` avec une `libssl3` non corrigée (CVE-2026-84782) et le builder était `rust:1.99-slim-bookworm`. Maintenant le runtime est `cc-debian13:nonroot` et le builder `rust:1.99-slim-trixie`, sur la même release Debian. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- CICD passé de v4.5.0 à v5.2.2 dans les workflows, avec un test qui vérifie que tous les workflows appellent la même release, au minimum v4.12.0 (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133)), puis mise à jour de CICD en v5.3.0 ([#141](https://github.com/mairie360/API_template/pull/141)).
- Ajout de la checklist RGPD sur les PR, images Docker épinglées par digest, banc de test du marqueur RGPD (`gdpr_marker_test.sh`), Node 26 pour le marqueur, dépendances mises à jour (`mairie360_api_lib` 4.3.0, `actix-web`, `tokio`, `log`, `utoipa-swagger-ui`), corrections cargo audit (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)), documentation du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les traces OpenTelemetry sont exportées en OTLP/HTTP seulement si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. Chaque requête produit un span racine `<METHOD> <route>` qui continue un `traceparent` entrant. `LOG_FORMAT=json` passe les logs en JSON, une ligne par événement. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les JWT HS256 ne sont plus la voie d'authentification : l'API lit `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, PEM) et vérifie des tokens EdDSA. Sans `JWT_SECRET`, un token HS256 reçoit un 401. Tant que `JWT_SECRET` est défini, les HS256 signés avec lui restent acceptés (transition). La lib refuse de démarrer sans l'une des deux variables. Un appelant qui signe encore en HS256 (BFF, scripts) doit passer à EdDSA. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Les logs de requête n'affichent plus la query string ni le `Referer`. Les erreurs de la lib ne reprennent plus le message Postgres, le `DETAIL` ni la valeur reçue. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Image d'exécution : le runtime passe de `gcr.io/distroless/cc-debian12:nonroot` (libssl3 non corrigée, CVE-2026-84782) à `cc-debian13:nonroot`, et le builder de `rust:1.99-slim-bookworm` à `rust:1.99-slim-trixie`. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- CICD passé de v4.5.0 à v5.3.0, avec un test qui vérifie que tous les workflows appellent la même release (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133), [#141](https://github.com/mairie360/API_template/pull/141)). Ajout de la checklist RGPD sur les PR, images épinglées par digest, marqueur RGPD, Node 26 pour le marqueur, dépendances mises à jour, corrections cargo audit (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)), documentation du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Mise à jour de la release CICD vers v5.3.0 et de `mairie360/CICD` vers v4.3.x, et transmission des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow CICD (#4258372, [#94](https://github.com/mairie360/API_template/pull/94)).
- Mises à jour de dépendances sans changement de code : `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
