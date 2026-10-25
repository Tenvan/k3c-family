package main

import (
	"context"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"testing"
	"time"

	"k3c/tools/k3c-workbench/internal/balance"
	"k3c/tools/k3c-workbench/internal/mcpsrv"
	"k3c/tools/k3c-workbench/internal/simrun"
)

// balanceApp: Workbench mit Register, dessen Prozesse sofort enden (Bau) bzw. einen Bericht schreiben (Rechnen).
func balanceApp(t *testing.T, hold chan struct{}) *App {
	t.Helper()
	root := t.TempDir()
	a := &App{root: root, srv: mcpsrv.New(mcpsrv.Config{Root: root})}
	a.srv.Sims().Exec = func(ctx context.Context, p simrun.Proc) simrun.Result {
		if p.Args[0] == "go" {
			return simrun.Result{}
		}
		select {
		case <-ctx.Done():
			return simrun.Result{Exit: -1}
		case <-hold:
		}
		dir := filepath.Join(p.Dir, p.Args[3])
		if err := os.MkdirAll(dir, 0o755); err != nil {
			t.Error(err)
		}
		if err := os.WriteFile(filepath.Join(dir, "balance-1.json"), []byte(`{"version":1,"seeds":1,"pass":true}`), 0o644); err != nil {
			t.Error(err)
		}
		return simrun.Result{}
	}
	return a
}

func waitRun(t *testing.T, a *App, id string, ok func(simrun.Info) bool) simrun.Info {
	t.Helper()
	for range 1000 {
		for _, r := range a.BalanceRuns() {
			if r.ID == id && ok(r) {
				return r
			}
		}
		time.Sleep(5 * time.Millisecond)
	}
	t.Fatalf("%s: Zustand nicht erreicht: %+v", id, a.BalanceRuns())
	return simrun.Info{}
}

func TestBalanceStartStopUndBerichte(t *testing.T) {
	hold := make(chan struct{})
	a := balanceApp(t, hold)
	var ids []string
	for range 3 {
		run, err := a.BalanceStart(simrun.Spec{Seeds: 1})
		if err != nil {
			t.Fatal(err)
		}
		ids = append(ids, run.ID)
	}
	waitRun(t, a, ids[2], func(r simrun.Info) bool { return r.State == simrun.Waiting })
	if err := a.BalanceStop(ids[0]); err != nil {
		t.Fatal(err)
	}
	waitRun(t, a, ids[0], func(r simrun.Info) bool { return r.State == simrun.Canceled && r.Report != "" })
	waitRun(t, a, ids[2], func(r simrun.Info) bool { return r.State == simrun.Running })
	close(hold)
	done := waitRun(t, a, ids[2], func(r simrun.Info) bool { return r.State == simrun.Done && r.Report != "" })
	waitRun(t, a, ids[1], func(r simrun.Info) bool { return r.State == simrun.Done && r.Report != "" })
	if done.Verdict != "Pass" || done.Spec == nil || done.Spec.Seeds != 1 {
		t.Errorf("lauf: %+v", done)
	}
	checkBalanceReports(t, a)
}

// checkBalanceReports: Ablehnungen, Berichtsliste und Pfadprüfung nach zwei fertigen Läufen.
func checkBalanceReports(t *testing.T, a *App) {
	t.Helper()
	if _, err := a.BalanceStart(simrun.Spec{Seeds: 999}); err == nil {
		t.Error("seeds 999 angenommen")
	}
	if err := a.BalanceStop("run-99"); err == nil {
		t.Error("unbekannter Lauf gestoppt")
	}
	reports, err := a.BalanceReports()
	if err != nil || len(reports) != 2 {
		t.Fatalf("berichte: %+v, %v", reports, err)
	}
	if sum, err := a.BalanceReport(reports[0].Path); err != nil || !sum.Pass {
		t.Errorf("bericht: %+v, %v", sum, err)
	}
	for _, bad := range []string{"../go.mod", "data/x.json", filepath.Join(a.root, reports[0].Path)} {
		if _, err := a.BalanceReport(bad); err == nil {
			t.Errorf("%q gelesen", bad)
		}
	}
}

