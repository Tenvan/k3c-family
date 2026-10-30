package main

import "k3c/tools/k3c-workbench/internal/simrun"

// Ereignisse der Balancing-Seite (B-662): Statt alle 1,5 s abzufragen, lädt die Seite Läufe und Berichte nur bei
// diesen Ereignissen neu. Das Register (internal/simrun, notify.go) meldet Läufe gedrosselt (höchstens etwa 2 je
// Sekunde je Lauf) und Berichte nach einem fertigen Lauf oder einer gesicherten Baseline; Bewerten und Löschen
// melden die Methoden in app_balance.go selbst.
const (
	evBalanceRuns    = "balance:runs"    // ein Lauf hat Zustand, Phase oder Fortschritt geändert; Nutzlast simrun.Info
	evBalanceReports = "balance:reports" // Berichte oder Baselines haben sich geändert; ohne Nutzlast
)

// sims ist das Register der Läufe mit den Rückrufen für die Ereignisse der Seite. Die Rückrufe hängen sich bei jedem
// Aufruf der Seite ein (SetNotify ersetzt, das ist billig): vor dem ersten Laden der Seite hört keine Oberfläche zu,
// und startup bleibt unberührt. Gemeldet wird nur, was die Seite zeigt: Läufe und Berichte der Repo-Wurzel.
func (a *App) sims() *simrun.Registry {
	g := a.srv.Sims()
	g.SetNotify(simrun.Notify{
		Run: func(root string, info simrun.Info) {
			if root == a.root {
				a.balanceEmit(evBalanceRuns, info)
			}
		},
		Reports: func(root string) {
			if root == a.root {
				a.balanceEmit(evBalanceReports, nil)
			}
		},
	})
	return g
}

// balanceEmit schickt ein Ereignis der Seite; ohne emit (Tests ohne startup) geht es verloren.
func (a *App) balanceEmit(name string, data any) {
	if a.emit != nil {
		a.emit(a.ctx, name, data)
	}
}
