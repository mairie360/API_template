//! Database access shared by the test binaries (MAIR-556).
//!
//! The code under test (`AppState`, query views run through `SmartDatabase`) connects as the
//! API's own Postgres role, like in staging/prod: a superuser ignores the grants of Database's
//! `api_grants.sql`, so a query the role may not run would pass here and fail in staging with
//! `permission denied`. Fixtures (creating, seeding or cleaning rows) keep the superuser URL of
//! `get_shared_db()`. When a test fails with `permission denied` (SQLSTATE 42501), add the grant
//! in Database's `api_grants.sql`, never connect the code under test as the superuser.

use mairie360_api_lib::test_setup::queries_setup::get_shared_db_as;

/// Postgres role this API connects with in every deployment (Database's `api_roles.sql`).
/// The template has no role of its own and borrows Core's so its tests run.
pub const API_DB_ROLE: &str = "core_api"; // change api role (<api>_api)

/// URL of the shared test database as [`API_DB_ROLE`]: the one the code under test uses.
pub async fn api_db_url() -> String {
    get_shared_db_as(API_DB_ROLE).await
}
