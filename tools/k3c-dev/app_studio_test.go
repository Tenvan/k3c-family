package main

import (
	"errors"
	"log/slog"
	"testing"

	"k3c/tools/k3c-dev/internal/applog"
)

// B-459: Das Studio öffnet sich auf dem Port des Dienstes „Spielserver“ (im Worktree mit Versatz); ohne Dienst kommt der
// Hinweis und nichts wird geöffnet.
func TestStudioOpen(t *testing.T) {
	app, _ := serviceApp(t)
	var opened []string
	app.log = &applog.Log{Logger: slog.Default()}
	app.open = func(url string) { opened = append(opened, url) }
	if err := app.StudioOpen(); err != nil || len(opened) != 1 || opened[0] != "http://localhost:8080/studio/index.html" {
		t.Fatalf("geöffnet %v, Fehler %v", opened, err)
	}
}

func TestStudioOpenOhneSpielserver(t *testing.T) {
	app := &App{svcErr: errors.New("services.json fehlt")}
	app.open = func(string) { t.Fatal("trotz Fehler geöffnet") }
	if err := app.StudioOpen(); err == nil {
		t.Fatal("Fehler erwartet")
	}
}
