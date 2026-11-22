package mcpsrv

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/simcli"
)

// writeReplay lässt die echte Engine einen kurzen Lauf (1 Spieler, 1 Tag, Bot passive) aufnehmen und liefert die
// Aufnahme nach dir/<Name>; das Tool spielt sie wieder ab (B-924: keine Attrappe, kein Kindprozess).
func writeReplay(t *testing.T, dir string) (name string, r replayHead) {
	t.Helper()
	in := `{"seed-list":"2","bots":"passive","players":"1","depths":"0","days":1,"replay-dir":` + jsonText(dir) +
		`,"out":` + jsonText(filepath.Join(t.TempDir(), "matrix.json")) + `}`
	if err := simcli.Stream(context.Background(), []string{"balance", "--input", "-"}, []byte(in), nil, func(simcli.Event) {}); err != nil {
		t.Fatal(err)
	}
	files, _ := filepath.Glob(filepath.Join(dir, "*.json"))
	if len(files) != 1 {
		t.Fatalf("erwartet eine Aufnahme in %s: %v", dir, files)
	}
	b, err := os.ReadFile(files[0])
	if err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(b, &r); err != nil {
		t.Fatal(err)
	}
	return filepath.Base(files[0]), r
}

func jsonText(s string) string {
	b, _ := json.Marshal(s)
	return string(b)
}

func TestReplayRunGleicherHashUndBurgfall(t *testing.T) {
	root := t.TempDir()
	if err := os.Mkdir(filepath.Join(root, "replays"), 0o755); err != nil {
		t.Fatal(err)
	}
	name, r := writeReplay(t, filepath.Join(root, "replays"))
	cs := connect(t, New(Config{Version: "test", Root: root}))

	rel := "replays/" + name
	text, isErr := callText(t, cs, "replay_run", map[string]any{"path": rel})
	want := "Endzustand-Hash " + r.EndHash + " · Burgfall-Tick " + tickText(r.CastleFallTick)
	if isErr || !strings.HasPrefix(text, "replay_run "+rel) || !strings.Contains(text, want) ||
		!strings.Contains(text, "Aufnahme: gleicher Endzustand") || strings.Contains(text, "Warnung") {
		t.Errorf("replay_run: %q, erwartet %q", text, want)
	}
}

func TestReplayRunNurImRepo(t *testing.T) {
	t.Parallel()
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
