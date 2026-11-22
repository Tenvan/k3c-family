package main

import (
	"context"
	"log/slog"
	"os"
	"os/signal"
	"path/filepath"
	"runtime"
	"strings"
	"syscall"
	"time"

	"k3c/engine/conlog"
	"k3c/engine/devrun"
	"k3c/tools/assetkit/windowstate"
	"k3c/tools/k3c-studio/internal/mcpsrv"
	"k3c/tools/workkit/uikit"
)

// Betrieb ohne Fenster: -headless startet App mit dem MCP-Server k3c-studio und der Oberflächen-API (uiapi.go,
// tools/workkit/uikit; die Oberfläche läuft im Browser unter Vite, frontend/src/api/http.ts), -dev baut und startet
// -headless als Kindprozess und startet ihn bei Go-Änderungen neu (engine/devrun, wie k3c-workbench). Ohne Wails gibt
// es keine Einzelinstanz-Sperre.
const (
	envUIPort = "K3C_STUDIO_UI_API_PORT"
	// defaultUIPort: nächster freie nach der Workbench (8191); 8182 und 8091 gehörten dem alten Asset-Studio.
	defaultUIPort = 8193
	devChild      = "build/bin/k3c-studio-devchild" // (+ .exe unter Windows, runDev): feste EXE, keine neue Firewall-Abfrage
	// devMCPPort gilt im -dev ohne K3C_STUDIO_PORT: nie der Port des laufenden Studios (mcpsrv.DefaultPort).
	devMCPPort = "5182"
	// devGrace lässt dem Kind Zeit für shutdown, bevor devrun hart beendet.
	devGrace = 15 * time.Second
)

// runHeadless läuft bis Strg+C oder SIGTERM: App wie im Fenster (MCP-Server, Log), dazu die Oberflächen-API auf
// 127.0.0.1:<K3C_STUDIO_UI_API_PORT> (die Oberfläche liefert Vite).
func runHeadless(root string, port int) error {
	// Ohne eigenen Wert folgt der UI-Port dem MCP-Port (Versatz 10, 20 … der Worktree-Dienste): 5191 → 8203.
	uiDefault := defaultUIPort
	if port > 0 {
		uiDefault += port - mcpsrv.DefaultPort
	}
	uiPort, err := uikit.ResolvePort(envUIPort, os.Getenv(envUIPort), uiDefault)
	if err != nil {
		return err
	}
	ln, err := uikit.Listen(envUIPort, uiPort)
	if err != nil {
		return err
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	app := newApp(root, port)
	if devrun.IsChild() {
		app.usage = windowstate.ConfigPath("mcp-usage-studio-dev.json") // die Statistik des Studios bleibt unberührt
	}
	app.startup(context.Background())
	pageURL := uikit.PageURL(ln)
	app.log.Info("🚀 Oberflächen-API bereit", "ns", "main", "url", pageURL+strings.TrimPrefix(uikit.Prefix, "/"), "seite", pageURL)
	if err = uikit.Serve(ctx, ln, app.uiHandler()); err != nil {
		app.log.Error("💥 Oberflächen-API beendet", "ns", "main", "error", err.Error())
	}
	app.shutdown(context.Background())
	return err
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
	if v, ok := os.LookupEnv(envUIPort); ok {
		env = append(env, envUIPort+"="+v)
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
