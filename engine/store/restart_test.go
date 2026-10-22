package store

import (
	"errors"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// WZ25.1 (B-508): Die Merkdatei ist atomar, wird beim Lesen gelöscht, steht nicht in der Spielstandliste und gilt nur kurz.
func TestGemerkteRaeume(t *testing.T) {
	s := &Saves{Dir: t.TempDir()}
	if _, err := s.TakeRememberedRooms(); !errors.Is(err, ErrNotFound) {
		t.Fatalf("ohne Datei: %v", err)
	}
	if err := s.RememberRooms([]byte(`[{"code":"ABCD","name":"familie"}]`)); err != nil {
		t.Fatal(err)
	}
	if n := len(s.List()); n != 0 {
		t.Errorf("Merkdatei steht in der Spielstandliste (%d Einträge)", n)
	}
	data, err := s.TakeRememberedRooms()
	if err != nil || string(data) != `[{"code":"ABCD","name":"familie"}]` {
		t.Fatalf("lesen: %q, %v", data, err)
	}
	if _, err := os.Stat(filepath.Join(s.Dir, "restart-rooms.state")); !os.IsNotExist(err) {
		t.Error("Datei bleibt nach dem Lesen liegen")
	}
}

func TestAlteMerkdateiWirdVerworfen(t *testing.T) {
	s := &Saves{Dir: t.TempDir()}
	if err := s.RememberRooms([]byte(`[]`)); err != nil {
		t.Fatal(err)
	}
	old := time.Now().Add(-2 * RememberedRoomsMaxAge)
	if err := os.Chtimes(filepath.Join(s.Dir, "restart-rooms.state"), old, old); err != nil {
		t.Fatal(err)
	}
	if _, err := s.TakeRememberedRooms(); !errors.Is(err, ErrNotFound) {
		t.Errorf("alte Merkdatei: %v, erwartet ErrNotFound", err)
	}
	if _, err := os.Stat(filepath.Join(s.Dir, "restart-rooms.state")); !os.IsNotExist(err) {
		t.Error("alte Datei bleibt liegen")
	}
}
