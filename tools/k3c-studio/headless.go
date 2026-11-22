package main

import (
	"context"
	"log/slog"
	"os"
	"os/signal"
	"path/filepath"
	"runtime"
	"syscall"
	"time"

	"k3c/engine/conlog"
	"k3c/engine/devrun"
	"k3c/tools/assetkit/windowstate"
	"k3c/tools/k3c-studio/internal/mcpsrv"
)

// Betrieb ohne Fenster: -headless startet App mit dem MCP-Server k3c-studio, -dev baut und startet -headless als
// Kindprozess und startet ihn bei Go-Änderungen neu (engine/devrun, wie k3c-workbench). Die Oberflächen-API für Vite
// folgt mit dem Frontend (WZ63.3). Ohne Wails gibt es keine Einzelinstanz-Sperre.
const (
	devChild = "build/bin/k3c-studio-devchild" // (+ .exe unter Windows, runDev): feste EXE, keine neue Firewall-Abfrage
	// devMCPPort gilt im -dev ohne K3C_STUDIO_PORT: nie der Port des laufenden Studios (mcpsrv.DefaultPort).
	devMCPPort = "5182"
	// devGrace lässt dem Kind Zeit für shutdown, bevor devrun hart beendet.
	devGrace = 15 * time.Second
)

// runHeadless läuft bis Strg+C oder SIGTERM: App wie im Fenster (MCP-Server, Log), ohne Oberfläche.
func runHeadless(root string, port int) error {
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	app := newApp(root, port)
	if devrun.IsChild() {
		app.usage = windowstate.ConfigPath("mcp-usage-studio-dev.json") // die Statistik des Studios bleibt unberührt
	}
	app.startup(context.Background())
	<-ctx.Done()
	app.shutdown(context.Background())
	return nil
}

// runDev baut -headless als Kindprozess und startet ihn neu, wenn sich Go-Code in k3c-studio, assetkit, workkit, engine
// oder data/ (embed.go) oder go.mod/go.sum des Hauptmoduls ändert (devrun beobachtet nur *.go). Der MCP-Port kommt aus
// K3C_STUDIO_PORT (sonst devMCPPort), damit das laufende Studio (5181) unberührt bleibt.
func runDev(root string) error {
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	env := []string{mcpsrv.EnvPort + "=" + devMCPPort}
	if v := os.Getenv(mcpsrv.EnvPort); v != "" {
		env[0] = mcpsrv.EnvPort + "=" + v
	}
	child := devChild
	if runtime.GOOS == "windows" {
		child += ".exe"
	}
	return devrun.Run(ctx, devrun.Config{
		Name: "k3c-studio", Dir: filepath.Join(root, "tools", "k3c-studio"), Package: ".", Child: child, Args: []string{"-headless"},
		Env: env, Watch: []string{".", "../assetkit", "../workkit", "../../engine", "../../data", "../../go.mod", "../../go.sum"},
		Grace: devGrace, Log: slog.New(conlog.New(os.Stderr, slog.LevelInfo, conlog.Color())),
	})
}
