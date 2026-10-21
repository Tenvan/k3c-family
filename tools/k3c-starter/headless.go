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
	"syscall"
	"time"

	"k3c/engine/devrun"
)

const (
	// defaultAPIPort ist der Port der Starter-API im Modus -headless (Umgebungsvariable K3C_STARTER_API_PORT).
	defaultAPIPort = "8194"
	// apiListenWait ist die Frist, in der der Port nach einem Neustart des Kindprozesses wieder frei werden darf (devrun.Listen).
	apiListenWait = 10 * time.Second
	apiShutdown   = 2 * time.Second
)

// runHeadless ist der Starter ohne Fenster (Entwicklungsmodus, `task k3c-starter:dev`): Er startet den Server wie OnStartup,
// bietet die Aufrufe der Wails-Bindungen als HTTP-API auf Loopback an (api.go) und beendet den Server bei einem Signal wie OnBeforeClose.
func runHeadless(app *App) error {
	port := os.Getenv("K3C_STARTER_API_PORT")
	if port == "" {
		port = defaultAPIPort
	}
	ln, err := devrun.Listen("tcp", net.JoinHostPort("127.0.0.1", port), apiListenWait)
	if err != nil {
		return fmt.Errorf("API-Port %s: %w", port, err)
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	slog.Info("🚀 Starter headless: API auf " + ln.Addr().String())
	app.start()
	return serveAPI(ctx, app, ln)
}

// serveAPI bedient die API, bis ctx endet (Signal) oder der Listener ausfällt; danach wird der Server beendet.
func serveAPI(ctx context.Context, app *App, ln net.Listener) error {
	srv := &http.Server{Handler: newAPIHandler(app), ReadHeaderTimeout: 5 * time.Second}
	errc := make(chan error, 1)
	go func() { errc <- srv.Serve(ln) }()
	var serveErr error
	select {
	case <-ctx.Done():
		slog.Info("🛑 Starter headless: Signal, Server wird beendet")
	case serveErr = <-errc:
		slog.Error("💥 Starter-API ausgefallen: " + serveErr.Error())
	}
	app.stop()
	down, cancel := context.WithTimeout(context.Background(), apiShutdown)
	defer cancel()
	if err := srv.Shutdown(down); err != nil && !errors.Is(err, context.DeadlineExceeded) {
		slog.Warn("🚫 Starter-API: Beenden: " + err.Error())
	}
	if errors.Is(serveErr, http.ErrServerClosed) {
		return nil
	}
	return serveErr
}
