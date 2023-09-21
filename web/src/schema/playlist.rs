use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllUserPlaylist {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SinglePlaylist {
    pub playlist_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEmptyPlaylist {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSongsFromPlaylist {
    pub playlist_id: String,
    pub songs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePlaylistName {
    pub playlist_id: String,
    pub new_name: String,
}
