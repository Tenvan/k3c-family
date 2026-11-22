package main

import (
	"context"
	"net/http"

	"github.com/wailsapp/wails/v2/pkg/runtime"

	"k3c/tools/workkit/uikit"
)

// HTTP-Schicht der Oberfläche für den Client-Server-Betrieb (-headless, -dev): dieselben Methoden wie die
// Wails-Bindings über tools/workkit/uikit (RPC, Ereignisstrom, Schutz vor fremden Seiten; WZ63.3).

// uiMethods sind die Methoden von App, die die Oberfläche aufrufen darf: genau die von GoApp in
// frontend/src/api/wails.ts (TestUIMethodsWieWailsTS hält beide gleich). TaskList u. a. bleiben MCP-intern.
// Assets, Welten, Hub und Studio-Ansichten sind Bindings von k3c-studio (WZ63).
var uiMethods = map[string]bool{
	"Info": true, "Services": true, "ServicesReload": true, "ServiceStart": true, "ServiceStop": true,
	"ServiceRestart": true, "ServicesStartAll": true, "ServicesStopAll": true, "ServiceLogLevels": true,
	"Sources": true, "ConsoleTail": true, "LogsQuery": true, "LogsErrors": true,
	"McpOverview": true, "McpRestart": true, "McpInstructions": true, "McpCalls": true, "McpUsage": true,
	"Tasks": true, "TasksReload": true, "TaskStart": true, "TaskStop": true, "TaskRuns": true,
	"PlanningData": true, "PlanningDocs": true, "PlanningSet": true, "PlanningDoc": true, "PlanningGet": true, "PlanningList": true, "PlanningItems": true,
	"GitHubStatus": true, "WorktreeCheck": true, "OpenInClaude": true, "Resources": true,
	"GitOverview": true, "GitAction": true, "GitDeleteBranches": true, "GitPRs": true, "GitPRCreate": true, "GitPRAction": true,
	"GitCheckMerged": true, "GitStamp": true, "TicketCreate": true, "TicketRecent": true,
	"BalanceStart": true, "BalanceRuns": true, "BalanceStop": true, "BalanceReports": true, "BalanceReport": true,
	"BalanceTuning": true, "BalanceDataFiles": true, "BalanceDataDoc": true, "BalanceDataValues": true,
	"BalanceReportRate": true, "BalanceReportDelete": true, "BalanceInsights": true, "BalanceReportHTML": true,
	"BalanceBaselines": true, "BalanceReportsRate": true, "BalanceReportsDelete": true, "BalanceSaveBaseline": true,
	"BalanceDataValuesAll": true,
}

// uiHandler beantwortet alle Adressen unter uikit.Prefix, die Oberfläche liefert Vite.
func (a *App) uiHandler() http.Handler {
	api := &uikit.API{App: "k3c-workbench", Target: a, Methods: uiMethods, Bus: a.bus,
		Track: func(name string) func() { return a.res.Track(name) }}
	return api.Handler()
}

// emitAll ist emit der Produktion: an die Wails-Laufzeit nur im Fenster (sie beendet das Programm bei einem ctx ohne
// Wails), immer an den Bus für die Ströme der HTTP-Oberfläche.
func (a *App) emitAll(ctx context.Context, name string, data ...any) {
	if a.window {
		runtime.EventsEmit(ctx, name, data...)
	}
	a.bus.Publish(name, data)
}
