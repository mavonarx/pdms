mod db;
mod models;

use axum::{
    routing::{get, post, delete},
    Router, Json,
    extract::State,
    response::IntoResponse,
    http::StatusCode
};
use utoipa::{ToSchema, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use crate::models::user::User;
use crate::db::users::{create_user, delete_user};

#[tokio::main]
async fn main() {
    // Setup logging so we can see what's happening
    tracing_subscriber::fmt::init();

    tracing::info!("Starting Patient Management System");

    // Load .env file
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Create database connection pool
    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connection established");

    // Test the connection
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .expect("Failed to execute test query");
    
    tracing::info!("Database test query successful: {}", row.0);

    // Wrap pool in Arc for sharing across handlers
    let app_state = Arc::new(AppState { pool });

    // Create a simple router
    let app = Router::new()
        .route("/db-check", get(db_check))
        .route("/users", post(add_user_handler))
        .route("/users", delete(delete_user_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(app_state);

    // Listen on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Shared application state
struct AppState {
    pool: sqlx::PgPool,
}

// Database check endpoint
async fn db_check(State(state): State<Arc<AppState>>) -> String {
    match sqlx::query("SELECT 1")
        .fetch_one(&state.pool)
        .await
    {
        Ok(_) => "Database connection OK".to_string(),
        Err(e) => format!("Database error: {}", e),
    }
}

#[derive(Deserialize, ToSchema)]
struct CreateUserRequest {
    username: String, 
    first_name: Option<String>, 
    last_name: Option<String>,
    role: Option<String>,
}

#[derive(Deserialize, ToSchema)]
struct DeleteUserRequest {
    username: String,
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
async fn add_user_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let user = User { 
        username: payload.username,
        first_name: payload.first_name.unwrap_or_default(),
        last_name: payload.last_name.unwrap_or_default(),
        role: payload.role.unwrap_or_else(|| "user".to_string()),
    };

    match create_user(&state.pool, user).await {
        Ok(_) => (StatusCode::CREATED, "User created").into_response(), 
        Err(e) => {
            tracing::error!("Failed to create user: {:?}",e );
            (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)).into_response()
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
async fn delete_user_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteUserRequest>,
) -> impl IntoResponse {
    match delete_user(&state.pool, &payload.username).await {
        Ok(_) =>  (StatusCode::OK, "User deleted").into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to delete user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)).into_response()
        }
    }
}


#[derive(OpenApi)]
#[openapi(
    paths(
        add_user_handler,delete_user_handler
    ),
    components(
        schemas(User, CreateUserRequest)
    ),
    tags (
        (name = "users", description = "User management endpoints")
    )
)]
struct ApiDoc;