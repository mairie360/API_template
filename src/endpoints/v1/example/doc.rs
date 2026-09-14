use crate::endpoints::v1::example::id::doc::IdDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/{id}", api = IdDoc),
))]
pub struct ExampleDoc;
