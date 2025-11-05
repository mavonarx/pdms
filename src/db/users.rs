use crate::models::user::User;
use sqlx::{Executor, Postgres};

pub async fn create_user<'e, E>(executor: E, user: &User) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
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
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn delete_user<'e, E>(executor: E, username: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query!("DELETE FROM USERS WHERE username = $1", username)
    .execute(executor)
    .await?;

    if result.rows_affected() == 0 {
        Err(sqlx::Error::RowNotFound)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_utils::{setup_test_db};
    use crate::models::user::User;
    use tokio::sync::OnceCell;

    static POOL: OnceCell<sqlx::PgPool> = OnceCell::const_new();
    pub async fn get_pool() -> &'static sqlx::PgPool {
        POOL.get_or_init(|| async {
            // setup_test_db() must be an async fn that returns a PgPool
            crate::db::test_utils::setup_test_db().await
        }).await
    }

    async fn get_transaction() -> sqlx::Transaction<'static, Postgres> {
        let pool = get_pool().await;
        pool.begin()
            .await
            .expect("Failed to start Transaction for test")
    }

    #[tokio::test]
    async fn test_create_user(){
        let mut tx = get_transaction().await;

        let user = User {
            username : "test_user".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            role: "TestRole".to_string(),
        };

        let result = create_user(&mut *tx, &user).await;
        assert!(result.is_ok());
        let _ = tx.rollback().await.unwrap();
    }


    #[tokio::test]
    async fn test_create_duplicate_user() {
        let mut tx = get_transaction().await;

        let user = User {
            username: "duplicate_user".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            role: "nurse".to_string(),
        };

        // Create first user
        create_user(&mut *tx, &user).await.expect("Failed to create first user");

        // Try to create duplicate (should fail due to unique constraint)
        let result = create_user(&mut *tx, &user).await;
        assert!(result.is_err());
        let _ = tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_user() {
        let mut tx = get_transaction().await;

        let user = User {
            username: "user_to_delete".to_string(),
            first_name: "Delete".to_string(),
            last_name: "Me".to_string(),
            role: "admin".to_string(),
        };

        // Create user
        create_user(&mut *tx, &user).await.expect("Failed to create user");

        // Delete user
        let result = delete_user(&mut *tx, &user.username).await;
        assert!(result.is_ok(), "The result was not ok it was: {}",result.err().unwrap());
        let _ = tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_nonexistent_user() {
        let mut tx = get_transaction().await;

        let result = delete_user(&mut *tx, "nonexistent_user").await;
        assert!(result.is_err());
        
        match result {
            Err(sqlx::Error::RowNotFound) => {},
            _ => panic!("Expected RowNotFound error"),
        }
        let _ = tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_create_user_with_different_roles() {
        let mut tx = get_transaction().await;

        let roles = vec!["doctor", "nurse", "admin", "receptionist"];

        for (i, role) in roles.iter().enumerate() {
            let user = User {
                username: format!("user_{}", i),
                first_name: "Test".to_string(),
                last_name: format!("User{}", i),
                role: role.to_string(),
            };

            let result = create_user(&mut *tx, &user).await;
            assert!(result.is_ok(), "Failed to create user with role: {}", role);
        }

        let _ = tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_create_user_with_empty_optional_fields() {
        let mut tx = get_transaction().await;

        let user = User {
            username: "minimal_user".to_string(),
            first_name: String::new(),
            last_name: String::new(),
            role: "user".to_string(),
        };

        let result = create_user(&mut *tx, &user).await;
        assert!(result.is_ok());
        let _ = tx.rollback().await.unwrap();
    }
}
