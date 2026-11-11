package props

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func TestTruheOeffnetSichKurz(t *testing.T) {
	prev := []candidate{{1, 10}, {2, 20}}
	now := []candidate{{2, 20}}
	open := gone(nil, prev, now, 5)
	if len(open) != 1 || open[0] != (opening{x: 10, start: 5}) {
		t.Fatalf("aufgehobene Truhe: %v", open)
	}
	if openAlpha(open[0], 5.1) != 1 || openAlpha(open[0], 5.55) != 0.5 || openAlpha(open[0], 5.9) != 0 {
		t.Error("offen 0,3 s, dann bis 0,8 s ausgeblendet")
	}
	open = gone(open, now, now, 5.5)
	if len(open) != 1 {
		t.Error("läuft noch")
	}
	if open = gone(open, now, now, 6); len(open) != 0 {
		t.Errorf("nach 0,8 s weg: %v", open)
	}
}

func TestListenJeStufeUndPreisschild(t *testing.T) {
	w := island(t)
	d := New(nil).(*drawer)
	f := &view.Frame{Stages: []view.Stage{{Index: 0, Level: "forest", World: w}}}
	d.Listen(f)
	info := d.info(0)
	if info == nil || len(info.flags) != len(w.Sites) {
		t.Fatalf("Zustand der Stufe fehlt: %+v", info)
	}
	if d.info(1) != nil {
		t.Error("unbekannte Stufe")
	}
	site := -1
	for i, fl := range info.flags {
		if fl.payable && fl.visible {
			site = i
			break
		}
	}
	if site < 0 {
		t.Fatal("kein bezahlbarer Platz am Start")
	}
	for _, p := range w.Players {
		p.X = w.Sites[site].X
	}
	if len(w.Players) == 0 {
		t.Skip("Insel ohne Monarchen")
	}
	d.Listen(f)
	if !d.info(0).tagged(w.Sites[site].ID) {
		t.Errorf("Preisschild am Platz %d fehlt: %v", w.Sites[site].ID, d.info(0).tags)
	}
	d.Listen(&view.Frame{})
	if d.info(0) != nil {
		t.Error("Stufe ohne Spieler fällt aus dem Frame")
	}
}

// Kommt eine Stufe nach einer Lücke wieder in den Frame, öffnet sich keine Truhe, die inzwischen verschwunden ist.
func TestWiedereintrittOhneOffeneTruhe(t *testing.T) {
	w := &sim.World{Pickups: []*sim.Pickup{{ID: 1, Kind: "chest", X: 10}}}
	d := New(nil).(*drawer)
	f := &view.Frame{Clock: 1, Stages: []view.Stage{{Index: 0, Level: "forest", World: w}}}
	d.Listen(f)
	d.Listen(&view.Frame{Clock: 2})
	w.Pickups = nil
	f.Clock = 3
	d.Listen(f)
	if n := len(d.info(0).openings); n != 0 {
		t.Errorf("%d Truhen öffnen sich nach dem Wiedereintritt", n)
	}
	w.Pickups = []*sim.Pickup{{ID: 2, Kind: "chest", X: 12}}
	d.Listen(f)
	w.Pickups = nil
	d.Listen(f)
	if n := len(d.info(0).openings); n != 1 {
		t.Errorf("aufgehobene Truhe im laufenden Frame: %d offen, erwartet 1", n)
	}
}
