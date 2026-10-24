package main

import (
	"k3c/tools/k3c-workbench/internal/balance"
	"k3c/tools/k3c-workbench/internal/datadoc"
	"k3c/tools/k3c-workbench/internal/simrun"
)

// Balancing-Seite (WZ28.4, B-596): Offline-Läufe im Register von sim_test (internal/simrun), gleiche Grenze von
// simrun.MaxPerRoot Läufen je Checkout; weitere warten. Die Seite bedient die Repo-Wurzel.

// BalanceStart reiht einen Offline-Lauf ein; er rechnet sofort, wenn ein Platz frei ist.
func (a *App) BalanceStart(spec simrun.Spec) (simrun.Info, error) {
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
