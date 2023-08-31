use std::{collections::HashMap, error::Error};

use urlencoding::encode;

use super::models::{SearchResult, SongDetailed};

pub async fn search(query: &str) -> Result<SearchResult, Box<dyn Error>> {
    let response = reqwest::get(format!("https://www.jiosaavn.com/api.php?__call=autocomplete.get&_format=json&_marker=0&cc=in&includeMetaTags=1&query={}", encode(query))).await?.text().await?;

    let result: SearchResult = serde_json::from_str(&response)?;

    Ok(result)
}

pub async fn get_song(song_id: &str) -> Result<SongDetailed, Box<dyn Error>> {
    let response = reqwest::get(format!("https://www.jiosaavn.com/api.php?__call=song.getDetails&cc=in&_marker=0%3F_marker%3D0&_format=json&pids={}", song_id)).await?.text().await?;

    let mut result: HashMap<String, SongDetailed> = serde_json::from_str(&response)?;

    let song = result.remove(song_id).ok_or("Song Not Found")?;

    Ok(song)
}
