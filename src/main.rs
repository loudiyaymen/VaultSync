use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use config::load_watch_dir;
use encryptor::{decrypt_file, encrypt_file};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use watcher::start_watching;

mod config;
mod encryptor;
mod watcher;

fn main() -> std::io::Result<()> {
    load_watch_dir();
    let shutdown_flag = Arc::new(AtomicBool::new(false));

    let shutdown_handle = shutdown_flag.clone();
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C. Shutting down...");
        shutdown_handle.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl+C handler");

    let watcher_flag = shutdown_flag.clone();
    let watcher_handle = std::thread::spawn(move || {
        let _ = start_watching(".", watcher_flag);
    });

    let key = Aes256Gcm::generate_key(OsRng);
    encrypt_file("test.png", &key)?;
    decrypt_file("test.vault", &key);

    watcher_handle.join().unwrap();
    Ok(())
}
