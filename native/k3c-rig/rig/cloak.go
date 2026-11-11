package rig

import "math"

// CloakDT ist der feste Zeitschritt der Umhang-Physik (tools/paper/rig/umhang.ts).
const CloakDT = 1.0 / 120

// Einschwingen vor Frame 0 und Schritte je Sekunde (tools/paper/rig/animation.ts).
const (
	cloakSettle  = 240
	stepsPerSec  = 120
	cloakBinding = 8
)

type cloakPoint struct{ x, y, px, py float64 }

// Cloak ist der Umhang als Verlet-Kette: oben am Anker, Schwerkraft und Fahrtwind, fester Zeitschritt 1/120 s.
type Cloak struct {
	pts  []cloakPoint
	seg  float64
	time float64
}

// NewCloak hängt die Kette am Anker auf, leicht nach hinten geneigt.
func NewCloak(u *CloakSpec, anchor Point) *Cloak {
	n := u.Segments
	if n == 0 {
		n = 8
	}
	c := &Cloak{pts: make([]cloakPoint, n), seg: u.Length / float64(n-1)}
	for i := range c.pts {
		x, y := anchor[0]-float64(i)*1.5, anchor[1]+float64(i)*c.seg
		c.pts[i] = cloakPoint{x, y, x, y}
	}
	return c
}

// Step rechnet einen Schritt: wind = Fahrtwind (Einheiten/s, 0 = Stand mit Böe), floor = tiefster erlaubter y-Wert.
func (c *Cloak) Step(anchor Point, wind, floor float64) {
	c.time += CloakDT
	t, dt := c.time, CloakDT
	gust := 0.0
	if wind == 0 {
		gust = 22 + 18*(0.5+0.5*math.Sin(t*1.1)) + 6*math.Sin(t*2.9)
	}
	air := -wind*0.8 - 8 - 6*math.Sin(t*1.7) - gust
	for i := 1; i < len(c.pts); i++ {
		p := &c.pts[i]
		vx, vy := (p.x-p.px)/dt, (p.y-p.py)/dt
		ax, ay := 2.4*(air-vx), 260-1.2*vy
		nx, ny := p.x+(p.x-p.px)*0.99+ax*dt*dt, p.y+(p.y-p.py)*0.99+ay*dt*dt
		p.px, p.py, p.x, p.y = p.x, p.y, nx, ny
	}
	for range cloakBinding {
		c.bind(anchor, floor)
	}
}

// bind hält die Abstände der Kette, den Anker, die Grenze nach vorn und den Boden ein.
func (c *Cloak) bind(anchor Point, floor float64) {
	c.pts[0].x, c.pts[0].y = anchor[0], anchor[1]
	for i := 1; i < len(c.pts); i++ {
		a, b := &c.pts[i-1], &c.pts[i]
		dx, dy := b.x-a.x, b.y-a.y
		d := math.Hypot(dx, dy)
		if d == 0 {
			d = 1
		}
		k := (d - c.seg) / d
		if i > 1 {
			a.x += dx * k * 0.5
			a.y += dy * k * 0.5
			b.x -= dx * k * 0.5
			b.y -= dy * k * 0.5
		} else {
			b.x -= dx * k
			b.y -= dy * k
		}
		b.x = math.Min(b.x, anchor[0]+2)
		b.y = math.Min(b.y, floor)
	}
}

// Points schreibt die Kette nach out (wiederverwendet) und liefert sie.
func (c *Cloak) Points(out []Point) []Point {
	out = out[:0]
	for _, p := range c.pts {
		out = append(out, Point{p.x, p.y})
	}
	return out
}

// FrameSteps: Physik-Schritt je Frame eines Sheets: Frame n bei Schritt round(n · 120 / (freq · frames)).
func FrameSteps(freq float64, frames int) []int {
	out := make([]int, frames)
	for n := range out {
		out[n] = int(math.Floor(float64(n)*stepsPerSec/(freq*float64(frames)) + 0.5))
	}
	return out
}

// CloakRun liefert die Umhang-Kette je Frame wie im Sheet: Physik ab Schritt −240 bis zum letzten Frame; state(k) liefert
// Anker und Wind bei Schritt k (Zeit k/120 s).
func CloakRun(u *CloakSpec, steps []int, state func(k int) (Point, float64), floor float64) [][]Point {
	anchor, _ := state(-cloakSettle)
	c := NewCloak(u, anchor)
	out := make([][]Point, 0, len(steps))
	for k, n := -cloakSettle+1, 0; n < len(steps); k++ {
		a, wind := state(k)
		c.Step(a, wind, floor)
		for n < len(steps) && steps[n] == k {
			out = append(out, c.Points(nil))
			n++
		}
	}
	return out
}

// StepTime: Zeit eines Physik-Schritts in Sekunden.
func StepTime(k int) float64 { return float64(k) / stepsPerSec }
