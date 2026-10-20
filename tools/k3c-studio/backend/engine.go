package backend

import (
	"context"
	"io"
	"log/slog"
	"net/http"
	"os"
	"path/filepath"
	"sync"
	"time"

	k3cnet "k3c/engine/net"
	"k3c/engine/room"
	"k3c/engine/store"
)

// Eingebundene Engine des Studios (B-507/AC-02, TS1.2): Räume, Spiel-API und WebSocket wie im Spielserver, aber im Studio
// selbst und mit eigenen Testspielständen. So laufen Vorschau und „Im Spiel öffnen“ ohne Spielserver und ohne dessen
// Dev-Mode. Die Richtung ist einseitig: das Studio importiert engine/…, nie umgekehrt.

// testSavesDir ist der Ordner der Testspielstände (relativ zur Repo-Wurzel, von git ignoriert), nie saves/ des Servers.
const testSavesDir = "tools/k3c-studio/.studio-saves"

// Engine hält Spielstände, Räume und die Spiel-API des Studios.
type Engine struct {
	Saves   *store.Saves
	Rooms   *room.Manager
	handler http.Handler
	done    chan struct{} // schließt Start, nachdem alle Räume gespeichert und geschlossen sind
}

// NewEngine legt die Engine für die Repo-Wurzel root an: Dev-Mode an (alle Welten aus data/worlds, „voll ausgebaut“),
// Spielstände und Berichte unter testSavesDir; log darf nil sein.
func NewEngine(root string, log *slog.Logger) *Engine {
	if log == nil {
		log = slog.New(slog.NewTextHandler(io.Discard, nil))
	}
	dir := filepath.Join(root, filepath.FromSlash(testSavesDir))
	saves := &store.Saves{Dir: dir}
	rooms := room.NewManager(saves)
	rooms.Log = log
	rooms.Dev = true
	rooms.Worlds = os.DirFS(filepath.Join(root, filepath.FromSlash(worldsDir)))
	handler := k3cnet.NewHandler(k3cnet.Config{
		Dist: filepath.Join(root, "dist"), Saves: saves, Reports: &store.Reports{Dir: filepath.Join(dir, "reports")},
		Log: log, Version: "studio", StartedAt: time.Now(), Rooms: rooms, Conns: &sync.WaitGroup{},
	})
	log.Info("🚀 Engine des Studios bereit", "ns", "room", "spielstaende", dir)
	return &Engine{Saves: saves, Rooms: rooms, handler: handler, done: make(chan struct{})}
}

// Start lässt die Räume laufen (30 Hz); endet ctx, speichert und schließt die Engine alle Räume.
func (e *Engine) Start(ctx context.Context) {
	go func() {
		e.Rooms.Run(ctx)
		e.Rooms.Close()
		close(e.done)
	}()
}

// Wait wartet höchstens d, bis die Engine nach dem Ende von ctx alle Räume gespeichert hat; false = Frist abgelaufen.
func (e *Engine) Wait(d time.Duration) bool {
	select {
	case <-e.done:
		return true
	case <-time.After(d):
		return false
	}
}

// EngineWait wartet höchstens d auf das Speichern der Räume (Ende des Studios); ohne Engine sofort true.
func (a *App) EngineWait(d time.Duration) bool {
	return a.engine == nil || a.engine.Wait(d)
}

// Game ist die Spiel-API der eingebundenen Engine (/api/…, /ws) für den Server des Studios; ohne Engine 404.
func (a *App) Game() http.Handler {
	if a.engine == nil {
		return http.NotFoundHandler()
	}
	return a.engine.handler
}
