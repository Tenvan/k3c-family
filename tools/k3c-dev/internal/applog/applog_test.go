package applog

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/logs"
)

func TestSchreibtJSONUndSpiegelt(t *testing.T) {
	dir := filepath.Join(t.TempDir(), "logs")
	store := console.New(0, nil)
	l, err := Open(dir, store)
	if err != nil {
		t.Fatal(err)
	}
	l.With("ns", "check").Warn("lauf beendet", "target", "npm:test", "exit", 1)
	if err := l.Close(); err != nil {
		t.Fatal(err)
	}
	res, err := logs.Scan(filepath.Join(dir, "k3c-dev.jsonl"), logs.Query{})
	if err != nil || len(res.Entries) != 1 {
		t.Fatalf("Scan: %+v, %v", res, err)
	}
	e := res.Entries[0]
	if e.Level != "WARN" || e.NS != "check" || e.Msg != "lauf beendet" || e.Data["target"] != "npm:test" || e.Data["exit"] != 1.0 {
		t.Errorf("Eintrag: %+v", e)
	}
	lines, _ := store.Tail(Source, 0)
	if len(lines) != 1 || lines[0].Stream != "log" || !strings.Contains(lines[0].Text, "level=WARN") {
		t.Errorf("Spiegel: %+v", lines)
	}
}

func TestOrdnerNichtBeschreibbar(t *testing.T) {
	file := filepath.Join(t.TempDir(), "keinordner")
	if err := os.WriteFile(file, nil, 0o644); err != nil {
		t.Fatal(err)
	}
	store := console.New(0, nil)
	l, err := Open(filepath.Join(file, "logs"), store)
	if err == nil {
		t.Fatal("kein Fehler bei nicht anlegbarem Ordner")
	}
	l.Info("geht trotzdem")
	if lines, _ := store.Tail(Source, 0); len(lines) != 1 {
		t.Errorf("Spiegel ohne Datei: %+v", lines)
	}
	if err := l.Close(); err != nil {
		t.Error(err)
	}
}
