use config::{load_encryption_key, load_watch_dir};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use watcher::start_watching;
use zeroize::Zeroize;
mod config;
mod encryptor;
mod sftp;
mod watcher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let watch_dir = load_watch_dir();
    if !Path::new(&watch_dir).exists() {
        eprintln!("WATCH_DIR '{}' does not exist.", watch_dir);
        std::process::exit(1);
    }
    let shutdown_flag = Arc::new(AtomicBool::new(false));

    let shutdown_handle = shutdown_flag.clone();
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C. Shutting down...");
        shutdown_handle.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl+C handler");

    let mut key = load_encryption_key();

    let watcher_key = key.clone();
    println!("Watching directory: {}", watch_dir);

    let watcher_handle = std::thread::spawn(move || {
        let _ = start_watching(&watch_dir, shutdown_flag, watcher_key);
    });
    key.zeroize();
    watcher_handle.join().unwrap();
    Ok(())
}
