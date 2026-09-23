# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`API_template` is the scaffold every Mairie 360 Rust API (`Core_API`, `Project_API`, `Calendar_API`,
`Message_API`, `ELearning_API`) was generated from. Keep it in sync with those siblings: when a
shared pattern changes there (lib version, `main.rs` bootstrap, Docker/compose files, CI
workflow, Renovate config), port it here. The crate is `api_template`; every spot a new API must
rename carries a `change api name` / `change port` marker, on the same line (or on the line just
above in the `Dockerfile`s, which forbid trailing comments). Any API name, port or URL added to the
template must get one. The template ships no business endpoint on purpose (`v1` is an empty scope).

## Commands

Same aliases as the siblings (`.cargo/config.toml`): `cargo lint_check`, `cargo lint_fix`,
`cargo check_code` (clippy `-D warnings`), `cargo test` (Docker + GHCR pull access, testcontainers
via `mairie360_api_lib::test_setup::queries_setup::get_shared_db`), `cargo cov_test` (60 % line
gate, `endpoints/`, `main.rs`, `lib.rs` excluded), `cargo open_api` (OpenAPI JSON on stdout).

End-to-end harnesses (what CI runs on `main` after the dev release; each spins up its own stack from a
standalone compose file, so env/image changes must be mirrored in all of them): `./integration_test.sh`
(`docker-compose-integration.yml`, newman replaying `tests/postman/collection.json` with
`tests/postman/environment.json`), `./security_test.sh` (ZAP), `./performance_test.sh` (k6). The
collection is a Postman v2.1 export; its pre-request script forges HS256 JWTs with the stack's
`JWT_SECRET` (`jwt_admin` for the seeded Admin, `jwt_agent` for user 2 from `init-test.sql`, plus a
wrong-secret and an expired token), so a new API only adds requests for its endpoints. `baseUrl` is
overridden with `--env-var` by the compose file; the committed default targets `localhost:3000`.

The ZAP scan is authenticated and blocking: `security-scan` waits for the `seeder`, injects a static admin JWT
(`sub=1`, signed with `JWT_SECRET=b"secret"`, see the comment in `docker-compose-security.yml`) on every request
and fails on any alert not set to `IGNORE` / `OUTOFSCOPE` in `.zap/rules.tsv` (no `-I`; the file is the same in
every API). `-O http://<service>:<port>` is required, the spec's `servers` being unreachable from the ZAP
container. ZAP fuzzes every field from the spec examples, so an example that does not deserialize, a `500`
(value too long for its column, NUL byte, unmapped constraint violation) or a `<script>` echoed back fails the
job: fix the example or validate the input, don't silence the alert.

## Layout

- `src/main.rs` builds `AppState` from `REDIS_URL` + `DB_*` (the Postgres URL goes through
  `database::pg_url::build_pg_url`, which percent-encodes user, password and database name), serves Swagger UI at
  `/swagger-ui/` (spec at `/api-docs/openapi.json`, also the ZAP scan target), public `/health` and `/`, and mounts
  `endpoints::config` under `/api` wrapped in the lib's `JwtMiddleware`.
- `src/endpoints/` mirrors the URL path: each node has `mod.rs` (`config()`), and leaves have
  `endpoint.rs` (handler + `trigger_*` + error enum implementing `ResponseError` and
  `From<ApiLibError>`), `view.rs` (DTOs, private fields + getters) and `doc.rs` (utoipa), nested
  up to `endpoints/swagger.rs::ApiDoc` (prefix `/api/v1`).
- `src/endpoints/validation.rs`: request views with text fields implement `Validate` (length matching the
  Postgres column, no control character, no `<` / `>` in displayed labels) and handlers extract them with
  `ValidatedJson` / `ValidatedQuery` instead of `web::Json` / `web::Query`, which answer `400` naming the field.
  Map the lib's `DbError::ForeignKeyViolation` / `UniqueViolation` to `4xx`, never `500`. Responses carry
  `X-Content-Type-Options: nosniff` (`DefaultHeaders` in `main.rs`).
- `src/database/<resource>/<op>/view.rs`: query views implementing `ApiRequestDto`, run through
  `state.get_smart_db()`. `fetch_one`/`fetch_all` SQL must return one JSON column
  (`SELECT to_jsonb(t) FROM (...) t`).

## CI and Renovate

`.github/workflows/cicd.yml` calls `mairie360/CICD` `APIs_cicd.yml` but only on
`workflow_dispatch` in the template (add `push:` in a real API). Its `integration_tests` job runs
`./integration_test.sh`; no Postman variable or secret is needed. `renovate.json` deliberately
overrides the org preset to automerge everything (majors, 0.x, prod `Dockerfile`) with
`ignoreTests: true` and `platformAutomerge: false`, so PRs merge even when CI fails. That is
template-only: real APIs keep the standard config (org preset + `cicd_version` custom manager),
and the README tells new APIs to swap it back. Don't propagate the automerge-all file to siblings.
