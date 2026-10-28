package mcpsrv

import (
	"context"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"sync"
	"testing"
)

// B-640/AC-02: sim_test sichert durchgelaufene Läufe als Baseline und vergleicht gegen eine benannte Baseline.

const passingReport = `{"version":1,"seeds":1,"seedMode":"dev","pass":true,"targets":[` +
	`{"name":"Endboss","measure":"endbossDefeated","kind":"share","players":2,"bot":"coop2","depth":0,"days":40,"world":"standard","value":100,"lower":100,"upper":100,"status":"ok","pass":true}]}`

// passingBalance schreibt einen durchgelaufenen Bericht und merkt sich die Argumente der CLI.
func passingBalance(t *testing.T, seen *[][]string, mu *sync.Mutex) func(runSpec) runResult {
	return func(spec runSpec) runResult {
		i := slices.Index(spec.args, "--report-dir")
		if i < 0 {
			return runResult{}
		}
		mu.Lock()
		*seen = append(*seen, spec.args)
		mu.Unlock()
		dir := filepath.Join(spec.dir, spec.args[i+1])
		if err := os.MkdirAll(dir, 0o755); err != nil {
			t.Error(err)
		}
		if err := os.WriteFile(filepath.Join(dir, "balance-1.json"), []byte(passingReport), 0o644); err != nil {
			t.Error(err)
		}
		return runResult{}
	}
}

func TestSimTestBaselineSichernUndVergleichen(t *testing.T) {
	var seen [][]string
	var mu sync.Mutex
	s, _ := fakeServer(t, passingBalance(t, &seen, &mu))
	ctx := context.Background()

	if out, err := s.simTest(ctx, simTestIn{Action: "baseline"}); err != nil || !strings.Contains(out, "keine gesicherten Baselines") {
		t.Fatalf("leere Liste: %q %v", out, err)
	}
	id := start(t, s, simTestIn{Bots: "coop2"})
	waitDone(t, s, id)
	out, err := s.simTest(ctx, simTestIn{Action: "baseline", ID: id, SaveBaseline: "M1"})
	if err != nil || !strings.Contains(out, "Baseline M1 gesichert") || !strings.Contains(out, "coop2") {
		t.Fatalf("sichern: %q %v", out, err)
	}
	if _, err := os.Stat(filepath.Join(s.cfg.Root, "reports", "baseline", "M1", "balance.json")); err != nil {
		t.Error(err)
	}
	if list, _ := s.simTest(ctx, simTestIn{Action: "baseline"}); !strings.HasPrefix(list, "M1 · ") {
		t.Errorf("Liste: %q", list)
	}
	if _, err := s.simTest(ctx, simTestIn{Action: "baseline", ID: id, SaveBaseline: "M1"}); err == nil {
		t.Error("M1 wurde überschrieben")
	}

	second := start(t, s, simTestIn{Bots: "coop2", Baseline: "M1"})
	waitDone(t, s, second)
	mu.Lock()
	defer mu.Unlock()
	last := seen[len(seen)-1]
	if i := slices.Index(last, "--baseline"); i < 0 || last[i+1] != "reports/baseline/M1/balance.json" {
		t.Errorf("Vergleich gegen M1 fehlt in den Argumenten: %v", last)
	}
	if first := seen[0]; first[slices.Index(first, "--baseline")+1] != "testdata/balance/baseline.json" {
		t.Errorf("ohne baseline gilt die Baseline des Repos: %v", first)
	}
}

func TestSimTestBaselineLehntAb(t *testing.T) {
	s, started := fakeServer(t, func(runSpec) runResult { return runResult{} })
	cases := []simTestIn{
		{Action: "start", Baseline: "M9"},                      // gibt es nicht
		{Action: "start", Baseline: "../M1"},                   // ungültiger Name
		{Action: "start", Mode: "online", Baseline: "M1"},      // nur offline
		{Action: "start", SaveBaseline: "M1"},                  // nur bei action baseline
		{Action: "baseline", SaveBaseline: "M1"},               // id fehlt
		{Action: "baseline", ID: "run-99", SaveBaseline: "M1"}, // Lauf unbekannt
	}
	for _, in := range cases {
		if out, err := s.simTest(context.Background(), in); err == nil {
			t.Errorf("%+v angenommen: %s", in, out)
		} else if strings.Contains(err.Error(), "\n") {
			t.Errorf("%+v: Grund nicht in einer Zeile: %v", in, err)
		}
	}
	if started.Load() != 0 {
		t.Errorf("%d Prozesse gestartet", started.Load())
	}
}
