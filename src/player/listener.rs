use std::{
    ptr::NonNull,
    sync::{Arc, Mutex},
};

use block2::RcBlock;
use objc2_foundation::{NSDistributedNotificationCenter, NSNotification, NSString, ns_string};

use crate::{
    lastfm::{PlayerState, ScrobbleManager, Track},
    player::util,
};
pub struct MusicListener {
    scrobbler: Mutex<ScrobbleManager>,
}

impl MusicListener {
    pub fn new(scrobbler: ScrobbleManager) -> Arc<MusicListener> {
        Arc::new(MusicListener {
            scrobbler: scrobbler.into(),
        })
    }

    pub fn start(self: &Arc<Self>) {
        println!("[i] Listening for Apple Music events...");

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
        let Some(user_info) = notification.userInfo() else {
            return;
        };

        if user_info.count() == 0 {
            return;
        }

        let player_state =
            util::get_value_from_user_info(&user_info, "Player State", String::from("Stopped"));

        let track_name =
            util::get_value_from_user_info(&user_info, "Name", String::from("Unknown Title"));
        let track_artist =
            util::get_value_from_user_info(&user_info, "Artist", String::from("Unknown Artist"));
        let track_album = util::get_value_from_user_info(&user_info, "Album", String::from(""));

        let duration: f64 = util::get_value_from_user_info(&user_info, "Total Time", 0.0) / 1000.0;

        if let Ok(mut scrobbler) = self.scrobbler.lock() {
            scrobbler.process_track(
                Track::new(track_name, track_artist, track_album, duration),
                PlayerState::from_str_or_default(player_state.as_str()),
            );
        }
    }
}
