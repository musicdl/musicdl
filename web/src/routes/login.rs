use actix_web::{post, web, HttpResponse, Responder};

use crate::models;
use crate::utils;

#[post("/login")]
pub async fn login_user(req: web::Json<models::auth::LoginReq>) -> impl Responder {
    let hash = utils::password::hash(&req.password).await;
    let saved_password = "Redacted";

    if let Ok(()) = utils::password::verify_password(&hash, &saved_password).await {
        let session_id = utils::tokens::generate_session_key();
        HttpResponse::Ok().body(session_id)
    } else {
        // TODO: Add Appropriate Unauthorized Error
        HttpResponse::InternalServerError().body("Internal Server Error")
    }
}
