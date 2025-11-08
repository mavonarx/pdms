#[cfg(test)]
use sqlx::{PgPool};

#[cfg(test)]
pub async fn setup_test_db() -> PgPool {
    dotenvy::from_filename(".env.test").expect(".env.test file not found");
    let data_base_url =
        std::env::var("TEST_DATABASE_URL").expect("Test database must be set int .env.test file");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&data_base_url)
        .await
        .expect("Failed to connect to test database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    pool
}

#[cfg(test)]
pub async fn cleanup_test_db(pool: &PgPool) {
    sqlx::query!("TRUNCATE TABLE users CASCADE")
        .execute(pool)
        .await
        .expect("Failed to clean up test database");
}