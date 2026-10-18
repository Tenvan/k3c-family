package assets

import (
	"os"
	"path/filepath"
	"testing"
)

// WriteAtomic (WZ9.3): Inhalt vollständig, Rechte einer vorhandenen Datei bleiben, keine Temp-Datei bleibt liegen.
func TestWriteAtomicBehaeltRechte(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "hub.json")
	if err := os.WriteFile(path, []byte("alt"), 0o640); err != nil {
		t.Fatal(err)
	}
	before, err := os.Stat(path)
	if err != nil {
		t.Fatal(err)
	}
	if err := WriteAtomic(path, []byte("neu")); err != nil {
		t.Fatal(err)
	}
	after, err := os.Stat(path)
	if err != nil {
		t.Fatal(err)
	}
	if got, _ := os.ReadFile(path); string(got) != "neu" {
		t.Errorf("Inhalt %q", got)
	}
	if after.Mode().Perm() != before.Mode().Perm() {
		t.Errorf("Rechte %v, vorher %v", after.Mode().Perm(), before.Mode().Perm())
	}
	if entries, _ := os.ReadDir(dir); len(entries) != 1 {
		t.Errorf("%d Dateien im Ordner, erwartet 1", len(entries))
	}
}

func TestWriteAtomicNeueDatei(t *testing.T) {
	path := filepath.Join(t.TempDir(), "neu.json")
	if err := WriteAtomic(path, []byte("{}")); err != nil {
		t.Fatal(err)
	}
	if got, _ := os.ReadFile(path); string(got) != "{}" {
		t.Errorf("Inhalt %q", got)
	}
}
