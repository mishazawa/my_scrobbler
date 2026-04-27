import objc
import Foundation


class MusicListener(Foundation.NSObject):
    def initWithManager_(self, manager):
        self = objc.super(MusicListener, self).init()
        if self:
            self.manager = manager
        return self

    def startListening(self):
        """Subscribes to Apple Music's global macOS notifications."""
        nc = Foundation.NSDistributedNotificationCenter.defaultCenter()

        # listen 'com.apple.Music.playerInfo'
        nc.addObserver_selector_name_object_(
            self,
            "playerInfoChanged:",
            "com.apple.Music.playerInfo",
            None
        )
        print("[i] Listening for Apple Music events... (Press Ctrl+C to stop)")

    def playerInfoChanged_(self, notification):
        """Callback triggered by macOS when a track changes, pauses, or plays."""
        user_info = notification.userInfo()
        if not user_info:
            return

        player_state = user_info.get("Player State", "Stopped")

        total_time_ms = user_info.get("Total Time", 0)
        duration_sec = total_time_ms / 1000.0 if total_time_ms else 0

        track_info = {
            "name": user_info.get("Name", "Unknown Title"),
            "artist": user_info.get("Artist", "Unknown Artist"),
            "album": user_info.get("Album", ""),
            "duration": duration_sec
        }

        self.manager.process_event(track_info, player_state)
