use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, FromRow)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}
