use crate::{db::models::Playlist, utils::idgen};
use sqlx::{Error, PgPool};
use std::collections::HashSet;

#[derive(Debug)]
pub struct PlaylistDAO {
    pool: PgPool,
}

impl PlaylistDAO {
    pub fn new(pool: PgPool) -> Self {
        PlaylistDAO { pool }
    }

    pub async fn get_all_user_playlists(&self, user_id: &str) -> Result<Vec<Playlist>, Error> {
        let playlists: Vec<Playlist> =
            sqlx::query!(r#"SELECT * FROM playlists WHERE user_id = $1"#, user_id)
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|record| Playlist {
                    playlist_id: record.playlist_id,
                    name: record.name,
                    song_ids: HashSet::from_iter(record.song_ids.into_iter()),
                    user_id: record.user_id,
                })
                .collect();

        Ok(playlists)
    }

    pub async fn get_playlist_by_id(&self, playlist_id: &str) -> Result<Option<Playlist>, Error> {
        let playlist = sqlx::query!(
            r#"SELECT * FROM playlists WHERE playlist_id = $1"#,
            playlist_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|record| Playlist {
            playlist_id: record.playlist_id,
            name: record.name,
            song_ids: HashSet::from_iter(record.song_ids.into_iter()),
            user_id: record.user_id,
        });

        Ok(playlist)
    }

    pub async fn create_empty_playlist(&self, user_id: &str, name: &str) -> Result<String, Error> {
        let playlist_id = idgen::gen_playlist_id();

        let empty_vec: Vec<String> = Vec::new();
        sqlx::query!(
            r#"INSERT INTO playlists (playlist_id, user_id, name, song_ids) VALUES ($1, $2, $3, $4)"#,
            playlist_id,
            user_id,
            name,
            &empty_vec,
        ).execute(&self.pool).await?;

        Ok(playlist_id)
    }

    pub async fn add_songs(&self, playlist: Playlist, song_ids: Vec<String>) -> Result<(), Error> {
        let mut songs_mut = playlist.song_ids.clone();
        songs_mut.extend(song_ids);

        if songs_mut == playlist.song_ids {
            return Ok(());
        }

        sqlx::query!(
            r#"UPDATE playlists SET song_ids = $1 WHERE playlist_id = $2"#,
            &Vec::from_iter(songs_mut.into_iter()),
            playlist.playlist_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn remove_songs(
        &self,
        playlist: Playlist,
        song_ids: Vec<String>,
    ) -> Result<(), Error> {
        let mut songs_mut = playlist.song_ids.clone();
        songs_mut.retain(move |song| !song_ids.contains(song));

        if songs_mut == playlist.song_ids {
            return Ok(());
        }

        sqlx::query!(
            r#"UPDATE playlists SET song_ids = $1 WHERE playlist_id = $2"#,
            &Vec::from_iter(songs_mut.into_iter()),
            playlist.playlist_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn rename_playlist(&self, playlist: &Playlist, new_name: &str) -> Result<(), Error> {
        let current = &playlist.name;

        if current == new_name {
            Ok(())
        } else {
            sqlx::query!(
                r#"UPDATE playlists SET name = $1 WHERE playlist_id = $2"#,
                new_name,
                playlist.playlist_id
            )
            .execute(&self.pool)
            .await?;

            Ok(())
        }
    }

    pub async fn delete_playlist(&self, playlist: &Playlist) -> Result<(), Error> {
        sqlx::query!(
            r#"DELETE FROM playlists WHERE playlist_id = $1"#,
            playlist.playlist_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
