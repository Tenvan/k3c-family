package main

import (
	"context"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/signal"
	"path/filepath"
	"testing"
	"time"
)

func quietLog() *slog.Logger { return slog.New(slog.NewTextHandler(io.Discard, nil)) }

func write(t *testing.T, path, text string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

func TestParseArgs(t *testing.T) {
	cfg, err := parseArgs([]string{"--paths", "cmd, engine", "--build", "task x:build", "--", "bin/x.exe", "-v"}, io.Discard)
	if err != nil {
		t.Fatal(err)
	}
	if len(cfg.Paths) != 2 || cfg.Paths[1] != "engine" || len(cfg.Build) != 2 || cfg.Cmd[0] != "bin/x.exe" || cfg.Cmd[1] != "-v" {
		t.Fatalf("falsch gelesen: %+v", cfg)
	}
	if cfg.Name != "bin/x.exe" {
		t.Errorf("Name = %q, will den EXE-Pfad als Vorgabe", cfg.Name)
	}
	for _, bad := range [][]string{{"--", "x.exe"}, {"--paths", "a"}, {"--paths", "a", "--poll", "0s", "--", "x"}} {
		if _, err := parseArgs(bad, io.Discard); err == nil {
			t.Errorf("%v: Fehler erwartet", bad)
		}
	}
}

func TestSnapshotFiltertEndungenTestsUndVersteckte(t *testing.T) {
	dir := t.TempDir()
	write(t, filepath.Join(dir, "a.go"), "x")
	write(t, filepath.Join(dir, "a_test.go"), "x")
	write(t, filepath.Join(dir, "note.md"), "x")
	write(t, filepath.Join(dir, ".git", "b.go"), "x")
	write(t, filepath.Join(dir, "node_modules", "c.go"), "x")
	write(t, filepath.Join(dir, "sub", "d.json"), "x")
	cfg := config{Paths: []string{dir, filepath.Join(dir, "note.md"), filepath.Join(dir, "fehlt")}, Exts: []string{".go", ".json"}}
	got := snapshot(cfg)
	want := []string{"a.go", "note.md", "sub/d.json"} // note.md steht einzeln da, also zählt es trotz Endung
	if len(got) != len(want) {
		t.Fatalf("snapshot = %v, will %v", got, want)
	}
	for _, w := range want {
		if _, ok := got[filepath.ToSlash(filepath.Join(dir, w))]; !ok {
			t.Errorf("%s fehlt in %v", w, got)
		}
	}
}

func TestSnapshotNimmtAusgenommenesAus(t *testing.T) {
	dir := t.TempDir()
	write(t, filepath.Join(dir, "a.json"), "x")
	write(t, filepath.Join(dir, "worlds", "w.json"), "x")
	write(t, filepath.Join(dir, "worlds2", "w.json"), "x")
	cfg := config{Paths: []string{dir}, Skip: []string{filepath.Join(dir, "worlds"), filepath.Join(dir, "a.json")}}
	got := snapshot(cfg)
	if len(got) != 1 {
		t.Fatalf("snapshot = %v, will nur worlds2/w.json", got)
	}
}

func TestChanged(t *testing.T) {
	a := map[string]fileSig{"x": {1, 1}, "y": {1, 1}}
	if got := changed(a, map[string]fileSig{"x": {1, 1}, "y": {1, 1}}); got != "" {
		t.Errorf("gleich → %q", got)
	}
	if got := changed(a, map[string]fileSig{"x": {2, 1}, "y": {1, 1}}); got != "x" {
		t.Errorf("geändert → %q", got)
	}
	if got := changed(a, map[string]fileSig{"y": {1, 1}}); got != "x" {
		t.Errorf("gelöscht → %q", got)
	}
	if got := changed(a, map[string]fileSig{"x": {1, 1}, "y": {1, 1}, "a": {1, 1}}); got != "a" {
		t.Errorf("neu → %q", got)
	}
}

// fakeProc zählt Stops; start liefert eines, ohne ein Programm zu starten.
type fakeProc struct {
	stopped int
	done    chan struct{}
}

func (f *fakeProc) Stop()                 { f.stopped++ }
func (f *fakeProc) Done() <-chan struct{} { return f.done }
func newFake() *fakeProc                  { return &fakeProc{done: make(chan struct{})} }
func exeSupervisor(exe string) *supervisor { // Build schreibt "neu", Start liefert ein fakeProc
	s := newSupervisor(config{Cmd: []string{exe}, Build: []string{"build"}}, quietLog())
	return s
}

func TestRebuildTauschtEXEUndStopptDasAlteProgramm(t *testing.T) {
	exe := filepath.Join(t.TempDir(), "app.exe")
	write(t, exe, "alt")
	s := exeSupervisor(exe)
	old := newFake()
	s.cur = old
	var started *fakeProc
	s.build = func(context.Context) error { write(t, exe, "neu"); return nil }
	s.start = func() (proc, error) { started = newFake(); return started, nil }

	s.rebuild(context.Background())

	if b, _ := os.ReadFile(exe); string(b) != "neu" {
		t.Errorf("EXE = %q, will neu", b)
	}
	if _, err := os.Stat(exe + ".old"); !errors.Is(err, os.ErrNotExist) {
		t.Errorf(".old bleibt liegen: %v", err)
	}
	if old.stopped != 1 || started == nil || s.cur != proc(started) {
		t.Errorf("altes Programm gestoppt %d×, neues gestartet %v", old.stopped, started != nil)
	}
}

func TestRebuildMitBuildFehlerLaesstDieAlteVersionLaufen(t *testing.T) {
	exe := filepath.Join(t.TempDir(), "app.exe")
	write(t, exe, "alt")
	s := exeSupervisor(exe)
	old := newFake()
	s.cur = old
	s.build = func(context.Context) error { write(t, exe, "halb"); return errors.New("kaputt") }
	s.start = func() (proc, error) { t.Error("kein Start nach Build-Fehler"); return nil, nil }

	s.rebuild(context.Background())

	if b, _ := os.ReadFile(exe); string(b) != "alt" {
		t.Errorf("EXE = %q, will die alte Version zurück", b)
	}
	if old.stopped != 0 || s.cur != proc(old) {
		t.Errorf("altes Programm wurde gestoppt (%d×) oder ersetzt", old.stopped)
	}
}

func TestRebuildOhneBuildLaesstDieEXEStehen(t *testing.T) {
	exe := filepath.Join(t.TempDir(), "app.exe")
	write(t, exe, "alt")
	s := newSupervisor(config{Cmd: []string{exe}}, quietLog())
	s.start = func() (proc, error) { return newFake(), nil }
	s.rebuild(context.Background())
	if b, _ := os.ReadFile(exe); string(b) != "alt" {
		t.Errorf("EXE = %q, will unverändert", b)
	}
	if s.cur == nil {
		t.Error("das Programm läuft nicht")
	}
}

func TestRebuildMitAbbruchImBuildStelltDieEXEWiederHer(t *testing.T) {
	exe := filepath.Join(t.TempDir(), "app.exe")
	write(t, exe, "alt")
	s := exeSupervisor(exe)
	old := newFake()
	s.cur = old
	ctx, cancel := context.WithCancel(context.Background())
	s.build = func(ctx context.Context) error {
		write(t, exe, "halb")
		cancel()
		<-ctx.Done()
		return ctx.Err()
	}
	s.start = func() (proc, error) { t.Error("kein Start nach Abbruch"); return nil, nil }
	s.rebuild(ctx)
	if b, _ := os.ReadFile(exe); string(b) != "alt" || old.stopped != 0 {
		t.Errorf("EXE = %q, altes Programm %d× gestoppt", b, old.stopped)
	}
}

func TestReapVergisstBeendetesProgramm(t *testing.T) {
	s := exeSupervisor("x")
	f := newFake()
	s.cur = f
	s.reap()
	if s.cur == nil {
		t.Fatal("laufendes Programm vergessen")
	}
	close(f.done)
	s.reap()
	if s.cur != nil {
		t.Error("beendetes Programm nicht vergessen")
	}
}

func TestRestoreAsideMeldetFehler(t *testing.T) {
	dir := t.TempDir()
	if err := restoreAside(filepath.Join(dir, "a.exe"), filepath.Join(dir, "fehlt.old")); err == nil {
		t.Error("Fehler beim Zurückbenennen erwartet")
	}
}

func TestRebuildOhneBisherigeEXE(t *testing.T) {
	exe := filepath.Join(t.TempDir(), "app.exe")
	s := exeSupervisor(exe)
	s.build = func(context.Context) error { write(t, exe, "neu"); return nil }
	s.start = func() (proc, error) { return newFake(), nil }
	s.rebuild(context.Background())
	if s.cur == nil {
		t.Error("das Programm läuft nicht")
	}
}

func TestRunBautBeiAenderungNeu(t *testing.T) {
	dir := t.TempDir()
	src := filepath.Join(dir, "main.go")
	write(t, src, "v1")
	exe := filepath.Join(dir, "app.exe")
	cfg := config{Paths: []string{dir}, Exts: []string{".go"}, Cmd: []string{exe}, Build: []string{"build"},
		Poll: 10 * time.Millisecond, Quiet: 20 * time.Millisecond}
	s := newSupervisor(cfg, quietLog())
	builds := make(chan int, 10)
	n := 0
	s.build = func(context.Context) error { n++; write(t, exe, "x"); builds <- n; return nil }
	s.start = func() (proc, error) { return newFake(), nil }
	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan error)
	go func() { done <- s.run(ctx) }()

	wait := func(want int) {
		t.Helper()
		select {
		case got := <-builds:
			if got != want {
				t.Fatalf("Build %d, will %d", got, want)
			}
		case <-time.After(5 * time.Second):
			t.Fatalf("Build %d blieb aus", want)
		}
	}
	wait(1)
	time.Sleep(50 * time.Millisecond)
	write(t, src, "v2 mit anderer Größe")
	wait(2)
	cancel()
	if err := <-done; err != nil {
		t.Fatal(err)
	}
}

