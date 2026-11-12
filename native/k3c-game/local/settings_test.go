package local

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

// Fluss-Fassung und Leistungsanzeige (B-816) überleben Speichern und Laden; ohne gewählte Fassung steht kein Feld in
// der Datei (der Startwert K3C_RIVER gilt weiter), und eine alte Datei ohne die Felder bleibt gültig.
func TestSettingsRiverStatsPersist(t *testing.T) {
	dir := t.TempDir()
	s := Open(Config{Dir: dir})
	want := view.DefaultSettings()
	want.River, want.Stats = view.RiverTiles, true
	if err := s.SetSettings(want); err != nil {
		t.Fatal(err)
	}
	if got := Open(Config{Dir: dir}).Settings(); got != want {
		t.Errorf("geladen %+v, erwartet %+v", got, want)
	}
	want.River = ""
	if err := s.SetSettings(want); err != nil {
		t.Fatal(err)
	}
	raw, err := os.ReadFile(filepath.Join(dir, SettingsFile))
	if err != nil || strings.Contains(string(raw), "river") {
		t.Errorf("Startwert soll kein Feld schreiben: %s %v", raw, err)
	}
	if got := Open(Config{Dir: dir}).Settings(); got != want {
		t.Errorf("ohne Fassung geladen %+v", got)
	}
}

func TestSettingsOldFileAndBadRiver(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, SettingsFile)
	if err := os.WriteFile(path, []byte(`{"music":0.5,"river":"lava","stats":true}`), 0o644); err != nil {
		t.Fatal(err)
	}
	got := Open(Config{Dir: dir}).Settings()
	if got.Music != 0.5 || got.River != "" || !got.Stats {
		t.Errorf("unbekannte Fassung -> Startwert: %+v", got)
	}
	if err := os.WriteFile(path, []byte(`{"music":0.5}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if got := Open(Config{Dir: dir}).Settings(); got.River != "" || got.Stats {
		t.Errorf("alte Datei: %+v", got)
	}
}
