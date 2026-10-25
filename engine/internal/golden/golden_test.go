package golden

import (
	"bytes"
	"os"
	"path/filepath"
	"testing"
)

// TestGzRundreise (GD1): Eine .json.gz-Datei liest sich wie JSON zurück, zweimal geschrieben sind die Bytes gleich
// (fester gzip-Kopf ohne Zeitstempel, `task go:golden` ohne Änderung erzeugt keinen Diff).
func TestGzRundreise(t *testing.T) {
	dir := t.TempDir()
	src := filepath.Join(dir, "lauf.json")
	if err := os.WriteFile(src, []byte("{\n  \"name\": \"x\",\n  \"ticks\": 3\n}\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	f := ReadFile(t, src)
	f.Set(t, "ticks", 4)
	gz := filepath.Join(dir, "lauf.json.gz")
	f.Write(t, gz)
	first, err := os.ReadFile(gz)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.HasPrefix(first, []byte{0x1f, 0x8b}) || !bytes.Equal(first[4:8], []byte{0, 0, 0, 0}) {
		t.Fatalf("kein gzip mit Zeitstempel 0: % x", first[:10])
	}
	ReadFile(t, gz).Write(t, gz)
	second, err := os.ReadFile(gz)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(first, second) {
		t.Fatal("zweites Schreiben ändert die Bytes")
	}
	if got, want := string(Read(t, gz)), "{\n  \"name\": \"x\",\n  \"ticks\": 4\n}\n"; got != want {
		t.Fatalf("Inhalt %q, erwartet %q", got, want)
	}
}