// Hilfsprogramm der Stop-Tests: der Test startet sich selbst mit K3C_WATCH_HELPER (kein eigenes Binary nötig).
func TestMain(m *testing.M) {
	switch os.Getenv("K3C_WATCH_HELPER") {
	case "graceful": // schreibt eine Marke und endet, sobald es um ein Ende gebeten wird
		c := make(chan os.Signal, 1)
		signal.Notify(c, os.Interrupt)
		fmt.Println("bereit")
		<-c
		_ = os.WriteFile(os.Getenv("K3C_WATCH_MARK"), []byte("ok"), 0o644)
		os.Exit(0)
	case "stubborn": // ignoriert die Bitte
		signal.Notify(make(chan os.Signal, 1), os.Interrupt) // fängt das Signal ab und tut nichts
		fmt.Println("bereit")
		time.Sleep(time.Minute)
	}
	os.Exit(m.Run())
}

func startHelper(t *testing.T, mode string, grace time.Duration) (proc, string) {
	t.Helper()
	mark := filepath.Join(t.TempDir(), "mark")
	t.Setenv("K3C_WATCH_HELPER", mode)
	t.Setenv("K3C_WATCH_MARK", mark)
	exe, err := os.Executable()
	if err != nil {
		t.Fatal(err)
	}
	p, err := startProc([]string{exe}, grace)
	if err != nil {
		t.Fatal(err)
	}
	time.Sleep(500 * time.Millisecond) // Handler des Kindes ist installiert, bevor das Signal kommt
	return p, mark
}

func TestStopBeendetSauber(t *testing.T) {
	p, mark := startHelper(t, "graceful", 10*time.Second)
	if err := requestStop(p.(*childProc).cmd); err != nil {
		t.Skipf("kein Signal zustellbar (keine Konsole?): %v", err)
	}
	start := time.Now()
	p.Stop() // das Signal ist schon raus, Stop wartet nur noch
	if _, err := os.Stat(mark); err != nil {
		t.Errorf("das Programm wurde nicht sauber beendet: %v", err)
	}
	if time.Since(start) > 5*time.Second {
		t.Errorf("Stop wartete die Frist ab: %v", time.Since(start))
	}
}

func TestStopToetetNachDerFristHart(t *testing.T) {
	p, mark := startHelper(t, "stubborn", 700*time.Millisecond)
	start := time.Now()
	p.Stop()
	select {
	case <-p.Done():
	default:
		t.Fatal("das Programm läuft noch")
	}
	if _, err := os.Stat(mark); err == nil {
		t.Error("ein hart beendetes Programm schreibt keine Marke")
	}
	if d := time.Since(start); d < 600*time.Millisecond {
		t.Errorf("Kill vor Ablauf der Frist: %v", d)
	}
}
