# Journal des releases — API_template

<!-- release-journal-state {"dev": "7071f055d603cbc6e8932bfd022894d8dcd667aa", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-03 00:08 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `7071f05` | 2026-11-03 00:08 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Chaque requête produit un span OpenTelemetry (méthode et route), y compris les requêtes refusées par le JWT (401), qui poursuit le `traceparent` entrant. L'export OTLP/HTTP n'est actif que si `OTEL_EXPORTER_OTLP_ENDPOINT` est défini, et `LOG_FORMAT=json` passe les logs en JSON. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Le log de requête ne contient plus la query string : le middleware `Logger::default()` est remplacé par `request_logger()`, qui logue l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée (d'après la description de la PR, le diff tronqué ne permet pas de le vérifier ligne à ligne). Un appelant qui lisait ces champs dans les logs doit s'adapter. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Les erreurs de `mairie360_api_lib` 4.2.0 ne reprennent plus les valeurs reçues : les erreurs de base sont décrites par SQLSTATE, contrainte, table et colonne au lieu du message Postgres. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Données personnelles dans les logs : avant, la query string des requêtes et les valeurs citées dans les erreurs pouvaient apparaître dans les logs ; maintenant elles n'y figurent plus, ce que vérifie le test du marqueur RGPD. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : avant, runtime `gcr.io/distroless/cc-debian12` avec `libssl3` non corrigée et builder `rust:1.99-slim-bookworm` ; maintenant runtime `cc-debian13:nonroot` et builder `rust:1.99-slim-trixie`, pour corriger CVE-2026-84782. (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Alertes RustSec : avant, `zstd`, `ctutils`, `rand` et `spin` en versions vulnérables ; maintenant versions corrigées, et RUSTSEC-2026-0387 (crypto-bigint) ignoré avec sa justification dans `.cargo/audit.toml`. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))

#### Technique
- CI : `cicd.yml` et `gdpr-pr.yml` appellent mairie360/CICD v5.2.2 (passage depuis v4.12.x puis v5.2.0) ; nouveau check « GDPR checklist » sur les PR ; test `tests/mair_771_cicd_version.rs`. (MAIR-771 / MAIR-295, [#133](https://github.com/mairie360/API_template/pull/133), [#70](https://github.com/mairie360/API_template/pull/70))
- Images Docker épinglées par digest (mises à jour Renovate de rust, nginx, node, postgres, redis, curl, k6, newman, zap) et Node 26 pour le marqueur RGPD. ([#121](https://github.com/mairie360/API_template/pull/121), [#134](https://github.com/mairie360/API_template/pull/134), [#138](https://github.com/mairie360/API_template/pull/138))
- Dépendances : `actix-web` 4.18.0 et 4.18.1, `tokio` 1.54.1, `log` 0.4.35 en dev, `mairie360_api_lib` 4.3.0 (le diff de cette dernière PR est vide dans le dépôt, à vérifier). ([#113](https://github.com/mairie360/API_template/pull/113), [#114](https://github.com/mairie360/API_template/pull/114), [#109](https://github.com/mairie360/API_template/pull/109), [#120](https://github.com/mairie360/API_template/pull/120), [#130](https://github.com/mairie360/API_template/pull/130))
- Tests et documentation : `tests/telemetry_test.rs`, `mair_741_every_image_runs_on_debian_13`, marqueur RGPD (`gdpr-marker.yaml`, `gdpr_marker_test.sh`, `docker-compose-gdpr-marker.yml`), section `CLAUDE.md` sur le pool Postgres `DB_POOL_*` (documentation seule). (MAIR-290, MAIR-503, MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82))

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Chaque requête produit un span OpenTelemetry (méthode et route), y compris les requêtes refusées par le JWT (401), qui poursuit le `traceparent` entrant. L'export OTLP/HTTP n'est actif que si `OTEL_EXPORTER_OTLP_ENDPOINT` est défini, et `LOG_FORMAT=json` passe les logs en JSON. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Le log de requête ne contient plus la query string : `request_logger()` remplace `Logger::default()` (adresse client, méthode, chemin sans query, statut, taille, durée, selon la description de la PR). Un appelant qui lisait ces champs dans les logs doit s'adapter. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Les erreurs de `mairie360_api_lib` 4.2.0 ne reprennent plus les valeurs reçues (SQLSTATE, contrainte, table et colonne au lieu du message Postgres). (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Corrections
- Données personnelles dans les logs : avant, la query string et les valeurs citées dans les erreurs pouvaient apparaître dans les logs ; maintenant elles n'y figurent plus. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))
- Image d'exécution : avant, runtime distroless Debian 12 avec `libssl3` non corrigée ; maintenant `cc-debian13:nonroot` et builder `rust:1.99-slim-trixie` (CVE-2026-84782). (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126))
- Alertes RustSec : avant, `zstd`, `ctutils`, `rand` et `spin` en versions vulnérables ; maintenant versions corrigées et RUSTSEC-2026-0387 ignoré avec justification. (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125))

#### Technique
- CI : `cicd.yml` et `gdpr-pr.yml` appellent mairie360/CICD v5.2.2 ; nouveau check « GDPR checklist » sur les PR ; test `tests/mair_771_cicd_version.rs`. (MAIR-771, MAIR-295, [#133](https://github.com/mairie360/API_template/pull/133), [#70](https://github.com/mairie360/API_template/pull/70))
- Images Docker épinglées par digest (mises à jour Renovate) et Node 26 pour le marqueur RGPD. ([#121](https://github.com/mairie360/API_template/pull/121), [#134](https://github.com/mairie360/API_template/pull/134))
- Dépendances : `actix-web` 4.18.x, `tokio` 1.54.1, `log` 0.4.35 en dev, `mairie360_api_lib` 4.3.0 (diff vide dans le dépôt). ([#130](https://github.com/mairie360/API_template/pull/130), [#120](https://github.com/mairie360/API_template/pull/120))
- Tests et documentation : `tests/telemetry_test.rs`, test Debian 13 des images, marqueur RGPD, section `CLAUDE.md` sur le pool Postgres (documentation seule). ([#82](https://github.com/mairie360/API_template/pull/82), [#66](https://github.com/mairie360/API_template/pull/66))

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Montée de la référence mairie360/CICD dans `cicd.yml` vers v4.3.0, v4.3.1 puis v4.3.2 (journal des releases rédigé par Claude, côté CICD, sans effet visible sur l'API). (MAIR-571, [#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98))
- `cicd.yml` transmet désormais `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow réutilisable. (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94))
- Mises à jour `tokio` 1.54.0 et `utoipa-swagger-ui` 10.0.2 (diff vide dans le dépôt). ([#96](https://github.com/mairie360/API_template/pull/96), [#99](https://github.com/mairie360/API_template/pull/99))

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
