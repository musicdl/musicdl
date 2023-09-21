use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserLoginReq {
    pub username: String,
    pub password: Secret<String>,
}

#[derive(Deserialize, Debug)]
pub struct UserCreateReq {
    pub username: String,
    pub password: Secret<String>,
}
