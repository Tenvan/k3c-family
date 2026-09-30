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
	tracker := usage.Open(usagePath(), time.Now, func(err error) { log.Warn(err.Error(), "ns", "usage") })
	srv := mcpsrv.New(mcpsrv.Config{Root: root, Port: port, Version: version, Console: store, Log: log.Logger,
		Usage: tracker})
	if err := srv.Start(); err != nil {
		log.Error("start fehlgeschlagen", "ns", "main", "error", err.Error())
		return err
	}
	log.Info("k3c-dev gestartet", "ns", "main", "url", srv.URL(), "version", version)
	fmt.Fprintf(os.Stderr, "k3c-dev lauscht an %s (Strg+C beendet)\n", srv.URL())
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()
	<-ctx.Done()
	log.Info("k3c-dev beendet", "ns", "main")
	err = srv.Stop()
	_ = tracker.Flush() // ein Fehler steht schon im Log (Rückruf)
	return err
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
