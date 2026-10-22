package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// Verletzte Ziele sind kein Fehler (Exit 0, BAL2.3): Der Bericht entsteht, run liefert nil.
func TestVerletztesZielIstKeinFehler(t *testing.T) {
	dir := t.TempDir()
	if err := run([]string{"--targets", "--seeds", "3", "--report-dir", dir}); err != nil {
		t.Fatalf("Bericht ohne Baseline: %v", err)
	}
	md, _ := filepath.Glob(filepath.Join(dir, "balance-*.md"))
	if len(md) != 1 {
		t.Fatalf("erwartet einen Markdown-Bericht, gefunden %v", md)
	}
	raw, _ := os.ReadFile(md[0])
	if !strings.Contains(string(raw), "verletzt") {
		t.Log("kein Ziel verletzt: der Test belegt dann nur Exit 0 mit Bericht")
	}
}

// Ein Fehler des Werkzeugs selbst liefert einen Fehler (Exit ≠ 0 in main).
func TestWerkzeugFehlerIstFehler(t *testing.T) {
	if err := run([]string{"--targets", "--seeds", "1", "--write-baseline"}); err == nil {
		t.Fatal("--write-baseline ohne --baseline muss scheitern")
	}
	datei := filepath.Join(t.TempDir(), "datei")
	if err := os.WriteFile(datei, nil, 0o644); err != nil {
		t.Fatal(err)
	}
	if err := run([]string{"--targets", "--seeds", "1", "--report-dir", datei}); err == nil {
		t.Fatal("Berichtsordner, der eine Datei ist, muss scheitern")
	}
}
