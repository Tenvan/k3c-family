package main

import (
	"os"
	"path/filepath"
	"testing"
)

func TestFindRoot(t *testing.T) {
	root := t.TempDir()
	sub := filepath.Join(root, "tools", "k3c-dev")
	if err := os.MkdirAll(sub, 0o755); err != nil {
		t.Fatal(err)
	}
	write := func(path, text string) {
		if err := os.WriteFile(path, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write(filepath.Join(root, "go.mod"), "module k3c\n\ngo 1.27.0\n")
	write(filepath.Join(sub, "go.mod"), "module k3c/tools/k3c-dev\n")
	if got, err := findRoot(sub); err != nil || got != root {
		t.Errorf("findRoot(sub) = %q, %v", got, err)
	}
	if _, err := findRoot(t.TempDir()); err == nil {
		t.Error("ohne go.mod kein Fehler")
	}
}
