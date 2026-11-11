package world

import (
	"math"
	"testing"
)

// AC-02: Ufer und Fluss füllen genau das untere Drittel der Zelle; sechs Reihen, nach vorn höher und breiter.
func TestRiverFillsLowerThird(t *testing.T) {
	c := layerCell(40)
	rows := riverLayout(c, 0)
	third := c.RiverRect()
	if third.Min.Y != c.GroundY() || third.Max.Y != c.Rect.Max.Y {
		t.Fatalf("Fluss-Drittel %v, Zelle %v", third, c.Rect)
	}
	if got := rows[0].y - bankHeight(c); math.Abs(got-float64(third.Min.Y)) > 1e-9 {
		t.Errorf("erste Reihe beginnt bei %v unter dem Ufer, Drittel bei %d", got, third.Min.Y)
	}
	last := rows[riverRows-1]
	if math.Abs(last.y+last.h-float64(third.Max.Y)) > 1e-9 {
		t.Errorf("letzte Reihe endet bei %v, Zelle bei %d", last.y+last.h, third.Max.Y)
	}
	for i := 1; i < riverRows; i++ {
		if rows[i].h <= rows[i-1].h || rows[i].tile <= rows[i-1].tile || math.Abs(rows[i].y-(rows[i-1].y+rows[i-1].h)) > 1e-9 {
			t.Errorf("Reihe %d: %+v nach %+v", i, rows[i], rows[i-1])
		}
	}
}

// Die Kacheln decken die Zelle ab dem linken Rand: Die erste liegt höchstens eine Kachel links davon.
func TestRiverCoversLeftEdge(t *testing.T) {
	c := layerCell(17.25)
	for _, tm := range []float64{0, 3.7, 1000} {
		for i, r := range riverLayout(c, tm) {
			if r.x0 > float64(c.Rect.Min.X) || r.x0 <= float64(c.Rect.Min.X)-r.tile {
				t.Errorf("t=%v Reihe %d: x0 %v, Kachel %v", tm, i, r.x0, r.tile)
			}
		}
	}
}

// Der Fluss fließt nach links, vordere Reihen schneller; eine Kamerafahrt nach rechts schiebt ihn zusätzlich nach links.
func TestRiverFlowsLeft(t *testing.T) {
	const dt = 0.1
	c := layerCell(40)
	a, b := riverLayout(c, 5), riverLayout(c, 5+dt)
	for i := range riverRows {
		g := 1 + float64(i)*riverGrow
		want := -riverFlowUnits * g * c.Camera.Scale * dt
		if d := shift(a[i], b[i]); math.Abs(d-want) > 1e-6 {
			t.Errorf("Reihe %d: Weg %v, erwartet %v", i, d, want)
		}
	}
	moved := layerCell(40.5)
	m := riverLayout(moved, 5)
	if d := shift(a[0], m[0]); math.Abs(d+0.5*c.Camera.Scale) > 1e-6 {
		t.Errorf("Kamerafahrt: Ufer-Reihe verschiebt sich um %v, erwartet wie die Welt", d)
	}
}

// shift ist die Verschiebung von a nach b, auf eine halbe Kachel um 0 gefaltet.
func shift(a, b riverRow) float64 {
	d := math.Mod(b.x0-a.x0, a.tile)
	if d > a.tile/2 {
		d -= a.tile
	}
	if d < -a.tile/2 {
		d += a.tile
	}
	return d
}

// Rechnen ohne Allokation (läuft je Bild und Zelle).
func TestRiverNoAlloc(t *testing.T) {
	c := layerCell(40)
	if n := testing.AllocsPerRun(100, func() { _ = riverLayout(c, 3) }); n != 0 {
		t.Errorf("riverLayout legt %v an", n)
	}
}
