package devtools

import (
	"image"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/level"
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// geoWorld ist eine Welt von 100 Units mit Chunks zu 20 Units, Monarch bei 50, Gegner bei 52, Bauplatz bei 46 und
// Truppe weit außerhalb der Kamera.
func geoWorld() *sim.World {
	return &sim.World{WidthUnits: 100,
		Level:   level.Layout{Chunks: []level.Chunk{{StartUnits: 0}, {StartUnits: 20}, {StartUnits: 40}, {StartUnits: 60}, {StartUnits: 80}}},
		Players: []*sim.Player{{Index: 0, X: 50}}, Enemies: []*sim.Enemy{{ID: 2, X: 52}},
		Sites: []*sim.Site{{ID: 3, X: 46}}, Troops: []*sim.Troop{{ID: 4, X: 95}}}
}

// geoCell ist eine Zelle 640 × 360 mit Kamera bei x 50 und 32 px je Unit (sichtbar 40–60).
func geoCell() view.Cell {
	r := image.Rect(0, 0, 640, 360)
	return view.Cell{Index: 0, Player: 0, Rect: r, Camera: view.NewCamera(r, 50, 32)}
}

func countKind(ss []shape, kind int) int {
	n := 0
	for _, s := range ss {
		if s.kind == kind {
			n++
		}
	}
	return n
}

// AC-04 (B-888): Rahmen sitzen um die Welt-x der Figuren (Mitte der Zelle = Kamera), Bauplätze zeigen die
// Zahl-Reichweite, unsichtbare Figuren fehlen; Chunk-Grenzen nur im sichtbaren Bereich, dazu die Kamera-Mitte.
func TestGeometry(t *testing.T) {
	c := geoCell()
	ss := geometry(nil, geoWorld(), c)
	for _, s := range ss {
		switch s.kind {
		case geoMonarch:
			if mid := (s.x0 + s.x1) / 2; mid != 320 || s.x1-s.x0 != 2*geoHalf[geoMonarch]*32 || s.y1 != float64(c.GroundY()) {
				t.Errorf("Monarch: %+v", s)
			}
		case geoEnemy:
			if mid := (s.x0 + s.x1) / 2; mid != 320+2*32 {
				t.Errorf("Gegner: %+v", s)
			}
		case geoSite:
			if s.x1-s.x0 != 2*payRange*32 {
				t.Errorf("Bauplatz: %+v", s)
			}
		}
	}
	if countKind(ss, geoTroop) != 0 || countKind(ss, geoChunk) != 2 || countKind(ss, geoCamera) != 1 || countKind(ss, geoEdge) != 0 {
		t.Errorf("Truppe %d, Chunks %d, Kamera %d, Kanten %d", countKind(ss, geoTroop), countKind(ss, geoChunk),
			countKind(ss, geoCamera), countKind(ss, geoEdge))
	}
	c.Camera = view.NewCamera(c.Rect, 5, 32)
	if ss = geometry(nil, geoWorld(), c); countKind(ss, geoEdge) != 1 {
		t.Errorf("linke Weltkante: %d", countKind(ss, geoEdge))
	}
}

// Das Overlay zeichnet ohne Schrift und ohne Welt ohne Absturz.
func TestDebugDraw(t *testing.T) {
	o := New(nil)[0].(*overlay)
	dst := ebiten.NewImage(640, 360)
	c := geoCell()
	f := &view.Frame{Cells: []view.Cell{c}, Dev: view.DevView{Debug: view.DebugGeometry}}
	o.Draw(dst, f, view.WindowCell(dst.Bounds()))
	f.Seats = []view.Seat{{Player: 0}}
	f.Stages = []view.Stage{{Index: 0, World: geoWorld()}}
	o.Draw(dst, f, view.WindowCell(dst.Bounds()))
	if len(o.shapes) == 0 {
		t.Error("keine Formen")
	}
}
