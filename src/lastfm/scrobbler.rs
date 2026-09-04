use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use keyring::Entry;
use rustfm_scrobble::Scrobbler;

use crate::lastfm::ScrobbleManagerError;
use crate::lastfm::Track;

#[derive(PartialEq, Eq)]
pub enum PlayerState {
    Stopped,
    Playing,
    Paused,
}

impl PlayerState {
    pub fn from_str_or_default(s: &str) -> Self {
        match s {
            "Stopped" => PlayerState::Stopped,
            "Playing" => PlayerState::Playing,
            "Paused" => PlayerState::Paused,
            _ => PlayerState::Stopped,
        }
    }
}

pub struct ScrobbleManager {
    track: Option<Track>,
    has_scrobbled: bool,
    is_playing: bool,
    time_started: Instant,
    last_play_ts: Instant,
    accumulated_time: Duration,
    api: Scrobbler,
}

static SERVICE: &str = "my-scrobbler";

impl ScrobbleManager {
    pub fn new(
        credentials: HashMap<String, String>,
    ) -> Result<ScrobbleManager, ScrobbleManagerError> {
        let api_key = credentials
            .get("LASTFM_API_KEY")
            .ok_or(ScrobbleManagerError::MissingCredential("LASTFM_API_KEY"))?;
        let api_secret = credentials
            .get("LASTFM_API_SECRET")
            .ok_or(ScrobbleManagerError::MissingCredential("LASTFM_API_SECRET"))?;

        let username = credentials
            .get("LASTFM_USERNAME")
            .ok_or(ScrobbleManagerError::MissingCredential("LASTFM_USERNAME"))?;

        let mut scrobbler = Scrobbler::new(api_key, api_secret);

        let password = Entry::new(SERVICE, username)?.get_password()?;

        scrobbler
            .authenticate_with_password(username, &password)
            .map_err(ScrobbleManagerError::ScrobblerError)?;

        Ok(ScrobbleManager {
            track: None,
            has_scrobbled: false,
            is_playing: false,
            accumulated_time: Duration::default(),
            time_started: Instant::now(),
            last_play_ts: Instant::now(),
            api: scrobbler,
        })
    }

    pub fn process_track(&mut self, track_data: Track, player_state: PlayerState) {
        match player_state {
            PlayerState::Stopped => self.finalize_track(),
            _ => {
                if self.track.as_ref() == Some(&track_data) {
                    self.update_player_state(player_state);
                } else {
                    self.finalize_track();
                    self.start_new_track(track_data, player_state);
                }
            }
        };
    }

    fn finalize_track(&mut self) {
        if self.track.is_none() || self.has_scrobbled {
            return;
        }

        self.add_accumulated_time();

        if let Some(t) = &self.track {
            if self.is_threshold_passed() {
                match self.api.scrobble(&t.to_scrobble()) {
                    Ok(_) => {
                        self.has_scrobbled = true;
                        println!("[i] Scrobbling API: {}", t);
                    }
                    Err(err) => println!("[e] Scrobbling API: {}", err),
                };
            } else {
                let rounded =
                    Duration::from_secs(self.accumulated_time.as_secs_f64().round() as u64);
                println!("[i] Skipped: Listened for {:?}.", rounded)
            }

            self.track = None;
            self.is_playing = false;
        }
    }

    fn update_player_state(&mut self, player_state: PlayerState) {
        match (player_state, self.is_playing) {
            (PlayerState::Playing, false) => {
                self.is_playing = true;
                self.last_play_ts = Instant::now();
                println!("[i] Resumed");
            }
            (PlayerState::Paused, true) => {
                self.add_accumulated_time();
                self.is_playing = false;
                println!("[i] Paused. Accumulated time: {:?}", self.accumulated_time);
            }
            _ => {}
        };
    }

    fn start_new_track(&mut self, track_data: Track, player_state: PlayerState) {
        self.track = Some(track_data);
        self.has_scrobbled = false;
        self.accumulated_time = Duration::default();
        self.time_started = Instant::now();

        if player_state == PlayerState::Playing {
            self.is_playing = true;
            self.last_play_ts = Instant::now();

            if let Some(t) = self.track.as_ref() {
                match self.api.now_playing(&t.to_scrobble()) {
                    Ok(_) => println!("[i] Now Playing: {}", t),
                    Err(err) => println!("[e] Scrobbling API: {}", err),
                };
            }
        }
    }

    fn add_accumulated_time(&mut self) {
        if self.is_playing {
            self.accumulated_time += self.last_play_ts.elapsed();
        }
    }

    fn is_threshold_passed(&self) -> bool {
        self.track
            .as_ref()
            .is_some_and(|t| t.is_even_scrobblable() && self.accumulated_time >= t.get_threshold())
    }
}
