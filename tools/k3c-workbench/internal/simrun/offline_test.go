package simrun

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"
)

// Fake-Binary (WZ28.4/AC-04): Der Testprozess selbst spielt die Balance-CLI (Hilfsprozess-Muster). „Bauen“ entfällt,
// „rechnen“ startet ihn als echten Kindprozess über runProcess (Job-Objekt, Abbruch, Ausgabe).

// TestHelperProcess ist die falsche Balance-CLI; ohne SIMRUN_HELPER tut er nichts.
func TestHelperProcess(t *testing.T) {
	if os.Getenv("SIMRUN_HELPER") != "1" {
		return
	}
	args := os.Args[slices.Index(os.Args, "--")+1:]
	dir := args[slices.Index(args, "--report-dir")+1]
	if err := os.MkdirAll(dir, 0o755); err != nil {
		os.Exit(3)
	}
	_ = os.WriteFile(filepath.Join(dir, "args.txt"), []byte(strings.Join(args, "\n")), 0o644)
	write := func() {
		raw := `{"version":1,"seeds":2,"pass":true,"varianten":["gegner.json:a.b=2"],"targets":[` +
			`{"name":"Ziel","kind":"share","value":80,"lower":75,"upper":90,"pass":true}]}`
		_ = os.WriteFile(filepath.Join(dir, "balance-1.json"), []byte(raw), 0o644)
	}
	switch os.Getenv("SIMRUN_FAKE") {
	case "slow":
		fmt.Println("PROGRESS 1/3")
		write()
		time.Sleep(time.Minute)
	default:
		fmt.Println("PROGRESS 1/2")
		fmt.Println("PROGRESS 2/2")
		write()
	}
	os.Exit(0)
}

// fakeRegistry: Bauen ist sofort fertig, Rechnen startet TestHelperProcess.
func fakeRegistry(mode string) *Registry {
	g := New(nil)
	g.Exec = func(ctx context.Context, p Proc) Result {
		if p.Args[0] == "go" {
			return Result{}
		}
		p.Args = append([]string{os.Args[0], "-test.run=^TestHelperProcess$", "--"}, p.Args...)
		p.Env = append(p.Env, "SIMRUN_HELPER=1", "SIMRUN_FAKE="+mode)
		return runProcess(ctx, p)
	}
	return g
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
	for _, want := range []string{"--targets", "--progress", "--seeds\n2", "--depth\n0", "--bot\nsaver", "--vary\ngegner.json:a.b=2"} {
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
	time.Sleep(200 * time.Millisecond) // balance-1.json steht nach der PROGRESS-Zeile
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
	if _, err := readBalanceSummary(dir, time.Now()); err == nil {
		t.Error("alter Bericht gelesen")
	}
}
