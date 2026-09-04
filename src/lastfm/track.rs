use std::{fmt, time::Duration};

use rustfm_scrobble::Scrobble;

// seconds
static MIN_DURATION: f64 = 30.0;
static MAX_DURATION: f64 = 240.0;

#[derive(Debug)]
pub struct Track {
    name: String,
    artist: String,
    album: String,
    duration: f64,
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

    pub fn is_even_scrobblable(&self) -> bool {
        self.duration > MIN_DURATION
    }

    pub fn get_threshold(&self) -> Duration {
        // 50% of track length vs 4 minutes maximum threshold
        let half_duration = self.duration / 2.0;
        Duration::from_secs_f64(half_duration.min(MAX_DURATION))
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.artist == other.artist
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.artist, self.name)
    }
}
