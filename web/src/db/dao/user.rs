use secrecy::{ExposeSecret, Secret};
use sqlx::{Error, PgPool};

use crate::{db::models::User, utils::idgen};

#[derive(Debug)]
pub struct UserDAO {
    pool: PgPool,
}

impl UserDAO {
    pub fn new(pool: PgPool) -> Self {
        UserDAO { pool }
    }

    pub async fn username_exists(&self, username: &str) -> Result<bool, Error> {
        let res = sqlx::query!(
            r#"SELECT 1 as exists FROM users WHERE username = $1"#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(res.is_some())
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query!(r#"SELECT * FROM users WHERE username = $1"#, username)
            .fetch_optional(&self.pool)
            .await?
            .map(|record| User {
                username: record.username,
                user_id: record.user_id,
                password_hash: Secret::new(record.password_hash),
            });

        Ok(user)
    }

    pub async fn create_user(
        &self,
        username: &str,
        password_hash: Secret<String>,
    ) -> Result<(), Error> {
        let user_id = idgen::gen_user_id();

        sqlx::query!(
            r#"INSERT INTO users (user_id, username, password_hash) VALUES ($1, $2, $3)"#,
            user_id,
            username,
            password_hash.expose_secret()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // NOTE: Do not forget to check that user_id is immutable
    #[allow(unused)]
    pub async fn update_user(&self, new_user: User) -> Result<(), Error> {
        sqlx::query!(
            r#"UPDATE users SET username = $1, password_hash = $2 WHERE user_id = $3"#,
            new_user.username,
            new_user.password_hash.expose_secret(),
            new_user.user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
