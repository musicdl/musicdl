use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SongSearch {
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongGet {
    pub song_id: String,
}
