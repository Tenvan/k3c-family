package simrun

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// blocking ist ein Runner, der bis zum Abbruch oder bis release wartet.
func blocking(g *Registry, release <-chan struct{}) Runner {
	return func(ctx context.Context, run *Run) {
		select {
		case <-ctx.Done():
		case <-release:
		}
		g.Finish(run, Done, "Pass", nil, ReportDir(run))
	}
}

func TestWarteschlange(t *testing.T) {
	root := t.TempDir()
	g := New(nil)
	release := make(chan struct{})
	var ids []string
	for range 4 {
		run, err := g.Start(root, "Repo-Wurzel", "x", nil, blocking(g, release), true)
		if err != nil {
			t.Fatal(err)
		}
		ids = append(ids, run.ID())
	}
	states := func() []string {
		var out []string
		for _, id := range ids {
			info, err := g.Get(root, id)
			if err != nil {
				out = append(out, "weg")
				continue
			}
			out = append(out, info.State)
		}
		return out
	}
	if got := strings.Join(states(), ","); got != "läuft,läuft,wartet,wartet" {
		t.Fatalf("zustände %s", got)
	}
	if before, err := g.Stop(root, ids[2]); err != nil || before != Waiting {
		t.Fatalf("stop wartend: %q, %v", before, err)
	}
	if _, err := g.Get(root, ids[2]); !errors.Is(err, ErrUnknown) {
		t.Errorf("wartender Lauf nach stop noch da: %v", err)
	}
	if _, err := g.Stop(root, ids[0]); err != nil {
		t.Fatal(err)
	}
	waitFor(t, g, root, ids[3], func(i Info) bool { return i.State == Running })
	if info, _ := g.Get(root, ids[0]); info.State != Canceled {
		t.Errorf("abgebrochener Lauf: %+v", info)
	}
	close(release)
	waitFor(t, g, root, ids[3], func(i Info) bool { return i.State == Done && i.Report != "" })
	waitFor(t, g, root, ids[1], func(i Info) bool { return i.Report != "" })
	if n := len(g.List(root)); n != 3 {
		t.Errorf("%d Läufe in der Liste", n)
	}
}

func TestOhneWarteschlangeAbgelehnt(t *testing.T) {
	root := t.TempDir()
	g := New(nil)
	release := make(chan struct{})
	defer close(release)
	wait := func(ctx context.Context, _ *Run) { // ohne Finish: schreibt nichts in den Checkout
		select {
		case <-ctx.Done():
		case <-release:
		}
	}
	for range MaxPerRoot {
		if _, err := g.Start(root, "", "x", nil, wait, false); err != nil {
			t.Fatal(err)
		}
	}
	if _, err := g.Start(root, "", "x", nil, wait, false); !errors.Is(err, ErrFull) {
		t.Errorf("dritter Lauf: %v", err)
	}
	if _, err := g.Start(t.TempDir(), "", "x", nil, wait, false); err != nil {
		t.Errorf("anderer Checkout blockiert: %v", err)
	}
}

func TestSpecLehntAb(t *testing.T) {
	bad := []Spec{
		{Seeds: 201}, {Depth: new(-1)}, {Players: 5}, {Days: 101}, {Bot: "zauberer"}, {Grade: "A B"},
		{Vary: []string{"../data/x.json:a=1"}}, {Vary: []string{"x.json:a=1 2"}}, {Vary: []string{"x.txt:a=1"}},
		{Vary: []string{"x.json:a"}}, {Label: strings.Repeat("x", 61)}, {Label: "a\nb"},
	}
	for _, s := range bad {
		if err := s.Check(); err == nil {
			t.Errorf("%+v angenommen", s)
		}
	}
	if err := (Spec{Seeds: 3, Bot: "saver", Grade: "leicht", Vary: []string{"gegner.json:wolf.hp=12.5"}}).Check(); err != nil {
		t.Error(err)
	}
}

// reportsFixture legt Berichte an: zwei gültige, einen kaputten und eine Datei außerhalb von reports/.
func reportsFixture(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	write := func(rel, body string) {
		p := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(body), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write("reports/balance-1.json", `{"version":1,"seeds":4,"pass":false,"targets":[]}`)
	write("reports/simtest-run-2/balance-2.json", `{"version":1,"seeds":2,"pass":true,"varianten":["a.json:b=1"]}`)
	write("reports/kaputt/balance-3.json", `{`)
	write("geheim.json", `{}`)
	return root
}

func TestBerichtPfadpruefung(t *testing.T) {
	root := reportsFixture(t)
	for _, bad := range []string{"", "geheim.json", "../geheim.json", "reports/../geheim.json", "reports/x.md",
		root + "/reports/balance-1.json", "/reports/balance-1.json", "reports/nichtda.json", "reports/simtest-run-2"} {
		if _, err := ReadReport(root, bad); err == nil {
			t.Errorf("%q gelesen", bad)
		}
	}
	sum, err := ReadReport(root, "reports/simtest-run-2/balance-2.json")
	if err != nil || sum.Seeds != 2 || !sum.Pass {
		t.Errorf("bericht: %+v, %v", sum, err)
	}
}

func TestBerichteListe(t *testing.T) {
	list, err := Reports(reportsFixture(t))
	if err != nil || len(list) != 2 {
		t.Fatalf("liste: %+v, %v", list, err)
	}
	for _, r := range list {
		if r.Path == "reports/simtest-run-2/balance-2.json" && (r.Run != "simtest-run-2" || len(r.Varianten) != 1) {
			t.Errorf("eintrag: %+v", r)
		}
		if r.Path == "reports/balance-1.json" && r.Run != "" {
			t.Errorf("eintrag: %+v", r)
		}
	}
}

func TestPlaetzeBekommenEigeneEXE(t *testing.T) {
	if balanceExe(0) == balanceExe(1) {
		t.Fatalf("Platz 0 und 1 teilen sich %s", balanceExe(0))
	}
	g := New(nil)
	g.mu.Lock()
	defer g.mu.Unlock()
	g.runs = []*Run{{root: "r", active: true, slot: 0}, {root: "x", active: true, slot: 1}}
	if s := g.freeSlot("r"); s != 1 {
		t.Errorf("freier Platz in r: %d, erwartet 1", s)
	}
	if s := g.freeSlot("x"); s != 0 {
		t.Errorf("freier Platz in x: %d, erwartet 0", s)
	}
}
