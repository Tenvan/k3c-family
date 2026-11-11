package actors

import (
	"image"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func TestBoxesOrderAndCulling(t *testing.T) {
	w := &sim.World{
		Players: []*sim.Player{{X: 10}, {X: 12, RespawnIn: 3}},
		Troops:  []*sim.Troop{{X: 11}, {X: 500}},
		Enemies: []*sim.Enemy{{X: 9}},
	}
	k := view.NewCamera(image.Rect(0, 0, 640, 360), 10, 32) // sichtbar 0 … 20
	got := Boxes(nil, w, k)
	want := []uint8{troopColor.R, enemyColor.R, monarchColor.R}
	if len(got) != len(want) {
		t.Fatalf("%d Figuren, erwartet %d: %+v", len(got), len(want), got)
	}
	for i, b := range got {
		if b.Color.R != want[i] {
			t.Errorf("Platz %d: Farbe %v", i, b.Color)
		}
	}
	if Boxes(nil, nil, k) != nil {
		t.Error("ohne Welt Figuren")
	}
}
