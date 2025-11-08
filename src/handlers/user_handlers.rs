use crate::db::users::{create_user, delete_user};
use crate::models::user::User;
use crate::state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct DeleteUserRequest {
    pub username: String,
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created"),
        (status = 500, description = "Database error")
    )
)]
pub async fn add_user_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let user = User {
        username: payload.username,
        first_name: payload.first_name.unwrap_or_default(),
        last_name: payload.last_name.unwrap_or_default(),
        role: payload.role.unwrap_or_else(|| "user".to_string()),
    };

    match create_user(&state.pool, &user).await {
        Ok(_) => {
            tracing::info!("created user {}", user.username);
            (StatusCode::CREATED, "User created").into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/users",
    request_body = DeleteUserRequest,
    responses(
        (status = 200, description = "User deleted"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Database error")
    )
)]
pub async fn delete_user_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteUserRequest>,
) -> impl IntoResponse {
    match delete_user(&state.pool, &payload.username).await {
        Ok(_) => (StatusCode::OK, "User deleted").into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to delete user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
                .into_response()
        }
    }
}

pub async fn get_users_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, String> {
    let users = sqlx::query_as::<_, User>("SELECT id, username, first_name, last_name, role FROM users")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(Json(users))
}