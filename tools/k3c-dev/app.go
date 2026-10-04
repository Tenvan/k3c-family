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
	"k3c/tools/k3c-dev/internal/github"
	"k3c/tools/k3c-dev/internal/mcpsrv"
	"k3c/tools/k3c-dev/internal/planning"
	"k3c/tools/k3c-dev/internal/services"
	"k3c/tools/k3c-dev/internal/taskrun"
	"k3c/tools/k3c-dev/internal/usage"
)

// Ereignisse an die Oberfläche (B-064 › Ereignisse). Weitere kommen mit den Seiten dazu, die sie senden.
const (
	evMCPState     = "mcp:state"
	evServiceState = "service:state"
	evSourceState  = "source:state"
	evConsoleLine  = "console:line"
	evMCPStart     = "mcp:start"
	evMCPCall      = "mcp:call"
	evTaskState    = "task:state"
	evPlanning     = "planning:changed"
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

// App ist das Backend des Fensters: es hostet MCP-Server, Dienste und Log; seine exportierten Methoden sind die
// Bindings der Oberfläche (frontend/src/api/).
type App struct {
	root  string
	port  int
	emit  func(ctx context.Context, name string, data ...any) // Test-Naht; Produktion: runtime.EventsEmit
	usage string                                              // Datei der Nutzungsstatistik (Test-Naht)
	// ready schließt startup: Wails ruft OnStartup in einer eigenen Goroutine, Bindings können davor kommen.
	ready chan struct{}

	ctx        context.Context
	svcCtx     context.Context // endet beim Beenden; Befehle der Oberfläche laufen darin
	cancel     context.CancelFunc
	store      *console.Store // Konsolenpuffer aller Quellen
	log        *applog.Log
	tracker    *usage.Tracker
	ctl        *services.Controller
	svcErr     error // services.json nicht geladen
	srv        *mcpsrv.Server
	tasks      taskState
	taskRunner *taskrun.Runner
	gh         *github.Client // GitHub-Stand der Planungsseite (B-212)

	mu  sync.Mutex
	mcp MCPState
}

func newApp(root string, port int) *App {
	return &App{root: root, port: port, emit: runtime.EventsEmit, usage: configPath("mcp-usage.json"),
		ready: make(chan struct{}), gh: github.New(root)}
}

// wait blockiert, bis startup fertig ist; danach sind alle Felder gesetzt und werden nicht mehr geschrieben.
// Ohne ready (Tests, die App direkt bauen) wartet es nicht.
func (a *App) wait() {
	if a.ready != nil {
		<-a.ready
	}
}

// startup öffnet Log, Statistik und Dienste und startet den MCP-Server. Ein belegter Port hält das Fenster nicht
// auf: der Grund steht im Log und im Badge.
func (a *App) startup(ctx context.Context) {
	defer a.markReady()
	a.ctx = ctx
	// console:line trägt eine Liste, damit Go später bündeln kann, ohne den Vertrag zu ändern.
	a.store = console.New(console.DefaultCapacity, func(l console.Line) { a.emit(a.ctx, evConsoleLine, []console.Line{l}) })
	store := a.store
	var err error
	a.log, err = applog.Open(filepath.Join(a.root, "logs"), store)
	if err != nil {
		fmt.Fprintln(os.Stderr, "k3c-dev: eigenes Log nur im Speicher:", err)
	}
	a.tracker = usage.Open(a.usage, time.Now,
		func(err error) { a.log.Warn("📄 "+err.Error(), "ns", "usage") })
	a.initTasks()
	a.svcCtx, a.cancel = context.WithCancel(ctx)
	planning.Watch(a.svcCtx, a.root, planningPoll, func() { a.emit(a.ctx, evPlanning, nil) })
	a.ctl, a.svcErr = openServices(a.svcCtx, a.root, store, a.log,
		func(st services.Status) {
			a.emit(a.ctx, evServiceState, st)
			a.emit(a.ctx, evSourceState, serviceSource(st))
		})
	a.srv = mcpsrv.New(mcpsrv.Config{Root: a.root, Port: a.port, Version: version, Console: store,
		Log: a.log.Logger, Usage: a.tracker, Services: a.ctl, ServicesErr: a.svcErr,
		OnCheck: func(st mcpsrv.CheckState) { a.emit(a.ctx, evSourceState, checkSource(st)) },
		OnStart: func(c mcpsrv.Call) { a.emit(a.ctx, evMCPStart, c) },
		OnCall:  func(c mcpsrv.Call) { a.emit(a.ctx, evMCPCall, c) }})
	err = a.srv.Start()
	if err != nil {
		a.log.Error("💥 start fehlgeschlagen", "ns", "main", "error", err.Error())
	} else {
		a.log.Info("🚀 k3c-dev gestartet", "ns", "main", "url", a.srv.URL(), "version", version)
	}
	a.setMCP(err)
}

func (a *App) markReady() {
	if a.ready != nil {
		close(a.ready)
	}
}

// shutdown bricht laufende Befehle ab (ein Start wartet sonst bis 60 s auf gesund), stoppt die eigenen Dienste
// (rückwärts; übernommene laufen weiter) und die der Worktrees, dann Server, Statistik und Log.
func (a *App) shutdown(context.Context) {
	a.wait()
	a.log.Info("🛑 k3c-dev beendet", "ns", "main")
	a.cancel()
	// Ein halb gelaufener Testlauf darf die Anwendung nicht überleben, unabhängig von den Diensten.
	stopCtx, stop := context.WithTimeout(context.Background(), taskStopTimeout)
	if err := a.taskRunner.StopAll(stopCtx); err != nil {
		a.log.Warn("⏳ Tasks nicht vollständig beendet: "+err.Error(), "ns", "tasks")
	}
	stop()
	if a.ctl != nil {
		a.ctl.StopAll(context.Background())
	}
	a.srv.StopWorktreeServices(context.Background())
	_ = a.srv.Stop()
	_ = a.tracker.Flush() // ein Fehler steht schon im Log (Rückruf)
	_ = a.log.Close()
	a.store.Close() // danach keine console:line mehr
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
	if err := saveWindow(configPath("k3c-dev.json"), windowState{X: x, Y: y, Width: w, Height: h}); err != nil {
		a.log.Warn("📄 fenster nicht gemerkt: "+err.Error(), "ns", "main")
	}
	return false
}

// secondInstance holt das offene Fenster nach vorn, wenn k3c-dev ein zweites Mal gestartet wird.
func (a *App) secondInstance() {
	a.wait()
	runtime.WindowUnminimise(a.ctx)
	runtime.WindowShow(a.ctx)
	runtime.WindowSetAlwaysOnTop(a.ctx, true) // Windows holt ein Fenster sonst nicht aus dem Hintergrund
	runtime.WindowSetAlwaysOnTop(a.ctx, false)
}
