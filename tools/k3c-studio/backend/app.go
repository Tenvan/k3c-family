// Package backend ist das Asset-Studio von K3C als Seite des Go-Servers (B-459, Umzug von der Wails-EXE): Es bedient die
// Oberfläche unter /studio/ mit dem Slot-Register data/assets.json, dem Welt-Betrachter, den geplanten Chunks und dem
// Gebäude-Planer. Die Methoden von App erreicht die Oberfläche über /api/studio/rpc/<Methode> (rpc.go).
package backend

import (
	"context"
	"log/slog"
	"sync"
	"time"

	assets "k3c/tools/assetkit"
)

// evAssetsChanged meldet der Oberfläche, dass data/assets.json von außen (MCP, Editor) oder durch eigenes Speichern
// geändert wurde; sie lädt dann neu (B-459/AC-07).
const evAssetsChanged = "assets:changed"

// assetsPoll ist der Takt, in dem das Studio data/assets.json prüft.
const assetsPoll = time.Second

// App ist das Backend des Studios; seine exportierten Methoden sind die Aufrufe der Oberfläche.
type App struct {
	root string
	poll time.Duration // Takt der Beobachtung (Test-Naht)
	ctx  context.Context
	bus  *bus
	mu   sync.Mutex // schützt world
	// save schützt Lesen, Prüfen und Schreiben der Welt-Datei in PlanSave und PlanWorldSave: zwei gleichzeitige Aufrufe
	// lesen sonst denselben Stand, und der zweite überschriebe den ersten (B-497)
	save sync.Mutex
	// world ist die gewählte Welt des Welten Editors (app_world_files.go); leer = standard
	world string
	// chat ist der Chat mit der Claude Code CLI (chat_*.go), angelegt beim ersten Gebrauch
	chatOnce sync.Once
	chat     *chatHub
	// engine ist die eingebundene Engine für Vorschau-Spiele und Testspielstände (engine.go); nil = keine
	engine *Engine
	// serverBase ist die Adresse des Spielservers für „Im Spiel öffnen“ (Test-Naht, B-513); leer = Loopback-Standardport
	serverBase string
}

// New legt das Studio für die Repo-Wurzel root an; ctx begrenzt Beobachtung, Recherche und die Räume der Engine (Ende
// des Servers). log nimmt die Meldungen der Engine auf und darf nil sein.
func New(ctx context.Context, root string, log *slog.Logger) *App {
	a := &App{root: root, poll: assetsPoll, ctx: ctx, bus: newBus(), engine: NewEngine(root, log)}
	a.engine.Start(ctx)
	a.watch(ctx)
	a.chatCleanup()
	return a
}

// watch beobachtet data/assets.json und meldet Änderungen an alle offenen Event-Ströme.
func (a *App) watch(ctx context.Context) {
	assets.Watch(ctx, a.root, a.poll, func() { a.bus.publish(evAssetsChanged) })
}

// context ist der Kontext des Servers, in Tests context.Background().
func (a *App) context() context.Context {
	if a.ctx == nil {
		return context.Background()
	}
	return a.ctx
}
