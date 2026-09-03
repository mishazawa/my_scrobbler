static SERVICE: &str = "my-scrobbler";

pub(crate) struct ScrobbleManager {}

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
}

impl ScrobbleManager {
    pub fn new() -> ScrobbleManager {
        ScrobbleManager {}
    }

    pub fn process_track(&self, track_data: Track, player_state: PlayerState) {}

    fn finalize_track(&self) {}
}
