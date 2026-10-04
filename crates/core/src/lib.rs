//! Shared configuration, validation and queriable data-model types.
//!
//! Love is the secret ingredient.

use std::{net::SocketAddr, path::PathBuf};

use thiserror::Error;

/// Every way the shared core layer can fail.
#[derive(Debug, Error)]
pub enum CoreError {
    /// `--bind` / `BIND` did not parse as a socket address.
    #[error("invalid bind address '{raw}': {source}")]
    BadBind {
        raw: String,
        #[source]
        source: std::net::AddrParseError,
    },
    /// The base URL ended up empty after trimming slashes.
    #[error("base URL must not be empty")]
    EmptyBaseUrl,
    /// A username broke the rules; the message is user-facing.
    #[error("{0}")]
    BadUsername(String),
    /// A password broke the rules; the message is user-facing.
    #[error("{0}")]
    BadPassword(String),
}

/// Runtime configuration, sourced from CLI flags with env fallback.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Address to listen on, e.g. `127.0.0.1:4001`.
    pub bind: SocketAddr,
    /// Directory holding `internal.db` and `public.db`.
    pub data_dir: PathBuf,
    /// Public origin, e.g. `https://schrottindex.de` (no trailing slash).
    pub base_url: String,
}

impl AppConfig {
    /// Build config from explicit values, falling back to
    /// `BIND` / `DATA_DIR` / `BASE_URL` env vars, then to defaults.
    pub fn from_parts(
        bind: Option<String>,
        data_dir: Option<String>,
        base_url: Option<String>,
    ) -> Result<Self, CoreError> {
        fn pick(opt: Option<String>, key: &str, default: &str) -> String {
            opt.or_else(|| std::env::var(key).ok())
                .unwrap_or_else(|| default.to_owned())
        }
        let bind_raw = pick(bind, "BIND", "127.0.0.1:4001");
        let bind: SocketAddr = bind_raw.parse().map_err(|source| CoreError::BadBind {
            raw: bind_raw.clone(),
            source,
        })?;
        let data_dir = PathBuf::from(pick(data_dir, "DATA_DIR", "./data"));
        let mut base_url = pick(base_url, "BASE_URL", "http://localhost:4001");
        while base_url.ends_with('/') {
            base_url.pop();
        }
        if base_url.is_empty() {
            return Err(CoreError::EmptyBaseUrl);
        }
        Ok(Self {
            bind,
            data_dir,
            base_url,
        })
    }
}

/// Whole-corpus counters.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    pub traders: i64,
    pub materials: i64,
    pub prices: i64,
}

/// Usernames: 3-32 chars, alphanumeric plus `_` and `-`.
pub fn validate_username(name: &str) -> Result<(), CoreError> {
    let len = name.chars().count();
    if !(3..=32).contains(&len) {
        return Err(CoreError::BadUsername(
            "Benutzername muss 3–32 Zeichen lang sein".to_owned(),
        ));
    }
    let ok = name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if !ok {
        return Err(CoreError::BadUsername(
            "Benutzername darf nur Buchstaben, Ziffern, '_' und '-' enthalten".to_owned(),
        ));
    }
    Ok(())
}

/// Passwords: at least 8 characters. Nothing else is required.
pub fn validate_password(password: &str) -> Result<(), CoreError> {
    if password.chars().count() < 8 {
        return Err(CoreError::BadPassword(
            "Passwort muss mindestens 8 Zeichen lang sein".to_owned(),
        ));
    }
    Ok(())
}

/// Tiny percent-encoder for query values and URL parts.
pub fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{validate_password, validate_username};

    #[test]
    fn username_edges() {
        assert!(validate_username("ab").is_err());
        assert!(validate_username("abc").is_ok());
        assert!(validate_username(&"a".repeat(32)).is_ok());
        assert!(validate_username(&"a".repeat(33)).is_err());
        assert!(validate_username("has space").is_err());
        assert!(validate_username("semi;colon").is_err());
        assert!(validate_username("under_score-9").is_ok());
    }

    #[test]
    fn password_edges() {
        assert!(validate_password("short").is_err());
        assert!(validate_password("exactly8").is_ok());
        assert!(validate_password("").is_err());
    }

    #[test]
    fn url_encode_basics() {
        assert_eq!(super::url_encode("abc-_.~"), "abc-_.~");
        assert_eq!(super::url_encode("a b+c"), "a%20b%2Bc");
    }
}
