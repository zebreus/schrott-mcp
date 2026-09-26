//! Password hashing, random tokens and PKCE helpers.
//!
//! The `store` crate persists whatever these functions produce; nothing here
//! touches a database.

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rngs::OsRng, RngCore as _};
use sha2::{Digest as _, Sha256};
use thiserror::Error;

/// Every way the auth helpers can fail.
#[derive(Debug, Error)]
pub enum AuthError {
    /// Argon2 rejected the hashing operation.
    /// (`password_hash::Error` only implements `Display`, so it is
    /// rendered into the message instead of chained as a source.)
    #[error("password hashing failed: {0}")]
    Hash(argon2::password_hash::Error),
}

/// Hash a password with argon2id and a fresh random salt.
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(AuthError::Hash)
}

/// Check a password against a stored argon2 hash.
pub fn verify_password(hash: &str, password: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

/// Fresh URL-safe random token (`num_bytes` of entropy, no padding).
pub fn new_token(num_bytes: usize) -> String {
    let mut bytes = vec![0u8; num_bytes];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Fresh base62 token (`num_bytes` of entropy) for public URL ids.
/// Proper base conversion (no modulo bias), left-padded to fixed width —
/// 16 bytes become 22 chars holding the full 128 bits.
pub fn new_base62_token(num_bytes: usize) -> String {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut bytes = vec![0u8; num_bytes];
    OsRng.fill_bytes(&mut bytes);
    let mut out: Vec<char> = Vec::new();
    while bytes.iter().any(|&b| b != 0) {
        let mut rem: u32 = 0;
        for b in bytes.iter_mut() {
            let cur = (rem << 8) | u32::from(*b);
            *b = (cur / 62) as u8;
            rem = cur % 62;
        }
        out.push(ALPHABET[rem as usize] as char);
    }
    // Fixed width: ceil(bits / log2(62)); 128 bits -> 22 chars.
    let width = (num_bytes * 8).div_ceil(6);
    while out.len() < width {
        out.push('0');
    }
    out.iter().rev().collect()
}

/// Lowercase hex SHA-256 of the input (used to store token hashes).
pub fn sha256_hex(input: &str) -> String {
    let digest = Sha256::digest(input.as_bytes());
    format!("{digest:x}")
}

/// Verify a PKCE `S256` code verifier against its challenge.
pub fn verify_pkce_s256(verifier: &str, challenge: &str) -> bool {
    let digest = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(digest) == challenge
}

#[cfg(test)]
mod tests {
    use super::{new_token, sha256_hex, verify_password, verify_pkce_s256};

    #[test]
    fn pkce_round_trip_and_mismatch() {
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        // Challenge from RFC 7636 appendix B.
        let challenge = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
        assert!(verify_pkce_s256(verifier, challenge));
        assert!(!verify_pkce_s256("wrong-verifier", challenge));
        assert!(!verify_pkce_s256(verifier, "wrong-challenge"));
    }

    #[test]
    fn password_hash_verifies_and_rejects() {
        let hash = super::hash_password("correct horse").expect("hashes");
        assert!(verify_password(&hash, "correct horse"));
        assert!(!verify_password(&hash, "wrong horse"));
        assert!(!verify_password("not-a-hash", "correct horse"));
    }

    #[test]
    fn tokens_are_unique_and_hash_stable() {
        assert_ne!(new_token(32), new_token(32));
        assert_eq!(sha256_hex("abc"), sha256_hex("abc"));
        assert_eq!(new_token(0), String::new());
    }

    #[test]
    fn base62_tokens_are_url_ids() {
        use super::new_base62_token;
        // 128 bits -> 22 base62 chars.
        let id = new_base62_token(16);
        assert_eq!(id.len(), 22);
        assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));
        assert_ne!(id, new_base62_token(16));
    }
}
