// Command k3c-dev ist das Entwickler-Werkzeug von K3C: ein Fenster, das den MCP-Server für Coding-Agenten hostet
// (B-046) und Dienste, Läufe und Logs zeigt (B-064). Schließen beendet Programm und MCP-Server.
package main

import (
	"context"
	"embed"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
	"github.com/wailsapp/wails/v2/pkg/runtime"

	"k3c/tools/k3c-dev/internal/applog"
	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/mcpsrv"
	"k3c/tools/k3c-dev/internal/services"
)

const version = "0.1.0"

// instanceID verbindet einen zweiten Start mit dem laufenden Fenster.
const instanceID = "k3c-dev-5d0e8b7c-6f1a-4a51-9d0e-6b3c2a1f9e47"

//go:embed all:frontend/dist
var assets embed.FS

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "k3c-dev:", err)
		os.Exit(1)
	}
}

// run startet das Fenster; -headless nur MCP-Server und Oberflächen-API (Oberfläche im Browser), -dev dasselbe als
// neu startender Kindprozess bei Go-Änderungen (headless.go).
func run() error {
	headless := flag.Bool("headless", false, "ohne Fenster: MCP-Server und Oberflächen-API ("+envUIPort+")")
	dev := flag.Bool("dev", false, "wie -headless, neu gebaut und gestartet bei jeder Go-Änderung")
	flag.Parse()
	wd, err := os.Getwd()
	if err != nil {
		return err
	}
	root, err := mcpsrv.FindRoot(wd)
	if err != nil {
		return err
	}
	port, err := mcpsrv.ResolvePort(os.Getenv(mcpsrv.EnvPort))
	if err != nil {
		return err
	}
	switch {
	case *dev:
		return runDev(root)
	case *headless:
		return runHeadless(root, port)
	}
	app := newApp(root, port)
	app.window = true
	return wails.Run(windowOptions(app))
}

// windowOptions beschreibt das Fenster: gemerkte Größe, verdeckter Start, bis die gemerkte Position gesetzt ist.
func windowOptions(app *App) *options.App {
	win, placed := loadWindow(configPath("k3c-dev.json"))
	return &options.App{
		Title: "K3C Workbench", Width: win.Width, Height: win.Height, MinWidth: minWidth, MinHeight: minHeight,
		StartHidden:      true,
		BackgroundColour: &options.RGBA{R: 0x0b, G: 0x10, B: 0x26, A: 0xff}, // --night-1
		AssetServer:      &assetserver.Options{Assets: assets},
		OnStartup:        app.startup,
		OnDomReady: func(ctx context.Context) {
			if placed && onScreen(win) {
				placeWindow(ctx, win.X, win.Y)
			}
			runtime.WindowShow(ctx)
		},
		OnBeforeClose: app.beforeClose,
		OnShutdown:    app.shutdown,
		SingleInstanceLock: &options.SingleInstanceLock{UniqueId: instanceID,
			OnSecondInstanceLaunch: func(options.SecondInstanceData) { app.secondInstance() }},
		Bind: []any{app},
	}
}

// openServices lädt services.json, übernimmt laufende Dienste und misst alle 2 s, bis ctx endet. Eine kaputte
// Konfiguration hält den Server nicht auf: dann gibt es keine Dienste, und svc_status nennt den Grund.
// onChange meldet jede Änderung eines Dienstes (Oberfläche).
func openServices(ctx context.Context, root string, store *console.Store, log *applog.Log,
	onChange func(services.Status)) (*services.Controller, error) {
	list, err := services.Load(filepath.Join(root, "tools", "k3c-dev", "services.json"))
	if err != nil {
		log.Error("💥 dienste nicht geladen: "+err.Error(), "ns", "svc")
		return nil, err
	}
	ctl := services.New(services.Shift(list, 0), services.Options{Root: root, Console: store, Log: log.Logger, OnChange: onChange})
	ctl.Adopt(ctx)
	go ctl.Monitor(ctx, 2*time.Second)
	return ctl, nil
}
