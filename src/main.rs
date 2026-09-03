use dotenv;
use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;
use std::env;

mod lastfm;
mod player;

use lastfm::ScrobbleManager;

use crate::player::MusicListener;

fn main() {
    let my_path = env::home_dir()
        .and_then(|a| Some(a.join(".config").join("my_scrobbler").join("config.env")))
        .unwrap();

    // TODO: just read file?
    dotenv::from_path(my_path.as_path()).expect("[e] Error: Could not load .env config file.");

    println!("[i] Starting my_scrobbler...");

    let sm = ScrobbleManager::new().expect("[e] Can't init scrobbler. Check keyring.");

    let listener = MusicListener::new(sm);

    listener.start();

    let mtm = MainThreadMarker::new().expect("[e] Must be on the main thread");
    let app = NSApplication::sharedApplication(mtm);

    app.run();
}
