// Command k3c-studio ist das Werkzeug für Game- und Level-Designer von K3C (WZ63): Welten, Inseln, Chunks, Hub und
// Assets bearbeiten, ohne die Entwickler-Workbench. Ein Fenster hostet den MCP-Server k3c-studio für Coding-Agenten;
// Schließen beendet Programm und MCP-Server.
package main

import (
	"context"
	"embed"
	"flag"
	"fmt"
	"os"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
	"github.com/wailsapp/wails/v2/pkg/runtime"

	"k3c/tools/assetkit/windowstate"
	"k3c/tools/k3c-studio/internal/mcpsrv"
)

const version = "0.1.0"

// instanceID verbindet einen zweiten Start mit dem laufenden Fenster.
const instanceID = "k3c-studio-3f6c1d2a-8b4e-4c71-a0d9-5e2b7c9f1a36"

// windowFile merkt Größe und Position des Fensters (windowstate).
const windowFile = "k3c-studio.json"

// assets ist die gebaute Oberfläche. Bis WZ63.3 liegt dort nur der eingecheckte Platzhalter .gitkeep, damit
// go build und go test ohne Frontend laufen.
//
//go:embed all:frontend/dist
var assets embed.FS

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "k3c-studio:", err)
		os.Exit(1)
	}
}

// run startet das Fenster; -headless nur den MCP-Server, -dev dasselbe als neu startender Kindprozess bei
// Go-Änderungen (headless.go).
func run() error {
	headless := flag.Bool("headless", false, "ohne Fenster: nur der MCP-Server ("+mcpsrv.EnvPort+")")
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
	win, placed := windowstate.Load(windowstate.ConfigPath(windowFile))
	return &options.App{
		Title: "K3C Studio", Width: win.Width, Height: win.Height, MinWidth: windowstate.MinWidth, MinHeight: windowstate.MinHeight,
		StartHidden:      true,
		BackgroundColour: &options.RGBA{R: 0x0b, G: 0x10, B: 0x26, A: 0xff}, // --night-1
		AssetServer:      &assetserver.Options{Assets: assets},
		OnStartup:        app.startup,
		OnDomReady: func(ctx context.Context) {
			if placed && windowstate.OnScreen(win) {
				windowstate.Place(func() (int, int) { return runtime.WindowGetPosition(ctx) },
					func(x, y int) { runtime.WindowSetPosition(ctx, x, y) }, win.X, win.Y)
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
