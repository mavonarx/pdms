use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}
