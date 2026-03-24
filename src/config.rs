use aes_gcm::Aes256Gcm;
use aes_gcm::Key;
use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;
pub fn load_watch_dir() -> String {
    dotenv().ok();
    let val = env::var("WATCH_DIR").expect("WATCH_DIR must be set in .env");
    println!("{}", val);
    val
}
#[allow(dead_code)]
pub fn load_encryption_key() -> Key<Aes256Gcm> {
    let key_b64 = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not set in .env");
    let key_bytes = general_purpose::STANDARD
        .decode(&key_b64)
        .expect("Failed to decode base64 ENCRYPTION_KEY");

    if key_bytes.len() != 32 {
        panic!("ENCRYPTION_KEY must decode to exactly 32 bytes");
    }

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    Key::<Aes256Gcm>::clone_from_slice(key)
}
pub fn load_sftp_retry_config() -> (u32, u64) {
    let retry_count = env::var("SFTP_RETRY")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(3);

    let backoff_ms = env::var("SFTP_RETRY_BACKOFF_MS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(1000);

    (retry_count, backoff_ms)
}
#[allow(dead_code)]
pub fn encrypted_output_dir() -> PathBuf {
    env::var("ENCRYPTED_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("encrypted"))
}
#[allow(dead_code)]
pub fn decrypted_output_dir() -> PathBuf {
    env::var("DECRYPTED_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("decrypted"))
}
// #[allow(dead_code)]

pub fn pgp_public_key_path() -> String {
    env::var("PGP_PUBLIC_KEY").expect("PGP_PUBLIC_KEY must be set in .env")
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;

    use crate::config::{decrypted_output_dir, encrypted_output_dir};

    #[test]
    fn test_encrypted_output_dir_env_override() {
        env::set_var("ENCRYPTED_DIR", "encrypted");
        assert_eq!(encrypted_output_dir(), PathBuf::from("encrypted"));
    }

    #[test]
    fn test_decrypted_output_dir_env_override() {
        env::set_var("DECRYPTED_DIR", "decrypted");
        assert_eq!(decrypted_output_dir(), PathBuf::from("decrypted"));
    }
}
