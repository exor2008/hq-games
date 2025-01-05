use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub(crate) fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub(crate) fn verify_password(hashed_password: &str, hash: &str) -> bool {
    let argon = Argon2::default();
    let hash = PasswordHash::new(hash).unwrap();
    argon
        .verify_password(hashed_password.as_bytes(), &hash)
        .is_ok()
}
