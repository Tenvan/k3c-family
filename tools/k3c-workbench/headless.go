package main

import (
	"context"
	"log/slog"
	"os"
	"os/signal"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"syscall"
	"time"

	"k3c/engine/conlog"
	"k3c/engine/devrun"
	"k3c/tools/assetkit/windowstate"
	"k3c/tools/k3c-workbench/internal/mcpsrv"
	"k3c/tools/workkit/uikit"
)

// Client-Server-Betrieb ohne Fenster: -headless startet App und die HTTP-Schicht der Oberfläche (uiapi.go,
// tools/workkit/uikit), die Oberfläche läuft im Browser (Vite, frontend/src/api/http.ts). -dev baut und startet
// -headless als Kindprozess und startet ihn bei Go-Änderungen neu (engine/devrun). Ohne Wails gibt es keine
// Einzelinstanz-Sperre.
const (
	envUIPort     = "K3C_DEV_UI_API_PORT"
	defaultUIPort = 8191
	devChild      = "build/bin/k3c-workbench-devchild-" // + UI-Port (+ .exe unter Windows, runDev): je Lauf eine eigene EXE
	// devMCPPort gilt im -dev ohne K3C_DEV_PORT: nie der Port der laufenden Workbench (mcpsrv.DefaultPort).
	devMCPPort = "5190"
	// devGrace lässt dem Kind Zeit für shutdown (Tasks und eigene Dienste stoppen), bevor devrun hart beendet.
	devGrace = 15 * time.Second
)

// runHeadless läuft bis Strg+C oder SIGTERM: App wie im Fenster (MCP-Server, Dienste), dazu die Oberflächen-API auf
// 127.0.0.1:<K3C_DEV_UI_API_PORT> (die Oberfläche liefert Vite).
func runHeadless(root string, port int) error {
	uiPort, err := uikit.ResolvePort(envUIPort, os.Getenv(envUIPort), defaultUIPort)
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
		app.usage = windowstate.ConfigPath("mcp-usage-dev.json") // die Statistik der Workbench bleibt unberührt
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

// runDev baut -headless als Kindprozess und startet ihn neu, wenn sich Go-Code in k3c-workbench, assetkit, workkit, engine oder
// data/ (embed.go) oder go.mod/go.sum des Hauptmoduls ändert; geänderte JSON-Dateien in data/ lösen keinen Neubau aus
// (devrun beobachtet nur *.go). Der MCP-Port kommt aus K3C_DEV_PORT (Taskfile: 5190), damit die laufende Workbench
// (5180) unberührt bleibt.
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
	uiPort := os.Getenv(envUIPort)
	if uiPort == "" {
		uiPort = strconv.Itoa(defaultUIPort)
	}
	child := devChild + uiPort
	if runtime.GOOS == "windows" {
		child += ".exe"
	}
	return devrun.Run(ctx, devrun.Config{
		Name: "k3c-workbench", Dir: filepath.Join(root, "tools", "k3c-workbench"), Package: ".", Child: child, Args: []string{"-headless"},
		Env: env, Watch: []string{".", "../assetkit", "../workkit", "../../engine", "../../data", "../../go.mod", "../../go.sum"},
		Grace: devGrace, Log: slog.New(conlog.New(os.Stderr, slog.LevelInfo, conlog.Color())),
	})
}
