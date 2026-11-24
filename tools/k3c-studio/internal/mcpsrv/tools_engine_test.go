package mcpsrv

import (
	"context"
	"encoding/json"
	"path/filepath"
	"strings"
	"testing"

	"k3c/tools/k3c-studio/internal/gameexe"
	"k3c/tools/workkit/mcpkit"
)

// fakeGame ersetzt den Aufruf der EXE und merkt sich die Optionen (nicht parallel: gameCall ist eine Paket-Variable).
func fakeGame(t *testing.T, text string) *gameexe.Options {
	t.Helper()
	got := &gameexe.Options{}
	gameCall = func(_ context.Context, o gameexe.Options) (gameexe.Event, error) {
		*got = o
		return gameexe.Event{Event: "result", Text: text}, nil
	}
	t.Cleanup(func() { gameCall = gameexe.Call })
	return got
}

// RU34.2/AC-02: head wählt das Fenster samt Zeitraffer; die Eingabe der EXE kennt head und speed nicht als Felder.
func TestSimRunHeadWaehltFenster(t *testing.T) {
	t.Setenv(simUnlockEnv, "1")
	got := fakeGame(t, "Lauf")
	s := New(Config{Root: t.TempDir(), Version: "test"})
	text, err := s.simRun(context.Background(), simIn{Seed: "a", Ticks: 30, Head: true, Speed: 4})
	if err != nil || text != "Lauf" {
		t.Fatalf("sim_run: %q %v", text, err)
	}
	if got.Sub != "run" || !got.Window || got.Speed != 4 {
		t.Errorf("Optionen: %+v", got)
	}
	var in map[string]any
	if err := json.Unmarshal(got.Input, &in); err != nil || in["head"] != nil || in["speed"] != nil || in["seed"] != "a" {
		t.Errorf("Eingabe der EXE: %s", got.Input)
	}
}

// RU34.2: Ohne head läuft der Aufruf headless; speed ohne head oder außerhalb 1 bis 64 wird abgelehnt.
func TestSimRunHeadlessUndSpeedPruefung(t *testing.T) {
	t.Setenv(simUnlockEnv, "1")
	got := fakeGame(t, "Lauf")
	s := New(Config{Root: t.TempDir(), Version: "test"})
	if _, err := s.simRun(context.Background(), simIn{Seed: "a", Ticks: 30}); err != nil || got.Window || got.Speed != 0 {
		t.Errorf("ohne head headless: %+v %v", got, err)
	}
	for _, bad := range []simIn{{Seed: "a", Ticks: 30, Speed: 2}, {Seed: "a", Ticks: 30, Head: true, Speed: 65}} {
		if _, err := s.simRun(context.Background(), bad); err == nil || !strings.Contains(err.Error(), "speed") {
			t.Errorf("speed %d: %v", bad.Speed, err)
		}
	}
}

// RU34.2: level_generate rechnet mit der EXE des gewählten Checkouts, auch eines Worktrees, ohne Fenster.
func TestLevelGenerateNimmtCheckoutDerAnfrage(t *testing.T) {
	got := fakeGame(t, "Level")
	main := t.TempDir()
	s := New(Config{Root: main, Version: "test"})
	wt := filepath.Join(main, ".claude", "worktrees", "wt-a")
	ctx := mcpkit.WithWorkspace(context.Background(), workspace{Root: wt})
	text, err := s.levelGenerate(ctx, levelIn{Seed: "1"})
	if err != nil || text != "Level" {
		t.Fatalf("level_generate: %q %v", text, err)
	}
	if got.Root != wt || got.Sub != "level" || got.Window {
		t.Errorf("Optionen: %+v", got)
	}
}

// RU34.2: Ohne gebaute EXE rechnet nichts und der Fehler nennt den Bau-Befehl.
func TestLevelGenerateOhneExe(t *testing.T) {
	s := New(Config{Root: t.TempDir(), Version: "test"})
	_, err := s.levelGenerate(context.Background(), levelIn{Seed: "1"})
	if err == nil || !strings.Contains(err.Error(), gameexe.BuildCommand) {
		t.Errorf("level_generate: %v", err)
	}
}
