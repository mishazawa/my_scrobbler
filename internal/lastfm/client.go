package lastfm

import (
	"log"
	"math"
	"os"
	"time"

	"github.com/shkh/lastfm-go/lastfm"
	"github.com/zalando/go-keyring"
)

type Track struct {
	Name     string
	Artist   string
	Album    string
	Duration float64 // seconds
}

type ScrobbleManager struct {
	api               *lastfm.Api
	currentTrack      *Track
	isPlaying         bool
	lastPlayTimestamp time.Time
	accumulatedTime   time.Duration
	timeStarted       int64
	hasScrobbled      bool
}

const (
	SERVICE = "my-scrobbler"
)

func NewScrobbleManager() *ScrobbleManager {
	apiKey := os.Getenv("LASTFM_API_KEY")
	apiSecret := os.Getenv("LASTFM_API_SECRET")
	username := os.Getenv("LASTFM_USERNAME")

	password, err := keyring.Get(SERVICE, username)
	if err != nil {
		log.Fatalf("[e] Keychain request failed: %v", err)
	}

	api := lastfm.New(apiKey, apiSecret)
	err = api.Login(username, password)

	if err != nil {
		log.Fatalf("[e] Last.fm authentication failed: %v", err)
	} else {
		log.Println("[i] Successfully authenticated with Last.fm")
	}

	return &ScrobbleManager{
		api: api,
	}
}

func (sm *ScrobbleManager) ProcessEvent(trackInfo *Track, playerState string) {
	if playerState == "Stopped" || trackInfo == nil {
		sm.finalizeCurrentTrack()
		return
	}

	isNewTrack := !sm.isSameTrack(trackInfo)

	if isNewTrack {
		sm.finalizeCurrentTrack()
		sm.startNewTrack(trackInfo, playerState)
	} else {
		sm.updatePlaybackState(playerState)
	}
}

func (sm *ScrobbleManager) isSameTrack(trackInfo *Track) bool {
	if sm.currentTrack == nil {
		return false
	}
	return sm.currentTrack.Name == trackInfo.Name && sm.currentTrack.Artist == trackInfo.Artist
}

func (sm *ScrobbleManager) startNewTrack(trackInfo *Track, playerState string) {
	sm.currentTrack = trackInfo
	sm.hasScrobbled = false
	sm.accumulatedTime = 0
	sm.timeStarted = time.Now().Unix()

	if playerState == "Playing" {
		sm.isPlaying = true
		sm.lastPlayTimestamp = time.Now()

		_, err := sm.api.Track.UpdateNowPlaying(lastfm.P{
			"artist": sm.currentTrack.Artist,
			"track":  sm.currentTrack.Name,
			"album":  sm.currentTrack.Album,
		})
		if err != nil {
			log.Printf("[e] Error updating Now Playing: %v", err)
			return
		}
		log.Printf("[i] Now Playing: %s - %s", sm.currentTrack.Artist, sm.currentTrack.Name)
	}
}

func (sm *ScrobbleManager) updatePlaybackState(playerState string) {
	if playerState == "Paused" && sm.isPlaying {
		sm.addAccumulatedTime()
		sm.isPlaying = false
		log.Printf("[i] Paused. Accumulated time: %v", sm.accumulatedTime.Round(time.Second))

	} else if playerState == "Playing" && !sm.isPlaying {
		sm.isPlaying = true
		sm.lastPlayTimestamp = time.Now()
		log.Println("[i] Resumed")
	}
}

func (sm *ScrobbleManager) finalizeCurrentTrack() {
	if sm.currentTrack == nil || sm.hasScrobbled {
		sm.currentTrack = nil
		return
	}

	sm.addAccumulatedTime()

	if sm.isThresholdPassed() {
		_, err := sm.api.Track.Scrobble(lastfm.P{
			"artist":    sm.currentTrack.Artist,
			"track":     sm.currentTrack.Name,
			"album":     sm.currentTrack.Album,
			"timestamp": sm.timeStarted,
		})
		if err != nil {
			log.Printf("[e] Failed to scrobble: %v", err)
		} else {
			sm.hasScrobbled = true
			log.Printf("[i] SCROBBLED: %s - %s", sm.currentTrack.Artist, sm.currentTrack.Name)
		}
	} else {
		log.Printf("[i] Skipped: Listened for %v.", sm.accumulatedTime.Round(time.Second))
	}

	sm.currentTrack = nil
	sm.isPlaying = false
}

func (sm *ScrobbleManager) addAccumulatedTime() {
	if sm.isPlaying {
		sm.accumulatedTime += time.Since(sm.lastPlayTimestamp)
	}
}

func (sm *ScrobbleManager) isThresholdPassed() bool {
	if sm.currentTrack.Duration <= 30 {
		return false
	}

	// 50% of track length vs 4 minutes maximum threshold
	halfDuration := sm.currentTrack.Duration / 2.0
	thresholdSeconds := math.Min(halfDuration, 240.0)
	threshold := time.Duration(thresholdSeconds) * time.Second

	return sm.accumulatedTime >= threshold
}
