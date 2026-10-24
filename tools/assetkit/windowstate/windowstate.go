// Package windowstate merkt Größe und Position eines Wails-Fensters zwischen den Starts (k3c-workbench, k3c-studio).
// Es kennt Wails nicht: Aufrufer reichen Lesen und Setzen der Position als Funktionen herein.
package windowstate

import (
	"encoding/json"
	"os"
	"path/filepath"
)

// Vorgabe und Minimum der Fenstergröße (B-064 › Fenster).
const (
	DefaultWidth  = 1280
	DefaultHeight = 800
	MinWidth      = 900
	MinHeight     = 600
)

// State ist die gemerkte Größe und Position des Fensters.
type State struct {
	X      int `json:"x"`
	Y      int `json:"y"`
	Width  int `json:"width"`
	Height int `json:"height"`
}

// Load liest den gemerkten Zustand. ok ist false, wenn die Datei fehlt oder kaputt ist; dann gilt die Vorgabe
// und die Position bleibt Wails überlassen. Zu kleine Maße werden aufs Minimum gehoben. Negative Positionen bleiben
// (Bildschirm links oder oberhalb des Hauptbildschirms); ob der Bildschirm noch da ist, prüft OnScreen beim Start.
func Load(path string) (w State, ok bool) {
	w = State{Width: DefaultWidth, Height: DefaultHeight}
	data, err := os.ReadFile(path)
	if err != nil || json.Unmarshal(data, &w) != nil {
		return State{Width: DefaultWidth, Height: DefaultHeight}, false
	}
	w.Width, w.Height = max(w.Width, MinWidth), max(w.Height, MinHeight)
	return w, true
}

// Place setzt die absolute Position (x, y). Wails liest sie absolut, setzt sie aber relativ zum Arbeitsbereich
// des Bildschirms, auf dem das Fenster gerade steht; der Zug auf (0, 0) misst diesen Versatz. get und set sind
// runtime.WindowGetPosition und runtime.WindowSetPosition des Fensters.
func Place(get func() (x, y int), set func(x, y int), x, y int) {
	set(0, 0)
	ox, oy := get()
	set(x-ox, y-oy)
}

// Save schreibt den Zustand; der Ordner wird bei Bedarf angelegt.
func Save(path string, w State) error {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	data, err := json.Marshal(w)
	if err != nil {
		return err
	}
	return os.WriteFile(path, data, 0o644)
}

// ConfigPath ist eine Datei unter <UserConfigDir>/k3c, nicht im Repo.
func ConfigPath(name string) string {
	dir, err := os.UserConfigDir()
	if err != nil {
		dir = os.TempDir()
	}
	return filepath.Join(dir, "k3c", name)
}
