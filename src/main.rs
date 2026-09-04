use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;

mod lastfm;
mod player;
mod utils;

use lastfm::ScrobbleManager;
use player::MusicListener;
use utils::parse_config;

fn main() {
    let creds = match parse_config() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[e] {e}");
            std::process::exit(1);
        }
    };

    println!("[i] Starting my_scrobbler...");

    let manager = match ScrobbleManager::new(creds) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("[e] {e}");
            std::process::exit(1);
        }
    };

    MusicListener::new(manager).start();

    let mtm = MainThreadMarker::new().expect("[e] Must be on the main thread");
    let app = NSApplication::sharedApplication(mtm);

    app.run();
}
