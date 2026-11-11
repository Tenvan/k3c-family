package paint

import (
	"math"
	"testing"
)

// Die Deckkraft des Painters gilt begrenzt auf 0..1; NaN zeichnet deckend (Standard von NewPainter ist 1).
func TestPainterAlpha(t *testing.T) {
	for _, c := range []struct {
		in   float64
		want float32
	}{{1, 1}, {0.35, 0.35}, {0, 0}, {-2, 0}, {3, 1}, {math.NaN(), 1}} {
		if got := (&Painter{Alpha: c.in}).alpha(); got != c.want {
			t.Errorf("Alpha %v: %v, erwartet %v", c.in, got, c.want)
		}
	}
	if p := NewPainter(nil); p.Alpha != 1 {
		t.Errorf("NewPainter: Alpha %v, erwartet 1", p.Alpha)
	}
}
