use actix_web::dev::WebService;
use actix_web::{web, HttpResponse, Responder};
use actix_web::{HttpRequest, Scope};
use secrecy::ExposeSecret;
use serde_json::json;

use crate::db::dao::session::SessionDAO;
use crate::db::dao::user::UserDAO;
use crate::schema;
use crate::utils;

pub fn make_service() -> Scope {
    web::scope("/users")
        .route("/createuser", web::post().to(create_user))
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout))
}

#[tracing::instrument(name = "Create User")]
async fn create_user(
    data: web::Data<sqlx::PgPool>,
    req: web::Json<schema::user::UserCreateReq>,
) -> impl Responder {
    let conn = data.get_ref();
    let username = &req.username;
    let password = &req.password;

    let user_dao = UserDAO::new(conn.clone());

    // TODO: Move this validation to a schema so that it can be used even for logging in
    if username.len() < 4 {
        HttpResponse::BadRequest()
            .json(json!({ "message": "Username must be atleast 4 character long" }))
    } else if password.expose_secret().len() < 8 {
        HttpResponse::BadRequest()
            .json(json!({ "message": "Password must be atleast 8 character long" }))
    } else {
        match user_dao.username_exists(username).await {
            Ok(true) => {
                HttpResponse::Conflict().json(json!({ "message": "Username already exists" }))
            }
            Ok(false) => {
                match user_dao
                    .create_user(username, utils::password::hash(password.clone()).await)
                    .await
                {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(_) => {
                        tracing::error!("Cannot create user for username: {}", username);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
            _ => {
                tracing::error!("Cannot create user for username: {}", username);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[tracing::instrument(name = "Login Route")]
async fn login(
    data: web::Data<sqlx::PgPool>,
    req: web::Json<schema::user::UserLoginReq>,
) -> impl Responder {
    let conn = data.get_ref();

    match UserDAO::new(conn.clone())
        .get_user_by_username(&req.username)
        .await
    {
        Ok(Some(user)) => {
            if utils::password::verify_password(user.password_hash.clone(), req.password.clone())
                .await
                .is_ok()
            {
                let session_id = utils::tokens::generate_session_key();

                match SessionDAO::new(conn.clone())
                    .create_session(&user.user_id, session_id.clone())
                    .await
                {
                    Ok(_) => {
                        HttpResponse::Ok().json(json!({ "session_id": session_id.expose_secret() }))
                    }
                    Err(_) => {
                        tracing::error!("Cannot create session for user: {}", &user.user_id);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => {
            tracing::error!("Cannot get user from db for login: {}", &req.username);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Logout Route", skip(data, req))]
async fn logout(data: web::Data<sqlx::PgPool>, req: HttpRequest) -> impl Responder {
    let session_token = match req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
    {
        Some(token) if token.len() == 256 => token.to_string(),
        _ => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let session_dao = SessionDAO::new(data.get_ref().clone());

    if session_dao
        .verify_session(&session_token)
        .await
        .unwrap_or(None)
        .is_none()
    {
        return HttpResponse::Unauthorized().finish();
    }

    if session_dao.expire_session(&session_token).await.is_err() {
        tracing::error!("Cannot expire session for logout: {}", session_token);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}
