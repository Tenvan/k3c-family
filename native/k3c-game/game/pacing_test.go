package game

import (
	"math"
	"testing"
)

// TestPacer prüft die Zeitsammlung (RU0.2): Bei 60 Bildern je Sekunde läuft in jedem zweiten Bild ein Takt, alpha
// wechselt zwischen 0,5 und 0; ein Hänger holt höchstens pacerMaxTicks nach.
func TestPacer(t *testing.T) {
	if newPacer("") != nil || newPacer(PacingFrame) == nil || newPacer(PacingSync) == nil {
		t.Fatal("Schalter: nur frame liefert eine Zeitsammlung")
	}
	p := newPacer(PacingFrame)
	now, ticks := 10.0, 0
	if n := p.step(now); n != 0 {
		t.Fatalf("erstes Bild: %d Takte", n)
	}
	for i := range 120 {
		now += 1.0 / 60
		n := p.step(now)
		ticks += n
		want := 0.5
		if i%2 == 1 {
			want = 0
		}
		if a := p.alpha(); math.Abs(a-want) > 1e-6 && math.Abs(a-want-1) > 1e-6 {
			t.Fatalf("Bild %d: alpha %v, erwartet %v", i, a, want)
		}
	}
	if ticks < 59 || ticks > 60 {
		t.Errorf("2 Sekunden: %d Takte", ticks)
	}
	checkPacerHitch(t, p, now)
}

// checkPacerHitch prüft nach einem Hänger die Grenze der Takte und alpha bei 144 Hz.
func checkPacerHitch(t *testing.T, p *pacer, now float64) {
	t.Helper()
	now += 1.0 // Hänger von einer Sekunde
	if n := p.step(now); n != pacerMaxTicks || p.alpha() != 0 {
		t.Errorf("Hänger: %d Takte, alpha %v", n, p.alpha())
	}
	for i := range 10 { // 144 Hz: alpha bleibt unter 1
		now += 1.0 / 144
		p.step(now)
		if a := p.alpha(); a < 0 || a >= 1 {
			t.Fatalf("144 Hz, Bild %d: alpha %v", i, a)
		}
	}
}
