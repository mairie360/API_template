use crate::endpoints::v1::example::id::get::doc::GetDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/", api = GetDoc),
))]
pub struct IdDoc;
