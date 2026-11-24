package main

import (
	"context"
	"net/http"

	"github.com/wailsapp/wails/v2/pkg/runtime"

	"k3c/tools/workkit/uikit"
)

// HTTP-Schicht der Oberfläche für den Client-Server-Betrieb (-headless, -dev): dieselben Methoden wie die
// Wails-Bindings über tools/workkit/uikit, wie in k3c-workbench (WZ63.3).

// uiMethods sind die Methoden von App, die die Oberfläche aufrufen darf: genau die von GoApp in
// frontend/src/api/wails.ts (TestUIMethodsWieWailsTS hält beide gleich). Balance* seit WZ63.5 (Balancing-Seite).
var uiMethods = map[string]bool{
	"Info": true,
	"AssetsList": true, "AssetsIndex": true, "AssetsCheck": true, "AssetsGroesse": true, "AssetsGroesseBasis": true,
	"AssetsFrameOhne": true, "AssetsStufen": true, "AssetsHintergruende": true, "AssetsHintergrund": true,
	"AssetsGet": true, "AssetsStyle": true, "AssetsPrompts": true, "AssetsReview": true, "AssetsFrame": true,
	"AssetsPixelDiff": true,
	"StudioSounds":    true, "StudioWeltSicht": true, "StudioSoundData": true, "StudioSources": true, "StudioGameOpen": true,
	"WorldList": true, "WorldDetail": true, "WorldLevel": true, "WorldValues": true, "WorldCreate": true,
	"WorldDelete": true, "WorldOverrideSet": true, "WorldIslandEtappe": true, "WorldSetKind": true,
	"WorldSetExtends": true, "WorldNatureSet": true, "WorldPicks": true, "WorldSetPick": true, "WorldIslandEndBoss": true, "WorldSkins": true,
	"HubPlan": true, "HubCheck": true, "HubRules": true, "HubSave": true,
	"BalanceStart": true, "BalanceRuns": true, "BalanceStop": true, "BalanceReports": true, "BalanceReport": true,
	"BalanceTuning": true, "BalanceDataFiles": true, "BalanceDataDoc": true, "BalanceDataValues": true,
	"BalanceReportRate": true, "BalanceReportDelete": true, "BalanceInsights": true, "BalanceReportHTML": true,
	"BalanceBaselines": true, "BalanceReportsRate": true, "BalanceReportsDelete": true, "BalanceSaveBaseline": true,
	"BalanceDataValuesAll": true,
}

// uiHandler beantwortet alle Adressen unter uikit.Prefix, die Oberfläche liefert Vite.
func (a *App) uiHandler() http.Handler {
	api := &uikit.API{App: "k3c-studio", Target: a, Methods: uiMethods, Bus: a.bus}
	return api.Handler()
}

// emitAll ist emit der Produktion: an die Wails-Laufzeit nur im Fenster (sie beendet das Programm bei einem ctx ohne
// Wails), immer an den Bus für die Ströme der HTTP-Oberfläche.
func (a *App) emitAll(ctx context.Context, name string, data ...any) {
	if a.window && ctx != nil {
		runtime.EventsEmit(ctx, name, data...)
	}
	if a.bus != nil {
		a.bus.Publish(name, data)
	}
}
