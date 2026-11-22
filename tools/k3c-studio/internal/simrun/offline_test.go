package simrun

import (
	"context"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"
)

// Falsche Engine (WZ28.4/AC-04, B-924): g.Exec rechnet `balance --targets` nicht, sondern schreibt einen festen
// Bericht und meldet Events wie runEngine; die Rechnung selbst prüft engine/simcli. runEngine prüft der Test unten.

// fakeRegistry: g.Exec spielt je nach mode einen Lauf (ok, slow, crash, error).
func fakeRegistry(mode string) *Registry {
	g := New(nil)
	g.Exec = func(ctx context.Context, p Proc) Result {
		dir := filepath.Join(p.Dir, p.Args[slices.Index(p.Args, "--report-dir")+1])
		if err := os.MkdirAll(dir, 0o755); err != nil {
			return Result{Err: err}
		}
		_ = os.WriteFile(filepath.Join(dir, "args.txt"), []byte(strings.Join(p.Args, "\n")), 0o644)
		write := func() {
			raw := `{"version":1,"seeds":2,"pass":true,"varianten":["gegner.json:a.b=2"],"targets":[` +
				`{"name":"Ziel","kind":"share","value":80,"lower":75,"upper":90,"pass":true}]}`
			_ = os.WriteFile(filepath.Join(dir, "balance-1.json"), []byte(raw), 0o644)
		}
		switch mode {
		case "slow":
			p.Out("stdout", `{"event":"progress","done":1,"total":3}`)
			write()
			<-ctx.Done()
			return Result{Exit: 130}
		case "crash":
			p.Out("stdout", `{"event":"progress","done":1,"total":2}`)
			p.Out("stderr", "❌ k3c-sim: 💥 Absturz in der Engine: boom")
			return Result{Exit: 1}
		case "error":
			p.Out("stdout", `{"event":"error","message":"welt \"x\" fehlt"}`)
			p.Out("stderr", `❌ k3c-sim: welt "x" fehlt`)
			return Result{Exit: 1}
		}
		p.Out("stdout", `{"event":"progress","done":1,"total":2}`)
		p.Out("stdout", `{"event":"progress","done":2,"total":2}`)
		write()
		return Result{}
	}
	return g
}

// B-924: runEngine rechnet im Prozess; ein Fehler der Engine wird Exit 1 mit der Meldung auf stderr, ein beendeter
// Context ein Abbruch, das Zeitlimit ein TimedOut. Kein Lauf wird gerechnet (unbekannter Unterbefehl).
func TestRunEngineFehlerAbbruchUndZeitlimit(t *testing.T) {
	var lines []string
	out := func(stream, text string) { lines = append(lines, stream+": "+text) }
	res := runEngine(context.Background(), Proc{Dir: t.TempDir(), Args: []string{"fliegen"}, Timeout: time.Minute, Out: out})
	if res.Exit != 1 || len(lines) < 2 || !strings.Contains(strings.Join(lines, "\n"), "stderr: ❌ k3c-sim: unbekannter Unterbefehl") {
		t.Errorf("Fehler: %+v %v", res, lines)
	}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if res := runEngine(ctx, Proc{Dir: t.TempDir(), Args: []string{"level"}, Timeout: time.Minute}); res.Exit != 130 {
		t.Errorf("Abbruch: %+v", res)
	}
	if res := runEngine(context.Background(), Proc{Dir: t.TempDir(), Args: []string{"level"}, Timeout: -time.Second}); !res.TimedOut {
		t.Errorf("Zeitlimit: %+v", res)
	}
}

// waitFor fragt den Lauf ab, bis ok gilt.
func waitFor(t *testing.T, g *Registry, root, id string, ok func(Info) bool) Info {
	t.Helper()
	deadline := time.Now().Add(30 * time.Second)
	for time.Now().Before(deadline) {
		info, err := g.Get(root, id)
		if err != nil {
			t.Fatal(err)
		}
		if ok(info) {
			return info
		}
		time.Sleep(10 * time.Millisecond)
	}
	info, _ := g.Get(root, id)
	t.Fatalf("Zustand nicht erreicht: %+v", info)
	return Info{}
}

