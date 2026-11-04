# Journal des releases — API_template

<!-- release-journal-state {"dev": "c94246c284b53cbcdd3fd94cefe92eef5cf428de", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-04 01:40 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `c94246c` | 2026-11-04 01:40 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, activé seulement si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini ; une span racine par requête (route, statut), y compris pour les requêtes refusées par `JwtMiddleware` (401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Logs sur stdout : format texte coloré par défaut (`RUST_LOG`, défaut `info`), un objet JSON par ligne avec `LOG_FORMAT=json`. L'initialisation passe par `init_telemetry` de la lib au lieu d'un `tracing_subscriber::fmt()` local. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Journal des requêtes : `request_logger()` remplace `middleware::Logger::default()`. La ligne ne contient plus la query string ni le `Referer`, seulement adresse client, méthode, chemin, statut, taille et durée. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Erreurs de `mairie360_api_lib` 4.2.0 : les erreurs de base sont décrites par SQLSTATE, contrainte, table et colonne (sans message Postgres ni `DETAIL`), et les erreurs serde ne reprennent plus la valeur fautive. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Authentification (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139)) : les jetons de session sont vérifiés en EdDSA avec la clé publique de Core (`JWT_PUBLIC_KEY`, PEM). `JWT_SECRET` n'est plus nécessaire ; tant qu'il est défini, les jetons HS256 signés avec lui restent acceptés (transition). Sans `JWT_SECRET`, tout jeton HS256 reçoit 401. La lib refuse de démarrer sans l'une de ces variables, ou avec un secret court ou connu. Les appelants qui signent en HS256 doivent passer à EdDSA. La description OpenAPI du schéma `jwt` change aussi (`swagger.rs`). Le diff est tronqué après `docker-compose-security.yml` : ce détail vient de la description de PR et de CLAUDE.md.

#### Corrections
- Journaux : avant, la query string et le `Referer` des requêtes étaient écrits dans les logs par `Logger::default()` ; maintenant ils ne le sont plus. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : avant `gcr.io/distroless/cc-debian12:nonroot` (libssl3 non patchée, CVE-2026-84782) avec un builder bookworm ; maintenant `cc-debian13:nonroot` et builder `rust:1.99-slim-trixie`, dev compris. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- CI/CD : `mairie360/CICD` passe de v4.3.2 à v5.3.0 (workflows `cicd.yml` et `gdpr-pr.yml`), avec la checklist RGPD sur chaque PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)), un test de cohérence des versions (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133)) et la transmission de secrets supplémentaires au workflow (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).
- Stacks de test : clé et jetons générés à chaque run (`test_secrets.sh`), jeton admin committé retiré, test de marqueur RGPD ajouté (MAIR-290, MAIR-680).
- Dépendances : `mairie360_api_lib` 4.2.1 puis 4.3.0, `actix-web` 4.18.1, `tokio` 1.54.1, `log` 0.4.35, `utoipa-swagger-ui` 10.0.2, `cargo audit` corrigé et `RUSTSEC-2026-0387` ignoré (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)).
- Images Docker épinglées par digest (Renovate) : rust, postgres, redis, nginx, node (26 pour le conteneur gdpr-marker), curl, k6, newman, zap ; une dizaine de PR de mise à jour sans effet visible.
- Documentation : section CLAUDE.md sur les variables du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, activé seulement si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini ; une span racine par requête (route, statut), y compris pour les requêtes refusées par `JwtMiddleware` (401). (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Logs sur stdout : format texte coloré par défaut (`RUST_LOG`, défaut `info`), un objet JSON par ligne avec `LOG_FORMAT=json`. L'initialisation passe par `init_telemetry` de la lib au lieu d'un `tracing_subscriber::fmt()` local. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- Journal des requêtes : `request_logger()` remplace `middleware::Logger::default()`. La ligne ne contient plus la query string ni le `Referer`, seulement adresse client, méthode, chemin, statut, taille et durée. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Erreurs de `mairie360_api_lib` 4.2.0 : les erreurs de base sont décrites par SQLSTATE, contrainte, table et colonne (sans message Postgres ni `DETAIL`), et les erreurs serde ne reprennent plus la valeur fautive. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Authentification (MAIR-680, [#139](https://github.com/mairie360/API_template/pull/139)) : les jetons de session sont vérifiés en EdDSA avec la clé publique de Core (`JWT_PUBLIC_KEY`, PEM). `JWT_SECRET` n'est plus nécessaire ; tant qu'il est défini, les jetons HS256 signés avec lui restent acceptés (transition). Sans `JWT_SECRET`, tout jeton HS256 reçoit 401. La lib refuse de démarrer sans l'une de ces variables, ou avec un secret court ou connu. Les appelants qui signent en HS256 doivent passer à EdDSA. La description OpenAPI du schéma `jwt` change aussi (`swagger.rs`). Le diff est tronqué après `docker-compose-security.yml` : ce détail vient de la description de PR et de CLAUDE.md.

#### Corrections
- Journaux : avant, la query string et le `Referer` des requêtes étaient écrits dans les logs par `Logger::default()` ; maintenant ils ne le sont plus. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : avant `gcr.io/distroless/cc-debian12:nonroot` (libssl3 non patchée, CVE-2026-84782) avec un builder bookworm ; maintenant `cc-debian13:nonroot` et builder `rust:1.99-slim-trixie`, dev compris. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))

#### Technique
- CI/CD : `mairie360/CICD` passe de v4.4.0 à v5.3.0 dans cette plage (workflows `cicd.yml` et `gdpr-pr.yml`), avec la checklist RGPD sur chaque PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)) et un test de cohérence des versions (MAIR-771, [#133](https://github.com/mairie360/API_template/pull/133)).
- Stacks de test : clé et jetons générés à chaque run (`test_secrets.sh`), jeton admin committé retiré, test de marqueur RGPD ajouté (MAIR-290, MAIR-680).
- Dépendances : `mairie360_api_lib` 4.2.1 puis 4.3.0, `actix-web` 4.18.1, `tokio` 1.54.1, `log` 0.4.35, `utoipa-swagger-ui` 10.0.2, `cargo audit` corrigé et `RUSTSEC-2026-0387` ignoré (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)).
- Images Docker épinglées par digest (Renovate) : rust, postgres, redis, nginx, node (26 pour le conteneur gdpr-marker), curl, k6, newman, zap ; une vingtaine de PR de mise à jour sans effet visible.
- Documentation : section CLAUDE.md sur les variables du pool Postgres (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- CI/CD : `mairie360/CICD` passe de v4.3.0 à v4.3.2 (workflows `cicd.yml` et `gdpr-pr.yml`) ; la journalisation des releases (MAIR-571) reçoit ses secrets (`ANTHROPIC_API_KEY`, `RELEASE_APP_ID`, `RELEASE_APP_PK`) via [#94](https://github.com/mairie360/API_template/pull/94).
- Dépendances : `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)) et `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)), diffs vides dans le journal, sans effet visible.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
