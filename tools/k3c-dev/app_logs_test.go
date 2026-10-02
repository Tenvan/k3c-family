package main

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"sync/atomic"
	"testing"
	"time"

	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/mcpsrv"
	"k3c/tools/k3c-dev/internal/services"
)

// startedApp startet eine App auf einem leeren Repo (ohne services.json); lines zählt gemeldete console:line.
func startedApp(t *testing.T) (*App, *atomic.Int32) {
	t.Helper()
	root := t.TempDir()
	if err := os.MkdirAll(filepath.Join(root, "logs"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "logs", "server.jsonl"), nil, 0o644); err != nil {
		t.Fatal(err)
	}
	app := newApp(root, 0)
	app.usage = filepath.Join(root, "usage.json")
	var lines atomic.Int32
	app.emit = func(_ context.Context, name string, data ...any) {
		if l, ok := data[0].([]console.Line); ok && name == evConsoleLine && len(l) == 1 {
			lines.Add(1)
		}
	}
	app.startup(context.Background())
	t.Cleanup(func() { app.shutdown(context.Background()) })
	return app, &lines
}

func TestSourcesUndConsoleTail(t *testing.T) {
	app, _ := startedApp(t)
	got := map[string]Source{}
	for _, s := range app.Sources() {
		got[s.Name] = s
	}
	if s := got["k3c-dev"]; s.Kind != kindLog || s.State != logEntries {
		t.Errorf("eigenes Log = %+v", s)
	}
	if s := got["server"]; s.Kind != kindLog || s.State != logEmpty {
		t.Errorf("leeres Log = %+v", s)
	}
	own, err := app.ConsoleTail("k3c-dev")
	if err != nil || len(own) == 0 || !strings.Contains(own[len(own)-1].Text, "k3c-dev gestartet") {
		t.Errorf("ConsoleTail(k3c-dev) = %+v, %v", own, err)
	}
	if empty, err := app.ConsoleTail("server"); err != nil || empty == nil || len(empty) != 0 {
		t.Errorf("bekannte Quelle ohne Ausgabe = %v, %v", empty, err)
	}
	if _, err := app.ConsoleTail("../geheim"); err == nil || !strings.Contains(err.Error(), "gültig: k3c-client, k3c-dev, server, vite") {
		t.Errorf("unbekannte Quelle: %v", err)
	}
}

// Der Spiegel des eigenen Logs kommt als console:line an die Oberfläche.
func TestConsoleLineEreignis(t *testing.T) {
	_, lines := startedApp(t)
	for deadline := time.Now().Add(time.Second); lines.Load() == 0 && time.Now().Before(deadline); {
		time.Sleep(5 * time.Millisecond)
	}
	if lines.Load() == 0 {
		t.Error("keine console:line für den Spiegel des eigenen Logs")
	}
}

func TestSourceZustaende(t *testing.T) {
	at := time.Date(2026, 9, 30, 8, 3, 15, 0, time.Local)
	cases := []struct {
		st          mcpsrv.CheckState
		state, want string
	}{
		{mcpsrv.CheckState{Name: "task:test", Running: true, At: at}, runRunning, "läuft seit 08:03:15"},
		{mcpsrv.CheckState{Name: "task:test", Ms: 12400, At: at}, runOK, "Exit 0 · 12,4 s · 08:03:15"},
		{mcpsrv.CheckState{Name: "task:test", Exit: 1, Ms: 800, At: at}, runFailed, "Exit 1 · 0,8 s · 08:03:15"},
		{mcpsrv.CheckState{Name: "task:test", TimedOut: true, Ms: 300000, At: at}, runTimeout, "Zeitlimit nach 300,0 s · 08:03:15"},
		{mcpsrv.CheckState{Name: "task:test", Error: "exec: task nicht gefunden", At: at}, runFailed, "nicht gestartet: exec: task nicht gefunden"},
	}
	for _, c := range cases {
		if s := checkSource(c.st); s.Name != "check:task:test" || s.Kind != kindRun || s.State != c.state || s.Detail != c.want {
			t.Errorf("checkSource(%+v) = %+v", c.st, s)
		}
	}
	s := serviceSource(services.Status{Name: "Vite", Port: 5173, PID: 41232, State: services.Running})
	if s.Kind != kindService || s.State != "läuft" || s.Detail != "Port 5173 · PID 41232" {
		t.Errorf("serviceSource = %+v", s)
	}
}
