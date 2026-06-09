package main

import (
	"log"
	"os"
	"path/filepath"
	"runtime"

	"github.com/joho/godotenv"
	"github.com/progrium/darwinkit/macos/appkit"
	"github.com/mishazawa/my_scrobbler/internal/lastfm"
	"github.com/mishazawa/my_scrobbler/internal/player"
)

func init() {
	// CRITICAL: Force the Go runtime to lock this main function to the 
	// operating system's main thread (Thread 0). Cocoa events will fail silently
	// or crash if called from dynamic background goroutines.
	runtime.LockOSThread()
}

func main() {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		log.Fatalf("[e] Could not locate user home directory: %v", err)
	}

	configPath := filepath.Join(homeDir, ".config", "my_scrobbler", "config.env")
	
	err = godotenv.Load(configPath)
	if err != nil {
		log.Printf("[w] Warning: Could not load config file at %s, relying on system env", configPath)
	}

	log.Println("[i] Starting my_scrobbler...")

	manager := lastfm.NewScrobbleManager()

	listener := player.NewMusicListener(manager)
	listener.StartListening()

	// 4. Run the macOS native Application Event Loop (Equivalent to AppHelper.runEventLoop())
	// Go handles SIGINT (Ctrl+C) automatically when running under launchd/Homebrew services.
	app := appkit.Application_SharedApplication()
	app.Run()
}