package world

import (
	"image"
	"math"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// testWorld ist eine Stufe der Breite 100 Units mit Burg bei 50 und Monarchen 0 bei x.
func testWorld(x float64) *sim.World {
	w := &sim.World{WidthUnits: 100, Castle: sim.Castle{X: 50}, Players: []*sim.Player{{Index: 0, X: x}}, Seed: "a"}
	w.Biome.ID = "forest"
	return w
}

// testFrame stellt Spieler 0 in die Welt w (Stufe 0).
func testFrame(w *sim.World) *view.Frame {
	return &view.Frame{
		Stages: []view.Stage{{Index: 0, Level: "forest", World: w}},
		Seats:  []view.Seat{{Player: 0, Monarch: 0, Stage: 0}},
	}
}

// testCell ist eine Zelle 320 × 300 px mit 10 px je Unit: sichtbar sind 32 Units (halb 16).
func testCell(player int, x float64) view.Cell {
	c := view.Cell{Index: player, Player: player, Rect: image.Rect(0, 0, 320, 300)}
	c.Camera = view.NewCamera(c.Rect, x, 10)
	return c
}

func near(a, b float64) bool { return math.Abs(a-b) < 1e-9 }

// Erstes Bild (prev ohne Maßstab): Die Kamera springt auf den Monarchen und behält Lage und Maßstab der Zelle.
func TestFollowJumpsFirst(t *testing.T) {
	var s cameras
	c := testCell(0, 0)
	k := s.follow(testFrame(testWorld(40)), c, view.Camera{})
	if k.X != 40 || k.Scale != 10 || k.Center != 160 {
		t.Errorf("erstes Bild: %+v", k)
	}
}

// Totzone: Ein kleiner Schritt bewegt die Kamera nicht, ein großer holt followRate des Abstands zum Rand auf.
func TestFollowDeadZone(t *testing.T) {
	var s cameras
	f := testFrame(testWorld(40))
	c := testCell(0, 40)
	prev := s.follow(f, c, view.Camera{})
	f.Stages[0].World.Players[0].X = 40 + deadZone/2
	if k := s.follow(f, c, prev); k.X != 40 {
		t.Errorf("in der Totzone bewegt: %v", k.X)
	}
	f.Stages[0].World.Players[0].X = 50
	want := 40 + (10-deadZone)*followRate
	if k := s.follow(f, c, prev); !near(k.X, want) {
		t.Errorf("außerhalb der Totzone %v, erwartet %v", k.X, want)
	}
}

// Die Kamera holt den stehenden Monarchen bis auf die Totzone ein und schwingt nicht über.
func TestFollowConverges(t *testing.T) {
	var s cameras
	f := testFrame(testWorld(40))
	c := testCell(0, 40)
	k := s.follow(f, c, view.Camera{})
	f.Stages[0].World.Players[0].X = 60
	for range 200 {
		k = s.follow(f, c, k)
		if k.X > 60 {
			t.Fatalf("übergeschwungen: %v", k.X)
		}
	}
	if math.Abs(60-k.X) > deadZone+1e-6 {
		t.Errorf("nicht eingeholt: %v", k.X)
	}
}

// Grenzen der Stufe: Am Rand bleibt die Kamera so stehen, dass die Zelle nicht über die Stufe hinausschaut.
func TestFollowBounds(t *testing.T) {
	cases := []struct {
		name    string
		monarch float64
		want    float64
	}{
		{"links", 2, 16},
		{"rechts", 99, 84},
		{"Mitte", 50, 50},
	}
	for _, tc := range cases {
		var s cameras
		k := s.follow(testFrame(testWorld(tc.monarch)), testCell(0, 0), view.Camera{})
		if k.X != tc.want {
			t.Errorf("%s: %v, erwartet %v", tc.name, k.X, tc.want)
		}
	}
	if x := clampX(3, 16, 20); x != 10 {
		t.Errorf("schmale Stufe: %v, erwartet Mitte 10", x)
	}
}

// Ohne Monarchen folgt die Kamera der Burg; ohne Welt bleibt sie stehen (cameraFollow.ts: Ziel verschwindet).
func TestFollowFallbacks(t *testing.T) {
	var s cameras
	f := testFrame(testWorld(10))
	f.Seats[0].Monarch = 7
	if k := s.follow(f, testCell(0, 0), view.Camera{}); k.X != 50 {
		t.Errorf("ohne Monarchen %v, erwartet Burg 50", k.X)
	}
	prev := view.Camera{X: 33, Scale: 10}
	if k := s.follow(&view.Frame{}, testCell(0, 0), prev); k.X != 33 {
		t.Errorf("ohne Welt %v, erwartet 33", k.X)
	}
}

// Zwei Zellen folgen ihren eigenen Monarchen unabhängig, auch auf verschiedenen Stufen.
func TestFollowTwoCells(t *testing.T) {
	var s cameras
	a, b := testWorld(30), testWorld(70)
	b.Players[0].Index = 1
	f := &view.Frame{
		Stages: []view.Stage{{Index: 0, Level: "forest", World: a}, {Index: 1, Depth: 1, Level: "cave", World: b}},
		Seats:  []view.Seat{{Player: 0, Monarch: 0, Stage: 0}, {Player: 1, Monarch: 1, Stage: 1}},
	}
	k0 := s.follow(f, testCell(0, 0), view.Camera{})
	k1 := s.follow(f, testCell(1, 0), view.Camera{})
	if k0.X != 30 || k1.X != 70 {
		t.Errorf("Zellen %v / %v, erwartet 30 / 70", k0.X, k1.X)
	}
}

// Stufenwechsel: Die Kamera springt ans neue Ziel statt hinzugleiten, der Übergang beginnt zur Zeit des Wechsels.
func TestFollowStageSwitch(t *testing.T) {
	var s cameras
	f := testFrame(testWorld(20))
	c := testCell(0, 0)
	k := s.follow(f, c, view.Camera{})
	f.Clock = 10
	if a := s.fade(0, f.Clock); a != 0 {
		t.Errorf("Übergang ohne Wechsel: %v", a)
	}
	other := testWorld(80)
	other.Seed = "b"
	f.Stages = []view.Stage{{Index: 1, Depth: 1, Level: "cave", World: other}}
	f.Seats[0].Stage = 1
	if k = s.follow(f, c, k); k.X != 80 {
		t.Errorf("nach dem Wechsel %v, erwartet Sprung auf 80", k.X)
	}
	if a := s.fade(0, 10); a != 1 {
		t.Errorf("Übergang am Anfang %v, erwartet 1", a)
	}
	if a := s.fade(0, 10+fadeSeconds/2); !near(a, 0.5) {
		t.Errorf("Übergang in der Mitte %v", a)
	}
	if a := s.fade(0, 10+fadeSeconds+0.01); a != 0 {
		t.Errorf("Übergang am Ende %v", a)
	}
}

// New meldet genau eine Kamera-Regel und eine Asset-Quelle (game nimmt höchstens eine Regel an).
func TestNewRoles(t *testing.T) {
	rules, sources := 0, 0
	for _, d := range New(nil) {
		if _, ok := d.(view.CameraRule); ok {
			rules++
		}
		if _, ok := d.(view.AssetSource); ok {
			sources++
		}
		if !d.Layer().PerCell() {
			t.Errorf("Ebene %v nicht je Zelle", d.Layer())
		}
	}
	if rules != 1 || sources != 1 {
		t.Errorf("Regeln %d, Quellen %d, erwartet je 1", rules, sources)
	}
}
