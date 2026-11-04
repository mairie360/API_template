# Journal des releases — API_template

<!-- release-journal-state {"dev": "246f16ab188e8a9144fbc93d957648761a131c1d", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-04 15:29 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `246f16a` | 2026-11-04 15:29 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement

- L'API ne vérifie plus les jetons de session avec `JWT_SECRET` seul : elle lit `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, au format PEM). Tant que `JWT_SECRET` est défini, les jetons HS256 signés avec ce secret sont encore acceptés (transition). Sans `JWT_SECRET`, un jeton HS256 reçoit un `401`, et la lib refuse de démarrer si aucune des deux variables n'est correcte. Les appelants (BFF, front) doivent présenter des jetons EdDSA signés par Core_API. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139), dépendance `mairie360_api_lib` 4.3.0 : [#130](https://github.com/mairie360/API_template/pull/130)). Le diff de #139 est tronqué : le comportement décrit ici vient des fichiers visibles.
- Les stacks de test (intégration, ZAP, k6, GDPR marker) exigent désormais `JWT_PUBLIC_KEY` et échouent sans lui. Le secret HS256 et le jeton admin commité ont été retirés des fichiers de test. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Les journaux de requêtes n'enregistrent plus la query string ni le `Referer` : seuls l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée sont logués. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Nouveautés

- Les logs peuvent être émis en JSON (une ligne par événement) avec `LOG_FORMAT=json`. L'export de traces OpenTelemetry est opt-in via `OTEL_EXPORTER_OTLP_ENDPOINT` ; chaque requête, y compris celles refusées par le JWT (`401`), produit une span. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique

- Image d'exécution distroless passée sur Debian 13 (`cc-debian13`), builder et image de dev alignés sur trixie, avec un test qui vérifie la version de Debian des images. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Mises à jour de dépendances : crates (`tokio`, `actix-web`, `log`, `utoipa-swagger-ui`), images Docker épinglées par digest (Rust, Postgres, Redis, nginx, curl, k6, newman, ZAP), Node 24 vers 26 pour le marker GDPR. ([#109](https://github.com/mairie360/API_template/pull/109), [#114](https://github.com/mairie360/API_template/pull/114), [#120](https://github.com/mairie360/API_template/pull/120), [#121](https://github.com/mairie360/API_template/pull/121), [#134](https://github.com/mairie360/API_template/pull/134), [#140](https://github.com/mairie360/API_template/pull/140), [#142](https://github.com/mairie360/API_template/pull/142), [#143](https://github.com/mairie360/API_template/pull/143), [#138](https://github.com/mairie360/API_template/pull/138), [#118](https://github.com/mairie360/API_template/pull/118), [#117](https://github.com/mairie360/API_template/pull/117), [#116](https://github.com/mairie360/API_template/pull/116), [#115](https://github.com/mairie360/API_template/pull/115), [#112](https://github.com/mairie360/API_template/pull/112), [#111](https://github.com/mairie360/API_template/pull/111), [#108](https://github.com/mairie360/API_template/pull/108), [#107](https://github.com/mairie360/API_template/pull/107), [#104](https://github.com/mairie360/API_template/pull/104), [#103](https://github.com/mairie360/API_template/pull/103), [#102](https://github.com/mairie360/API_template/pull/102)).
- `mairie360/CICD` monté de v4.5.0 à v5.2.2 (workflows réutilisables, GDPR, RGAA), avec un test qui vérifie que tous les workflows appellent la même version. ([#119](https://github.com/mairie360/API_template/pull/119), [#128](https://github.com/mairie360/API_template/pull/128), [#129](https://github.com/mairie360/API_template/pull/129), [#131](https://github.com/mairie360/API_template/pull/131), [#133](https://github.com/mairie360/API_template/pull/133), [#135](https://github.com/mairie360/API_template/pull/135), [#127](https://github.com/mairie360/API_template/pull/127), [#110](https://github.com/mairie360/API_template/pull/110), [#100](https://github.com/mairie360/API_template/pull/100), [#101](https://github.com/mairie360/API_template/pull/101), [#105](https://github.com/mairie360/API_template/pull/105), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98), [#95](https://github.com/mairie360/API_template/pull/95))
- Nouvelle check « GDPR checklist » sur les pull requests (workflow `gdpr-pr.yml`), et script de test du marqueur GDPR dans les logs. (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70) ; MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Corrections d'alertes RustSec (`zstd`, `ctutils`, `rand`, `spin`) et ignore documenté de `RUSTSEC-2026-0387`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))
- Documentation du pool Postgres paramétrable (variables `DB_POOL_*`). (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement

- L'API ne vérifie plus les jetons de session avec `JWT_SECRET` seul : elle lit `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, au format PEM). Tant que `JWT_SECRET` est défini, les jetons HS256 signés avec ce secret sont encore acceptés (transition). Sans `JWT_SECRET`, un jeton HS256 reçoit un `401`, et la lib refuse de démarrer si aucune des deux variables n'est correcte. Les appelants (BFF, front) doivent présenter des jetons EdDSA signés par Core_API. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139), dépendance `mairie360_api_lib` 4.3.0 : [#130](https://github.com/mairie360/API_template/pull/130)). Le diff de #139 est tronqué : le comportement décrit ici vient des fichiers visibles.
- Les stacks de test (intégration, ZAP, k6, GDPR marker) exigent désormais `JWT_PUBLIC_KEY` et échouent sans lui. Le secret HS256 et le jeton admin commité ont été retirés des fichiers de test. (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139))
- Les journaux de requêtes n'enregistrent plus la query string ni le `Referer` : seuls l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée sont logués. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Nouveautés

- Les logs peuvent être émis en JSON (une ligne par événement) avec `LOG_FORMAT=json`. L'export de traces OpenTelemetry est opt-in via `OTEL_EXPORTER_OTLP_ENDPOINT` ; chaque requête, y compris celles refusées par le JWT (`401`), produit une span. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Technique

- Image d'exécution distroless passée sur Debian 13, builder et image de dev alignés sur trixie. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Mises à jour de dépendances et d'images Docker épinglées par digest (Rust, Postgres, Redis, nginx, curl, k6, newman, ZAP), Node 24 vers 26 pour le marqueur GDPR, crates `tokio`, `actix-web`, `log`. ([#102](https://github.com/mairie360/API_template/pull/102) à [#143](https://github.com/mairie360/API_template/pull/143), [#109](https://github.com/mairie360/API_template/pull/109), [#114](https://github.com/mairie360/API_template/pull/114), [#120](https://github.com/mairie360/API_template/pull/120), [#121](https://github.com/mairie360/API_template/pull/121))
- `mairie360/CICD` monté de v4.5.0 à v4.12.x puis v5.2.2 (workflows réutilisables), avec un test qui vérifie que tous les workflows appellent la même version. ([#100](https://github.com/mairie360/API_template/pull/100) à [#135](https://github.com/mairie360/API_template/pull/135))
- Check « GDPR checklist » sur les pull requests et script de test du marqueur GDPR dans les logs. (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70) ; MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Corrections d'alertes RustSec (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)) et documentation du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique

- Passage de `mairie360/CICD` en v5.3.0, v5.3.1 puis v4.3.0, v4.3.1, v4.3.2 (mises à jour de workflows réutilisables, gestion du journal de release). ([#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98))
- Le workflow `cicd.yml` transmet désormais les secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow réutilisable, pour le journal de release. Aucun effet sur le comportement de l'API. ([#94](https://github.com/mairie360/API_template/pull/94))
- Mises à jour sans diff visible dans ce dépôt : `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
