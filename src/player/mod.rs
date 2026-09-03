use crate::lastfm::{PlayerState, ScrobbleManager, Track};

use std::{ptr::NonNull, sync::Arc};

use block2::RcBlock;
use objc2::rc::Retained;
use objc2_foundation::{
    NSDictionary, NSDistributedNotificationCenter, NSNotification, NSNumber, NSString, ns_string,
};

pub(crate) struct MusicListener {
    scrobbler: ScrobbleManager,
}

impl MusicListener {
    pub fn new(scrobbler: ScrobbleManager) -> Arc<MusicListener> {
        Arc::new(MusicListener { scrobbler })
    }

    pub fn start(self: &Arc<Self>) {
        println!("[i] Listening for Apple Music events via DarwinKit...");

        let nc = NSDistributedNotificationCenter::defaultCenter();

        let this = Arc::clone(self);

        let block = RcBlock::new(move |notif: NonNull<NSNotification>| {
            let notification = unsafe { notif.as_ref() };
            this.player_info_changed(notification);
        });

        let event_name: &'static NSString = ns_string!("com.apple.Music.playerInfo");
        unsafe {
            nc.addObserverForName_object_queue_usingBlock(Some(event_name), None, None, &block)
        };
    }

    fn player_info_changed(&self, notification: &NSNotification) {
        let user_info = notification.userInfo().unwrap();
        if user_info.count() == 0 {
            return;
        }

        let player_state =
            get_value_from_user_info(&user_info, "Player State", String::from("Stopped"));

        let track_name =
            get_value_from_user_info(&user_info, "Name", String::from("Unknown Title"));
        let track_artist =
            get_value_from_user_info(&user_info, "Artist", String::from("Unknown Artist"));
        let track_album = get_value_from_user_info(&user_info, "Album", String::from(""));

        let duration: f64 = user_info
            .objectForKey(&NSString::from_str("Total Time"))
            .and_then(|v| v.downcast_ref::<NSNumber>().map(|s| s.as_f64() / 1000.0))
            .unwrap_or(0.0);

        self.scrobbler.process_track(
            Track::new(track_name, track_artist, track_album, duration),
            PlayerState::from_str_or_default(player_state.as_str()),
        );
    }
}

fn get_value_from_user_info(ui: &Retained<NSDictionary>, key: &str, default: String) -> String {
    let k = NSString::from_str(key);

    ui.objectForKey(&k)
        .and_then(|v| v.downcast_ref::<NSString>().map(|s| s.to_string()))
        .unwrap_or(default)
}
