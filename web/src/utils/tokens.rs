use argon2::password_hash::rand_core::{OsRng, RngCore};
use hex;
use secrecy::Secret;

pub fn generate_session_key() -> Secret<String> {
    let mut buff = [0_u8; 128];
    OsRng.fill_bytes(&mut buff);
    Secret::new(hex::encode(buff))
}
