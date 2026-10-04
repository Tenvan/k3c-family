package mcpsrv

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/tools/k3c-dev/internal/balance"
)

// recordReplay nimmt einen Bot-Lauf auf (passiv verliert die Burg in Nacht 1) und schreibt ihn nach dir/name.
func recordReplay(t *testing.T, dir, name string) balance.Replay {
	t.Helper()
	rep, err := balance.Run(balance.Matrix{Seeds: []string{"2"}, Players: []int{1}, Bots: []string{"passive"}, Depths: []int{0}, Days: 1})
	if err != nil || rep.Results[0].Replay == nil {
		t.Fatalf("Aufnahme: %v %+v", err, rep.Results)
	}
	r := *rep.Results[0].Replay
	b, err := r.JSON()
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, name), b, 0o644); err != nil {
		t.Fatal(err)
	}
	return r
}

func TestReplayRunGleicherHashUndBurgfall(t *testing.T) {
	root := t.TempDir()
	if err := os.Mkdir(filepath.Join(root, "replays"), 0o755); err != nil {
		t.Fatal(err)
	}
	r := recordReplay(t, filepath.Join(root, "replays"), "2.replay.json")
	if r.CastleFallTick == nil {
		t.Fatal("Aufnahme ohne Burgfall: Test braucht einen Burgfall-Tick")
	}
	cs := connect(t, New(Config{Version: "test", Root: root}))

	text, isErr := callText(t, cs, "replay_run", map[string]any{"path": "replays/2.replay.json"})
	want := "Endzustand-Hash " + r.EndHash + " · Burgfall-Tick " + tickText(r.CastleFallTick)
	if isErr || !strings.HasPrefix(text, "replay_run replays/2.replay.json") || !strings.Contains(text, want) ||
		!strings.Contains(text, "Aufnahme: gleicher Endzustand") || strings.Contains(text, "Warnung") {
		t.Errorf("replay_run: %q, erwartet %q", text, want)
	}
}

func TestReplayRunNurImRepo(t *testing.T) {
	base := t.TempDir()
	root := filepath.Join(base, "repo")
	if err := os.Mkdir(root, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(base, "draussen.replay.json"), []byte(`{"version":1}`), 0o644); err != nil {
		t.Fatal(err)
	}
	cs := connect(t, New(Config{Version: "test", Root: root}))
	for _, p := range []string{"../draussen.replay.json", filepath.Join(base, "draussen.replay.json")} {
		if text, isErr := callText(t, cs, "replay_run", map[string]any{"path": p}); !isErr || !strings.Contains(text, "abgelehnt") {
			t.Errorf("%s: %q", p, text)
		}
	}
	if text, isErr := callText(t, cs, "replay_run", map[string]any{"path": "fehlt.json"}); !isErr || !strings.Contains(text, "nicht gefunden") {
		t.Errorf("fehlende Datei: %q", text)
	}
}

func TestReplayRunKaputteDatei(t *testing.T) {
	root := t.TempDir()
	if err := os.WriteFile(filepath.Join(root, "alt.json"), []byte(`{"version":99}`), 0o644); err != nil {
		t.Fatal(err)
	}
	cs := connect(t, New(Config{Version: "test", Root: root}))
	if text, isErr := callText(t, cs, "replay_run", map[string]any{"path": "alt.json"}); !isErr || !strings.Contains(text, "unbekannte Version 99") {
		t.Errorf("Version: %q", text)
	}
}
