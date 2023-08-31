use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}
