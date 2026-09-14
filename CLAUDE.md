# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`API_template` is the scaffold every Mairie 360 Rust API (`Core_API`, `Project_API`, `Calendar_API`,
`Message_API`, `ELearning_API`) was generated from. Keep it in sync with those siblings: when a
shared pattern changes there (lib version, `main.rs` bootstrap, Docker/compose files, CI
workflow, Renovate config), port it here. The crate is `api_template`; every spot a new API must
rename carries a `change api name` / `change port` marker, on the same line (or on the line just
above in the `Dockerfile`s, which forbid trailing comments); wiring of the sample resource carries
`change example resource`. Any API name, port or URL added to the template must get one.

## Commands

Same aliases as the siblings (`.cargo/config.toml`): `cargo lint_check`, `cargo lint_fix`,
`cargo check_code` (clippy `-D warnings`), `cargo test` (Docker + GHCR pull access, testcontainers
via `mairie360_api_lib::test_setup::queries_setup::get_shared_db`), `cargo cov_test` (60 % line
gate, `endpoints/`, `main.rs`, `lib.rs` excluded), `cargo open_api` (OpenAPI JSON on stdout).

## Layout

- `src/main.rs` builds `AppState` from `REDIS_URL` + `DB_*`, serves Swagger UI at
  `/swagger-ui/` (spec at `/api-docs/openapi.json`, also the ZAP scan target), public `/health` and `/`, and mounts
  `endpoints::config` under `/api` wrapped in the lib's `JwtMiddleware`.
- `src/endpoints/` mirrors the URL path: each node has `mod.rs` (`config()`), and leaves have
  `endpoint.rs` (handler + `trigger_*` + error enum implementing `ResponseError` and
  `From<ApiLibError>`), `view.rs` (DTOs, private fields + getters) and `doc.rs` (utoipa), nested
  up to `endpoints/swagger.rs::ApiDoc` (prefix `/api/v1`).
- `src/database/<resource>/<op>/view.rs`: query views implementing `ApiRequestDto`, run through
  `state.get_smart_db()`. `fetch_one`/`fetch_all` SQL must return one JSON column
  (`SELECT to_jsonb(t) FROM (...) t`).
- The `example` resource (`GET /api/v1/example/{id}` reading `users`) exists only to show the
  pattern end to end and keep the test suite / coverage gate non-empty.

## CI and Renovate

`.github/workflows/cicd.yml` calls `mairie360/CICD` `APIs_cicd.yml` but only on
`workflow_dispatch` in the template (add `push:` in a real API). `renovate.json` deliberately
overrides the org preset to automerge everything (majors, 0.x, prod `Dockerfile`) with
`ignoreTests: true` and `platformAutomerge: false`, so PRs merge even when CI fails; branch
protection requiring status checks would still block that.
