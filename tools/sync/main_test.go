package main

import (
	"os"
	"path/filepath"
	"testing"
	"time"
)

// Ein Ordner wird ins vorhandene Ziel zusammengeführt; nur Geändertes wird kopiert, Fremdes im Ziel bleibt liegen.
func TestSyncPathCopiesOnlyChanged(t *testing.T) {
	src, dst := t.TempDir(), t.TempDir()
	write := func(p, s string) {
		t.Helper()
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(s), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write(filepath.Join(src, "a", "idle.png"), "eins")
	write(filepath.Join(src, "b.json"), "zwei")
	write(filepath.Join(dst, "alt.txt"), "bleibt")
	if n, err := syncPath(src, dst); err != nil || n != 2 {
		t.Fatalf("erster Lauf: %d, %v", n, err)
	}
	if n, _ := syncPath(src, dst); n != 0 {
		t.Errorf("zweiter Lauf kopiert %d", n)
	}
	write(filepath.Join(src, "a", "idle.png"), "neu!")
	later := time.Now().Add(time.Minute)
	_ = os.Chtimes(filepath.Join(src, "a", "idle.png"), later, later)
	if n, _ := syncPath(src, dst); n != 1 {
		t.Errorf("geänderte Grafik: %d kopiert", n)
	}
	if b, _ := os.ReadFile(filepath.Join(dst, "a", "idle.png")); string(b) != "neu!" {
		t.Errorf("Inhalt %q", b)
	}
	if _, err := os.Stat(filepath.Join(dst, "alt.txt")); err != nil {
		t.Errorf("fremde Datei weg: %v", err)
	}
}

// Eine einzelne Datei als Quelle landet unter dem Zielnamen.
func TestSyncPathSingleFile(t *testing.T) {
	dir := t.TempDir()
	from, to := filepath.Join(dir, "x.exe"), filepath.Join(dir, "out", "x.exe")
	if err := os.WriteFile(from, []byte("exe"), 0o644); err != nil {
		t.Fatal(err)
	}
	if n, err := syncPath(from, to); err != nil || n != 1 {
		t.Fatalf("%d, %v", n, err)
	}
}
