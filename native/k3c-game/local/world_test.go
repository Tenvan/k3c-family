package local

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

// B-938: WorldList (ohne Dev-Mode nur die Standard-Welt) und World (laufendes Spiel); der Spielstand trägt seine Welt.
func TestWorldListAndWorldOfRunningGame(t *testing.T) {
	s := testSession(t, t.TempDir())
	if l := s.WorldList(); len(l) != 1 || l[0].ID != "standard" || l[0].Name == "" {
		t.Errorf("Weltliste %+v", l)
	}
	if w := s.World(); w != "" {
		t.Errorf("Welt ohne Spiel: %q", w)
	}
	g := view.NewGame{Name: "welt-1", World: "standard", Devices: []view.DeviceID{{Kind: view.DeviceKeyboard}}}
	if err := s.New(g); err != nil {
		t.Fatal(err)
	}
	if w := s.World(); w != "standard" {
		t.Errorf("Welt im Spiel: %q", w)
	}
	if err := s.Close(); err != nil {
		t.Fatal(err)
	}
	list, _ := s.List()
	if len(list) != 1 || !list[0].Loadable || list[0].World != "standard" {
		t.Errorf("Spielstände %+v", list)
	}
	if err := s.Load("welt-1", nil); err != nil {
		t.Fatal(err)
	}
	if w := s.World(); w != "standard" {
		t.Errorf("Welt nach Laden: %q", w)
	}
}

// Lokale Welten erscheinen nur im Dev-Mode und nur, wenn der Ordner sie enthält (K3C_WORLDS_DIR).
func TestWorldListShowsLocalWorldsInDevMode(t *testing.T) {
	dir := t.TempDir()
	std, err := os.ReadFile("../../../data/worlds/standard.json")
	if err != nil {
		t.Fatal(err)
	}
	raw := strings.Replace(string(std), `"id": "standard"`, `"id": "lokal"`, 1)
	raw = strings.Replace(raw, `"name": "Standard"`, `"name": "Lokal"`, 1)
	if err := os.WriteFile(filepath.Join(dir, "lokal.json"), []byte(raw), 0o600); err != nil {
		t.Fatal(err)
	}
	t.Setenv(WorldsDirEnv, dir)
	s := testSession(t, t.TempDir())
	if l := s.WorldList(); len(l) != 1 {
		t.Errorf("ohne Dev-Mode: %+v", l)
	}
	s.settings.DevMode = true
	if l := s.WorldList(); len(l) != 2 || l[1].ID != "lokal" {
		t.Errorf("im Dev-Mode: %+v", l)
	}
}
