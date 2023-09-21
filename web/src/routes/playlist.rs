use crate::db::dao::song::SongsDAO;
use crate::db::models::Song;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::db::dao::playlist::PlaylistDAO;
use crate::schema;

#[tracing::instrument(name = "Get All User Playlists")]
pub async fn get_all_playlists(
    data: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    req_json: web::Json<schema::playlist::GetAllUserPlaylist>,
) -> impl Responder {
    let conn = data.get_ref();
    let ext = req.extensions();

    // let user_id = ext.get::<String>().unwrap_or(&"".to_string());
    let user_id = ext.get::<String>().unwrap();
    let req_user_id = &req_json.user_id;

    let dao = PlaylistDAO::new(conn.clone());

    if user_id != req_user_id {
        HttpResponse::Unauthorized().finish()
    } else {
        let playlists = dao.get_all_user_playlists(req_user_id).await;

        match playlists {
            Ok(pl) => HttpResponse::Ok().json(json!({ "playlists": pl })),
            Err(_) => {
                tracing::error!("Could not get all playlists for user: {}", req_user_id);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[tracing::instrument(name = "Get All Songs By Playlist ID")]
pub async fn get_all_playlist_songs(
    data: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    req_json: web::Json<schema::playlist::SinglePlaylist>,
) -> impl Responder {
    let conn = data.get_ref();
    let ext = req.extensions();

    let user_id = ext.get::<String>().unwrap();
    let playlist_dao = PlaylistDAO::new(conn.clone());
    let songs_dao = SongsDAO::new(conn.clone());
    let playlist_id = &req_json.playlist_id;

    let playlist = playlist_dao.get_playlist_by_id(playlist_id).await;
    let mut songs: Vec<Song> = Vec::new();

    match playlist {
        Ok(pl) => match pl {
            Some(plist) => {
                if plist.user_id != *user_id {
                    HttpResponse::Unauthorized().finish()
                } else {
                    for ele in plist.song_ids.iter() {
                        let song = songs_dao.get_or_fetch_song(ele).await;

                        match song {
                            Ok(s) => songs.push(s.unwrap()),
                            Err(_) => {
                                tracing::error!("Error getting song: {}", ele);
                            }
                        };
                    }

                    HttpResponse::Ok().json(json!({ "songs": songs }))
                }
            }
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => {
            tracing::error!("Cannot get playlist: {}", playlist_id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Create empty playlist")]
pub async fn create_empty_playlist(
    data: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    req_json: web::Json<schema::playlist::CreateEmptyPlaylist>,
) -> impl Responder {
    let conn = data.get_ref();
    let ext = req.extensions();

    let user_id = ext.get::<String>().unwrap();
    let dao = PlaylistDAO::new(conn.clone());
    let name = &req_json.name;

    let playlist = dao.create_empty_playlist(user_id, name).await;

    match playlist {
        Ok(pl_id) => HttpResponse::Ok().json(json!({ "playlist_id": pl_id })),
        Err(_) => {
            tracing::error!("Could not create playlist: {:#?}", req_json);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Add Songs to Playlist (Batch)")]
pub async fn add_songs_to_playlist(
    data: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    req_json: web::Json<schema::playlist::UpdateSongsFromPlaylist>,
) -> impl Responder {
    let conn = data.get_ref();
    let ext = req.extensions();

    let user_id = ext.get::<String>().unwrap();
    let playlist_id = &req_json.playlist_id;
    let songs = &req_json.songs;

    let dao = PlaylistDAO::new(conn.clone());

    let playlist = dao.get_playlist_by_id(playlist_id).await;

    match playlist {
        Ok(some_playlist) => {
            match some_playlist {
                Some(playlist) => {
                    if &playlist.user_id != user_id {
                        HttpResponse::Unauthorized().finish()
                    } else {
                        let res = dao.add_songs(playlist.clone(), songs.to_vec()).await;
                        if res.is_ok() {
                            HttpResponse::Ok().json(json!({ "playlist_id": playlist_id }))
                        } else {
                            tracing::error!("Cannot add songs to playlist: {:#?}", playlist);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                },
                None => HttpResponse::NotFound().finish()
            }
        },
        Err(e) => {
            tracing::error!("Cannot add songs to playlist: {:#?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[tracing::instrument(name = "Remove Songs from Playlist (Batch)")]
pub async fn remove_songs_from_playlist(
    data: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    req_json: web::Json<schema::playlist::UpdateSongsFromPlaylist>,
) -> impl Responder {
    let conn = data.get_ref();
    let ext = req.extensions();

    let user_id = ext.get::<String>().unwrap();
    let playlist_id = &req_json.playlist_id;
    let songs = &req_json.songs;

    let dao = PlaylistDAO::new(conn.clone());

    let playlist = dao.get_playlist_by_id(playlist_id).await;

    match playlist {
        Ok(some_playlist) => {
            match some_playlist {
                Some(playlist) => {
                    if &playlist.user_id != user_id {
                        HttpResponse::Unauthorized().finish()
                    } else {
                        let res = dao.remove_songs(playlist.clone(), songs.to_vec()).await;
                        if res.is_ok() {
                            HttpResponse::Ok().json(json!({ "playlist_id": playlist_id }))
                        } else {
                            tracing::error!("Cannot remove songs from playlist: {:#?}", playlist);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                },
                None => HttpResponse::NotFound().finish()
            }
        },
        Err(e) => {
            tracing::error!("Cannot remove songs from playlist: {:#?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Rename Playlist")]
pub async fn rename_playlist(
    data: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    req_json: web::Json<schema::playlist::UpdatePlaylistName>,
) -> impl Responder {
    let conn = data.get_ref();
    let ext = req.extensions();

    let user_id = ext.get::<String>().unwrap();
    let playlist_id = &req_json.playlist_id;
    let new_name = &req_json.new_name;

    let dao = PlaylistDAO::new(conn.clone());

    let playlist = dao.get_playlist_by_id(playlist_id).await;

    match playlist {
        Ok(some_playlist) => {
            match some_playlist {
                Some(playlist) => {
                    if &playlist.user_id != user_id {
                        HttpResponse::Unauthorized().finish()
                    } else {
                        let res = dao.rename_playlist(&playlist, new_name).await;
                        if res.is_ok() {
                            HttpResponse::Ok().json(json!({ "playlist_id": playlist_id }))
                        } else {
                            tracing::error!("Cannot rename playlist: {:#?}", playlist);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                },
                None => HttpResponse::NotFound().finish()
            }
        },
        Err(e) => {
            tracing::error!("Cannot rename playlist: {:#?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[tracing::instrument(name = "Delete Playlist")]
pub async fn delete_playlist(
    data: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    req_json: web::Json<schema::playlist::SinglePlaylist>,
) -> impl Responder {
    let conn = data.get_ref();
    let ext = req.extensions();

    let user_id = ext.get::<String>().unwrap();
    let playlist_id = &req_json.playlist_id;

    let dao = PlaylistDAO::new(conn.clone());

    let playlist = dao.get_playlist_by_id(playlist_id).await;

    match playlist {
        Ok(some_playlist) => {
            match some_playlist {
                Some(playlist) => {
                    if &playlist.user_id != user_id {
                        HttpResponse::Unauthorized().finish()
                    } else {
                        let res = dao.delete_playlist(&playlist).await;
                        if res.is_ok() {
                            HttpResponse::Ok().json(json!({ "playlist_id": playlist_id }))
                        } else {
                            tracing::error!("Cannot delete playlist: {:#?}", playlist);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                },
                None => HttpResponse::NotFound().finish()
            }
        },
        Err(e) => {
            tracing::error!("Cannot delete playlist: {:#?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
