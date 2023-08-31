use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[tracing::instrument(name = "Hashing user password", skip(password))]
pub async fn hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Unable to hash password.")
        .to_string()
}

#[tracing::instrument(name = "Verifying user password", skip(password, hash))]
pub async fn verify_password(
    hash: &str,
    password: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}
