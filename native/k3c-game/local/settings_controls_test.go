package local

import (
	"os"
	"path/filepath"
	"testing"

	"k3c/native/k3c-game/view"
)

// B-893/AC-03: Das Eingabe-Profil je Spieler überlebt Speichern und einen Neustart; zwei Tastatur-Profile aus einer
// Hand-Datei normalisiert das Laden (der spätere Spieler bekommt Gamepad), eine Datei ohne Feld gibt den Standard.
func TestSettingsControlsPersist(t *testing.T) {
	dir := t.TempDir()
	want := view.DefaultSettings()
	want.Controls[0], want.Controls[1] = view.ProfileGamepad, view.ProfileArrows
	if err := Open(Config{Dir: dir}).SetSettings(want); err != nil {
		t.Fatal(err)
	}
	if got := Open(Config{Dir: dir}).Settings(); got != want {
		t.Errorf("nach Neustart %v, erwartet %v", got.Controls, want.Controls)
	}
	raw := []byte(`{"controls":["pfeile","wasd","gamepad","gamepad","gamepad"]}`)
	if err := os.WriteFile(filepath.Join(dir, SettingsFile), raw, 0o644); err != nil {
		t.Fatal(err)
	}
	got := Open(Config{Dir: dir}).Settings().Controls
	if got[0] != view.ProfileArrows || got[1] != view.ProfileGamepad {
		t.Errorf("zwei Tastatur-Profile geladen: %v", got)
	}
	if err := os.WriteFile(filepath.Join(dir, SettingsFile), []byte(`{"music":0.5}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if got := Open(Config{Dir: dir}).Settings().Controls; got != view.DefaultSettings().Controls {
		t.Errorf("ohne Feld: %v", got)
	}
}
