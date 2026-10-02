package room

import (
	"testing"
	"time"

	"k3c/engine/sim"
)

// Tests zu den Befunden aus dem Review SP07 (SP07.4).

// Review SP07, Befund 3: Eine ersetzte Verbindung ändert nichts mehr am Gerät.
func TestErsetzteVerbindungKannNichtsMehr(t *testing.T) {
	f := newFixture()
	a, b := &peer{}, &peer{}
	r := need(f.m.Create("tab", a, "alt-tab", true, 0, []int{0}, Options{}))(t)
	need(f.m.Join("tab", b, r.Code, []int{0}))(t)
	r.Leave("tab", a)
	if err := r.AddSlot("tab", a, 1); err != ErrBadRequest {
		t.Errorf("addSlot über alte Verbindung: %v", err)
	}
	if err := r.Input("tab", a, map[int]sim.PlayerCommand{0: {MoveX: 1}}); err != ErrBadRequest {
		t.Errorf("input über alte Verbindung: %v", err)
	}
	if r.monarchs[0].state != Taken || !r.Has("tab") {
		t.Fatal("alte Verbindung hat das neue Gerät aus dem Raum geworfen")
	}
}

// Review SP07, Befund 2: Ein abgestürzter Raum speichert beim Verlassen nicht mehr.
func TestGeschlossenerRaumSpeichertNicht(t *testing.T) {
	f := newFixture()
	x := &peer{}
	r := need(f.m.Create("xbox", x, "absturz", true, 0, []int{0}, Options{}))(t)
	f.m.crash(r, "Test")
	saves := f.store.saves
	r.Leave("xbox", x)
	r.Drop("xbox", x)
	if f.store.saves != saves || !r.Closed() || !x.has("closed") {
		t.Fatalf("Speicherungen nach Absturz: %d", f.store.saves-saves)
	}
}

// Review SP07, Befund 4: Nach Close nimmt der Manager keine Räume mehr an.
func TestNachCloseKeineRaeume(t *testing.T) {
	f := newFixture()
	f.m.Close()
	if _, err := f.m.Create("x", &peer{}, "spaet", true, 0, []int{0}, Options{}); err != ErrClosed {
		t.Errorf("create nach Close: %v", err)
	}
	if _, err := f.m.Join("x", &peer{}, "KRNZ", []int{0}); err != ErrClosed {
		t.Errorf("join nach Close: %v", err)
	}
}

// Ein Panic beim Aufräumen beendet nicht den Prozess.
func TestSweepUeberlebtPanic(t *testing.T) {
	f := newFixture()
	f.m.Now = func() time.Time { panic("Uhr kaputt") }
	f.m.safeSweep()
	f.m.Now = func() time.Time { return f.now }
	need(f.m.Create("x", &peer{}, "danach", true, 0, []int{0}, Options{}))(t) // Sperre ist wieder frei
}
