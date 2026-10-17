package main

import (
	"context"
	"time"

	"github.com/wailsapp/wails/v2/pkg/runtime"

	assets "k3c/tools/assetkit"
)

// evAssetsChanged meldet der Oberfläche, dass data/assets.json von außen (MCP, Editor) oder durch eigenes Speichern
// geändert wurde; sie lädt dann neu (B-459/AC-07).
const evAssetsChanged = "assets:changed"

// assetsPoll ist der Takt, in dem das Asset-Studio data/assets.json prüft.
const assetsPoll = time.Second

// App ist das Backend des Asset-Studios; seine exportierten Methoden sind die Bindings der Oberfläche.
type App struct {
	root string
	emit func(ctx context.Context, name string, data ...any) // Test-Naht; Produktion: runtime.EventsEmit
	poll time.Duration                                       // Takt der Beobachtung (Test-Naht)
	// ready schließt startup: Wails ruft OnStartup in einer eigenen Goroutine, Bindings können davor kommen.
	ready chan struct{}
	ctx   context.Context // aus OnStartup; endet beim Beenden, Recherche und Download laufen darin
}

func newApp(root string) *App {
	return &App{root: root, emit: runtime.EventsEmit, poll: assetsPoll, ready: make(chan struct{})}
}

// startup merkt den Kontext und beobachtet data/assets.json.
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
	assets.Watch(ctx, a.root, a.poll, func() { a.emit(ctx, evAssetsChanged) })
	if a.ready != nil {
		close(a.ready)
	}
}

// wait blockiert, bis startup gelaufen ist; ohne ready (Tests) sofort.
func (a *App) wait() {
	if a.ready != nil {
		<-a.ready
	}
}

// context ist der Kontext aus startup, in Tests context.Background().
func (a *App) context() context.Context {
	if a.ctx == nil {
		return context.Background()
	}
	return a.ctx
}

// secondInstance holt das laufende Fenster nach vorn, wenn das Asset-Studio für denselben Checkout ein zweites Mal
// startet (wie k3c-dev).
func (a *App) secondInstance() {
	a.wait()
	runtime.WindowUnminimise(a.ctx)
	runtime.WindowShow(a.ctx)
	runtime.WindowSetAlwaysOnTop(a.ctx, true) // Windows holt ein Fenster sonst nicht aus dem Hintergrund
	runtime.WindowSetAlwaysOnTop(a.ctx, false)
}
