// The `/api` scope is limited per user by the lib's limiter, mounted like `main.rs` (token checked
// first, then the budget). The template has no business endpoint, so a probe route stands for it.

mod common;
#[allow(dead_code)]
mod session_tokens;

use std::num::NonZeroU32;

use actix_web::middleware::from_fn;
use actix_web::{get, http::StatusCode, test, web, App, HttpResponse};
use mairie360_api_lib::security::{
    rate_limit_middleware, AuthenticatedUser, JwtMiddleware, RateLimitConfig, RateLimiter,
};
use mairie360_api_lib::state::AppState;
use mairie360_api_lib::test_setup::queries_setup::{get_shared_db, ADMIN_ID, GROUP_OWNER_ID};

use common::api_db_url;
use session_tokens::{core_token, set_jwt_env};

/// Nothing listens on port 1: no token checked here carries a session id, so Redis is never read.
const UNREACHABLE_REDIS: &str = "redis://127.0.0.1:1";

#[get("/v1/whoami/")]
async fn whoami(user: AuthenticatedUser) -> HttpResponse {
    HttpResponse::Ok().body(user.id.to_string())
}

fn limiter(burst: u32) -> RateLimiter {
    RateLimiter::new(RateLimitConfig {
        per_second: NonZeroU32::MIN,
        burst: NonZeroU32::new(burst).unwrap(),
        trust_forwarded: false,
    })
}

#[actix_web::test]
async fn mair_688_each_user_has_their_own_budget_and_a_429_carries_retry_after() {
    set_jwt_env();
    get_shared_db().await;
    let (admin, bob) = (*ADMIN_ID.get().unwrap(), *GROUP_OWNER_ID.get().unwrap());
    let state = AppState::new(UNREACHABLE_REDIS.to_owned(), api_db_url().await).await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .app_data(limiter(2))
            .service(
                web::scope("/api")
                    .wrap(from_fn(rate_limit_middleware))
                    .wrap(JwtMiddleware)
                    .service(whoami),
            ),
    )
    .await;
    let get = |user: i32| {
        test::TestRequest::get()
            .uri("/api/v1/whoami/")
            .insert_header(("Authorization", format!("Bearer {}", core_token(user))))
            .to_request()
    };

    for _ in 0..2 {
        assert_eq!(
            test::call_service(&app, get(admin)).await.status(),
            StatusCode::OK
        );
    }
    let limited = test::call_service(&app, get(admin)).await;
    assert_eq!(limited.status(), StatusCode::TOO_MANY_REQUESTS);
    let retry_after: u64 = limited
        .headers()
        .get("retry-after")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    assert!(retry_after >= 1);
    assert_eq!(
        limited.headers().get("x-ratelimit-after"),
        limited.headers().get("retry-after")
    );

    // Another user is not affected by the first one's burst.
    assert_eq!(
        test::call_service(&app, get(bob)).await.status(),
        StatusCode::OK
    );
}

#[actix_web::test]
async fn mair_688_the_429_is_documented_on_operations_inheriting_the_jwt_requirement() {
    use api_template::endpoints::swagger::ApiDoc; // change api name
    use utoipa::OpenApi;

    let spec = ApiDoc::openapi();
    for (path, item) in &spec.paths.paths {
        let operations = [&item.get, &item.put, &item.post, &item.delete, &item.patch];
        for operation in operations.into_iter().flatten() {
            let public = operation.security.as_ref().is_some_and(Vec::is_empty);
            if !public && path.starts_with("/api/") {
                assert!(
                    operation.responses.responses.contains_key("429"),
                    "{path} does not document the 429"
                );
            }
        }
    }
    // The template has no business route yet: the loop is a guard for the ones a new API adds.
}
