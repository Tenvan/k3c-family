package world

import (
	"image"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Die Platzhalter-Kamera folgt dem Monarchen, ohne ihn der Burg, ohne Welt bleibt sie stehen; New liefert sie.
func TestFollow(t *testing.T) {
	w := &sim.World{Castle: sim.Castle{X: 50}, Players: []*sim.Player{{Index: 0, X: 10}}}
	f := &view.Frame{Stages: []view.Stage{{World: w}}, Seats: []view.Seat{{Player: 0, Monarch: 0}}}
	c := view.Cell{Player: 0, Rect: image.Rect(0, 0, 100, 100)}
	c.Camera = view.NewCamera(c.Rect, 3, 2)
	var rule view.CameraRule
	for _, d := range New(nil) {
		if r, ok := d.(view.CameraRule); ok {
			rule = r
		}
	}
	if rule == nil {
		t.Fatal("keine Kamera-Regel")
	}
	if k := rule.Follow(f, c, view.Camera{X: 3}); k.X != 10 || k.Scale != 2 || k.Center != 50 {
		t.Errorf("Monarch: %+v", k)
	}
	f.Seats[0].Monarch = 7
	if x := FollowX(f, c, 3); x != 50 {
		t.Errorf("ohne Monarchen %v, erwartet Burg", x)
	}
	if x := FollowX(&view.Frame{}, c, 3); x != 3 {
		t.Errorf("ohne Welt %v", x)
	}
}
