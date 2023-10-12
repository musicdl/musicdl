use actix_web::{App, HttpServer};
use color_eyre::Result;
use dotenvy::dotenv;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{prelude::*, Registry};
use actix_cors::Cors;

mod db;
mod middlewares;
mod routes;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

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
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .service(routes::make_routes(&pool))
    })
    //.listen(listener)?
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
