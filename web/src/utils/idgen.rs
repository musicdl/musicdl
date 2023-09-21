use nanoid::nanoid;

pub fn gen_user_id() -> String {
    format!("user_{}", nanoid!())
}

pub fn gen_playlist_id() -> String {
    format!("playlist_{}", nanoid!())
}
