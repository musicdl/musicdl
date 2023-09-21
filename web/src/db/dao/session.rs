use chrono::{Duration, NaiveDateTime, Utc};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use thiserror::Error;

use crate::db::models::Session;

#[derive(Debug)]
pub struct SessionDAO {
    pool: PgPool,
}

#[derive(Debug, Error)]
pub enum SessionVerifyError {
    #[error("Session not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl SessionDAO {
    pub fn new(pool: PgPool) -> Self {
        SessionDAO { pool }
    }

    pub async fn verify_session(
        &self,
        session_token: &str,
    ) -> Result<Option<String>, SessionVerifyError> {
        let current_time = self.get_current_time();

        let session: Result<Session, sqlx::Error> = sqlx::query!(
            "SELECT * FROM sessions WHERE session_id = $1",
            session_token
        )
        .map(|res| Session {
            user_id: res.user_id,
            session_id: Secret::new(res.session_id),
            created_at: res.created_at,
            expired_at: res.expired_at,
        })
        .fetch_one(&self.pool)
        .await;

        match session {
            Ok(session) => {
                if session.expired_at > current_time {
                    Ok(Some(session.user_id))
                } else {
                    Ok(None)
                }
            }
            Err(sqlx::Error::RowNotFound) => Err(SessionVerifyError::NotFound),
            Err(err) => Err(SessionVerifyError::DatabaseError(err)),
        }
    }

    fn get_current_time(&self) -> NaiveDateTime {
        Utc::now().naive_utc()
    }

    pub async fn create_session(
        &self,
        user_id: &str,
        session_id: Secret<String>,
    ) -> Result<(), sqlx::Error> {
        let current_time = self.get_current_time();
        let expiration_time = current_time + Duration::hours(6);

        sqlx::query!(
            r#"INSERT INTO sessions (user_id, session_id, created_at, expired_at) VALUES ($1, $2, $3, $4)"#,
            user_id,
            session_id.expose_secret(),
            current_time,
            expiration_time
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn expire_session(&self, session_id: &str) -> Result<(), sqlx::Error> {
        let current_time = self.get_current_time();

        sqlx::query!(
            "UPDATE sessions SET expired_at = $1 WHERE session_id = $2",
            current_time,
            session_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
