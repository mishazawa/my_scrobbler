static SERVICE: &str = "my-scrobbler";

pub(crate) struct ScrobbleManager {
    track: Option<Track>,
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
}

impl ScrobbleManager {
    pub fn new() -> ScrobbleManager {
        ScrobbleManager { track: None }
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

    fn finalize_track(&self) {}

    fn is_same(&self, track_data: &Track) -> bool {
        match &self.track {
            Some(t) => t.name == track_data.name && t.artist == track_data.artist,
            None => false,
        }
    }

    fn update_player_state(&self, player_state: PlayerState) {}

    fn start_new_track(&mut self, track_data: Track, player_state: PlayerState) {
        self.track = Some(track_data);
    }
}
