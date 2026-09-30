// Command k3c-dev ist das Entwickler-Werkzeug von K3C: ein MCP-Server für Coding-Agenten (B-046).
// Bis zur Oberfläche (B-064) läuft es ohne Fenster im Terminal und endet mit Strg+C.
package main

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"os"
	"os/signal"
	"path/filepath"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/applog"
	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/mcpsrv"
	"k3c/tools/k3c-dev/internal/services"
	"k3c/tools/k3c-dev/internal/usage"
)

const version = "0.1.0"

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "k3c-dev:", err)
		os.Exit(1)
	}
}

func run() error {
	wd, err := os.Getwd()
	if err != nil {
		return err
	}
	root, err := findRoot(wd)
	if err != nil {
		return err
	}
	port, err := mcpsrv.ResolvePort(os.Getenv(mcpsrv.EnvPort))
	if err != nil {
		return err
	}
	return serve(root, port)
}

// serve öffnet das eigene Log, startet den Server und wartet auf Strg+C.
func serve(root string, port int) error {
	store := console.New(console.DefaultCapacity, nil)
	log, err := applog.Open(filepath.Join(root, "logs"), store)
	if err != nil {
		fmt.Fprintln(os.Stderr, "k3c-dev: eigenes Log nur im Speicher:", err)
	}
	defer func() { _ = log.Close() }()
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()
	tracker := usage.Open(usagePath(), time.Now, func(err error) { log.Warn(err.Error(), "ns", "usage") })
	ctl, svcErr := openServices(ctx, root, store, log)
	srv := mcpsrv.New(mcpsrv.Config{Root: root, Port: port, Version: version, Console: store, Log: log.Logger,
		Usage: tracker, Services: ctl, ServicesErr: svcErr})
	if err := srv.Start(); err != nil {
		log.Error("start fehlgeschlagen", "ns", "main", "error", err.Error())
		return err
	}
	log.Info("k3c-dev gestartet", "ns", "main", "url", srv.URL(), "version", version)
	fmt.Fprintf(os.Stderr, "k3c-dev lauscht an %s (Strg+C beendet)\n", srv.URL())
	<-ctx.Done()
	log.Info("k3c-dev beendet", "ns", "main")
	if ctl != nil {
		ctl.StopAll(context.Background()) // nur eigene Dienste, rückwärts; übernommene laufen weiter
	}
	err = srv.Stop()
	_ = tracker.Flush() // ein Fehler steht schon im Log (Rückruf)
	return err
}

// openServices lädt services.json, übernimmt laufende Dienste und misst alle 2 s, bis ctx endet. Eine kaputte
// Konfiguration hält den Server nicht auf: dann gibt es keine Dienste, und svc_status nennt den Grund.
func openServices(ctx context.Context, root string, store *console.Store, log *applog.Log) (*services.Controller, error) {
	list, err := services.Load(filepath.Join(root, "tools", "k3c-dev", "services.json"))
	if err != nil {
		log.Error("dienste nicht geladen: "+err.Error(), "ns", "svc")
		return nil, err
	}
	ctl := services.New(list, services.Options{Root: root, Console: store, Log: log.Logger})
	ctl.Adopt(ctx)
	go ctl.Monitor(ctx, 2*time.Second)
	return ctl, nil
}

// usagePath ist die Datei der Nutzungsstatistik im Benutzerprofil, nicht im Repo.
func usagePath() string {
	dir, err := os.UserConfigDir()
	if err != nil {
		dir = os.TempDir()
	}
	return filepath.Join(dir, "k3c", "mcp-usage.json")
}

// findRoot sucht ab dir aufwärts das go.mod des Spiels (module k3c): die Repo-Wurzel.
func findRoot(dir string) (string, error) {
	for {
		if isGameModule(filepath.Join(dir, "go.mod")) {
			return dir, nil
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			return "", errors.New("repo-wurzel nicht gefunden: kein go.mod mit \"module k3c\" über dem Arbeitsverzeichnis")
		}
		dir = parent
	}
}

func isGameModule(path string) bool {
	f, err := os.Open(path)
	if err != nil {
		return false
	}
	defer func() { _ = f.Close() }()
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		if line := strings.TrimSpace(sc.Text()); strings.HasPrefix(line, "module ") {
			return strings.TrimSpace(strings.TrimPrefix(line, "module ")) == "k3c"
		}
	}
	return false
}
