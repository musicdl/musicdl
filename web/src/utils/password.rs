use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use secrecy::{ExposeSecret, Secret};

#[tracing::instrument(name = "Hashing user password", skip(password))]
pub async fn hash(password: Secret<String>) -> Secret<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed = Argon2::default()
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .expect("Unable to hash password.")
        .to_string();

    Secret::new(hashed)
}

#[tracing::instrument(name = "Verifying user password", skip(password, hash))]
pub async fn verify_password(
    hash: Secret<String>,
    password: Secret<String>,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash.expose_secret())?;
    Argon2::default().verify_password(password.expose_secret().as_bytes(), &parsed_hash)
}
