use notify::{recommended_watcher, Event, RecursiveMode, Result, Watcher};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
};

pub fn start_watching(path: &str, shutdown: Arc<AtomicBool>) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    for res in rx {
        if shutdown.load(Ordering::Relaxed) {
            println!("Watcher received shutdown signal.");
            break;
        }

        match res {
            Ok(event) => println!("Event: {:?}", event),
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
