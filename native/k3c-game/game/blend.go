package game

// Zwischenbilder (B-943): Die Simulation taktet mit 30 Hz, gezeichnet wird mit der Bildrate des Bildschirms (60 Hz und
// mehr). Ohne Zwischenbilder zeigt fast jedes zweite Bild denselben Takt (gemessen: 47 % der Bilder ohne neuen Takt),
// die Bewegung ruckelt, obwohl die FPS stabil sind. blend hält die Lage der Einheiten und der Kamera vom Takt davor und
// schiebt sie in Draw für die Dauer des Bilds (apply) um den Anteil alpha zwischen den Takten auf die aktuelle Lage;
// restore stellt die Simulation danach wieder her. Die Darstellung läuft eine Takt-Länge (33 ms) hinter der Simulation.
// Die Simulation selbst bleibt unberührt (Update und Draw laufen nacheinander im selben Thread, local hat keine Goroutinen).

import (
	"math"
	"time"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// jumpUnits ist der größte Weg in Units je Takt, der noch gleitet; weiter weg (Stufenwechsel, Zeitraffer) springt das Bild.
const jumpUnits = 6.0

// Arten der Einheiten im Schlüssel von blend.prev.
const (
	kindPlayer uint64 = iota + 1
	kindTroop
	kindEnemy
	kindProjectile
	kindCoin
)

// restoreX merkt den Zeiger auf eine verschobene x-Koordinate und ihren Wert der Simulation.
type restoreX struct {
	p *float64
	v float64
}

// blend ist der Zustand der Zwischenbilder; die Puffer werden wiederverwendet (keine Allokation je Bild).
type blend struct {
	on        bool
	prev      map[uint64]float64 // Stufe, Art, ID → x beim Takt davor
	cam       [view.MaxPlayers]float64
	camKnown  [view.MaxPlayers]bool
	clock     float64
	at        time.Time // Beginn des letzten Updates
	saved     []restoreX
	savedCam  [view.MaxPlayers]float64
	savedClk  float64
	camActive bool
}

func newBlend(on bool) *blend { return &blend{on: on, prev: make(map[uint64]float64, 256)} }

func key(stage int, kind uint64, id int) uint64 { return uint64(stage)<<48 | kind<<40 | uint64(uint32(id)) }

// capture merkt vor dem nächsten Takt die Lage des Frames f (der Stand vor dem Takt) und der Zellen cells.
func (b *blend) capture(f *view.Frame, cells []view.Cell) {
	if b == nil || !b.on {
		return
	}
	clear(b.prev)
	for _, st := range f.Stages {
		w := st.World
		if w == nil {
			continue
		}
		for _, p := range w.Players {
			b.prev[key(st.Index, kindPlayer, p.ID)] = p.X
		}
		for _, t := range w.Troops {
			b.prev[key(st.Index, kindTroop, t.ID)] = t.X
		}
		for _, e := range w.Enemies {
			b.prev[key(st.Index, kindEnemy, e.ID)] = e.X
		}
		for _, p := range w.Projectiles {
			b.prev[key(st.Index, kindProjectile, p.ID)] = p.X
		}
		for _, c := range w.Coins {
			b.prev[key(st.Index, kindCoin, c.ID)] = c.X
		}
	}
	b.camKnown = [view.MaxPlayers]bool{}
	for _, c := range cells {
		if c.Index >= 0 && c.Index < len(b.cam) {
			b.cam[c.Index], b.camKnown[c.Index] = c.Camera.X, true
		}
	}
	b.clock = f.Clock
}

// stamp hält den Zeitpunkt des Updates fest (Bezug für alpha).
func (b *blend) stamp() {
	if b != nil && b.on {
		b.at = time.Now()
	}
}

// alpha ist der Anteil der Takt-Dauer, der seit dem letzten Update vergangen ist, 0 bis 1.
func (b *blend) alpha() float64 {
	if b == nil || !b.on || b.at.IsZero() {
		return 1
	}
	return math.Min(float64(time.Since(b.at))*TPS/float64(time.Second), 1)
}

// glide liefert den Wert zwischen prev und cur beim Anteil a; springt cur weit weg, bleibt es bei cur.
func glide(prev, cur, a float64) float64 {
	if d := cur - prev; d == 0 || math.Abs(d) > jumpUnits {
		return cur
	}
	return prev + (cur-prev)*a
}

// apply schiebt Einheiten, Kameras und Uhr von f bzw. cells auf den Anteil a; restore macht es rückgängig.
func (b *blend) apply(f *view.Frame, cells []view.Cell, a float64) {
	if b == nil || !b.on || a >= 1 {
		return
	}
	b.saved, b.camActive = b.saved[:0], false
	for _, st := range f.Stages {
		if st.World != nil {
			b.shiftWorld(st.World, st.Index, a)
		}
	}
	for i := range cells {
		c := &cells[i]
		if c.Index >= 0 && c.Index < len(b.cam) && b.camKnown[c.Index] {
			b.savedCam[c.Index] = c.Camera.X
			c.Camera.X = glide(b.cam[c.Index], c.Camera.X, a)
		}
	}
	b.camActive = true
	if d := f.Clock - b.clock; d > 0 && d < 0.5 {
		b.savedClk, f.Clock = f.Clock, b.clock+d*a
	} else {
		b.savedClk = f.Clock
	}
}

// shift schiebt *x einer Einheit auf den Anteil a und merkt den Wert der Simulation.
func (b *blend) shift(x *float64, stage int, kind uint64, id int, a float64) {
	prev, ok := b.prev[key(stage, kind, id)]
	if !ok {
		return
	}
	if g := glide(prev, *x, a); g != *x {
		b.saved = append(b.saved, restoreX{x, *x})
		*x = g
	}
}

// restore stellt Einheiten, Kameras und Uhr nach dem Zeichnen wieder her.
func (b *blend) restore(f *view.Frame, cells []view.Cell) {
	if b == nil {
		return
	}
	for _, s := range b.saved {
		*s.p = s.v
	}
	b.saved = b.saved[:0]
	if !b.camActive {
		return
	}
	for i := range cells {
		if c := &cells[i]; c.Index >= 0 && c.Index < len(b.cam) && b.camKnown[c.Index] {
			c.Camera.X = b.savedCam[c.Index]
		}
	}
	f.Clock, b.camActive = b.savedClk, false
}

// shiftWorld schiebt die Einheiten der Welt w der Stufe stage auf den Anteil a.
func (b *blend) shiftWorld(w *sim.World, stage int, a float64) {
	for _, p := range w.Players {
		b.shift(&p.X, stage, kindPlayer, p.ID, a)
	}
	for _, t := range w.Troops {
		b.shift(&t.X, stage, kindTroop, t.ID, a)
	}
	for _, e := range w.Enemies {
		b.shift(&e.X, stage, kindEnemy, e.ID, a)
	}
	for _, p := range w.Projectiles {
		b.shift(&p.X, stage, kindProjectile, p.ID, a)
	}
	for _, c := range w.Coins {
		b.shift(&c.X, stage, kindCoin, c.ID, a)
	}
}
