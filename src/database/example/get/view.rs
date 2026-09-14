use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

/// Vue d'exemple : lit un utilisateur par id. À remplacer par les requêtes de l'API.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetExampleQueryView {
    params: Vec<QueryParam>,
}

impl GetExampleQueryView {
    pub fn new(id: u64) -> Self {
        Self {
            params: vec![QueryParam::I32(id as i32)],
        }
    }

    pub fn id(&self) -> u64 {
        self.params[0].as_i32() as u64
    }
}

impl Display for GetExampleQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetExampleQueryView(id={})", self.id())
    }
}

impl ApiRequestDto for GetExampleQueryView {
    fn query_sql(&self) -> &'static str {
        // fetch_one / fetch_all attendent une unique colonne JSON
        "SELECT to_jsonb(t) FROM (
            SELECT id, first_name, last_name, email
            FROM users WHERE id = $1
         ) t"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GetExampleQueryResultView {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
}

impl GetExampleQueryResultView {
    pub fn new(id: i32, first_name: &str, last_name: &str, email: &str) -> Self {
        Self {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
        }
    }

    pub fn id(&self) -> i32 {
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

impl Display for GetExampleQueryResultView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Example: id={} first_name={} last_name={} email={}",
            self.id, self.first_name, self.last_name, self.email
        )
    }
}
