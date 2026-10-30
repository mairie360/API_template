mod common;

use std::time::{Duration, Instant};

use actix_web::{http::StatusCode, test, web, App};
use api_template::endpoints::health::{self, wait_for_postgres}; // change api name
use mairie360_api_lib::state::{AppState, DB_CONNECT_TIMEOUT_ENV};
use mairie360_api_lib::test_setup::redis_setup::start_redis_container;

use common::api_db_url;

/// Nothing listens on port 1: both connections are refused right away.
const UNREACHABLE_PG: &str = "postgres://postgres:postgres@127.0.0.1:1/postgres";
const UNREACHABLE_REDIS: &str = "redis://127.0.0.1:1";

async fn get(state: AppState, uri: &str) -> (StatusCode, String) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .service(health::health)
            .service(health::ready),
    )
    .await;
    let response = test::call_service(&app, test::TestRequest::get().uri(uri).to_request()).await;
    let status = response.status();
    let body = test::read_body(response).await;
    (status, String::from_utf8(body.to_vec()).unwrap())
}

#[actix_web::test]
async fn ready_answers_200_when_postgres_and_redis_answer() {
    let pg_url = api_db_url().await;
    let (_redis, redis) = start_redis_container().await;
    let state = AppState::new(redis.url.clone(), pg_url).await;

    assert_eq!(
        get(state, "/ready").await,
        (StatusCode::OK, "ready".to_owned())
    );
}

#[actix_web::test]
async fn ready_names_only_the_unreachable_dependency() {
    let pg_url = api_db_url().await;
    let state = AppState::new(UNREACHABLE_REDIS.to_owned(), pg_url).await;

    assert_eq!(
        get(state, "/ready").await,
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "not ready: redis".to_owned()
        )
    );
}

#[actix_web::test]
async fn health_answers_200_even_without_redis() {
    let state = AppState::new(UNREACHABLE_REDIS.to_owned(), api_db_url().await).await;

    assert_eq!(
        get(state, "/health").await,
        (StatusCode::OK, "OK".to_owned())
    );
}

#[actix_web::test]
async fn startup_check_passes_once_postgres_answers() {
    let pg_url = api_db_url().await;
    let state = AppState::new(UNREACHABLE_REDIS.to_owned(), pg_url).await;

    assert!(wait_for_postgres(&state, 3, Duration::ZERO).await);
}

/// Since mairie360_api_lib 3.0 (MAIR-423) `AppState::new` itself refuses an unreachable Postgres:
/// it panics once `DB_CONNECT_TIMEOUT` has elapsed, so the API never starts without its database.
#[actix_web::test]
async fn startup_refuses_an_unreachable_postgres() {
    // Only this test builds a state on an unreachable Postgres; the others connect at once and
    // never reach the timeout.
    std::env::set_var(DB_CONNECT_TIMEOUT_ENV, "1");

    let started = Instant::now();
    let startup = actix_web::rt::spawn(AppState::new(
        UNREACHABLE_REDIS.to_owned(),
        UNREACHABLE_PG.to_owned(),
    ))
    .await;

    assert!(
        startup.as_ref().is_err_and(|e| e.is_panic()),
        "the state must not be built without Postgres"
    );
    assert!(started.elapsed() < Duration::from_secs(10));
}
