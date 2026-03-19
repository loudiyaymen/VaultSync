use crate::sftp::upload_file_with_retry;
use crate::{config, encryptor::encrypt_file};

use aes_gcm::{Aes256Gcm, Key};

use notify::{
    event::{EventKind, ModifyKind},
    recommended_watcher, Event, RecursiveMode, Result, Watcher,
};

use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    time::Duration,
};

pub fn start_watching(path: &str, shutdown: Arc<AtomicBool>, key: Key<Aes256Gcm>) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    while !shutdown.load(Ordering::Relaxed) {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Ok(event)) => {
                println!("Raw Event: {:?}", event);

                if matches!(
                    event.kind,
                    EventKind::Create(_)
                        | EventKind::Modify(ModifyKind::Data(_))
                        | EventKind::Modify(ModifyKind::Name(_))
                ) {
                    for path in event.paths {
                        if should_process(&path) {
                            println!("Processing: {:?}", path);
                            handle_file(&path, &key);
                        }
                    }
                }
            }
            Ok(Err(e)) => println!("Watch error: {:?}", e),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(e) => {
                println!("Channel error: {:?}", e);
                break;
            }
        }
    }

    println!("Watcher shutting down.");
    Ok(())
}

fn should_process(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        ext != "vault"
    } else {
        true
    }
}

fn handle_file(path: &PathBuf, key: &Key<Aes256Gcm>) {
    let path_str = path.to_string_lossy();

    if let Err(e) = encrypt_file(&path_str, key) {
        eprintln!("Encryption failed: {:?}", e);
        return;
    }

    let vault_path = format!("{}.vault", path_str);
    let (retry_count, backoff_ms) = config::load_sftp_retry_config();
    let upload_result = upload_file_with_retry(&vault_path, retry_count, backoff_ms);
    if let Err(e) = upload_result {
        eprintln!("Upload failed: {:?}", e);
    }

    match fs::remove_file(path) {
        Ok(_) => println!("Deleted original: {}", path_str),
        Err(e) => eprintln!("Failed to delete original file: {} — {}", path_str, e),
    }
}
#[cfg(test)]
mod tests {
    use aes_gcm::{Aes256Gcm, KeyInit};
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    use std::time::Duration;
    use tempfile::tempdir;

    use crate::watcher::start_watching;

    #[test]
    fn test_start_watching_does_not_crash() {
        let dir = tempdir().expect("couldn't make tempdir");
        let shutdown = Arc::new(AtomicBool::new(false));
        let key = Aes256Gcm::generate_key(aes_gcm::aead::OsRng);

        let watch_dir = dir.path().to_path_buf();
        let shutdown_clone = shutdown.clone();

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(2));
            shutdown_clone.store(true, Ordering::Relaxed);
        });

        let result = start_watching(watch_dir.to_str().unwrap(), shutdown, key);
        assert!(result.is_ok());
    }
}
