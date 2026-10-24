package main

import (
	"k3c/tools/k3c-workbench/internal/balance"
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
