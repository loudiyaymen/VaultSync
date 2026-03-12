use config::load_watch_dir;
use watcher::start_watching;
mod config;
mod watcher;
fn main() {
    println!("Hello, world!");
    load_watch_dir();
    let _ = start_watching(".");
}
