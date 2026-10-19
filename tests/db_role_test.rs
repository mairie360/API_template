// The code under test connects as the API's Postgres role, never as the superuser (MAIR-556): a
// superuser ignores the grants of Database's `api_grants.sql`, so a query the API may not run in
// staging would pass here. A new API keeps this file, with a table its own role is not granted.

mod common;

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};
use mairie360_api_lib::state::AppState;
use serde::{Deserialize, Serialize};

use common::{api_db_url, API_DB_ROLE};

/// Nothing listens on port 1: these queries declare no cache key, so Redis is never used.
const UNREACHABLE_REDIS: &str = "redis://127.0.0.1:1";

#[derive(Deserialize)]
struct CurrentRole;

/// The role a query runs as.
#[derive(Deserialize, Serialize)]
struct Role {
    name: String,
    superuser: bool,
}

impl ApiRequestDto for CurrentRole {
    fn query_sql(&self) -> &'static str {
        "SELECT json_build_object('name', current_user, 'superuser', rolsuper) \
         FROM pg_roles WHERE rolname = current_user"
    }

    fn query_params(&self) -> &[QueryParam] {
        &[]
    }
}

/// A table outside the role's grants: `core_api` never reads the project domain.
#[derive(Deserialize)]
struct ReadUngrantedTable;

impl ApiRequestDto for ReadUngrantedTable {
    fn query_sql(&self) -> &'static str {
        "SELECT count(*) FROM projects" // change api role (a table it is not granted)
    }

    fn query_params(&self) -> &[QueryParam] {
        &[]
    }
}

async fn state() -> AppState {
    AppState::new(UNREACHABLE_REDIS.to_owned(), api_db_url().await).await
}

#[actix_web::test]
async fn the_code_under_test_runs_as_the_api_role() {
    let state = state().await;

    let role = state
        .get_smart_db()
        .fetch_one::<Role, _>(&CurrentRole)
        .await
        .expect("current role");

    assert_eq!(role.name, API_DB_ROLE);
    assert!(!role.superuser);
}

#[actix_web::test]
async fn the_api_role_is_held_to_its_grants() {
    let state = state().await;

    let error = state
        .get_smart_db()
        .fetch_scalar::<i64, _>(&ReadUngrantedTable)
        .await
        .expect_err("the API role must not read a table it is not granted");

    // 42501: insufficient_privilege, what staging answers to a query outside the grants.
    assert!(error.to_string().contains("SQLSTATE 42501"), "{error}");
}
