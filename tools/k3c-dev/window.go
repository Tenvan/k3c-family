package main

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"

	"github.com/wailsapp/wails/v2/pkg/runtime"
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
// und die Position bleibt Wails überlassen. Zu kleine Maße werden aufs Minimum gehoben. Negative Positionen bleiben
// (Bildschirm links oder oberhalb des Hauptbildschirms); ob der Bildschirm noch da ist, prüft onScreen beim Start.
func loadWindow(path string) (w windowState, ok bool) {
	w = windowState{Width: defaultWidth, Height: defaultHeight}
	data, err := os.ReadFile(path)
	if err != nil || json.Unmarshal(data, &w) != nil {
		return windowState{Width: defaultWidth, Height: defaultHeight}, false
	}
	w.Width, w.Height = max(w.Width, minWidth), max(w.Height, minHeight)
	return w, true
}

// placeWindow setzt die absolute Position (x, y). Wails liest sie absolut, setzt sie aber relativ zum Arbeitsbereich
// des Bildschirms, auf dem das Fenster gerade steht; der Zug auf (0, 0) misst diesen Versatz.
func placeWindow(ctx context.Context, x, y int) {
	runtime.WindowSetPosition(ctx, 0, 0)
	ox, oy := runtime.WindowGetPosition(ctx)
	runtime.WindowSetPosition(ctx, x-ox, y-oy)
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
