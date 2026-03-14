use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use config::load_watch_dir;
use encryptor::{decrypt_file, encrypt_file};
use watcher::start_watching;

mod config;
mod encryptor;
mod watcher;

fn main() -> std::io::Result<()> {
    load_watch_dir();
    let watcher_handle = std::thread::spawn(|| {
        let _ = start_watching(".");
    });
    let key = Aes256Gcm::generate_key(OsRng);
    encrypt_file("test.txt", "encrypted_test.vault", &key)?;
    decrypt_file("encrypted_test.vault", &key);
    watcher_handle.join().unwrap();
    Ok(())
}
