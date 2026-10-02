package applog

import (
	"os"
	"path/filepath"
	"log/slog"
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
	l.With("ns", "check").Warn("lauf beendet", "target", "task:test", "exit", 1)
	if err := l.Close(); err != nil {
		t.Fatal(err)
	}
	res, err := logs.Scan(filepath.Join(dir, "k3c-dev.jsonl"), logs.Query{})
	if err != nil || len(res.Entries) != 1 {
		t.Fatalf("Scan: %+v, %v", res, err)
	}
	e := res.Entries[0]
	if e.Level != "WARN" || e.NS != "check" || e.Msg != "lauf beendet" || e.Data["target"] != "task:test" || e.Data["exit"] != 1.0 {
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

func TestParseLevel(t *testing.T) {
	for _, c := range []struct {
		in   string
		want slog.Level
		ok   bool
	}{
		{"debug", slog.LevelDebug, true}, {"INFO", slog.LevelInfo, true}, {" warn ", slog.LevelWarn, true},
		{"error", slog.LevelError, true}, {"", 0, false}, {"laut", 0, false},
	} {
		got, ok := ParseLevel(c.in)
		if got != c.want || ok != c.ok {
			t.Errorf("ParseLevel(%q) = %v, %v", c.in, got, ok)
		}
	}
}

func TestLevelsAusUmgebung(t *testing.T) {
	t.Setenv(LevelEnv, "")
	if f, m := Levels(); f != slog.LevelDebug || m != slog.LevelInfo {
		t.Errorf("Standard: %v, %v", f, m)
	}
	t.Setenv(LevelEnv, "warn")
	if f, m := Levels(); f != slog.LevelWarn || m != slog.LevelWarn {
		t.Errorf("warn: %v, %v", f, m)
	}
}

func TestLevelFiltertDatei(t *testing.T) {
	t.Setenv(LevelEnv, "warn")
	dir := t.TempDir()
	l, _ := Open(dir, console.New(0, nil))
	l.Info("leise")
	l.Warn("laut")
	_ = l.Close()
	res, _ := logs.Scan(filepath.Join(dir, "k3c-dev.jsonl"), logs.Query{})
	if len(res.Entries) != 1 || res.Entries[0].Msg != "laut" {
		t.Errorf("Einträge: %+v", res.Entries)
	}
}

func TestRotate(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "a.jsonl")
	for _, c := range []struct {
		name    string
		size    int
		rotated bool
	}{{"klein", 5, false}, {"gross", 50, true}} {
		_ = os.Remove(path)
		_ = os.Remove(path + ".1")
		_ = os.WriteFile(path+".1", []byte("alt"), 0o644)
		_ = os.WriteFile(path, make([]byte, c.size), 0o644)
		if err := Rotate(path, 10); err != nil {
			t.Fatal(err)
		}
		_, errNew := os.Stat(path)
		one, _ := os.ReadFile(path + ".1")
		if c.rotated != (errNew != nil) || c.rotated != (len(one) == c.size) {
			t.Errorf("%s: neu=%v, .1=%d Bytes", c.name, errNew, len(one))
		}
	}
	if err := Rotate(filepath.Join(dir, "fehlt.jsonl"), 10); err != nil {
		t.Errorf("fehlende Datei: %v", err)
	}
}
