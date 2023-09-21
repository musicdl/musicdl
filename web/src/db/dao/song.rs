use sqlx::PgPool;

use crate::db::models::Song;

#[derive(Debug)]
pub struct SongsDAO {
    pool: PgPool,
}

impl SongsDAO {
    pub fn new(pool: PgPool) -> Self {
        SongsDAO { pool }
    }

    pub async fn get_or_fetch_song(&self, song_id: &str) -> Result<Option<Song>, anyhow::Error> {
        let song = self.get_song_by_id(song_id).await?;

        match song {
            Some(song) => Ok(Some(song)),
            None => {
                let fetch = common::get_song(song_id).await;
                match fetch {
                    Ok(song_d) => {
                        let song = Song::from(song_d);
                        self.insert_song(song.clone()).await?;
                        Ok(Some(song))
                    }
                    Err(_) => Ok(None),
                }
            }
        }
    }

    pub async fn get_song_by_id(&self, song_id: &str) -> Result<Option<Song>, anyhow::Error> {
        let song = sqlx::query!(r#"SELECT * FROM songs WHERE song_id = $1"#, song_id)
            .fetch_optional(&self.pool)
            .await?
            .map(|record| Song {
                song_id: record.song_id,
                name: record.name,
                url: record.url,
                image: record.image,
                duration: record.duration,
                artists: record.artists,
            });

        Ok(song)
    }

    pub async fn insert_song(&self, song: Song) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"INSERT INTO songs (song_id, name, url, image, duration, artists) VALUES ($1, $2, $3, $4, $5, $6)"#,
            song.song_id,
            song.name,
            song.url,
            song.image,
            song.duration,
            song.artists
        ).execute(&self.pool).await?;

        Ok(())
    }
}
