package player

import (
	"log"

	"github.com/mishazawa/my_scrobbler/internal/lastfm"
	"github.com/progrium/darwinkit/macos/foundation"
)

type MusicListener struct {
	manager *lastfm.ScrobbleManager
}

func NewMusicListener(mgr *lastfm.ScrobbleManager) *MusicListener {
	return &MusicListener{manager: mgr}
}

func (ml *MusicListener) StartListening() {
	log.Println("[i] Listening for Apple Music events via DarwinKit...")

	nc := foundation.NewDistributedNotificationCenter().NotificationCenter

	nc.AddObserverForNameObjectQueueUsingBlock(
		"com.apple.Music.playerInfo",
		nil,
		nil,
		func(notification foundation.Notification) {
			ml.playerInfoChanged(notification)
		},
	)
}

func (ml *MusicListener) playerInfoChanged(notification foundation.Notification) {
	userInfo := notification.UserInfo()
	if userInfo.Count() == 0 {
		return
	}

	getStringValue := func(key string, fallback string) string {
		nsKey := foundation.String_StringWithString(key)

		val := userInfo.ObjectForKey(nsKey)
		if val.Ptr() == nil {
			return fallback
		}

		s := foundation.StringFrom(val.Ptr())
		return s.String()
	}

	playerState := getStringValue("Player State", "Stopped")

	var durationSec float64 = 0
	totalTimeVal := userInfo.ObjectForKey(foundation.String_StringWithString("Total Time"))
	if totalTimeVal.Ptr() != nil {
		num := foundation.NumberFrom(totalTimeVal.Ptr())
		durationSec = num.DoubleValue() / 1000.0
	}

	track := &lastfm.Track{
		Name:     getStringValue("Name", "Unknown Title"),
		Artist:   getStringValue("Artist", "Unknown Artist"),
		Album:    getStringValue("Album", ""),
		Duration: durationSec,
	}

	ml.manager.ProcessEvent(track, playerState)
}
