use notify::{recommended_watcher, Event, RecursiveMode, Result, Watcher};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    time::Duration,
};

pub fn start_watching(path: &str, shutdown: Arc<AtomicBool>) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    while !shutdown.load(Ordering::Relaxed) {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Ok(event)) => println!("Event: {:?}", event),
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
