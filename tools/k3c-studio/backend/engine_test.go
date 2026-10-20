package backend

import (
	"encoding/json"
	"errors"
	"io/fs"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/data"
	"k3c/engine/level"
	"k3c/engine/room"
)

// TS1.2 (B-507/AC-02): Das Studio rechnet Vorschau und Spielstart mit eingebundener Engine, ohne Spielserver; Spielstände
// landen nur im Testordner tools/k3c-studio/.studio-saves, nie in saves/.

// nopPeer ist ein Gerät, das nichts mitschreibt.
type nopPeer struct{}

func (nopPeer) Joined(string, string, string, []room.Seat) {}
func (nopPeer) Level(int, int, level.Layout)                {}
func (nopPeer) State(int, int, any)                         {}
func (nopPeer) Seats([]room.Seat, []string)                 {}
func (nopPeer) Replaced()                                   {}
func (nopPeer) Closed(bool)                                 {}

// engineRoot ist eine Repo-Wurzel in t.TempDir() mit der Test-Welt „probe“ (Kopie der Standard-Welt) in data/worlds.
func engineRoot(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	raw, err := data.Files.ReadFile("worlds/standard.json")
	if err != nil {
		t.Fatal(err)
	}
	probe := strings.Replace(string(raw), `"id": "standard"`, `"id": "probe"`, 1)
	dir := filepath.Join(root, filepath.FromSlash(worldsDir))
	if err := os.MkdirAll(dir, 0o750); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "probe.json"), []byte(probe), 0o600); err != nil {
		t.Fatal(err)
	}
	return root
}

// engineApp ist ein Studio mit eingebundener Engine über engineRoot.
func engineApp(t *testing.T) (*App, string) {
	t.Helper()
	root := engineRoot(t)
	a := &App{root: root, engine: NewEngine(root, nil)}
	t.Cleanup(a.engine.Rooms.Close)
	return a, root
}

// noServerSaves prüft, dass unter root kein saves/ des Servers entstanden ist.
func noServerSaves(t *testing.T, root string) {
	t.Helper()
	if _, err := os.Stat(filepath.Join(root, "saves")); !errors.Is(err, fs.ErrNotExist) {
		t.Errorf("saves/ des Servers angelegt: %v", err)
	}
}

func TestEngineStartetRaeumeInStandardUndTestWeltOhneServer(t *testing.T) {
	a, root := engineApp(t)
	worlds := map[string]string{"std": "", "probe-welt": "probe"}
	for name, world := range worlds {
		if _, err := a.engine.Rooms.Create("geraet-"+name, nopPeer{}, name, true, 0, []int{0}, room.Options{World: world}); err != nil {
			t.Fatalf("Raum %s (Welt %q): %v", name, world, err)
		}
	}
	a.engine.Rooms.Close() // speichert alle Räume
	for name, world := range worlds {
		raw, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(testSavesDir), name+".json"))
		if err != nil {
			t.Fatalf("Spielstand %s nicht im Testordner: %v", name, err)
		}
		var s struct {
			World string `json:"world"`
		}
		_ = json.Unmarshal(raw, &s)
		if world != "" && s.World != world {
			t.Errorf("Spielstand %s in Welt %q, erwartet %q", name, s.World, world)
		}
	}
	noServerSaves(t, root)
}

func TestEngineBeantwortetSpielAPIOhneServer(t *testing.T) {
	a, root := engineApp(t)
	if _, err := a.engine.Rooms.FullSave("abc"); err != nil {
		t.Fatal(err)
	}
	h := a.Game()
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, httptest.NewRequest(http.MethodGet, "/api/health", nil))
	if rec.Code != http.StatusOK {
		t.Errorf("/api/health: %d", rec.Code)
	}
	rec = httptest.NewRecorder()
	h.ServeHTTP(rec, httptest.NewRequest(http.MethodGet, "/api/saves", nil))
	if rec.Code != http.StatusOK || !strings.Contains(rec.Body.String(), `"voll-abc"`) {
		t.Errorf("/api/saves: %d %s", rec.Code, rec.Body.String())
	}
	noServerSaves(t, root)
}

func TestOpenGameVollOhneServerImTestordner(t *testing.T) {
	a, root := engineApp(t)
	base := "http://127.0.0.1:1" // dort läuft nichts: das Studio fragt keinen Spielserver
	res := a.WorldOpenGame(base, "abc", "cave", "full")
	if res.Error != "" || res.Save != "voll-abc" {
		t.Fatalf("Ergebnis %+v", res)
	}
	if want := base + "/index.html#game.html%3Fautostart%3D1%26save%3Dvoll-abc"; res.URL != want {
		t.Errorf("Adresse %s, erwartet %s", res.URL, want)
	}
	if _, err := os.Stat(filepath.Join(root, filepath.FromSlash(testSavesDir), "voll-abc.json")); err != nil {
		t.Errorf("Spielstand nicht im Testordner: %v", err)
	}
	noServerSaves(t, root)
}

func TestOpenGameNeuOhneServer(t *testing.T) {
	a, _ := engineApp(t)
	res := a.WorldOpenGame("http://127.0.0.1:1", "probe", "forest", "new")
	if want := "http://127.0.0.1:1/index.html#game.html%3Fautostart%3D1%26save%3Dprobe"; res.Error != "" || res.URL != want {
		t.Errorf("Ergebnis %+v, erwartet %s", res, want)
	}
}

func TestOpenGameOhneEngineHinweis(t *testing.T) {
	a := &App{}
	if res := a.WorldOpenGame("http://127.0.0.1:8091", "abc", "forest", "full"); res.Error == "" || res.URL != "" {
		t.Errorf("ohne Engine: %+v", res)
	}
}
