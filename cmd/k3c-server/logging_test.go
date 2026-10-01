package main

import (
	"bytes"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// B-066/AC-01: JSON-Zeile im Format aus B-046 neben dem Text-Log.
func TestJSONLogNebenStderr(t *testing.T) {
	dir := t.TempDir()
	var stderr bytes.Buffer
	log, closeLog := newLogger(dir, &stderr)
	log.Info("Raum offen", "ns", "room", "code", "FAMILIE")
	log.Error("kaputt", "ns", "store")
	closeLog()

	raw, err := os.ReadFile(filepath.Join(dir, logFile))
	if err != nil {
		t.Fatal(err)
	}
	lines := strings.Split(strings.TrimSpace(string(raw)), "\n")
	if len(lines) != 2 {
		t.Fatalf("%d Zeilen: %q", len(lines), raw)
	}
	var e map[string]any
	if err := json.Unmarshal([]byte(lines[0]), &e); err != nil {
		t.Fatalf("keine JSON-Zeile: %v", err)
	}
	if e["level"] != "INFO" || e["msg"] != "Raum offen" || e["ns"] != "room" || e["code"] != "FAMILIE" || e["time"] == nil {
		t.Errorf("Eintrag: %v", e)
	}
	if out := stderr.String(); !strings.Contains(out, "Raum offen") || !strings.Contains(out, "kaputt") {
		t.Errorf("Text-Log auf stderr fehlt: %q", out)
	}
}

// B-066/AC-02: ohne Ordner keine Datei, nur stderr.
func TestOhneLogOrdnerNurStderr(t *testing.T) {
	t.Chdir(t.TempDir())
	t.Setenv("K3C_LOG_DIR", "")
	if dir := logDir(); dir != "" {
		t.Fatalf("logDir = %q", dir)
	}
	var stderr bytes.Buffer
	log, closeLog := newLogger("", &stderr)
	log.Info("nur Text")
	closeLog()
	if !strings.Contains(stderr.String(), "nur Text") {
		t.Errorf("stderr: %q", stderr.String())
	}
	if entries, _ := os.ReadDir("."); len(entries) != 0 {
		t.Errorf("Dateien entstanden: %v", entries)
	}
}

func TestLogOrdnerAusUmgebungUndVorhandenemLogs(t *testing.T) {
	t.Chdir(t.TempDir())
	t.Setenv("K3C_LOG_DIR", "")
	if err := os.Mkdir("logs", 0o755); err != nil {
		t.Fatal(err)
	}
	if dir := logDir(); dir != "logs" {
		t.Errorf("vorhandenes logs/: %q", dir)
	}
	t.Setenv("K3C_LOG_DIR", "/anderswo")
	if dir := logDir(); dir != "/anderswo" {
		t.Errorf("K3C_LOG_DIR: %q", dir)
	}
}

// B-066/AC-02: nicht beschreibbar → stderr plus Warnung, kein Absturz.
func TestNichtBeschreibbarerLogOrdner(t *testing.T) {
	notDir := filepath.Join(t.TempDir(), "datei")
	if err := os.WriteFile(notDir, nil, 0o644); err != nil {
		t.Fatal(err)
	}
	var stderr bytes.Buffer
	log, closeLog := newLogger(notDir, &stderr)
	log.Info("weiter")
	closeLog()
	out := stderr.String()
	if !strings.Contains(out, "JSON-Log nicht geschrieben") || !strings.Contains(out, "weiter") {
		t.Errorf("stderr: %q", out)
	}
}
