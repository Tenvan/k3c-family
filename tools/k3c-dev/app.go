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

	"k3c/tools/k3c-dev/internal/applog"
	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/mcpsrv"
	"k3c/tools/k3c-dev/internal/services"
	"k3c/tools/k3c-dev/internal/usage"
)

// Ereignisse an die Oberfläche (B-064 › Ereignisse). Weitere kommen mit den Seiten dazu, die sie senden.
const evMCPState = "mcp:state"

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

// App ist das Backend des Fensters: es hostet MCP-Server, Dienste und Log; seine exportierten Methoden sind die
// Bindings der Oberfläche (frontend/src/api/).
type App struct {
	root  string
	port  int
	emit  func(ctx context.Context, name string, data ...any) // Test-Naht; Produktion: runtime.EventsEmit
	usage string                                              // Datei der Nutzungsstatistik (Test-Naht)

	ctx     context.Context
	cancel  context.CancelFunc
	log     *applog.Log
	tracker *usage.Tracker
	ctl     *services.Controller
	srv     *mcpsrv.Server

	mu  sync.Mutex
	mcp MCPState
}

func newApp(root string, port int) *App {
	return &App{root: root, port: port, emit: runtime.EventsEmit, usage: configPath("mcp-usage.json")}
}

// startup öffnet Log, Statistik und Dienste und startet den MCP-Server. Ein belegter Port hält das Fenster nicht
// auf: der Grund steht im Log und im Badge.
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
	store := console.New(console.DefaultCapacity, nil)
	var err error
	a.log, err = applog.Open(filepath.Join(a.root, "logs"), store)
	if err != nil {
		fmt.Fprintln(os.Stderr, "k3c-dev: eigenes Log nur im Speicher:", err)
	}
	a.tracker = usage.Open(a.usage, time.Now,
		func(err error) { a.log.Warn(err.Error(), "ns", "usage") })
	svcCtx, cancel := context.WithCancel(ctx)
	a.cancel = cancel
	ctl, svcErr := openServices(svcCtx, a.root, store, a.log)
	a.ctl = ctl
	a.srv = mcpsrv.New(mcpsrv.Config{Root: a.root, Port: a.port, Version: version, Console: store,
		Log: a.log.Logger, Usage: a.tracker, Services: ctl, ServicesErr: svcErr})
	err = a.srv.Start()
	if err != nil {
		a.log.Error("start fehlgeschlagen", "ns", "main", "error", err.Error())
	} else {
		a.log.Info("k3c-dev gestartet", "ns", "main", "url", a.srv.URL(), "version", version)
	}
	a.setMCP(err)
}

// shutdown stoppt die eigenen Dienste (rückwärts; übernommene laufen weiter), dann Server, Statistik und Log.
func (a *App) shutdown(context.Context) {
	a.log.Info("k3c-dev beendet", "ns", "main")
	if a.ctl != nil {
		a.ctl.StopAll(context.Background())
	}
	a.cancel()
	_ = a.srv.Stop()
	_ = a.tracker.Flush() // ein Fehler steht schon im Log (Rückruf)
	_ = a.log.Close()
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
	a.mu.Lock()
	defer a.mu.Unlock()
	return Info{Version: version, MCP: a.mcp}
}

// beforeClose merkt Größe und Position des Fensters; das Schließen läuft immer weiter.
func (a *App) beforeClose(ctx context.Context) bool {
	x, y := runtime.WindowGetPosition(ctx)
	w, h := runtime.WindowGetSize(ctx)
	if err := saveWindow(configPath("k3c-dev.json"), windowState{X: x, Y: y, Width: w, Height: h}); err != nil {
		a.log.Warn("fenster nicht gemerkt: "+err.Error(), "ns", "main")
	}
	return false
}

// secondInstance holt das offene Fenster nach vorn, wenn k3c-dev ein zweites Mal gestartet wird.
func (a *App) secondInstance() {
	runtime.WindowUnminimise(a.ctx)
	runtime.WindowShow(a.ctx)
	runtime.WindowSetAlwaysOnTop(a.ctx, true) // Windows holt ein Fenster sonst nicht aus dem Hintergrund
	runtime.WindowSetAlwaysOnTop(a.ctx, false)
}
