use config::load_watch_dir;
use pgp::load_public_key;
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use watcher::start_watching;

mod config;
mod pgp;
mod sftp;
mod watcher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = load_public_key("keys/recipient.asc")?;

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
    })?;

    println!("Watching directory: {}", watch_dir);

    let watcher_cert = cert.clone();
    let watcher_handle = std::thread::spawn(move || {
        let _ = start_watching(&watch_dir, shutdown_flag, watcher_cert);
    });

    watcher_handle.join().unwrap();

    Ok(())
}
