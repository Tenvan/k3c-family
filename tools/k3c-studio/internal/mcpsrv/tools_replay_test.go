package mcpsrv

import (
	"context"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"

	"k3c/tools/k3c-studio/internal/gameexe"
)

// writeReplay legt eine Aufnahme (Kopf einer Replay-Datei) nach dir/<Name> (RU40.3: kein echter Lauf, die EXE ist eine
// Attrappe über gameCall).
func writeReplay(t *testing.T, dir string) (name string, r replayHead) {
	t.Helper()
	name = "1-p1-passive-d0.replay.json"
	body := `{"seed":"2","players":1,"bot":"passive","depth":0,"days":1,"endHash":"abc123","castleFallTick":null}`
	if err := os.WriteFile(filepath.Join(dir, name), []byte(body), 0o644); err != nil {
		t.Fatal(err)
	}
	return name, replayHead{Seed: "2", Players: 1, Bot: "passive", EndHash: "abc123"}
}

// fakePlayback lässt gameCall das Ergebnis von `sim balance --play` liefern und merkt sich die Optionen.
func fakePlayback(t *testing.T, report string) *gameexe.Options {
	t.Helper()
	got := &gameexe.Options{}
	gameCall = func(_ context.Context, o gameexe.Options) (gameexe.Event, error) {
		*got = o
		return gameexe.Event{Event: "result", Report: []byte(report)}, nil
	}
	t.Cleanup(func() { gameCall = gameexe.Call })
	return got
}

func TestReplayRunGleicherHashUndBurgfall(t *testing.T) {
	root := t.TempDir()
	if err := os.Mkdir(filepath.Join(root, "replays"), 0o755); err != nil {
		t.Fatal(err)
	}
	name, r := writeReplay(t, filepath.Join(root, "replays"))
	got := fakePlayback(t, `{"ticks":900,"endHash":"abc123","castleFallTick":null,"warnings":[]}`)
	cs := connect(t, New(Config{Version: "test", Root: root}))

	rel := "replays/" + name
	text, isErr := callText(t, cs, "replay_run", map[string]any{"path": rel})
	want := "Endzustand-Hash " + r.EndHash + " · Burgfall-Tick " + tickText(r.CastleFallTick)
	if isErr || !strings.HasPrefix(text, "replay_run "+rel) || !strings.Contains(text, want) ||
		!strings.Contains(text, "Aufnahme: gleicher Endzustand") || strings.Contains(text, "Warnung") {
		t.Errorf("replay_run: %q, erwartet %q", text, want)
	}
	if got.Sub != "balance" || got.Root != root || got.Window || got.Speed != 0 ||
		!slices.Equal(got.Args, []string{"--play", filepath.Join(root, "replays", name)}) {
		t.Errorf("Optionen headless: %+v", *got)
	}
}

// RU40.3/AC-03: head öffnet das Spiel (Fenster, Zeitraffer); speed ohne head und außerhalb 1 bis 64 ist abgelehnt.
func TestReplayRunHeadUndSpeed(t *testing.T) {
	root := t.TempDir()
	name, _ := writeReplay(t, root)
	got := fakePlayback(t, `{"ticks":9,"endHash":"x","castleFallTick":null}`)
	s := New(Config{Version: "test", Root: root})
	if _, err := s.replayRun(context.Background(), replayIn{Path: name, Head: true, Speed: 8}); err != nil {
		t.Fatal(err)
	}
	if !got.Window || got.Speed != 8 {
		t.Errorf("Optionen mit Fenster: %+v", *got)
	}
	for _, in := range []replayIn{{Path: name, Speed: 2}, {Path: name, Head: true, Speed: 65}} {
		if _, err := s.replayRun(context.Background(), in); err == nil || !strings.Contains(err.Error(), "speed") {
			t.Errorf("%+v angenommen: %v", in, err)
		}
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
	if err := os.WriteFile(filepath.Join(root, "alt.json"), []byte(`{`), 0o644); err != nil {
		t.Fatal(err)
	}
	cs := connect(t, New(Config{Version: "test", Root: root}))
	if text, isErr := callText(t, cs, "replay_run", map[string]any{"path": "alt.json"}); !isErr || !strings.Contains(text, "kein Replay") {
		t.Errorf("kaputt: %q", text)
	}
}
