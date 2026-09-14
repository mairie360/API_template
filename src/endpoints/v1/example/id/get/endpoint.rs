use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::example::get::view::{GetExampleQueryResultView, GetExampleQueryView};
use crate::endpoints::v1::example::id::get::view::GetExampleResultView;

#[derive(Debug, Clone, PartialEq)]
pub enum GetExampleError {
    DatabaseError,
    UnknownExample,
}

impl std::fmt::Display for GetExampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetExampleError::DatabaseError => {
                write!(f, "An error occurred while accessing the database.")
            }
            GetExampleError::UnknownExample => {
                write!(f, "Unknown example.")
            }
        }
    }
}

impl ResponseError for GetExampleError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetExampleError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            GetExampleError::UnknownExample => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<ApiLibError> for GetExampleError {
    fn from(err: ApiLibError) -> Self {
        match err {
            ApiLibError::Database(DbError::NotFound) => GetExampleError::UnknownExample,
            _ => GetExampleError::DatabaseError,
        }
    }
}

async fn trigger_get_example(
    state: web::Data<AppState>,
    id: u64,
) -> Result<GetExampleResultView, GetExampleError> {
    let view = GetExampleQueryView::new(id);
    let result = state
        .get_smart_db()
        .fetch_one::<GetExampleQueryResultView, _>(&view)
        .await?;

    Ok(result.into())
}

#[utoipa::path(
    get,
    path = "",
    params(
        ("id" = u64, Path, description = "Example ID")
    ),
    responses(
        (status = 200, description = "Example details", body = GetExampleResultView),
        (status = 404, description = "Unknown example"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Example",
    security(
        ("jwt" = [])
    )
)]
#[get("/")]
pub async fn get_example(
    state: web::Data<AppState>,
    _: AuthenticatedUser,
    path_params: web::Path<u64>,
) -> Result<impl Responder, GetExampleError> {
    let example = trigger_get_example(state, path_params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(example))
}
