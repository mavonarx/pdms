use crate::models::user::User;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
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

pub async fn delete_user(pool: &PgPool, username: &str) -> Result<(), sqlx::Error> {
    let result = sqlx::query!("DELETE FROM USERS WHERE username = $1", username)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        Err(sqlx::Error::RowNotFound)
    } else {
        Ok(())
    }
}
