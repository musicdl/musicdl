use argon2::password_hash::rand_core::{OsRng, RngCore};
use hex;

pub fn generate_session_key() -> String {
    let mut buff = [0_u8; 128];
    OsRng.fill_bytes(&mut buff);
    hex::encode(buff)
}
