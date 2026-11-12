pub mod db_error;
pub mod headers;
pub mod health;
pub mod pagination;
pub mod swagger;
pub mod v1;
pub mod validation;

use actix_web::web;

/// Message of the `400` answered for a path parameter that does not parse. It names the rule,
/// never the value received (MAIR-290).
const INVALID_PATH_MESSAGE: &str = "Invalid path: ids are integers.";

/// A path id that is not an integer answers `400 text/plain`, as the contracts document, instead
/// of actix's default `404` (MAIR-262).
fn path_config() -> web::PathConfig {
    web::PathConfig::default()
        .error_handler(|_, _| actix_web::error::ErrorBadRequest(INVALID_PATH_MESSAGE))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(path_config()).configure(v1::config);
}

#[cfg(test)]
mod tests {
    use super::{config, INVALID_PATH_MESSAGE};
    use actix_web::http::header::CONTENT_TYPE;
    use actix_web::{http::StatusCode, test, web, App, HttpResponse};

    /// MAIR-262: an id that does not parse answers the documented `400 text/plain` without
    /// echoing it; actix answered `404`.
    #[actix_web::test]
    async fn a_path_id_that_is_not_an_integer_answers_400() {
        let app = test::init_service(App::new().service(
            web::scope("/api").configure(config).route(
                "/items/{id}",
                web::get().to(|id: web::Path<u64>| async move {
                    HttpResponse::Ok().body(id.into_inner().to_string())
                }),
            ),
        ))
        .await;

        let resp = test::call_service(
            &app,
            test::TestRequest::get().uri("/api/items/abc").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        assert!(resp
            .headers()
            .get(CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/plain"));
        assert_eq!(test::read_body(resp).await, INVALID_PATH_MESSAGE);

        let resp = test::call_service(
            &app,
            test::TestRequest::get().uri("/api/items/7").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
