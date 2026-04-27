import os
import time
import pylast


class ScrobbleManager:
    def __init__(self):
        """Initialize the Last.fm network connection and playback state."""
        self.network = pylast.LastFMNetwork(
            api_key=os.getenv("LASTFM_API_KEY"),
            api_secret=os.getenv("LASTFM_API_SECRET"),
            username=os.getenv("LASTFM_USERNAME"),
            password_hash=os.getenv("LASTFM_PASSWORD_HASH"),
        )

        self.current_track = None
        self.is_playing = False
        self.last_play_timestamp = 0
        self.accumulated_time = 0
        self.time_started = 0  # required by Last.fm
        self.has_scrobbled = False

    def process_event(self, track_info: dict, player_state: str):
        """
        Receives events from the macOS listener.
        player_state usually comes in as 'Playing', 'Paused', or 'Stopped'.
        """
        # If Apple Music is completely stopped, finalize whatever was playing
        if player_state == "Stopped" or not track_info:
            self._finalize_current_track()
            return

        is_new_track = not self._is_same_track(track_info)

        if is_new_track:
            self._finalize_current_track()
            self._start_new_track(track_info, player_state)
        else:
            self._update_playback_state(player_state)

    # hussle to check track position so no "on repeat" mode...
    def _is_same_track(self, track_info: dict) -> bool:
        if not self.current_track:
            return False
        return (
            self.current_track.get('name') == track_info.get('name') and
            self.current_track.get('artist') == track_info.get('artist')
        )

    def _start_new_track(self, track_info: dict, player_state: str):
        self.current_track = track_info
        self.has_scrobbled = False
        self.accumulated_time = 0
        self.time_started = int(time.time())

        if player_state == "Playing":
            self.is_playing = True
            self.last_play_timestamp = time.time()

            try:
                self.network.update_now_playing(
                    artist=self.current_track.get("artist"),
                    title=self.current_track.get("name"),
                    album=self.current_track.get("album", "")
                )
                print(
                    f"[i] Now Playing: {self.current_track['artist']} - {self.current_track['name']}")
            except Exception as e:
                print(f"[e] Error updating Now Playing: {e}")

    def _update_playback_state(self, player_state: str):
        if player_state == "Paused" and self.is_playing:
            self._add_accumulated_time()  # double check is ok
            self.is_playing = False
            print(
                f"[i] Paused. Accumulated time: {int(self.accumulated_time)}s")

        elif player_state == "Playing" and not self.is_playing:
            self.is_playing = True
            self.last_play_timestamp = time.time()
            print("[i] Resumed")

    def _finalize_current_track(self):
        """Evaluates the current track to see if it qualifies for a scrobble."""
        if not self.current_track or self.has_scrobbled:
            self.current_track = None
            return

        # add the final chunk of time
        self._add_accumulated_time()

        if self._is_threshold_passed():
            try:
                self.network.scrobble(
                    artist=self.current_track.get("artist"),
                    title=self.current_track.get("name"),
                    album=self.current_track.get("album", ""),
                    timestamp=self.time_started
                )
                self.has_scrobbled = True
                print(
                    f"[i] SCROBBLED: {self.current_track['artist']} - {self.current_track['name']}")
            except Exception as e:
                print(f"[e] Failed to scrobble: {e}")
        else:
            print(
                f"[i] Skipped: Listened for {int(self.accumulated_time)}s.")

        self.current_track = None
        self.is_playing = False

    def _add_accumulated_time(self):
        if self.is_playing:
            self.accumulated_time += (time.time() - self.last_play_timestamp)

    def _is_threshold_passed(self):
        duration = self.current_track.get("duration", 0)
        threshold = min(duration / 2, 240)
        # 1. Track must be > 30 seconds.
        # 2. Must listen to 50% of the track OR 4 minutes (240s), whichever is less.
        return duration > 30 and self.accumulated_time >= threshold
