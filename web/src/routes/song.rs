use actix_web::{web, HttpResponse, Responder, Scope};
use serde_json::json;

use crate::{db::dao::song::SongsDAO, schema};

pub fn make_service() -> Scope {
    web::scope("/songs")
        .route("/search", web::post().to(search))
        .route("/getsong", web::post().to(get_song))
}

#[tracing::instrument(name = "Search Songs")]
async fn search(req: web::Json<schema::song::SongSearch>) -> impl Responder {
    let search_res = common::search(&req.query).await;

    match search_res {
        Ok(search_results) => HttpResponse::Ok().json(json!({ "search_results": search_results})),
        Err(e) => {
            tracing::error!(e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Get Song")]
async fn get_song(
    data: web::Data<sqlx::PgPool>,
    req: web::Json<schema::song::SongGet>,
) -> impl Responder {
    let conn = data.get_ref();

    let songs_dao = SongsDAO::new(conn.clone());
    let song_id = &req.song_id;

    match songs_dao.get_or_fetch_song(song_id).await {
        Ok(maybe_song) => {
            if let Some(song) = maybe_song {
                HttpResponse::Ok().json(json!({ "song": song }))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => {
            tracing::error!("Error in fetching song {}", song_id);
            HttpResponse::InternalServerError().finish()
        }
    }
}
