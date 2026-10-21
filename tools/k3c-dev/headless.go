package main

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"net"
	"net/http"
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
	"k3c/tools/k3c-dev/internal/mcpsrv"
)

// Client-Server-Betrieb ohne Fenster: -headless startet App und die HTTP-Schicht der Oberfläche (uiapi.go), die
// Oberfläche läuft im Browser (Vite, frontend/src/api/http.ts). -dev baut und startet -headless als Kindprozess und
// startet ihn bei Go-Änderungen neu (engine/devrun). Ohne Wails gibt es keine Einzelinstanz-Sperre.
const (
	envUIPort     = "K3C_DEV_UI_API_PORT"
	defaultUIPort = 8191
	// uiListenWait wartet auf den Port, den ein eben beendeter Vorgänger (Neustart im -dev) noch hält.
	uiListenWait = 10 * time.Second
	// uiStopTimeout begrenzt das Warten auf offene Anfragen beim Beenden.
	uiStopTimeout = 2 * time.Second
	devChild      = "build/bin/k3c-dev-devchild" // + .exe unter Windows (runDev)
	// devMCPPort gilt im -dev ohne K3C_DEV_PORT: nie der Port der laufenden Workbench (mcpsrv.DefaultPort).
	devMCPPort = "5190"
	// devGrace lässt dem Kind Zeit für shutdown (Tasks und eigene Dienste stoppen), bevor devrun hart beendet.
	devGrace = 15 * time.Second
)

// resolveUIPort liest den Port der Oberflächen-API; leer heißt defaultUIPort.
func resolveUIPort(raw string) (int, error) {
	raw = strings.TrimSpace(raw)
	if raw == "" {
		return defaultUIPort, nil
	}
	port, err := strconv.Atoi(raw)
	if err != nil || port < 1 || port > 65535 {
		return 0, fmt.Errorf("%s=%q ist keine gültige Portzahl (1–65535)", envUIPort, raw)
	}
	return port, nil
}

// runHeadless läuft bis Strg+C oder SIGTERM: App wie im Fenster (MCP-Server, Dienste), dazu die Oberflächen-API auf
// 127.0.0.1:<K3C_DEV_UI_API_PORT>.
func runHeadless(root string, port int) error {
	uiPort, err := resolveUIPort(os.Getenv(envUIPort))
	if err != nil {
		return err
	}
	ln, err := devrun.Listen("tcp", net.JoinHostPort("127.0.0.1", strconv.Itoa(uiPort)), uiListenWait)
	if err != nil {
		return fmt.Errorf("oberflächen-port %d nicht verfügbar (%w); einen anderen über %s setzen", uiPort, err, envUIPort)
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	app := newApp(root, port)
	if devrun.IsChild() {
		app.usage = configPath("mcp-usage-dev.json") // die Statistik der Workbench bleibt unberührt
	}
	app.startup(context.Background())
	reqCtx, endStreams := context.WithCancel(context.Background()) // beendet die SSE-Ströme beim Stoppen
	srv := &http.Server{Handler: app.uiHandler(), ReadHeaderTimeout: 10 * time.Second,
		BaseContext: func(net.Listener) context.Context { return reqCtx }}
	served := make(chan error, 1)
	go func() { served <- srv.Serve(ln) }()
	app.log.Info("🚀 Oberflächen-API bereit", "ns", "main", "url", "http://"+ln.Addr().String()+uiPrefix)
	select {
	case <-ctx.Done():
	case err = <-served:
		app.log.Error("💥 Oberflächen-API beendet", "ns", "main", "error", err.Error())
	}
	endStreams()
	shutCtx, cancel := context.WithTimeout(context.Background(), uiStopTimeout)
	_ = srv.Shutdown(shutCtx)
	cancel()
	app.shutdown(context.Background())
	if errors.Is(err, http.ErrServerClosed) {
		return nil
	}
	return err
}

// runDev baut -headless als Kindprozess und startet ihn neu, wenn sich Go-Code in k3c-dev, assetkit, engine oder
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
	child := devChild
	if runtime.GOOS == "windows" {
		child += ".exe"
	}
	return devrun.Run(ctx, devrun.Config{
		Name: "k3c-dev", Dir: filepath.Join(root, "tools", "k3c-dev"), Package: ".", Child: child, Args: []string{"-headless"},
		Env: env, Watch: []string{".", "../assetkit", "../../engine", "../../data", "../../go.mod", "../../go.sum"},
		Grace: devGrace, Log: slog.New(conlog.New(os.Stderr, slog.LevelInfo, conlog.Color())),
	})
}
