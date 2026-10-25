package main

import (
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/tools/k3c-workbench/internal/balance"
)

// Verletzte Ziele sind kein Fehler (Exit 0, BAL2.3): Der Bericht entsteht, run liefert nil.
func TestVerletztesZielIstKeinFehler(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
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
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
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

// WZ28.4: --progress schreibt PROGRESS-Zeilen, --vary und die Kennzahlen stehen im Bericht; ein ungültiger Pfad ist
// ein Fehler (Exit ≠ 0).
func TestFortschrittVarianteImBericht(t *testing.T) {
	dir := t.TempDir()
	out := captureStdout(t, func() error {
		return run([]string{"--targets", "--seeds", "1", "--days", "1", "--grade", "normal", "--progress", "--vary",
			"economy.json:purse.startGold=120", "--report-dir", dir})
	})
	if !strings.Contains(out, "PROGRESS 1/1\n") {
		t.Errorf("Fortschritt fehlt: %q", out)
	}
	js, _ := filepath.Glob(filepath.Join(dir, "balance-*.json"))
	if len(js) != 1 {
		t.Fatalf("Bericht: %v", js)
	}
	raw, _ := os.ReadFile(js[0])
	s, err := balance.ReadSummary(raw)
	if err != nil || len(s.Varianten) != 1 || s.Kennzahlen == nil || len(s.Kennzahlen.Nights) != 1 {
		t.Fatalf("%v %+v", err, s)
	}
	for _, args := range [][]string{
		{"--targets", "--seeds", "1", "--vary", "economy.json:purse.gibtsnicht=1", "--report-dir", dir},
		{"--vary", "economy.json:purse.startGold=1"},
		{"--targets", "--players", "1,2", "--report-dir", dir},
	} {
		if err := run(args); err == nil {
			t.Errorf("%v: Fehler erwartet", args)
		}
	}
}

func captureStdout(t *testing.T, fn func() error) string {
	t.Helper()
	r, w, err := os.Pipe()
	if err != nil {
		t.Fatal(err)
	}
	old := os.Stdout
	os.Stdout = w
	runErr := fn()
	os.Stdout = old
	_ = w.Close()
	b, _ := io.ReadAll(r)
	if runErr != nil {
		t.Fatal(runErr)
	}
	return string(b)
}
