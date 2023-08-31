use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod utils;
mod routes;
mod models;

#[get("/search")]
async fn search_songs() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[post("/get")]
async fn get_song() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[post("/create")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("User Created")
}

#[get("/get")]
async fn get_user_playlist() -> impl Responder {
    // Works 🗿
    let session_id = utils::tokens::generate_session_key();
    HttpResponse::Ok().body(session_id)
}

#[post("/create")]
async fn create_user_playlist() -> impl Responder {
    HttpResponse::Ok().body("Playlist Created")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/songs").service(search_songs).service(get_song))
            .service(
                web::scope("/users")
                    .service(create_user)
                    .service(routes::login::login_user)
                    .service(
                        web::scope("/playlists")
                            .service(get_user_playlist)
                            .service(create_user_playlist),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
