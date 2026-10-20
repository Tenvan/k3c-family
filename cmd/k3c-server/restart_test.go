package main

import (
	"bytes"
	"errors"
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"

	"k3c/data"
)

// Gültiger neuer Stand: Hinweis an alle Geräte, dann die Frist, erst danach der Neustart (B-507/AC-04).
func TestNeustartNachHinweis(t *testing.T) {
	var steps []string
	var logs bytes.Buffer
	r := restarter{
		log:      slog.New(slog.NewTextHandler(&logs, nil)),
		check:    func() error { steps = append(steps, "prüfen"); return nil },
		announce: func() { steps = append(steps, "hinweis") },
		wait:     func(d time.Duration) { steps = append(steps, "warten "+d.String()) },
		restart:  func() { steps = append(steps, "neustart") },
	}
	r.changed([]string{"data/worlds/nord.json"})
	want := []string{"prüfen", "hinweis", "warten " + restartDelay.String(), "neustart"}
	if !slices.Equal(steps, want) {
		t.Fatalf("Ablauf %v, erwartet %v", steps, want)
	}
	if !strings.Contains(logs.String(), "🔁") || !strings.Contains(logs.String(), "⏳") {
		t.Errorf("Log ohne 🔁/⏳: %s", logs.String())
	}
}

// Ungültiger Stand: kein Hinweis, kein Neustart, der Fehler steht im Log (❌), der Server läuft weiter.
func TestUngueltigerStandKeinNeustart(t *testing.T) {
	var logs bytes.Buffer
	r := restarter{
		log:      slog.New(slog.NewTextHandler(&logs, nil)),
		check:    func() error { return errors.New("data/economy.json: kein gültiges JSON") },
		announce: func() { t.Error("Hinweis trotz ungültigem Stand") },
		wait:     func(time.Duration) { t.Error("Frist trotz ungültigem Stand") },
		restart:  func() { t.Error("Neustart trotz ungültigem Stand") },
	}
	r.changed([]string{"data/economy.json"})
	if !strings.Contains(logs.String(), "❌") || !strings.Contains(logs.String(), "economy.json") {
		t.Errorf("Log: %s", logs.String())
	}
}

// Ordner, in die der Server schreibt (Spielstände, Berichte, Logs), beobachtet er nicht: Er schreibt nie in
// beobachtete Ordner. Fehlende Ordner fallen weg.
func TestKeinSchreibenInBeobachteteOrdner(t *testing.T) {
	root := t.TempDir()
	cfg := config{data: filepath.Join(root, "data"), public: filepath.Join(root, "public")}
	for _, d := range []string{cfg.data, cfg.public} {
		if err := os.MkdirAll(d, 0o755); err != nil {
			t.Fatal(err)
		}
	}
	log := slog.New(slog.NewTextHandler(io.Discard, nil))
	names := func(writes ...string) []string {
		var out []string
		for _, r := range watchRoots(cfg, writes, log) {
			out = append(out, r.Name)
		}
		return out
	}
	if got := names(filepath.Join(root, "saves"), ""); !slices.Equal(got, []string{"data", "public"}) {
		t.Errorf("getrennte Ordner: %v", got)
	}
	if got := names(filepath.Join(cfg.data, "saves")); !slices.Equal(got, []string{"public"}) {
		t.Errorf("Spielstände in data/: %v", got)
	}
	if got := names(root); len(got) != 0 {
		t.Errorf("Log-Ordner über allem: %v", got)
	}
	cfg.public = filepath.Join(root, "fehlt")
	if got := names(); !slices.Equal(got, []string{"data"}) {
		t.Errorf("fehlender Ordner: %v", got)
	}
}

// fakeRun ist ein Kind im Test: Laufzeit in der eingespeisten Uhr, Exit-Code und optional ein Signal während des Laufs.
type fakeRun struct {
	ran    time.Duration
	code   int
	signal bool
}