func TestOfflineLaufMitFortschrittUndBericht(t *testing.T) {
	root := t.TempDir()
	g := fakeRegistry("ok")
	spec := Spec{Seeds: 2, Depth: new(0), Bot: "saver", Vary: []string{"gegner.json:a.b=2"}, Label: "Probe"}
	info, err := g.StartOffline(root, "Repo-Wurzel", spec)
	if err != nil || info.State != Running || info.Label != "Probe" {
		t.Fatalf("start: %+v, %v", info, err)
	}
	info = waitFor(t, g, root, info.ID, func(i Info) bool { return i.Report != "" })
	if info.State != Done || info.Verdict != "Pass" || info.Done != 2 || info.Total != 2 {
		t.Errorf("ende: %+v", info)
	}
	if !slices.Contains(info.Lines, "Pass Ziel: 80 % (75–90 %)") {
		t.Errorf("kennzahlen: %v", info.Lines)
	}
	if _, err := os.Stat(filepath.Join(root, info.Report)); err != nil {
		t.Error(err)
	}
	raw, _ := os.ReadFile(filepath.Join(root, "reports", "simtest-"+info.ID, "args.txt"))
	for _, want := range []string{"balance\n--targets", "--progress", "--seeds\n2", "--depth\n0", "--bot\nsaver", "--vary\ngegner.json:a.b=2"} {
		if !strings.Contains(string(raw), want) {
			t.Errorf("argumente ohne %q:\n%s", want, raw)
		}
	}
}

func TestOfflineAbbruchLiefertTeilbericht(t *testing.T) {
	root := t.TempDir()
	g := fakeRegistry("slow")
	info, err := g.StartOffline(root, "Repo-Wurzel", Spec{})
	if err != nil {
		t.Fatal(err)
	}
	waitFor(t, g, root, info.ID, func(i Info) bool { return i.Done == 1 && i.Total == 3 })
	time.Sleep(200 * time.Millisecond) // balance-1.json steht nach der Fortschrittszeile
	begin := time.Now()
	if before, err := g.Stop(root, info.ID); err != nil || before != Running {
		t.Fatalf("stop: %q, %v", before, err)
	}
	info = waitFor(t, g, root, info.ID, func(i Info) bool { return i.Report != "" })
	if time.Since(begin) > 15*time.Second {
		t.Errorf("abbruch dauerte %s", time.Since(begin))
	}
	if info.State != Canceled || !slices.Contains(info.Lines, "Teilbericht:") || info.Verdict != "Pass" {
		t.Errorf("nach abbruch: %+v", info)
	}
}

func TestOfflineAlterBerichtZaehltNicht(t *testing.T) {
	root := t.TempDir()
	dir := filepath.Join(root, "reports", "simtest-run-1")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	old := filepath.Join(dir, "balance-0.json")
	if err := os.WriteFile(old, []byte(`{"seeds":1,"pass":true}`), 0o644); err != nil {
		t.Fatal(err)
	}
	past := time.Now().Add(-time.Hour)
	if err := os.Chtimes(old, past, past); err != nil {
		t.Fatal(err)
	}
	if _, _, err := readBalanceSummary(dir, time.Now()); err == nil {
		t.Error("alter Bericht gelesen")
	}
}

// RS2.2: Ein Absturz oder Fehler von k3c-sim beendet den Lauf als fehlgeschlagen; der Status nennt Exit und Grund.
func TestOfflineAbsturzNenntStderr(t *testing.T) {
	for mode, want := range map[string][]string{"crash": {"rechnen: Exit 1", "💥 Absturz in der Engine: boom"},
		"error": {"rechnen: Exit 1", `❌ k3c-sim: welt "x" fehlt`}} {
		root := t.TempDir()
		g := fakeRegistry(mode)
		info, err := g.StartOffline(root, "Repo-Wurzel", Spec{})
		if err != nil {
			t.Fatal(err)
		}
		info = waitFor(t, g, root, info.ID, func(i Info) bool { return i.State != Running && i.State != Waiting })
		text := strings.Join(info.Lines, "\n")
		if info.State != Failed || !strings.Contains(text, want[0]) || !strings.Contains(text, want[1]) {
			t.Errorf("%s: %+v", mode, info)
		}
	}
}
