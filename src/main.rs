mod api_doc;
mod db;
mod handlers;
mod models;
mod state;

use axum::{
    Router,
    extract::State,
    routing::{delete, get, post},
};

use crate::api_doc::ApiDoc;
use crate::handlers::{add_user_handler, delete_user_handler};
use crate::state::AppState;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // Setup logging so we can see what's happening
    tracing_subscriber::fmt::init();

    tracing::info!("Starting Patient Management System");

    // Load .env file
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

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
        .merge(SwaggerUi::new("/api-doc").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(app_state);

    // Listen on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Database check endpoint
async fn db_check(State(state): State<Arc<AppState>>) -> String {
    match sqlx::query("SELECT 1").fetch_one(&state.pool).await {
        Ok(_) => "Database connection OK".to_string(),
        Err(e) => format!("Database error: {}", e),
    }
}
