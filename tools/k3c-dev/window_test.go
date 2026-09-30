package main

import (
	"os"
	"path/filepath"
	"testing"
)

func TestLoadWindow(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "sub", "k3c-dev.json")
	def := windowState{Width: defaultWidth, Height: defaultHeight}
	if w, ok := loadWindow(path); ok || w != def {
		t.Errorf("fehlende Datei: %+v, %v", w, ok)
	}
	want := windowState{X: 40, Y: 60, Width: 1400, Height: 900}
	if err := saveWindow(path, want); err != nil {
		t.Fatal(err)
	}
	if w, ok := loadWindow(path); !ok || w != want {
		t.Errorf("Rundlauf: %+v, %v", w, ok)
	}
	if err := os.WriteFile(path, []byte("{kaputt"), 0o644); err != nil {
		t.Fatal(err)
	}
	if w, ok := loadWindow(path); ok || w != def {
		t.Errorf("kaputte Datei: %+v, %v", w, ok)
	}
	if err := saveWindow(path, windowState{X: -8, Y: -3000, Width: 10, Height: 20}); err != nil {
		t.Fatal(err)
	}
	if w, _ := loadWindow(path); w != (windowState{Width: minWidth, Height: minHeight}) {
		t.Errorf("Minimum: %+v", w)
	}
}
