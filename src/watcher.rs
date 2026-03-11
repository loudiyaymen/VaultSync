use std::{path::Path, sync::mpsc};

use notify::{recommended_watcher, Event, RecursiveMode, Result, Watcher};
pub fn start_watching(path: &str) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
    for res in rx {
        match res {
            Ok(event) => println!("Event:{:?}", event),
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
    Ok(())
}
