use std::collections::HashSet;

use chrono::NaiveDateTime;
use common::{get_media_url, SongDetailed};
use secrecy::Secret;
use serde::{Deserialize, Serialize};

/*
 * NOTE: FromRow & sqlx(rename_all) is not supported by query_as! macro, hence, sql will also have to use snake case
 * https://github.com/launchbadge/sqlx/issues/514
 */

#[derive(Deserialize, Debug, sqlx::FromRow, Serialize, Clone)]
pub struct Song {
    pub song_id: String,
    pub name: String,
    pub url: String,
    pub image: String,
    pub duration: String,
    pub artists: String,
}

impl From<SongDetailed> for Song {
    fn from(song_d: SongDetailed) -> Self {
        let url =
            get_media_url(&song_d.encrypted_media_url, !song_d.high_quality.is_empty()).unwrap();
        // get_media_url(&song_d.encrypted_media_url, true)).unwrap();

        Song {
            song_id: song_d.id,
            name: song_d.song,
            url,
            image: song_d.image,
            duration: song_d.duration,
            artists: song_d.singers,
        }
    }
}

#[derive(Clone, Deserialize, Debug, sqlx::FromRow, Serialize)]
pub struct Playlist {
    pub playlist_id: String,
    pub name: String,
    pub song_ids: HashSet<String>,
    pub user_id: String,
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: String,
    pub password_hash: Secret<String>,
    pub username: String,
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct Session {
    pub user_id: String,
    pub session_id: Secret<String>,
    pub created_at: NaiveDateTime,
    pub expired_at: NaiveDateTime,
}
