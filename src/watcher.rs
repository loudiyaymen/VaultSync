use crate::{config, pgp::encrypt_file_with_pgp, sftp::upload_file_with_retry};

use notify::{
    event::{EventKind, ModifyKind},
    recommended_watcher, Event, RecursiveMode, Result, Watcher,
};
use sequoia_openpgp::Cert;

use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    time::Duration,
};

pub fn start_watching(path: &str, shutdown: Arc<AtomicBool>, cert: Cert) -> Result<()> {
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
                            handle_file(&path, &cert);
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

fn handle_file(path: &PathBuf, cert: &Cert) {
    let path_str = path.to_string_lossy();

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if ext == "pgp" {
            return;
        }
    }

    println!("Processing: {:?}", path_str);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let output_path = config::encrypted_output_dir().join(format!("{}.pgp", file_name));

    if let Err(e) = fs::create_dir_all(config::encrypted_output_dir()) {
        eprintln!("Failed to create encrypted output dir: {:?}", e);
        return;
    }

    if let Err(e) = encrypt_file_with_pgp(&path_str, cert) {
        eprintln!("PGP encryption failed: {:?}", e);
        return;
    }

    if !output_path.exists() {
        eprintln!("Encrypted file not found: {}", output_path.display());
        return;
    }

    let (retry_count, backoff_ms) = config::load_sftp_retry_config();
    match upload_file_with_retry(output_path.to_str().unwrap(), retry_count, backoff_ms) {
        Ok(_) => {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("Failed to delete original file: {} — {}", path_str, e);
            } else {
                println!("Deleted original: {}", path_str);
            }
        }
        Err(e) => {
            eprintln!("Upload failed: {:?}", e);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use aes_gcm::{Aes256Gcm, KeyInit};
//     use std::sync::{
//         atomic::{AtomicBool, Ordering},
//         Arc,
//     };
//     use std::time::Duration;
//     use tempfile::tempdir;

//     use crate::watcher::start_watching;

//     #[test]
//     fn test_start_watching_does_not_crash() {
//         let dir = tempdir().expect("couldn't make tempdir");
//         let shutdown = Arc::new(AtomicBool::new(false));
//         let watch_dir = dir.path().to_path_buf();
//         let shutdown_clone = shutdown.clone();

//         std::thread::spawn(move || {
//             std::thread::sleep(Duration::from_secs(2));
//             shutdown_clone.store(true, Ordering::Relaxed);
//         });

//         let result = start_watching(watch_dir.to_str().unwrap(), shutdown,&);
//         assert!(result.is_ok());
//     }
// }
