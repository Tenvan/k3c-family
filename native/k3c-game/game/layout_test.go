package game

import (
	"image"
	"math"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Lage der Zellen für 1 und 2 Spieler bei 1920×1080 und 1280×720: Welt zwei Drittel, Fluss ein Drittel je Zelle (AC-07).
func TestCellLayout(t *testing.T) {
	cases := []struct {
		w, h, n    int
		rects      []image.Rectangle
		world, riv int // Höhe von Welt und Fluss je Zelle
	}{
		{1920, 1080, 1, []image.Rectangle{image.Rect(0, 0, 1920, 1080)}, 720, 360},
		{1920, 1080, 2, []image.Rectangle{image.Rect(0, 0, 1920, 540), image.Rect(0, 540, 1920, 1080)}, 360, 180},
		{1280, 720, 1, []image.Rectangle{image.Rect(0, 0, 1280, 720)}, 480, 240},
		{1280, 720, 2, []image.Rectangle{image.Rect(0, 0, 1280, 360), image.Rect(0, 360, 1280, 720)}, 240, 120},
	}
	for _, tc := range cases {
		win := image.Rect(0, 0, tc.w, tc.h)
		for i, want := range tc.rects {
			r := CellRect(win, tc.n, i)
			if r != want {
				t.Errorf("%dx%d, %d Spieler, Zelle %d: %v, erwartet %v", tc.w, tc.h, tc.n, i, r, want)
			}
			c := view.Cell{Rect: r}
			if c.WorldRect().Dy() != tc.world || c.RiverRect().Dy() != tc.riv {
				t.Errorf("%dx%d, %d Spieler, Zelle %d: Welt %d, Fluss %d", tc.w, tc.h, tc.n, i, c.WorldRect().Dy(), c.RiverRect().Dy())
			}
			if s := Scale(c); math.Abs(s-float64(tc.world)/ViewUnits) > 1e-9 {
				t.Errorf("Maßstab %v", s)
			}
		}
	}
}

// Ungerade Höhe: die Zellen schließen lückenlos an.
func TestCellLayoutOddHeight(t *testing.T) {
	win := image.Rect(10, 5, 110, 106)
	a, b := CellRect(win, 2, 0), CellRect(win, 2, 1)
	if a.Min != win.Min || b.Max != win.Max || a.Max.Y != b.Min.Y {
		t.Errorf("Zellen %v %v im Fenster %v", a, b, win)
	}
}

func cellFrame() *view.Frame {
	w := &sim.World{Castle: sim.Castle{X: 50}, Players: []*sim.Player{{Index: 0, X: 10}, {Index: 1, X: 90}}}
	return &view.Frame{
		Stages: []view.Stage{{Index: 0, World: w}},
		Seats:  []view.Seat{{Player: 0, Monarch: 0}, {Player: 1, Monarch: 1}},
	}
}

// cellsFollow baut die Zellen und setzt die Kameras wie ein Update.
func cellsFollow(cells []view.Cell, win image.Rectangle, f *view.Frame, k *Cameras) []view.Cell {
	cells = Cells(cells, win, f)
	k.Follow(cells, f)
	return cells
}

// Ohne Regel folgt die Kamera dem Monarchen, außer der Aufnahme-Modus setzt sie fest; ohne Monarchen steht sie an der Burg.
func TestCellsCamera(t *testing.T) {
	f := cellFrame()
	win := image.Rect(0, 0, 1280, 720)
	k := &Cameras{}
	cells := cellsFollow(make([]view.Cell, 0, view.MaxPlayers), win, f, k)
	if len(cells) != 2 || cells[0].Camera.X != 10 || cells[1].Camera.X != 90 || cells[1].Player != 1 {
		t.Fatalf("Zellen %+v", cells)
	}
	if cells[1].Rect != image.Rect(0, 360, 1280, 720) || math.Abs(cells[1].Camera.Scale-240/ViewUnits) > 1e-9 {
		t.Errorf("Zelle 1: %v, Maßstab %v", cells[1].Rect, cells[1].Camera.Scale)
	}
	k.Fixed = []float64{math.NaN(), 200}
	if cells = cellsFollow(cells, win, f, k); cells[0].Camera.X != 10 || cells[1].Camera.X != 200 {
		t.Errorf("feste Kamera: %v %v", cells[0].Camera.X, cells[1].Camera.X)
	}
	k.Fixed = nil
	f.Seats[0].Monarch = 7
	if cells = cellsFollow(cells, win, f, k); cells[0].Camera.X != 50 {
		t.Errorf("ohne Monarchen: %v, erwartet Burg", cells[0].Camera.X)
	}
	f.Seats = f.Seats[:1]
	if cells = cellsFollow(cells, win, f, k); len(cells) != 1 || cells[0].Rect != win {
		t.Errorf("ein Spieler: %+v", cells)
	}
	if n := testing.AllocsPerRun(10, func() { cells = cellsFollow(cells, win, f, k) }); n != 0 {
		t.Errorf("Cells und Follow legen %v Objekte an", n)
	}
}

// lagRule ist eine Regel mit Zustand: rückt je Update höchstens 5 Units an den Monarchen heran, ohne Vorgänger
// springt sie; sie zählt ihre Aufrufe.
type lagRule struct{ calls int }

func (r *lagRule) Follow(f *view.Frame, c view.Cell, prev view.Camera) view.Camera {
	r.calls++
	k := c.Camera
	m := f.Monarch(c.Player)
	switch {
	case m == nil:
		k.X = math.NaN()
	case prev.Scale == 0:
		k.X = m.X
	default:
		k.X = prev.X + math.Max(-5, math.Min(5, m.X-prev.X))
	}
	return k
}

// Die Regel bekommt je Spieler die Kamera des vorigen Updates, ein NaN-Ergebnis gilt nicht, --cam setzt danach fest;
// geht ein Spieler, vergisst er seine Kamera.
func TestCamerasRuleState(t *testing.T) {
	f := cellFrame()
	win := image.Rect(0, 0, 1280, 720)
	rule := &lagRule{}
	k := &Cameras{Rule: rule}
	cells := cellsFollow(nil, win, f, k)
	if cells[0].Camera.X != 10 || cells[1].Camera.X != 90 || rule.calls != 2 {
		t.Fatalf("erstes Update springt: %v %v", cells[0].Camera.X, cells[1].Camera.X)
	}
	f.Stages[0].World.Players[0].X = 30
	if cells = cellsFollow(cells, win, f, k); cells[0].Camera.X != 15 {
		t.Errorf("Totzone mit Zustand: %v, erwartet 15", cells[0].Camera.X)
	}
	f.Seats[0].Monarch = 7
	if cells = cellsFollow(cells, win, f, k); cells[0].Camera.X != 15 {
		t.Errorf("NaN der Regel: %v, erwartet die alte Kamera", cells[0].Camera.X)
	}
	k.Fixed = []float64{-3}
	if cells = cellsFollow(cells, win, f, k); cells[0].Camera.X != -3 || cells[1].Camera.X != 90 {
		t.Errorf("--cam: %v %v", cells[0].Camera.X, cells[1].Camera.X)
	}
	k.Fixed, f.Seats[0].Monarch = nil, 0
	f.Seats = f.Seats[1:] // Spieler 0 geht, Spieler 1 steht allein in Zelle 0
	cells = cellsFollow(cells, win, f, k)
	f.Seats = cellFrame().Seats
	if cells = cellsFollow(cells, win, f, k); cells[0].Camera.X != 30 {
		t.Errorf("zurückgekehrter Spieler springt nicht: %v", cells[0].Camera.X)
	}
}

// Place legt die Zellen bei neuer Fenstergröße neu und behält das Welt-x; ohne Änderung bleibt alles.
func TestPlace(t *testing.T) {
	f := cellFrame()
	cells := cellsFollow(nil, image.Rect(0, 0, 1280, 720), f, &Cameras{})
	before := cells[1]
	Place(cells, image.Rect(0, 0, 1280, 720))
	if cells[1] != before {
		t.Errorf("ohne Änderung verändert: %+v", cells[1])
	}
	Place(cells, image.Rect(0, 0, 1920, 1080))
	if cells[1].Rect != image.Rect(0, 540, 1920, 1080) || cells[1].Camera.X != 90 || cells[1].Camera.Center != 960 {
		t.Errorf("neue Größe: %+v", cells[1])
	}
}
