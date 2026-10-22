package main

import (
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"testing"

	"k3c/engine/level"
	"k3c/engine/room"
	"k3c/engine/store"
)

// WZ25.1 (B-508/AC-01): Nach einem geplanten Neustart sind die vorher offenen Räume mit ihrem Code wieder offen.

type quietPeer struct{}

func (quietPeer) Joined(string, string, string, []room.Seat) {}
func (quietPeer) Level(int, int, level.Layout)               {}
func (quietPeer) State(int, int, any)                        {}
func (quietPeer) Seats([]room.Seat, []string)                {}
func (quietPeer) Replaced()                                  {}
func (quietPeer) Closed(bool)                                {}

func quietLog() *slog.Logger { return slog.New(slog.NewTextHandler(io.Discard, nil)) }

func roomsIn(dir string) (*room.Manager, *store.Saves) {
	saves := &store.Saves{Dir: dir}
	m := room.NewManager(saves)
	m.Log = quietLog()
	return m, saves
}

func TestRaeumeNachNeustartWiederOffen(t *testing.T) {
	dir := t.TempDir()
	before, saves := roomsIn(dir)
	r, err := before.Create("geraet-1", quietPeer{}, "familie", true, 0, []int{0}, room.Options{})
	if err != nil {
		t.Fatal(err)
	}
	if _, err := before.Create("geraet-2", quietPeer{}, room.TestPrefix+"probe", true, 0, []int{0}, room.Options{}); err != nil {
		t.Fatal(err)
	}
	rememberRooms(before, saves, quietLog())
	before.Close() // speichert die Räume wie beim Herunterfahren

	if _, err := os.Stat(filepath.Join(dir, "restart-rooms.state")); err != nil {
		t.Fatalf("Merkdatei fehlt: %v", err)
	}
	after, saves2 := roomsIn(dir)
	reopenRooms(after, saves2, quietLog())
	got := after.Room(r.Code)
	if got == nil || got.Name != "familie" {
		t.Fatalf("Raum %s nach dem Neustart: %v", r.Code, got)
	}
	if n := len(after.Rooms()); n != 1 {
		t.Errorf("%d Räume nach dem Neustart, erwartet 1 (der Test-Raum wird nicht gemerkt)", n)
	}
	if _, err := os.Stat(filepath.Join(dir, "restart-rooms.state")); !os.IsNotExist(err) {
		t.Error("Merkdatei bleibt liegen (ein Absturz öffnete die Räume später erneut)")
	}
	// Das Gerät kehrt mit demselben Code zurück (ohne Raumliste): join auf den alten Code.
	if _, err := after.Join("geraet-1", quietPeer{}, r.Code, []int{0}); err != nil {
		t.Errorf("Wiederverbinden mit altem Code: %v", err)
	}
}

func TestGemerkterRaumOhneSpielstandWirdUebersprungen(t *testing.T) {
	dir := t.TempDir()
	m, saves := roomsIn(dir)
	if err := saves.RememberRooms([]byte(`[{"code":"ABCD","name":"gibtsnicht"},{"code":"xx","name":"familie"}]`)); err != nil {
		t.Fatal(err)
	}
	reopenRooms(m, saves, quietLog())
	if n := len(m.Rooms()); n != 0 {
		t.Errorf("%d Räume geöffnet, erwartet 0 (kein Spielstand bzw. ungültiger Code)", n)
	}
}

func TestOhneMerkdateiKeineRaeume(t *testing.T) {
	m, saves := roomsIn(t.TempDir())
	reopenRooms(m, saves, quietLog())
	if len(m.Rooms()) != 0 {
		t.Error("Räume ohne Merkdatei geöffnet")
	}
	if err := os.WriteFile(filepath.Join(saves.Dir, "restart-rooms.state"), []byte("kaputt"), 0o644); err != nil {
		t.Fatal(err)
	}
	reopenRooms(m, saves, quietLog())
	if len(m.Rooms()) != 0 {
		t.Error("Räume aus kaputter Merkdatei geöffnet")
	}
}
