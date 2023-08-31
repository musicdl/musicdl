// mod saavn;
//
// use crate::saavn::helpers::get_media_url;
// use std::{error::Error, process};
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let search = saavn::api::search("Clandestina").await?;
//
//     println!("{:#?}", search);
//
//     if search.songs.data.len() < 1 {
//         eprintln!("Result not found!!");
//         process::exit(0)
//     }
//
//     let song_id = &search.songs.data[0].id;
//
//     let song = saavn::api::get_song(song_id).await?;
//
//     println!(
//         "{}: {}",
//         song.song,
//         get_media_url(&song.encrypted_media_url, true).unwrap(),
//     );
//
//     Ok(())
// }
mod api;
mod helpers;
mod models;

pub use api::*;
pub use helpers::*;
pub use models::*;
