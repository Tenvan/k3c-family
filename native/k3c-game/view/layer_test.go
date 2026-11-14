package view

import (
	"testing"

	"github.com/hajimehoshi/ebiten/v2"
)

type fakeDrawer struct {
	layer Layer
	name  string
}

func (d fakeDrawer) Layer() Layer                           { return d.layer }
func (d fakeDrawer) Draw(_ *ebiten.Image, _ *Frame, _ Cell) {}

// Die Ebenen-Reihenfolge ist Teil des Vertrags: eine Änderung hier ist eine Vertragsänderung.
func TestLayerOrderIsFixed(t *testing.T) {
	want := []string{"far", "mid", "near", "ground", "buildings", "actors", "river", "effects", "hud", "overlay"}
	got := Layers()
	if len(got) != len(want) {
		t.Fatalf("Ebenen: %d, erwartet %d", len(got), len(want))
	}
	for i, l := range got {
		if l.String() != want[i] {
			t.Errorf("Ebene %d = %s, erwartet %s", i, l, want[i])
		}
	}
	if LayerOverlay.PerCell() || !LayerHUD.PerCell() || Layer(99).Valid() || Layer(-1).PerCell() {
		t.Error("PerCell/Valid falsch")
	}
	if Layer(99).String() != "invalid" {
		t.Error("ungültige Ebene ohne Namen")
	}
}

func TestSortedIsStableByLayer(t *testing.T) {
	in := []Drawer{
		fakeDrawer{LayerHUD, "hud"},
		fakeDrawer{LayerFar, "far1"},
		nil,
		fakeDrawer{LayerActors, "actors"},
		fakeDrawer{Layer(42), "kaputt"},
		fakeDrawer{LayerFar, "far2"},
		fakeDrawer{LayerOverlay, "overlay"},
	}
	got := Sorted(in)
	want := []string{"far1", "far2", "actors", "hud", "overlay"}
	if len(got) != len(want) {
		t.Fatalf("%d Drawer, erwartet %d", len(got), len(want))
	}
	for i, d := range got {
		if d.(fakeDrawer).name != want[i] {
			t.Errorf("Platz %d = %s, erwartet %s", i, d.(fakeDrawer).name, want[i])
		}
	}
}
