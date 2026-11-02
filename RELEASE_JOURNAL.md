# Journal des releases — API_template

<!-- release-journal-state {"dev": "0d5dc99865b4a04bc4c894a2fc849a5f3f422c23", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-02 19:37 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `0d5dc99` | 2026-11-02 19:37 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, désactivé par défaut : il ne s'active que si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. `OTEL_SDK_DISABLED=true` le coupe et `OTEL_SERVICE_NAME` nomme le service. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Format des logs de l'API : texte coloré par défaut (niveau `RUST_LOG`, `info` par défaut), une ligne JSON par événement avec `LOG_FORMAT=json`. Chaque requête a désormais un span, y compris celles refusées par `JwtMiddleware` (401), qui continue un `traceparent` entrant. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- `mairie360_api_lib` passe en 4.3.0 (MAIR-680, [#130](https://github.com/mairie360/API_template/pull/130)). Le changelog annonce le remplacement du secret JWT HS256 partagé entre les APIs et BFFs. La PR ne fournit pas de diff : l'effet sur l'émission et la vérification des jetons reste à vérifier avant la prod.

#### Corrections
- Logs de requêtes et d'erreurs (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66)). Avant : `Logger::default()` d'actix journalisait la ligne de requête avec sa query string et le `Referer`, et les erreurs de base reprenaient le message Postgres et les valeurs. Maintenant : `request_logger()` ne journalise que l'adresse client, la méthode, le chemin sans query string, le statut, la taille et la durée. Les erreurs décrivent le type et le contexte (SQLSTATE, contrainte, table, colonne), sans la valeur reçue.

#### Technique
- Test de marqueur RGPD (`gdpr_marker_test.sh`, `docker-compose-gdpr-marker.yml`) qui cherche des valeurs de test dans les logs des conteneurs, et checklist RGPD sur les PR (MAIR-290 [#66](https://github.com/mairie360/API_template/pull/66), MAIR-295 [#70](https://github.com/mairie360/API_template/pull/70)). Tests ajoutés : `telemetry_test.rs`, `mair_741_every_image_runs_on_debian_13`.
- Image d'exécution des APIs passée en distroless Debian 13 (`cc-debian13`), builder et image de dev en `rust:1.99-*-trixie` (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126)).
- Alertes RustSec corrigées (`zstd`, `ctutils`, `rand`, `spin`) ; RUSTSEC-2026-0387 ignorée dans `.cargo/audit.toml` (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)). `mairie360_api_lib` passe en 4.2.1 (MAIR-740, [#122](https://github.com/mairie360/API_template/pull/122)).
- Pool Postgres paramétrable documenté dans `CLAUDE.md`, sans changement de code (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).
- Images Docker épinglées par digest, dont le `nginx` et le `node` 26 du marqueur RGPD (renovate, [#134](https://github.com/mairie360/API_template/pull/134), [#121](https://github.com/mairie360/API_template/pull/121)). Mises à jour de dépendances Rust : `actix-web` 4.18.1 ([#114](https://github.com/mairie360/API_template/pull/114)), `tokio` 1.54.1 ([#109](https://github.com/mairie360/API_template/pull/109)), `log` 0.4.35 en dev ([#120](https://github.com/mairie360/API_template/pull/120)).
- `mairie360/CICD` monte de v4.4.0 à v5.2.2 (workflows `cicd.yml` et `gdpr-pr.yml`, [#131](https://github.com/mairie360/API_template/pull/131), [#135](https://github.com/mairie360/API_template/pull/135)).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Export des traces OpenTelemetry en OTLP/HTTP, désactivé par défaut : il ne s'active que si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. `OTEL_SDK_DISABLED=true` le coupe et `OTEL_SERVICE_NAME` nomme le service. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Format des logs de l'API : texte coloré par défaut (niveau `RUST_LOG`, `info` par défaut), une ligne JSON par événement avec `LOG_FORMAT=json`. Chaque requête a désormais un span, y compris celles refusées par `JwtMiddleware` (401), qui continue un `traceparent` entrant. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- `mairie360_api_lib` passe en 4.3.0 (MAIR-680, [#130](https://github.com/mairie360/API_template/pull/130)). Le changelog annonce le remplacement du secret JWT HS256 partagé entre les APIs et BFFs. La PR ne fournit pas de diff : l'effet sur l'émission et la vérification des jetons reste à vérifier avant la prod.

#### Corrections
- Logs de requêtes et d'erreurs (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66)). Avant : `Logger::default()` d'actix journalisait la ligne de requête avec sa query string et le `Referer`, et les erreurs de base reprenaient le message Postgres et les valeurs. Maintenant : `request_logger()` ne journalise que l'adresse client, la méthode, le chemin sans query string, le statut, la taille et la durée. Les erreurs décrivent le type et le contexte (SQLSTATE, contrainte, table, colonne), sans la valeur reçue.

#### Technique
- Test de marqueur RGPD (`gdpr_marker_test.sh`, `docker-compose-gdpr-marker.yml`) qui cherche des valeurs de test dans les logs des conteneurs, et checklist RGPD sur les PR (MAIR-290 [#66](https://github.com/mairie360/API_template/pull/66), MAIR-295 [#70](https://github.com/mairie360/API_template/pull/70)). Tests ajoutés : `telemetry_test.rs`, `mair_741_every_image_runs_on_debian_13`.
- Image d'exécution des APIs passée en distroless Debian 13 (`cc-debian13`), builder et image de dev en `rust:1.99-*-trixie` (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126)).
- Alertes RustSec corrigées (`zstd`, `ctutils`, `rand`, `spin`) ; RUSTSEC-2026-0387 ignorée dans `.cargo/audit.toml` (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)). `mairie360_api_lib` passe en 4.2.1 (MAIR-740, [#122](https://github.com/mairie360/API_template/pull/122)).
- Pool Postgres paramétrable documenté dans `CLAUDE.md`, sans changement de code (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)).
- Images Docker épinglées par digest, dont le `nginx` et le `node` 26 du marqueur RGPD (renovate, [#134](https://github.com/mairie360/API_template/pull/134), [#121](https://github.com/mairie360/API_template/pull/121)). Mises à jour de dépendances Rust : `actix-web` 4.18.1 ([#114](https://github.com/mairie360/API_template/pull/114)), `tokio` 1.54.1 ([#109](https://github.com/mairie360/API_template/pull/109)), `log` 0.4.35 en dev ([#120](https://github.com/mairie360/API_template/pull/120)).
- `mairie360/CICD` monte de v4.4.0 à v5.2.2 (workflows `cicd.yml` et `gdpr-pr.yml`, [#131](https://github.com/mairie360/API_template/pull/131), [#135](https://github.com/mairie360/API_template/pull/135)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Mises à jour de dépendances : `utoipa-swagger-ui` 10.0.2 ([#99](https://github.com/mairie360/API_template/pull/99)) et `tokio` 1.54.0 ([#96](https://github.com/mairie360/API_template/pull/96)).
- Workflow CI : `mairie360/CICD` passe de v4.3.0 à v4.3.2 ([#95](https://github.com/mairie360/API_template/pull/95), [#97](https://github.com/mairie360/API_template/pull/97), [#98](https://github.com/mairie360/API_template/pull/98)), et `cicd.yml` transmet au workflow réutilisable les secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` pour le journal des releases (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
