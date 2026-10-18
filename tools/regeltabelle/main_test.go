package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// WZ15.2: Der Task ändert die Regeln auf dem heutigen Stand nicht (zweiter Lauf byte-gleich), geprüft auf einer Kopie.
func TestRunAufKopieIstStabil(t *testing.T) {
	dir := t.TempDir()
	for _, rel := range []string{"data/hub.json", "docs/rules/materialien-gebaeude.md"} {
		b, err := os.ReadFile(filepath.Join("..", "..", filepath.FromSlash(rel)))
		if err != nil {
			t.Fatal(err)
		}
		if err := os.MkdirAll(filepath.Dir(filepath.Join(dir, filepath.FromSlash(rel))), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(dir, filepath.FromSlash(rel)), b, 0o644); err != nil {
			t.Fatal(err)
		}
	}
	changed, err := run(dir)
	if err != nil || changed {
		t.Fatalf("erster Lauf: geändert=%v, Fehler=%v", changed, err)
	}
}

// WZ15.2: Weicht der Block ab, schreibt der Lauf ihn atomar zurück und lässt keine temporäre Datei liegen.
func TestRunSchreibtAbweichendenBlock(t *testing.T) {
	dir := t.TempDir()
	for _, rel := range []string{"data/hub.json", "docs/rules/materialien-gebaeude.md"} {
		b, err := os.ReadFile(filepath.Join("..", "..", filepath.FromSlash(rel)))
		if err != nil {
			t.Fatal(err)
		}
		if err := os.MkdirAll(filepath.Dir(filepath.Join(dir, filepath.FromSlash(rel))), 0o755); err != nil {
			t.Fatal(err)
		}
		if rel != "data/hub.json" {
			b = []byte(strings.Replace(string(b), "Werkstatt −16", "Werkstatt −17", 1))
		}
		if err := os.WriteFile(filepath.Join(dir, filepath.FromSlash(rel)), b, 0o644); err != nil {
			t.Fatal(err)
		}
	}
	changed, err := run(dir)
	if err != nil || !changed {
		t.Fatalf("geändert=%v, Fehler=%v", changed, err)
	}
	if again, err := run(dir); err != nil || again {
		t.Fatalf("zweiter Lauf: geändert=%v, Fehler=%v", again, err)
	}
	left, _ := filepath.Glob(filepath.Join(dir, "docs", "rules", "*.tmp"))
	if len(left) != 0 {
		t.Fatalf("temporäre Dateien: %v", left)
	}
	doc, _ := os.ReadFile(filepath.Join(dir, "docs", "rules", "materialien-gebaeude.md"))
	if strings.Contains(string(doc), "−17") {
		t.Fatal("Block nicht zurückgeschrieben")
	}
}
