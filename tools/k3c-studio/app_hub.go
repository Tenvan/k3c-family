package main

// Gebäude-Planer (B-923, Port aus dem früheren Asset-Studio, B-462): data/hub.json als Matrix Hub-Stufe × Bauplatz
// ansehen, einen Kandidaten prüfen, mit der Regeltabelle vergleichen und geprüft speichern. Die Logik liegt in
// internal/hubplan, Matrix und Prüfung rechnet sim hub; dieselben Aufrufe machen die MCP-Tools hub_*.

import "k3c/tools/k3c-studio/internal/hubplan"

// hubEngine ist die Engine-Seite des Planers (sim hub); Tests setzen einen Ersatz.
var hubEngine = hubplan.Exe

// hubs ist der Gebäude-Planer des Checkouts.
func (a *App) hubs() hubplan.Editor { return hubplan.Editor{Root: a.root, Engine: hubEngine(a.root)} }

// HubPlan lädt data/hub.json des Checkouts und liefert Matrix und Befunde (Binding).
func (a *App) HubPlan() hubplan.HubPlanView {
	a.wait()
	return a.hubs().Plan(a.worldCtx())
}

// HubCheck prüft einen Kandidaten (Text der hub.json), ohne etwas zu schreiben (Binding).
func (a *App) HubCheck(candidate string) hubplan.HubPlanView {
	a.wait()
	return a.hubs().Check(a.worldCtx(), candidate)
}

// HubRules vergleicht die Regeltabelle mit dem Planer-Stand hubJSON; leer = data/hub.json (Binding).
func (a *App) HubRules(hubJSON string) hubplan.HubRulesView {
	a.wait()
	return a.hubs().Rules(hubJSON)
}

// HubSave speichert den geprüften Kandidaten, wenn beide Dateien noch den Stand stand haben (Binding).
func (a *App) HubSave(candidate string, stand hubplan.HubStand) hubplan.HubSaveResult {
	a.wait()
	res := a.hubs().Save(a.worldCtx(), candidate, stand)
	if a.log != nil {
		switch {
		case res.Saved:
			a.log.Info("💾 Gebäude-Planer gespeichert", "ns", "dev", "file", "data/hub.json", "rulesError", res.RulesError)
		case res.Error != "" || res.Stale != "":
			a.log.Warn("🚫 Gebäude-Planer nicht gespeichert", "ns", "dev", "error", res.Error, "stale", res.Stale)
		}
	}
	return res
}
