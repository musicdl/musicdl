use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub albums: DataOf<Album>,
    pub songs: DataOf<Song>,
    pub playlists: DataOf<Playlist>,
    pub artists: DataOf<Artist>,
    pub topquery: DataOf<TopQuery>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataOf<T> {
    pub data: Vec<T>,
    pub position: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Album {
    pub id: String,
    pub title: String,
    pub image: String,
    pub music: String,
    pub url: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub description: String,
    pub ctr: u32,
    pub position: u32,
    pub more_info: MoreInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub image: String,
    pub album: String,
    pub url: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub description: String,
    pub ctr: u32,
    pub position: u32,
    pub more_info: MoreInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SongDetailed {
    pub id: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub song: String,
    pub year: String,
    pub album: String,
    pub music: String,
    pub music_id: String,
    pub primary_artists: String,
    pub primary_artists_id: String,
    pub featured_artists: String,
    pub featured_artists_id: String,
    pub singers: String,
    pub starring: String,
    pub image: String,
    pub albumid: String,
    pub language: String,
    pub origin: String,
    pub play_count: u64,
    pub copyright_text: String,
    #[serde(rename = "320kbps")]
    pub high_quality: String, // Fix this cause it'll be either "true" or "false"
    pub is_dolby_content: bool,
    pub explicit_content: u8,
    pub has_lyrics: String, // Stringified boolean ffs
    pub lyrics_snippet: String,
    pub encrypted_media_url: String,
    pub encrypted_media_path: String,
    pub media_preview_url: String,
    pub perma_url: String,
    pub album_url: String,
    pub duration: String,
    pub rights: SongRights,
    pub webp: bool,
    pub cache_state: String, // ""
    pub starred: String,     // ""
    #[serde(rename = "artistMap")]
    pub artist_map: HashMap<String, String>,
    pub release_date: String,
    pub triller_available: bool,
    pub label_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SongRights {
    pub code: u8,
    pub reason: String,
    pub cacheable: bool,
    pub delete_cached_object: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub image: String,
    pub extra: Option<String>,
    pub url: String,
    pub language: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub description: String,
    pub position: u32,
    pub more_info: Option<MoreInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    pub id: String,
    pub title: String,
    pub image: String,
    pub extra: Option<String>,
    pub url: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub description: String,
    pub ctr: u32,
    pub entity: Option<u32>,
    pub position: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TopQuery {
    pub id: String,
    pub title: String,
    pub image: String,
    pub extra: Option<String>,
    pub url: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub description: String,
    pub ctr: u32,
    pub entity: Option<u32>,
    pub position: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoreInfo {
    pub year: Option<String>,
    pub is_movie: Option<String>,
    pub language: Option<String>,
    pub song_pids: Option<String>,
    pub vcode: Option<String>,
    pub vlink: Option<String>,
    pub primary_artists: Option<String>,
    pub singers: Option<String>,
    pub video_available: Option<String>,
    pub triller_available: Option<bool>,
}
