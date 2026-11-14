package game

import (
	"image"
	"testing"

	"k3c/native/k3c-game/view"
)

// Takte je Sekunde: erst nach einer vollen Sekunde ein Wert; ein Neustart des Takts (neues Spiel) beginnt neu.
func TestTPSMeter(t *testing.T) {
	var m tpsMeter
	if _, ok := m.update(10, 100); ok {
		t.Fatal("erster Aufruf mit Wert")
	}
	if _, ok := m.update(10.5, 115); ok {
		t.Fatal("nach einer halben Sekunde mit Wert")
	}
	if tps, ok := m.update(12, 160); !ok || tps != 30 {
		t.Fatalf("nach zwei Sekunden: %v %v", tps, ok)
	}
	if _, ok := m.update(13, 5); ok {
		t.Error("Takt zurückgesetzt, trotzdem ein Wert")
	}
}

// Ein Klick wählt Spieler und Welt-x der Zelle unter der Maus (untere Zelle: Spieler 2).
func TestPickAt(t *testing.T) {
	top, bottom := image.Rect(0, 0, 640, 180), image.Rect(0, 180, 640, 360)
	cells := []view.Cell{{Player: 0, Rect: top, Camera: view.NewCamera(top, 50, 32)},
		{Index: 1, Player: 1, Rect: bottom, Camera: view.NewCamera(bottom, 10, 32)}}
	p, ok := pickAt(cells, image.Pt(320+64, 200))
	if !ok || p.Player != 1 || p.X != 12 || !p.Picked {
		t.Errorf("unten: %+v %v", p, ok)
	}
	if _, ok := pickAt(cells, image.Pt(700, 10)); ok {
		t.Error("außerhalb gewählt")
	}
}
