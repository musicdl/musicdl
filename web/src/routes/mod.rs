use actix_web::{web, Scope};
use sqlx::PgPool;
mod playlist;
mod song;
mod user;

pub fn make_routes(pool: &PgPool) -> Scope {
    web::scope("/api")
        .service(user::make_service())
        .service(song::make_service())
        .service(playlist::make_service())
        .app_data(web::Data::new(pool.clone()))
}
