use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use tracing;

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    tracing::debug!("Verifying password against hash");
    let result = verify(password, hash);
    tracing::debug!("Password verification result: {:?}", result);
    result
}
