use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use middlewares::auth::AuthMiddleware;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{prelude::*, Registry};

mod db;
mod middlewares;
mod routes;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let file_appender =
        tracing_appender::rolling::hourly(std::env::current_dir()?, "logs/debug.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let debug_log = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking);

    let subscriber = Registry::default().with(stdout_log).with(debug_log);

    tracing::subscriber::set_global_default(subscriber)?;

    dotenv().expect(".env not found");

    let pg_url: Secret<String> =
        Secret::new(std::env::var("DATABASE_URL").expect("DATABASE_URL not set"));

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(pg_url.expose_secret())
        .await
        .expect("Cannot create DB pool");

    HttpServer::new(move || {
        App::new().wrap(TracingLogger::default()).service(
            web::scope("/api")
                .service(
                    web::scope("/users")
                        .route("/createuser", web::post().to(routes::user::create_user))
                        .route("/login", web::post().to(routes::user::login))
                        .route("/logout", web::get().to(routes::user::logout)),
                )
                .service(
                    web::scope("/songs")
                        .route("/search", web::post().to(routes::song::search))
                        .route("/getsong", web::post().to(routes::song::get_song)),
                )
                .service(
                    web::scope("/playlists")
                        .wrap(AuthMiddleware)
                        .route(
                            "/getalluserplaylist",
                            web::post().to(routes::playlist::get_all_playlists),
                        )
                        .route(
                            "/getsongs",
                            web::post().to(routes::playlist::get_all_playlist_songs),
                            // .route("/ping", web::get().to(ping)),
                        )
                        .route(
                            "/createempty",
                            web::post().to(routes::playlist::create_empty_playlist),
                        )
                        .route(
                            "/addsongs",
                            web::post().to(routes::playlist::add_songs_to_playlist),
                        )
                        .route(
                            "/removesongs",
                            web::post().to(routes::playlist::remove_songs_from_playlist),
                        )
                        .route("/rename", web::post().to(routes::playlist::rename_playlist))
                        .route("/delete", web::post().to(routes::playlist::delete_playlist))
                        .app_data(web::Data::new(pool.clone())),
                ),
        )
    })
    //.listen(listener)?
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
