package art

import (
	"image"
	"math"
	"testing"

	"k3c/native/k3c-game/view"
	"k3c/native/k3c-rig/rig"
)

// RigView: Fußpunkt an Welt-x auf der Boden-Linie, Maßstab aus Kamera und Größe; gespiegelt bleibt der Rahmen an
// derselben Stelle im Fenster (wie native/k3c-ebiten/rigs.go).
func TestRigView(t *testing.T) {
	k := view.NewCamera(image.Rect(0, 0, 1000, 600), 50, 20) // Mitte bei x=500, 20 px je Unit
	frame := [4]float64{-30, -60, 80, 70}                    // Rahmen nicht mittig um den Fußpunkt
	m := RigView(k, 52, 400, 0.05, frame, false)             // 0,05 Units je Einheit: 1 px je Einheit
	if m != (M{1, 0, 0, 1, 540, 400}) {
		t.Fatalf("rechts: %v", m)
	}
	l := RigView(k, 52, 400, 0.05, frame, true)
	left := func(v M) float64 {
		return math.Min(v.Apply(rig.Point{frame[0], 0})[0], v.Apply(rig.Point{frame[0] + frame[2], 0})[0])
	}
	right := func(v M) float64 {
		return math.Max(v.Apply(rig.Point{frame[0], 0})[0], v.Apply(rig.Point{frame[0] + frame[2], 0})[0])
	}
	if l[0] != -1 || l[3] != 1 || left(l) != left(m) || right(l) != right(m) {
		t.Errorf("links: %v, Rahmen %v…%v statt %v…%v", l, left(l), right(l), left(m), right(m))
	}
	if n := testing.AllocsPerRun(10, func() { m = RigView(k, 1, 2, 3, frame, true) }); n != 0 {
		t.Errorf("RigView legt %v Objekte an", n)
	}
}

func TestActorFrame(t *testing.T) {
	rider, mount := &rig.Rig{Frame: [4]float64{1, 2, 3, 4}}, &rig.Rig{Frame: [4]float64{5, 6, 7, 8}}
	cases := []struct {
		a    Actor
		want [4]float64
	}{
		{Actor{Rider: rider}, rider.Frame},
		{Actor{Rider: rider, Mount: mount}, mount.Frame},
		{Actor{Mount: mount}, mount.Frame},
		{Actor{}, [4]float64{}},
	}
	for i, c := range cases {
		if got := ActorFrame(&c.a); got != c.want {
			t.Errorf("Fall %d: %v", i, got)
		}
	}
}
