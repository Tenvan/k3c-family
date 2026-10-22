package main

import (
	"log/slog"
	"os"
	"path/filepath"
	"testing"

	"k3c/tools/k3c-workbench/internal/applog"
)

// B-459: „Asset-Studio öffnen“ startet bin/k3c-studio im Checkout; fehlt die EXE, kommt der Hinweis und nichts startet.
func TestStudioOpen(t *testing.T) {
	root := t.TempDir()
	app := &App{root: root, log: &applog.Log{Logger: slog.Default()}}
	exe := app.studioExe()
	if err := os.MkdirAll(filepath.Dir(exe), 0o750); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(exe, nil, 0o600); err != nil {
		t.Fatal(err)
	}
	var started []string
	app.launch = func(e, dir string) error { started = append(started, e, dir); return nil }
	if err := app.StudioOpen(); err != nil || len(started) != 2 || started[0] != exe || started[1] != root {
		t.Fatalf("gestartet %v, Fehler %v", started, err)
	}
}

func TestStudioOpenOhneExe(t *testing.T) {
	app := &App{root: t.TempDir()}
	app.launch = func(string, string) error { t.Fatal("trotz fehlender EXE gestartet"); return nil }
	if err := app.StudioOpen(); err != errStudioNotBuilt {
		t.Fatalf("Fehler %v, erwartet %v", err, errStudioNotBuilt)
	}
}
