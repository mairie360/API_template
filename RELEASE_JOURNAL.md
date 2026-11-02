# Journal des releases — API_template

<!-- release-journal-state {"dev": "f21d81f6a79a8391082f0a7c84f7fbebc7378c0d", "prod": "9e6fe42fd00d01d51e2bee1aa846c3d839255f60", "prod_tag": "v1.1.0", "staging": "84055a3fb610b31a09c7c5ba515793bc022a1e74", "updated": {"dev": "2026-11-02 19:35 UTC", "staging": "2026-10-20 18:52 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `9e6fe42` | — |
| staging | — | `84055a3` | 2026-10-20 18:52 UTC |
| dev | — | `f21d81f` | 2026-11-02 19:35 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Nouveautés
- Les traces OpenTelemetry sont exportées en OTLP/HTTP, uniquement si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. Chaque requête produit un span racine nommé `<METHOD> <route>` qui continue un `traceparent` entrant, sans query string ni adresse client. Les requêtes refusées par `JwtMiddleware` ont aussi leur span. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les logs passent par `init_telemetry` : texte coloré sur stdout par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- `mairie360_api_lib` passe de 4.2.1 à 4.3.0 (#130). La release note de la lib citée dans la PR décrit le remplacement du secret JWT HS256 partagé (MAIR-680). Le diff de cette PR ne contient que la version : à vérifier côté appelants (BFF, autres APIs). ([#130](https://github.com/mairie360/API_template/pull/130))

#### Corrections
- Les logs de requêtes contenaient la query string et le `Referer` (actix `Logger::default()`). Maintenant ils ne contiennent que l'adresse client, la méthode, le chemin sans query string, le statut, la taille et la durée. Les erreurs de la lib ne reprennent plus le message Postgres ni la valeur reçue, mais le SQLSTATE, la contrainte, la table et la colonne. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Technique
- Images Docker épinglées par digest (Dockerfile, development.Dockerfile, docker-compose*.yml) : rust, postgres, redis, nginx, node 26, curl, k6, newman, ZAP. Passage de l'image d'exécution en distroless Debian 13 et du builder en trixie, avec le test `mair_741_every_image_runs_on_debian_13` (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126)).
- Audit des dépendances Rust corrigé : zstd, ctutils, rand, spin mis à jour, RUSTSEC-2026-0387 ignoré avec motif documenté (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)).
- Mises à jour de dépendances : actix-web 4.18.0 et 4.18.1, tokio 1.54.1, log 0.4.35 en dev, mairie360_api_lib 4.2.1 (#113, #114, #109, #120, #122).
- Workflows CI : mairie360/CICD passe de v4.5.0 à v5.2.0 en plusieurs étapes (#110 à #131). Ajout de la checklist RGPD sur les PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)) et du test de marqueur RGPD sur les logs (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66)).
- Documentation des variables du pool Postgres dans CLAUDE.md (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)). Ajout de `tests/telemetry_test.rs` (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79)).

## Différence dev → staging (en dev, pas encore en staging)

#### Nouveautés
- Les traces OpenTelemetry sont exportées en OTLP/HTTP, uniquement si `OTEL_EXPORTER_OTLP_ENDPOINT` (ou `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`) est défini. Chaque requête produit un span racine nommé `<METHOD> <route>` qui continue un `traceparent` entrant, sans query string ni adresse client. Les requêtes refusées par `JwtMiddleware` ont aussi leur span. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))

#### Changements de comportement
- Les logs passent par `init_telemetry` : texte coloré sur stdout par défaut, un objet JSON par ligne avec `LOG_FORMAT=json`. (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79))
- `mairie360_api_lib` passe de 4.2.1 à 4.3.0. La release note de la lib citée dans la PR décrit le remplacement du secret JWT HS256 partagé (MAIR-680). Le diff de cette PR ne contient que la version : à vérifier côté appelants (BFF, autres APIs). ([#130](https://github.com/mairie360/API_template/pull/130))

#### Corrections
- Les logs de requêtes contenaient la query string et le `Referer` (actix `Logger::default()`). Maintenant ils ne contiennent que l'adresse client, la méthode, le chemin sans query string, le statut, la taille et la durée. Les erreurs de la lib ne reprennent plus le message Postgres ni la valeur reçue, mais le SQLSTATE, la contrainte, la table et la colonne. (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66))

#### Technique
- Images Docker épinglées par digest (Dockerfile, development.Dockerfile, docker-compose*.yml). Passage de l'image d'exécution en distroless Debian 13 et du builder en trixie, avec le test `mair_741_every_image_runs_on_debian_13` (MAIR-741, [#126](https://github.com/mairie360/API_template/pull/126)).
- Audit des dépendances Rust corrigé : zstd, ctutils, rand, spin mis à jour, RUSTSEC-2026-0387 ignoré avec motif documenté (MAIR-740, [#125](https://github.com/mairie360/API_template/pull/125)).
- Mises à jour de dépendances : actix-web 4.18.0 et 4.18.1, tokio 1.54.1, log 0.4.35 en dev, mairie360_api_lib 4.2.1 (#113, #114, #109, #120, #122).
- Workflows CI : mairie360/CICD passe de v4.4.0 à v5.2.0 en plusieurs étapes (#100 à #131). Ajout de la checklist RGPD sur les PR (MAIR-295, [#70](https://github.com/mairie360/API_template/pull/70)) et du test de marqueur RGPD sur les logs (MAIR-290, [#66](https://github.com/mairie360/API_template/pull/66)).
- Documentation des variables du pool Postgres dans CLAUDE.md (MAIR-565, [#82](https://github.com/mairie360/API_template/pull/82)). Ajout de `tests/telemetry_test.rs` (MAIR-503, [#79](https://github.com/mairie360/API_template/pull/79)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Mises à jour de dépendances sans effet visible : utoipa-swagger-ui 10.0.2 (#99) et tokio 1.54.0 (#96).
- Montées de mairie360/CICD de v4.3.0 à v4.3.2 (#95, #97, #98, #105 non concerné ici) et transmission des secrets `ANTHROPIC_API_KEY`, `RELEASE_APP_ID` et `RELEASE_APP_PK` au workflow CICD pour le journal des releases (MAIR-571, [#94](https://github.com/mairie360/API_template/pull/94)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