// runSupervisor spielt die Kinder runs der Reihe nach; liefert Exit-Code, Modus je Start und weitergereichte Signale.
func runSupervisor(t *testing.T, embedded bool, runs ...fakeRun) (int, []bool, int) {
	t.Helper()
	tnow := time.Unix(0, 0)
	sigs := make(chan os.Signal, 2)
	var modes []bool
	forwarded := 0
	start := func(e bool) (proc, error) {
		if len(runs) == 0 {
			t.Fatal("Kind zu oft gestartet")
		}
		r := runs[0]
		runs = runs[1:]
		modes = append(modes, e)
		done := make(chan int, 1)
		p := proc{done: done, signal: func(os.Signal) { forwarded++; done <- r.code }}
		tnow = tnow.Add(r.ran)
		if r.signal {
			sigs <- os.Interrupt
		} else {
			done <- r.code
		}
		return p, nil
	}
	clk := testClock(&tnow, nil)
	code := supervise(start, sigs, clk, embedded, slog.New(slog.NewTextHandler(io.Discard, nil)))
	return code, modes, forwarded
}

// Neu gestartet wird bei restartCode; ein früher Absturz mit Daten von der Platte startet einmal eingebettet.
func TestSupervisorNeustartUndRueckfall(t *testing.T) {
	long := time.Minute
	if code, modes, _ := runSupervisor(t, false, fakeRun{long, restartCode, false}, fakeRun{long, restartCode, false}, fakeRun{long, 0, false}); code != 0 || !slices.Equal(modes, []bool{false, false, false}) {
		t.Errorf("Neustarts: %d %v", code, modes)
	}
	if code, modes, _ := runSupervisor(t, false, fakeRun{time.Second, 2, false}, fakeRun{long, restartCode, false}, fakeRun{time.Second, 1, false}, fakeRun{time.Second, 1, false}); code != 1 || !slices.Equal(modes, []bool{false, true, false, true}) {
		t.Errorf("früher Absturz mit Platte: %d %v", code, modes)
	}
	if code, modes, _ := runSupervisor(t, true, fakeRun{time.Second, 1, false}); code != 1 || !slices.Equal(modes, []bool{true}) {
		t.Errorf("eingebettet: %d %v", code, modes)
	}
	log := slog.New(slog.NewTextHandler(io.Discard, nil))
	if code := supervise(func(bool) (proc, error) { return proc{}, errors.New("kein exe") }, nil, clock{time.Now, time.After}, false, log); code != 1 {
		t.Errorf("Start scheitert: %d", code)
	}
}

// Ende von außen: Kill nach längerer Laufzeit (Windows, Code 1) oder durch ein Signal (Linux, -1) wird nur
// weitergereicht, kein Neustart.
func TestSupervisorEndeVonAussen(t *testing.T) {
	if code, modes, _ := runSupervisor(t, false, fakeRun{time.Minute, 1, false}); code != 1 || len(modes) != 1 {
		t.Errorf("Kill nach einer Minute: %d %v", code, modes)
	}
	if code, modes, _ := runSupervisor(t, false, fakeRun{time.Second, -1, false}); code != 1 || len(modes) != 1 {
		t.Errorf("durch Signal beendet: %d %v", code, modes)
	}
}

// Nach einem Signal startet nichts mehr neu, auch nicht bei restartCode; das Signal geht an das Kind.
func TestSupervisorSignal(t *testing.T) {
	code, modes, forwarded := runSupervisor(t, false, fakeRun{time.Second, restartCode, true})
	if code != 0 || len(modes) != 1 || forwarded != 1 {
		t.Errorf("Signal: Code %d, Starts %v, weitergereicht %d", code, modes, forwarded)
	}
	code, modes, _ = runSupervisor(t, false, fakeRun{time.Second, 0, true})
	if code != 0 || len(modes) != 1 {
		t.Errorf("Strg+C: %d %v", code, modes)
	}
}

// Das Kind bekommt K3C_CHILD und den Daten-Ordner; eingebettet heißt K3C_DATA_DIR leer (spätere Einträge gewinnen).
func TestKindUmgebung(t *testing.T) {
	env := childEnviron([]string{"PATH=x", "K3C_DATA_DIR=alt"}, "/repo/data", false)
	if !slices.Equal(env[len(env)-2:], []string{"K3C_CHILD=1", "K3C_DATA_DIR=/repo/data"}) {
		t.Errorf("Platte: %v", env)
	}
	env = childEnviron(nil, "/repo/data", true)
	if !slices.Equal(env, []string{"K3C_CHILD=1", "K3C_DATA_DIR="}) {
		t.Errorf("eingebettet: %v", env)
	}
}

