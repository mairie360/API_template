use crate::endpoints::v1::example::doc::ExampleDoc; // change example resource
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/example", api = ExampleDoc), // change example resource
))]
pub struct V1Doc;
