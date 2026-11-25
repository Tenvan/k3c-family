package simrun

import (
	"context"
	"errors"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"

	"k3c/tools/k3c-studio/internal/gameexe"
)

// RU40.3: runExe startet die Spiel-EXE über streamExe; die Tests ersetzen sie durch eine Attrappe (kein echter Lauf).

// fakeStream setzt streamExe für den Test und merkt sich die Optionen des Aufrufs.
func fakeStream(t *testing.T, f func(context.Context, gameexe.Options) error) *gameexe.Options {
	t.Helper()
	var got gameexe.Options
	old := streamExe
	streamExe = func(ctx context.Context, o gameexe.Options) error { got = o; return f(ctx, o) }
	t.Cleanup(func() { streamExe = old })
	return &got
}

func TestExeArgsOhneUnterbefehlMitAbsolutenPfaden(t *testing.T) {
	dir := t.TempDir()
	got := exeArgs(dir, []string{"balance", "--targets", "--report-dir", "reports/simtest-run-1", "--seeds", "2", "--baseline", "reports/b.json"})
	want := []string{"--targets", "--report-dir", filepath.Join(dir, "reports", "simtest-run-1"), "--seeds", "2",
		"--baseline", filepath.Join(dir, "reports", "b.json")}
	if !slices.Equal(got, want) {
		t.Errorf("exeArgs: %v", got)
	}
}

func TestRunExeHeadlessUndMitFenster(t *testing.T) {
	var lines []string
	out := func(stream, text string) { lines = append(lines, stream+": "+text) }
	got := fakeStream(t, func(_ context.Context, o gameexe.Options) error {
		o.OnEvent(gameexe.Event{Event: "progress", Done: 1, Total: 2})
		o.OnStderr("hallo")
		return nil
	})
	dir := t.TempDir()
	if res := runExe(context.Background(), Proc{Dir: dir, Args: []string{"balance", "--targets"}, Timeout: time.Minute, Out: out}); res != (Result{}) {
		t.Errorf("headless: %+v", res)
	}
	if got.Sub != "balance" || got.Window || got.Speed != 0 || got.Root != dir || !slices.Equal(got.Args, []string{"--targets"}) || string(got.Input) != "{}" {
		t.Errorf("headless: Optionen %+v", *got)
	}
	if strings.Join(lines, "|") != `stdout: {"event":"progress","done":1,"total":2}|stderr: hallo` {
		t.Errorf("Zeilen: %q", lines)
	}
	runExe(context.Background(), Proc{Dir: dir, Args: []string{"balance"}, Head: true, Speed: 8, Timeout: time.Minute})
	if !got.Window || got.Speed != 8 {
		t.Errorf("mit Fenster: Optionen %+v", *got)
	}
}

func TestRunExeFehlerAbbruchUndZeitlimit(t *testing.T) {
	var lines []string
	out := func(stream, text string) { lines = append(lines, stream+": "+text) }
	fakeStream(t, func(context.Context, gameexe.Options) error { return errors.New("welt fehlt") })
	if res := runExe(context.Background(), Proc{Dir: t.TempDir(), Timeout: time.Minute, Out: out}); res.Exit != 1 ||
		!strings.Contains(strings.Join(lines, "\n"), "stderr: ❌ k3c-game sim: welt fehlt") {
		t.Errorf("Fehler: %+v %v", res, lines)
	}
	fakeStream(t, func(ctx context.Context, _ gameexe.Options) error { <-ctx.Done(); return ctx.Err() })
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if res := runExe(ctx, Proc{Dir: t.TempDir(), Timeout: time.Minute}); res.Exit != 130 || res.TimedOut {
		t.Errorf("Abbruch: %+v", res)
	}
	if res := runExe(context.Background(), Proc{Dir: t.TempDir(), Timeout: time.Millisecond}); !res.TimedOut {
		t.Errorf("Zeitlimit: %+v", res)
	}
}

// AC-03: Stop eines Laufs bricht den Aufruf der EXE ab (stdin zu, danach Job-Objekt: gameexe, TestHarterAbbruch…).
func TestStopBeendetLaufDerExe(t *testing.T) {
	started := make(chan struct{})
	got := fakeStream(t, func(ctx context.Context, _ gameexe.Options) error {
		close(started)
		<-ctx.Done()
		return ctx.Err()
	})
	g := New(nil)
	root := t.TempDir()
	three := 3
	run, err := g.StartOffline(root, "", Spec{Seeds: 2, Depth: &three, Head: true, Speed: 4})
	if err != nil {
		t.Fatal(err)
	}
	<-started
	if !got.Window || got.Speed != 4 || !slices.Contains(got.Args, "--depth") {
		t.Errorf("Optionen %+v", *got)
	}
	if _, err := g.Stop(root, run.ID); err != nil {
		t.Fatal(err)
	}
	waitFor(t, g, root, run.ID, func(i Info) bool { return i.State == Canceled })
}

func TestFensterLaufLaeuftAllein(t *testing.T) {
	root := t.TempDir()
	g := New(nil)
	release := make(chan struct{})
	defer close(release)
	wait := func(ctx context.Context, _ *Run) {
		select {
		case <-ctx.Done():
		case <-release:
		}
	}
	if _, err := g.Start(root, "", "a", &Spec{}, wait, false); err != nil {
		t.Fatal(err)
	}
	if _, err := g.Start(root, "", "h", &Spec{Head: true}, wait, false); !errors.Is(err, ErrFull) {
		t.Errorf("Fensterlauf neben laufendem Lauf: %v", err)
	}
}

func TestNachFensterLaufStartetKeinZweiter(t *testing.T) {
	root := t.TempDir()
	g := New(nil)
	release := make(chan struct{})
	defer close(release)
	wait := func(ctx context.Context, _ *Run) {
		select {
		case <-ctx.Done():
		case <-release:
		}
	}
	if _, err := g.Start(root, "", "h", &Spec{Head: true}, wait, false); err != nil {
		t.Fatal(err)
	}
	if _, err := g.Start(root, "", "a", &Spec{}, wait, false); !errors.Is(err, ErrFull) {
		t.Errorf("Lauf neben Fensterlauf: %v", err)
	}
}

func TestSpecHeadSpeedPruefung(t *testing.T) {
	for _, s := range []Spec{{Speed: 2}, {Head: true, Speed: 65}, {Head: true, Speed: -1}, {Head: true, Tune: &Tune{Mode: "sweep", Params: []string{"a.json:b=1..2"}}}} {
		if err := s.Check(); err == nil {
			t.Errorf("%+v angenommen", s)
		}
	}
	if err := (Spec{Head: true, Speed: 64}).Check(); err != nil {
		t.Error(err)
	}
	if slices.Contains((Spec{Head: true, Speed: 4}).args("r"), "--window") {
		t.Error("--window gehört in Options.Window, nicht in args")
	}
}
