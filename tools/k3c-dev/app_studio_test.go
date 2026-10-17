package main

import (
	"errors"
	"os"
	"path/filepath"
	"testing"
)

// B-459/AC-03: Fehlt bin/k3c-studio.exe, kommt genau der Hinweis; sonst wird genau dieser Pfad im Ordner der
// Repo-Wurzel gestartet (ohne echten Prozess).
func TestOpenStudio(t *testing.T) {
	root := t.TempDir()
	var got []string
	start := func(path, dir string) error {
		got = []string{path, dir}
		return nil
	}
	err := openStudio(root, start)
	if !errors.Is(err, errStudioMissing) || err.Error() != "Asset-Studio nicht gebaut (`task k3c-studio:build`)" || got != nil {
		t.Fatalf("ohne EXE: %v, gestartet %v", err, got)
	}
	exe := filepath.Join(root, "bin", "k3c-studio.exe")
	if err := os.MkdirAll(exe, 0o755); err != nil { // ein Ordner mit dem Namen zählt nicht
		t.Fatal(err)
	}
	if err := openStudio(root, start); !errors.Is(err, errStudioMissing) {
		t.Fatalf("Ordner statt EXE: %v", err)
	}
	if err := os.Remove(exe); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(exe, []byte("exe"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := openStudio(root, start); err != nil || len(got) != 2 || got[0] != exe || got[1] != root {
		t.Fatalf("mit EXE: %v, gestartet %v", err, got)
	}
	if err := openStudio(root, func(string, string) error { return errors.New("kaputt") }); err == nil || err.Error() != "kaputt" {
		t.Fatalf("Startfehler: %v", err)
	}
}
