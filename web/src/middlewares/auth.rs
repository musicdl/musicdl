use crate::db::dao::session::SessionDAO;
use actix_service::Service;
use actix_web::{
    body::BoxBody,
    dev::{forward_ready, ServiceRequest, ServiceResponse},
    web, Error, HttpMessage, HttpResponse,
};
use futures::future::{ok, Ready};
use std::pin::Pin;
use std::sync::Arc;

pub async fn get_authorized_user(req: &ServiceRequest) -> Result<Option<String>, anyhow::Error> {
    let pool = req.app_data::<web::Data<sqlx::PgPool>>().unwrap().as_ref();

    let session_token = match req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
    {
        Some(token) => token.to_string(),
        None => return Ok(None),
    };

    if session_token.len() != 256 {
        return Ok(None);
    }

    let session_dao = SessionDAO::new(pool.clone());
    let user_id = session_dao.verify_session(&session_token).await?;
    Ok(user_id)
}

pub struct AuthMiddleware;

impl<S> actix_service::Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareWrapper<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let service = Arc::new(service);
        ok(AuthMiddlewareWrapper { service })
    }
}

pub struct AuthMiddlewareWrapper<S> {
    service: Arc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareWrapper<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Arc::clone(&self.service);

        Box::pin(async move {
            let user_id = get_authorized_user(&req).await;
            if let Ok(authorized) = user_id {
                if let Some(uid) = authorized {
                    req.extensions_mut().insert(uid);
                    Ok(service.call(req).await?.map_into_boxed_body())
                } else {
                    let resp = HttpResponse::Unauthorized().finish();
                    Ok(req.into_response(resp).map_into_boxed_body())
                }
            } else {
                let resp = HttpResponse::InternalServerError().finish();
                Ok(req.into_response(resp).map_into_boxed_body())
            }
        })
    }
}