// -check-data prüft im Dev-Mode jede Welt-Datei; eine kaputte ergibt Exit 1 mit dem Grund.
func TestPruefungWelten(t *testing.T) {
	dir := t.TempDir()
	std, err := data.Files.ReadFile("worlds/standard.json")
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "standard.json"), std, 0o644); err != nil {
		t.Fatal(err)
	}
	cfg := config{worlds: dir, dev: true}
	var out bytes.Buffer
	if code := checkData(cfg, &out); code != 0 {
		t.Fatalf("gültig: %d %s", code, out.String())
	}
	if err := os.WriteFile(filepath.Join(dir, "kaputt.json"), []byte(`{`), 0o644); err != nil {
		t.Fatal(err)
	}
	if code := checkData(cfg, &out); code != 1 || !strings.Contains(out.String(), "kaputt.json") {
		t.Fatalf("kaputt: %d %s", code, out.String())
	}
	cfg.dev = false
	if code := checkData(cfg, &out); code != 0 {
		t.Fatalf("ohne Dev-Mode liest der Server keine Welten: %d", code)
	}
}

// testClock ist eine eingespeiste Uhr; after rückt sie um d vor und meldet sich sofort, pauses zählt die Pausen.
func testClock(now *time.Time, pauses *[]time.Duration) clock {
	return clock{
		now: func() time.Time { return *now },
		after: func(d time.Duration) <-chan time.Time {
			if pauses != nil {
				*pauses = append(*pauses, d)
			}
			*now = now.Add(d)
			ch := make(chan time.Time, 1)
			ch <- *now
			return ch
		},
	}
}

// Schleifenbremse: mehr als 3 Neustarts binnen 60 s ergeben 30 s Pause (❌/⏳ im Log); langsame Neustarts nicht.
func TestSupervisorSchleifenbremse(t *testing.T) {
	run := func(gap time.Duration, n int) ([]time.Duration, string) {
		now := time.Unix(0, 0)
		var pauses []time.Duration
		var logs bytes.Buffer
		starts := 0
		start := func(bool) (proc, error) {
			starts++
			now = now.Add(gap)
			done := make(chan int, 1)
			if starts <= n {
				done <- restartCode
			} else {
				done <- 0
			}
			return proc{done: done, signal: func(os.Signal) {}}, nil
		}
		supervise(start, make(chan os.Signal, 1), testClock(&now, &pauses), false, slog.New(slog.NewTextHandler(&logs, nil)))
		return pauses, logs.String()
	}
	pauses, logs := run(5*time.Second, 5)
	if !slices.Equal(pauses, []time.Duration{loopPause}) || !strings.Contains(logs, "❌") || !strings.Contains(logs, "⏳") {
		t.Errorf("schnelle Neustarts: Pausen %v, Log %s", pauses, logs)
	}
	if pauses, _ := run(30*time.Second, 6); len(pauses) != 0 {
		t.Errorf("langsame Neustarts: Pausen %v", pauses)
	}
	if pauses, _ := run(5*time.Second, 3); len(pauses) != 0 {
		t.Errorf("3 Neustarts: Pausen %v", pauses)
	}
}

// Ein Signal während der Pause beendet den Supervisor ohne Neustart.
func TestSupervisorSignalInPause(t *testing.T) {
	sigs := make(chan os.Signal, 1)
	starts := 0
	start := func(bool) (proc, error) {
		starts++
		done := make(chan int, 1)
		done <- restartCode
		return proc{done: done, signal: func(os.Signal) {}}, nil
	}
	now := time.Unix(0, 0)
	clk := clock{now: func() time.Time { return now }, after: func(time.Duration) <-chan time.Time { sigs <- os.Interrupt; return nil }}
	if code := supervise(start, sigs, clk, false, slog.New(slog.NewTextHandler(io.Discard, nil))); code != 0 || starts != loopMax+1 {
		t.Errorf("Code %d, Starts %d", code, starts)
	}
}
