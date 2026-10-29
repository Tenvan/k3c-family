package main

import (
	"fmt"
	"os/exec"
	"path/filepath"

	"k3c/tools/k3c-workbench/internal/balance"
	"k3c/tools/k3c-workbench/internal/datadoc"
	"k3c/tools/k3c-workbench/internal/simrun"
)

// Balancing-Seite (WZ28.4, B-596): Offline-Läufe im Register von sim_test (internal/simrun), gleiche Grenze von
// simrun.MaxPerRoot Läufen je Checkout; weitere warten. Die Seite bedient die Repo-Wurzel.

// BalanceStart reiht einen Offline-Lauf ein; er rechnet sofort, wenn ein Platz frei ist.
func (a *App) BalanceStart(spec simrun.Spec) (simrun.Info, error) {
	if spec.Baseline != "" && !balance.HasBaseline(a.root, spec.Baseline) {
		return simrun.Info{}, fmt.Errorf("lauf abgelehnt: baseline %q gibt es nicht", spec.Baseline)
	}
	return a.srv.Sims().StartOffline(a.root, "Repo-Wurzel", spec)
}

// BalanceRuns nennt alle Läufe der Repo-Wurzel (auch die von sim_test), neueste zuerst.
func (a *App) BalanceRuns() []simrun.Info {
	return a.srv.Sims().List(a.root)
}

// BalanceStop bricht einen Lauf ab; ein wartender verschwindet aus der Liste.
func (a *App) BalanceStop(id string) error {
	_, err := a.srv.Sims().Stop(a.root, id)
	return err
}

// BalanceReports nennt die gespeicherten Berichte unter reports/, neueste zuerst.
func (a *App) BalanceReports() ([]simrun.ReportInfo, error) {
	return simrun.Reports(a.root)
}

// BalanceReport liest einen Bericht; nur Pfade unter reports/.
func (a *App) BalanceReport(path string) (balance.Summary, error) {
	return simrun.ReadReport(a.root, path)
}

// BalanceTuning liest ein Abstimm-Ergebnis tuning-*.json (WZ28.6); nur Pfade unter reports/.
func (a *App) BalanceTuning(path string) (balance.Tuning, error) {
	return simrun.ReadTuning(a.root, path)
}

// BalanceBaselines nennt die gesicherten Basis-Läufe (reports/baseline/, B-640) für „Beispiele › Basis-Läufe“.
func (a *App) BalanceBaselines() ([]balance.BaselineInfo, error) {
	return balance.ListBaselines(a.root)
}

// BalanceDataFiles nennt die Dateien in data/, die das Balancing beeinflussen, mit Zweck aus docs/data/ (WZ28.10, B-610).
func (a *App) BalanceDataFiles() []datadoc.File {
	return datadoc.List(a.root)
}

// BalanceDataDoc liest die Beschreibung docs/data/<name>.md; nur Namen der Liste.
func (a *App) BalanceDataDoc(name string) (string, error) {
	return datadoc.Doc(a.root, name)
}

// BalanceDataValues nennt die Zahlenwerte einer variierbaren Datei (Pfade wie in Varianten).
func (a *App) BalanceDataValues(name string) ([]datadoc.Value, error) {
	return datadoc.Values(a.root, name)
}

// BalanceReportRate setzt die Bewertung eines Berichts (gut, schlecht, unklar, referenz; leer = keine, WZ28.13).
func (a *App) BalanceReportRate(path, rating string) error {
	return simrun.RateReport(a.root, path, rating)
}

// BalanceReportDelete löscht einen Bericht unter reports/ samt .md und Bewertung (WZ28.13).
func (a *App) BalanceReportDelete(path string) error {
	return simrun.DeleteReport(a.root, path)
}

// BalanceInsights wertet einen Bericht aus (B-632): Ampeln, Kurzauswertung, Hinweise und nächster Lauf.
func (a *App) BalanceInsights(path string) (balance.Insights, error) {
	return simrun.ReadInsights(a.root, path)
}

// BalanceReportHTML schreibt .md und .html eines Berichts neu (auch für alte Berichte, B-632) und öffnet den
// HTML-Bericht im Standard-Browser; Ergebnis ist sein Pfad relativ zum Checkout.
func (a *App) BalanceReportHTML(path string) (string, error) {
	page, err := simrun.WriteHTML(a.root, path)
	if err != nil {
		return "", err
	}
	if err := openFile(page); err != nil {
		return "", err
	}
	if a.log != nil {
		a.log.Info("📄 HTML-Bericht geöffnet", "ns", "main", "datei", filepath.Base(page))
	}
	rel, _ := filepath.Rel(a.root, page)
	return filepath.ToSlash(rel), nil
}

// openFile öffnet eine Datei mit ihrem Standardprogramm (wie OpenInClaude ohne Job-Objekt); Tests ersetzen es.
var openFile = func(path string) error {
	cmd := exec.Command("rundll32", "url.dll,FileProtocolHandler", path)
	if err := cmd.Start(); err != nil {
		return err
	}
	go func() { _ = cmd.Wait() }()
	return nil
}
