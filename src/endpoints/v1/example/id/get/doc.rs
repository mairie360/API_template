use crate::endpoints::v1::example::id::get::{
    endpoint::__path_get_example, view::GetExampleResultView,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_example), components(schemas(GetExampleResultView)))]
pub struct GetDoc;
