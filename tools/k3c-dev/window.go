package main

import (
	"encoding/json"
	"os"
	"path/filepath"
)

// Vorgabe und Minimum der Fenstergröße (B-064 › Fenster).
const (
	defaultWidth  = 1280
	defaultHeight = 800
	minWidth      = 900
	minHeight     = 600
)

// windowState ist die gemerkte Größe und Position des Fensters.
type windowState struct {
	X      int `json:"x"`
	Y      int `json:"y"`
	Width  int `json:"width"`
	Height int `json:"height"`
}

// loadWindow liest den gemerkten Zustand. ok ist false, wenn die Datei fehlt oder kaputt ist; dann gilt die Vorgabe
// und die Position bleibt Wails überlassen. Zu kleine Maße werden aufs Minimum, negative Positionen auf 0 gehoben.
// ponytail: prüft nicht, ob die Position auf einem noch angeschlossenen Bildschirm liegt (Wails rechnet relativ zum
// aktuellen Bildschirm); bei Bedarf mit runtime.ScreenGetAll abgleichen.
func loadWindow(path string) (w windowState, ok bool) {
	w = windowState{Width: defaultWidth, Height: defaultHeight}
	data, err := os.ReadFile(path)
	if err != nil || json.Unmarshal(data, &w) != nil {
		return windowState{Width: defaultWidth, Height: defaultHeight}, false
	}
	w.Width, w.Height = max(w.Width, minWidth), max(w.Height, minHeight)
	w.X, w.Y = max(w.X, 0), max(w.Y, 0)
	return w, true
}

// saveWindow schreibt den Zustand; der Ordner wird bei Bedarf angelegt.
func saveWindow(path string, w windowState) error {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	data, err := json.Marshal(w)
	if err != nil {
		return err
	}
	return os.WriteFile(path, data, 0o644)
}

// configPath ist eine Datei unter <UserConfigDir>/k3c, nicht im Repo.
func configPath(name string) string {
	dir, err := os.UserConfigDir()
	if err != nil {
		dir = os.TempDir()
	}
	return filepath.Join(dir, "k3c", name)
}
