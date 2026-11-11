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

// Die Kamera folgt dem Monarchen, außer der Aufnahme-Modus setzt sie fest; ohne Monarchen steht sie an der Burg.
func TestCellsCamera(t *testing.T) {
	f := cellFrame()
	win := image.Rect(0, 0, 1280, 720)
	dst := make([]view.Cell, 0, view.MaxPlayers)
	cells := Cells(dst, win, f, nil)
	if len(cells) != 2 || cells[0].Camera.X != 10 || cells[1].Camera.X != 90 || cells[1].Player != 1 {
		t.Fatalf("Zellen %+v", cells)
	}
	if cells[1].Rect != image.Rect(0, 360, 1280, 720) || math.Abs(cells[1].Camera.Scale-240/ViewUnits) > 1e-9 {
		t.Errorf("Zelle 1: %v, Maßstab %v", cells[1].Rect, cells[1].Camera.Scale)
	}
	cells = Cells(cells, win, f, []float64{math.NaN(), 200})
	if cells[0].Camera.X != 10 || cells[1].Camera.X != 200 {
		t.Errorf("feste Kamera: %v %v", cells[0].Camera.X, cells[1].Camera.X)
	}
	f.Seats[0].Monarch = 7
	if cells = Cells(cells, win, f, nil); cells[0].Camera.X != 50 {
		t.Errorf("ohne Monarchen: %v, erwartet Burg", cells[0].Camera.X)
	}
	f.Seats = f.Seats[:1]
	if cells = Cells(cells, win, f, nil); len(cells) != 1 || cells[0].Rect != win {
		t.Errorf("ein Spieler: %+v", cells)
	}
	if n := testing.AllocsPerRun(10, func() { cells = Cells(cells, win, f, nil) }); n != 0 {
		t.Errorf("Cells legt %v Objekte an", n)
	}
}
