use std::{
    env,
    time::{Duration, Instant},
};

use keyring::Entry;
use rustfm_scrobble::{Scrobble, Scrobbler};

static SERVICE: &str = "my-scrobbler";

// seconds
static MIN_DURATION: f64 = 30.0;
static MAX_DURATION: f64 = 240.0;

pub(crate) struct ScrobbleManager {
    track: Option<Track>,
    has_scrobbled: bool,
    is_playing: bool,
    time_started: Instant,
    last_play_ts: Instant,
    accumulated_time: Duration,
    api: Scrobbler,
}

#[derive(Debug)]
pub(crate) struct Track {
    name: String,
    artist: String,
    album: String,
    duration: f64,
}

pub(crate) enum PlayerState {
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

impl Track {
    pub fn new(
        track_name: String,
        track_artist: String,
        track_album: String,
        duration: f64,
    ) -> Track {
        Track {
            name: track_name,
            artist: track_artist,
            album: track_album,
            duration: duration,
        }
    }
    pub fn to_scrobble(&self) -> Scrobble {
        Scrobble::new(&self.artist, &self.name, &self.album)
    }
}

impl ScrobbleManager {
    // TODO: cleanup this mess!
    pub fn new() -> Option<ScrobbleManager> {
        let api_key = env::var("LASTFM_API_KEY").unwrap();
        let api_secret = env::var("LASTFM_API_SECRET").unwrap();
        let username = env::var("LASTFM_USERNAME").unwrap();

        let mut scrobbler = Scrobbler::new(&api_key, &api_secret);

        if let Ok(password) = Entry::new(SERVICE, &username).and_then(|entry| entry.get_password())
        {
            return scrobbler
                .authenticate_with_password(&username, &password)
                .and_then(|_| {
                    return Ok(Some(ScrobbleManager {
                        track: None,
                        has_scrobbled: false,
                        is_playing: false,
                        accumulated_time: Duration::default(),
                        time_started: Instant::now(),
                        last_play_ts: Instant::now(),
                        api: scrobbler,
                    }));
                })
                .unwrap();
        }

        None
    }

    pub fn process_track(&mut self, track_data: Track, player_state: PlayerState) {
        match player_state {
            PlayerState::Stopped => self.finalize_track(),
            _ => {
                let is_the_same_track = self.is_same(&track_data);

                if is_the_same_track {
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
                        println!("[i] SCROBBLED: {} - {}", t.artist, t.name);
                    }
                    Err(err) => println!("{}", err),
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

    fn is_same(&self, track_data: &Track) -> bool {
        match &self.track {
            Some(t) => t.name == track_data.name && t.artist == track_data.artist,
            None => false,
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

        match player_state {
            PlayerState::Playing => {
                self.is_playing = true;
                self.last_play_ts = Instant::now();

                if let Some(t) = self.track.as_ref() {
                    match self.api.now_playing(&t.to_scrobble()) {
                        Ok(_) => {
                            println!("[i] Now Playing: {} - {}", t.artist, t.name);
                        }
                        Err(err) => println!("{}", err),
                    };
                }
            }
            _ => {}
        };
    }

    fn add_accumulated_time(&mut self) {
        if self.is_playing {
            self.accumulated_time += self.last_play_ts.elapsed();
        }
    }

    fn is_threshold_passed(&self) -> bool {
        self.track
            .as_ref()
            .and_then(|t| {
                if t.duration <= MIN_DURATION {
                    return Some(false);
                }

                // 50% of track length vs 4 minutes maximum threshold
                let half_duration = t.duration / 2.0;
                let threshold = Duration::from_secs_f64(half_duration.min(MAX_DURATION));

                return Some(self.accumulated_time >= threshold);
            })
            .unwrap()
    }
}
