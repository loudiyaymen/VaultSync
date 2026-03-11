use dotenv::dotenv;
use std::env;

pub fn load_watch_dir() -> String {
    dotenv().ok();
    let val = env::var("WATCH_DIR").expect("WATCH_DIR must be set in .env");
    println!("{}", val);
    val
}
