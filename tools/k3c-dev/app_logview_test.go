package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/tools/k3c-dev/internal/mcpsrv"
)

// logApp ist eine App nur mit Server über einem Repo mit logs/test.jsonl (fünf Einträge, der letzte vor 30 h).
func logApp(t *testing.T) *App {
	t.Helper()
	root := t.TempDir()
	now := time.Now().UTC()
	entry := func(ago time.Duration, level, ns, msg string) string {
		return `{"time":"` + now.Add(-ago).Format(time.RFC3339Nano) + `","level":"` + level + `","ns":"` + ns +
			`","msg":"` + msg + `","port":8080}` + "\n"
	}
	text := entry(30*time.Hour, "ERROR", "svc", "port 8079 belegt") +
		entry(3*time.Minute, "INFO", "mcp", "aufruf beendet") +
		entry(2*time.Minute, "WARN", "svc", "port 8080 belegt") +
		entry(time.Minute, "WARN", "svc", "port 8081 belegt") +
		entry(0, "ERROR", "check", "lauf abgebrochen")
	if err := os.MkdirAll(filepath.Join(root, "logs"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "logs", "test.jsonl"), []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
	return &App{srv: mcpsrv.New(mcpsrv.Config{Root: root})}
}

func TestLogsQuery(t *testing.T) {
	app := logApp(t)
	all, err := app.LogsQuery("test", LogQuery{Limit: 100})
	if err != nil || len(all.Entries) != 5 || all.Entries[0].Msg != "lauf abgebrochen" || all.Missing || all.BytesRead == 0 {
		t.Fatalf("alle: %+v, %v", all, err)
	}
	if v, _ := app.LogsQuery("test", LogQuery{MinLevel: "WARN", Limit: 100}); len(v.Entries) != 4 {
		t.Errorf("ab WARN: %d Einträge", len(v.Entries))
	}
	if v, _ := app.LogsQuery("test", LogQuery{NS: "svc", Pattern: "808[01]", Limit: 100}); len(v.Entries) != 2 {
		t.Errorf("svc und Suche: %d Einträge", len(v.Entries))
	}
	if v := all.Entries[1]; v.Data["port"] != float64(8080) {
		t.Errorf("Daten fehlen: %+v", v)
	}
	if v, err := app.LogsQuery("k3c-dev", LogQuery{Limit: 100}); err != nil || !v.Missing || len(v.Entries) != 0 {
		t.Errorf("fehlende Datei: %+v, %v", v, err)
	}
}

// Eingaben aus dem Frontend prüft Go: Suche, Limit, Level und Quelle (nie ein Pfad).
func TestLogsQueryAbgelehnt(t *testing.T) {
	app := logApp(t)
	bad := map[string]LogQuery{
		"ungültige Suche":   {Pattern: "(", Limit: 100},
		"limit 150":         {Limit: 150},
		"unbekanntes Level": {MinLevel: "FATAL", Limit: 100},
	}
	for want, q := range bad {
		if _, err := app.LogsQuery("test", q); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%+v: %v", q, err)
		}
	}
	if _, err := app.LogsQuery("../test", LogQuery{Limit: 100}); err == nil || !strings.Contains(err.Error(), "unbekannte Log-Quelle") {
		t.Errorf("Pfad aus dem Frontend: %v", err)
	}
}

func TestLogsErrors(t *testing.T) {
	app := logApp(t)
	v, err := app.LogsErrors("test", "WARN")
	if err != nil || v.Entries != 3 || len(v.Groups) != 2 {
		t.Fatalf("WARN: %+v, %v", v, err)
	}
	if g := v.Groups[0]; g.Count != 2 || g.NS != "svc" || g.Fingerprint != "port <n> belegt" || g.Example != "port 8081 belegt" {
		t.Errorf("Gruppe: %+v", g)
	}
	if v, _ := app.LogsErrors("test", "ERROR"); len(v.Groups) != 1 || v.Groups[0].Example != "lauf abgebrochen" {
		t.Errorf("ERROR (ohne den Eintrag vor 30 h): %+v", v.Groups)
	}
	if _, err := app.LogsErrors("test", "INFO"); err == nil {
		t.Error("INFO nicht abgelehnt")
	}
	if v, err := app.LogsErrors("k3c-dev", "WARN"); err != nil || !v.Missing || v.Groups == nil {
		t.Errorf("fehlende Datei: %+v, %v", v, err)
	}
}
