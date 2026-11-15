package game

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func blendFrame(x, cam float64) (*view.Frame, []view.Cell) {
	w := &sim.World{Players: []*sim.Player{{ID: 1, X: x}}, Enemies: []*sim.Enemy{{ID: 7, X: x * 2}}}
	cells := []view.Cell{{Index: 0, Camera: view.Camera{X: cam}}}
	return &view.Frame{Clock: x, Stages: []view.Stage{{Index: 0, World: w}}, Cells: cells}, cells
}

func TestBlendGlidesAndRestores(t *testing.T) {
	b := newBlend(true)
	f, cells := blendFrame(10, 5)
	b.capture(f, cells)
	f, cells = blendFrame(10.2, 5.1)
	f.Clock = 10 + 1.0/30
	b.clock = 10
	b.apply(f, cells, 0.5)
	w := f.Stages[0].World
	if got := w.Players[0].X; got < 10.09 || got > 10.11 {
		t.Fatalf("Spieler x = %v, want 10.1", got)
	}
	if got := cells[0].Camera.X; got < 5.04 || got > 5.06 {
		t.Fatalf("Kamera x = %v, want 5.05", got)
	}
	b.restore(f, cells)
	if w.Players[0].X != 10.2 || cells[0].Camera.X != 5.1 || f.Clock != 10+1.0/30 {
		t.Fatalf("restore: x=%v cam=%v clock=%v", w.Players[0].X, cells[0].Camera.X, f.Clock)
	}
}

func TestBlendJumpAndNewUnits(t *testing.T) {
	b := newBlend(true)
	f, cells := blendFrame(10, 5)
	b.capture(f, cells)
	f, cells = blendFrame(40, 5) // weiter als jumpUnits: springt; Gegner 7 ist neu im Schlüssel? nein, derselbe, springt auch
	f.Stages[0].World.Enemies[0].ID = 8
	b.apply(f, cells, 0.5)
	if f.Stages[0].World.Players[0].X != 40 || f.Stages[0].World.Enemies[0].X != 80 {
		t.Fatal("Sprung und neue Einheit dürfen nicht gleiten")
	}
	b.restore(f, cells)
}

func TestBlendOffAndFullAlpha(t *testing.T) {
	for _, b := range []*blend{newBlend(false), newBlend(true)} {
		f, cells := blendFrame(10, 5)
		b.capture(f, cells)
		f, cells = blendFrame(10.2, 5.1)
		b.apply(f, cells, 1)
		if f.Stages[0].World.Players[0].X != 10.2 {
			t.Fatal("alpha 1 oder aus: keine Verschiebung")
		}
		b.restore(f, cells)
	}
	if a := newBlend(false).alpha(); a != 1 {
		t.Fatalf("alpha aus = %v", a)
	}
}

func TestBlendNoAlloc(t *testing.T) {
	b := newBlend(true)
	f, cells := blendFrame(10, 5)
	b.capture(f, cells)
	f, cells = blendFrame(10.2, 5.1)
	b.apply(f, cells, 0.5)
	b.restore(f, cells)
	if n := testing.AllocsPerRun(50, func() { b.apply(f, cells, 0.5); b.restore(f, cells) }); n != 0 {
		t.Fatalf("Allokationen je Bild: %v", n)
	}
}
