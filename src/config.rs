#![allow(dead_code)]
use aes_gcm::Aes256Gcm;
use aes_gcm::Key;
use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
pub fn load_watch_dir() -> String {
    dotenv().ok();
    let val = env::var("WATCH_DIR").expect("WATCH_DIR must be set in .env");
    println!("{}", val);
    val
}
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
pub fn encrypted_output_dir() -> PathBuf {
    env::var("ENCRYPTED_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("encrypted"))
}
pub fn decrypted_output_dir() -> PathBuf {
    env::var("DECRYPTED_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("decrypted"))
}

pub fn pgp_public_key_path() -> String {
    env::var("PGP_PUBLIC_KEY").expect("PGP_PUBLIC_KEY must be set in .env")
}
#[cfg(target_os = "windows")]
pub fn setup_autostart() {
    setup_autostart_windows();
}

#[cfg(target_os = "macos")]
pub fn setup_autostart() {
    setup_autostart_macos();
}

#[cfg(target_os = "linux")]
pub fn setup_autostart() {
    setup_autostart_linux();
}

fn setup_autostart_windows() {
    if let Some(home_dir) = dirs::home_dir() {
        let startup_dir =
            home_dir.join("AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup");
        let dest = startup_dir.join("VaultSync.lnk");

        if !dest.exists() {
            let _ = fs::copy("autostart/VaultSync.lnk", &dest);
            println!("VaultSync autostart shortcut added to Windows startup folder.");
        }
    }
}

fn setup_autostart_macos() {
    if let Some(home_dir) = dirs::home_dir() {
        let launch_dir = home_dir.join("Library/LaunchAgents");
        fs::create_dir_all(&launch_dir).ok();
        let dest = launch_dir.join("com.vaultsync.autostart.plist");

        if !dest.exists() {
            let _ = fs::copy("autostart/com.vaultsync.autostart.plist", &dest);
            Command::new("launchctl")
                .arg("load")
                .arg(&dest)
                .output()
                .ok();
            println!("VaultSync plist loaded into macOS LaunchAgents.");
        }
    }
}

fn setup_autostart_linux() {
    if let Some(home_dir) = dirs::home_dir() {
        let autostart_dir = home_dir.join(".config/systemd/user");
        fs::create_dir_all(&autostart_dir).ok();
        let dest = autostart_dir.join("vaultsync.service");

        if !dest.exists() {
            let _ = fs::copy("autostart/vaultsync.service", &dest);
            Command::new("systemctl")
                .arg("--user")
                .arg("enable")
                .arg("vaultsync.service")
                .output()
                .ok();
            println!("VaultSync service enabled in systemd user mode.");
        }
    }
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
