package main

import (
	"context"
	"fmt"
	"net"
	"os"
	"path/filepath"
	"strconv"
	"sync"
	"time"

	"github.com/wailsapp/wails/v2/pkg/runtime"

	"k3c/tools/assetkit/windowstate"
	"k3c/tools/k3c-studio/internal/mcpsrv"
	"k3c/tools/k3c-studio/internal/studioview"
	"k3c/tools/k3c-studio/internal/world"
	"k3c/tools/workkit/applog"
	"k3c/tools/workkit/console"
	"k3c/tools/workkit/usage"
)

// logSource ist die Quelle des eigenen Logs (logs/k3c-studio.jsonl).
const logSource = "k3c-studio"

// Ereignisse an die Oberfläche.
const (
	evMCPState = "mcp:state"
	evWorld    = "world:changed"
)

// MCPState ist der Zustand des MCP-Servers für das Badge der Kopfzeile.
type MCPState struct {
	Addr      string `json:"addr"`
	Listening bool   `json:"listening"`
	Error     string `json:"error"`
}

// Info ist das, was die Oberfläche beim Laden abfragt.
type Info struct {
	Version string   `json:"version"`
	MCP     MCPState `json:"mcp"`
}

// App ist das Backend des Studios: es hostet den MCP-Server k3c-studio und das eigene Log; seine exportierten Methoden
// sind die Bindings der Oberfläche (Welten, Assets, Hub, Studio-Ansichten).
type App struct {
	root   string
	port   int
	emit   func(ctx context.Context, name string, data ...any) // Test-Naht; Produktion: emitWindow
	window bool                                                // läuft im Wails-Fenster (nicht -headless/-dev)
	usage  string                                              // Datei der Nutzungsstatistik (Test-Naht)
	// ready schließt startup: Wails ruft OnStartup in einer eigenen Goroutine, Bindings können davor kommen.
	ready chan struct{}

	ctx     context.Context
	svcCtx  context.Context // endet beim Beenden; Befehle der Oberfläche laufen darin
	cancel  context.CancelFunc
	store   *console.Store
	log     *applog.Log
	tracker *usage.Tracker
	srv     *mcpsrv.Server
	game    *studioview.Game // „Im Spiel ansehen“ im Bereich Assets (WB1.4), angelegt beim ersten Aufruf

	mu  sync.Mutex
	mcp MCPState
}

func newApp(root string, port int) *App {
	a := &App{root: root, port: port, usage: windowstate.ConfigPath("mcp-usage-studio.json"), ready: make(chan struct{})}
	a.emit = a.emitWindow
	return a
}

// emitWindow schickt ein Ereignis an das Wails-Fenster; ohne Fenster (-headless) geht es verloren.
func (a *App) emitWindow(ctx context.Context, name string, data ...any) {
	if a.window && ctx != nil {
		runtime.EventsEmit(ctx, name, data...)
	}
}

// wait blockiert, bis startup fertig ist. Ohne ready (Tests, die App direkt bauen) wartet es nicht.
func (a *App) wait() {
	if a.ready != nil {
		<-a.ready
	}
}

// startup öffnet Log und Statistik und startet den MCP-Server. Ein belegter Port hält das Fenster nicht auf: der
// Grund steht im Log und im Badge.
func (a *App) startup(ctx context.Context) {
	defer a.markReady()
	a.ctx = ctx
	a.store = console.New(console.DefaultCapacity, nil)
	var err error
	a.log, err = applog.OpenNamed(filepath.Join(a.root, "logs"), logSource, a.store)
	if err != nil {
		fmt.Fprintln(os.Stderr, "k3c-studio: eigenes Log nur im Speicher:", err)
	}
	a.tracker = usage.Open(a.usage, time.Now,
		func(err error) { a.log.Warn("📄 "+err.Error(), "ns", "usage") })
	a.svcCtx, a.cancel = context.WithCancel(ctx)
	world.Watch(a.svcCtx, a.root, worldPoll, func(ids []string) {
		a.emit(a.ctx, evWorld, world.Changed{IDs: ids})
	})
	a.srv = mcpsrv.New(mcpsrv.Config{Root: a.root, Port: a.port, Version: version, Log: a.log.Logger, Usage: a.tracker})
	err = a.srv.Start()
	if err != nil {
		a.log.Error("💥 start fehlgeschlagen", "ns", "main", "error", err.Error())
	} else {
		a.log.Info("🚀 k3c-studio gestartet", "ns", "main", "url", a.srv.URL(), "version", version)
	}
	a.setMCP(err)
}

func (a *App) markReady() {
	if a.ready != nil {
		close(a.ready)
	}
}

// shutdown beendet „Im Spiel ansehen“, Server, Statistik und Log.
func (a *App) shutdown(context.Context) {
	a.wait()
	a.log.Info("🛑 k3c-studio beendet", "ns", "main")
	a.cancel()
	a.closeStudio() // Räume von „Im Spiel ansehen“ speichern
	_ = a.srv.Stop()
	_ = a.tracker.Flush() // ein Fehler steht schon im Log (Rückruf)
	_ = a.log.Close()
	a.store.Close()
}

func (a *App) setMCP(err error) {
	st := MCPState{Addr: net.JoinHostPort("127.0.0.1", strconv.Itoa(a.port)), Listening: err == nil}
	if err != nil {
		st.Error = err.Error()
	}
	a.mu.Lock()
	a.mcp = st
	a.mu.Unlock()
	a.emit(a.ctx, evMCPState, st)
}

// Info liefert Version und MCP-Zustand (Binding).
func (a *App) Info() Info {
	a.wait()
	a.mu.Lock()
	defer a.mu.Unlock()
	return Info{Version: version, MCP: a.mcp}
}

// beforeClose merkt Größe und Position des Fensters; das Schließen läuft immer weiter.
func (a *App) beforeClose(ctx context.Context) bool {
	a.wait()
	if runtime.WindowIsMinimised(ctx) {
		return false // minimiert liefert Windows Platzhalter-Maße; die zuletzt gemerkten bleiben
	}
	x, y := runtime.WindowGetPosition(ctx)
	w, h := runtime.WindowGetSize(ctx)
	if err := windowstate.Save(windowstate.ConfigPath(windowFile), windowstate.State{X: x, Y: y, Width: w, Height: h}); err != nil {
		a.log.Warn("📄 fenster nicht gemerkt: "+err.Error(), "ns", "main")
	}
	return false
}

// secondInstance holt das offene Fenster nach vorn, wenn k3c-studio ein zweites Mal gestartet wird.
func (a *App) secondInstance() {
	a.wait()
	runtime.WindowUnminimise(a.ctx)
	runtime.WindowShow(a.ctx)
	runtime.WindowSetAlwaysOnTop(a.ctx, true) // Windows holt ein Fenster sonst nicht aus dem Hintergrund
	runtime.WindowSetAlwaysOnTop(a.ctx, false)
}
