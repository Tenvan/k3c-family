package main

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

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
