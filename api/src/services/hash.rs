use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::err::AppErr;

pub fn argon2_hash(password: &str) -> Result<String, AppErr> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppErr::HashPasswordError)?
        .to_string())
}

pub fn argon2_verify(password: &str, password_hash: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(password_hash) {
        return Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
    }
    false
}