// WZ28.6: BalanceTuning liest tuning-*.json unter reports/ und lehnt andere Pfade ab.
func TestBalanceTuningPfad(t *testing.T) {
	a := balanceApp(t, nil)
	dir := filepath.Join(a.root, "reports", "simtest-run-1")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	body := `{"version":1,"mode":"search","best":{"values":[3],"score":0,"pass":true},"reason":"ok","diff":"-a\n+b"}`
	if err := os.WriteFile(filepath.Join(dir, "tuning-1.json"), []byte(body), 0o644); err != nil {
		t.Fatal(err)
	}
	if tn, err := a.BalanceTuning("reports/simtest-run-1/tuning-1.json"); err != nil || tn.Mode != "search" || !tn.Best.Pass {
		t.Errorf("ergebnis: %+v, %v", tn, err)
	}
	for _, bad := range []string{"../go.mod", "data/x.json", filepath.Join(dir, "tuning-1.json"), "reports/x.md"} {
		if _, err := a.BalanceTuning(bad); err == nil {
			t.Errorf("%q gelesen", bad)
		}
	}
}

// WZ28.10: Die Auswahlboxen Bot und Grad der Balancing-Seite (help.ts) nennen genau die Bots und Grade der Engine.
func TestAuswahlWieGo(t *testing.T) {
	src, err := os.ReadFile("frontend/src/balancing/help.ts")
	if err != nil {
		t.Fatal(err)
	}
	list := func(name string) []string {
		m := regexp.MustCompile(`export const ` + name + ` = \[([^\]]*)\]`).FindSubmatch(src)
		if m == nil {
			t.Fatalf("help.ts: %s fehlt", name)
		}
		var out []string
		for _, q := range regexp.MustCompile(`'([a-z0-9-]+)'`).FindAllSubmatch(m[1], -1) {
			out = append(out, string(q[1]))
		}
		slices.Sort(out)
		return out
	}
	grades, err := balance.Grades()
	if err != nil {
		t.Fatal(err)
	}
	slices.Sort(grades)
	if got := list("BOTS"); !slices.Equal(got, balance.BotNames()) {
		t.Errorf("BOTS = %v, Go: %v", got, balance.BotNames())
	}
	if got := list("GRADES"); !slices.Equal(got, grades) {
		t.Errorf("GRADES = %v, Go: %v", got, grades)
	}
}

// B-632/AC-05, AC-06: Auswertung lesen; „HTML-Bericht öffnen“ schreibt .md/.html eines alten Berichts und öffnet die Datei.
func TestBalanceAuswertungUndHTML(t *testing.T) {
	a := balanceApp(t, nil)
	dir := filepath.Join(a.root, "reports", "simtest-run-4")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	raw, err := os.ReadFile("internal/balance/testdata/insights-run7.json")
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "balance-20261025-062117.json"), raw, 0o644); err != nil {
		t.Fatal(err)
	}
	rel := "reports/simtest-run-4/balance-20261025-062117.json"
	in, err := a.BalanceInsights(rel)
	if err != nil || in.Run != "simtest-run-4" || in.Counts[balance.LightRed] != 4 {
		t.Fatalf("Auswertung: %v %+v", err, in.Counts)
	}
	var opened string
	old := openFile
	t.Cleanup(func() { openFile = old })
	openFile = func(p string) error { opened = p; return nil }
	page, err := a.BalanceReportHTML(rel)
	if err != nil || page != "reports/simtest-run-4/balance-20261025-062117.html" || filepath.Base(opened) != filepath.Base(page) {
		t.Fatalf("HTML: %q %q %v", page, opened, err)
	}
	if _, err := a.BalanceInsights("reports/tuning-1.json"); err == nil {
		t.Error("tuning-*.json darf keine Auswertung liefern")
	}
}
