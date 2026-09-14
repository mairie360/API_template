use utoipa::ToSchema;

use crate::database::example::get::view::GetExampleQueryResultView;

#[derive(Debug, Clone, PartialEq, serde::Serialize, ToSchema)]
pub struct GetExampleResultView {
    id: u64,
    first_name: String,
    last_name: String,
    email: String,
}

impl GetExampleResultView {
    pub fn new(id: u64, first_name: String, last_name: String, email: String) -> Self {
        Self {
            id,
            first_name,
            last_name,
            email,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

impl From<GetExampleQueryResultView> for GetExampleResultView {
    fn from(result: GetExampleQueryResultView) -> Self {
        Self::new(
            result.id() as u64,
            result.first_name().to_string(),
            result.last_name().to_string(),
            result.email().to_string(),
        )
    }
}
