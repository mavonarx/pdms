use crate::models::user::User;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, user: User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO USERS (username, first_name, last_name, role)
        VALUES ($1, $2, $3, $4)
        "#,
        user.username,
        user.first_name,
        user.last_name,
        user.role
    )
    .execute(pool)
    .await?;

    Ok(())
}
