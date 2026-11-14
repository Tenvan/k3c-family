package props

// Darstellung der Natur-Eigenschaften, Teil 2 (B-904 bis B-906): Stützbalken und Verfall der Mine, Glut des
// Eisenstollens, Schein der Kristall-Adern. Regeln wie decor.go (Wert je Stufe, Orte aus der Position).

import (
	"image/color"
	"math"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Maße der Darstellung in Units.
const (
	beamStep    = 8.0  // Abstand der Stützbalken-Rahmen
	beamHub     = 10.0 // um die Burg kein Rahmen
	beamHeight  = 4.0  // Höhe des Querbalkens
	beamHalf    = 1.3  // halber Abstand der Pfosten
	beamWidth   = 0.25 // Breite der Balken
	emberStep   = 1.5  // Raster der Glutpunkte
	hazeHeight  = 1.2  // Höhe des Hitze-Schleiers
	veinGlowMid = 1.0  // Mitte des Scheins über dem Boden (halbe Höhe der Ader)
)

var (
	beamWood    = color.RGBA{107, 74, 47, 255}
	rubbleStone = color.RGBA{96, 80, 66, 255}
	emberColor  = color.RGBA{255, 122, 40, 255}
	hazeColor   = color.RGBA{200, 52, 20, 255}
	// veinGlow: Farbe des Scheins je Kristallfarbe (Basis-Ton der Materialien kristall, purpur, smaragd).
	veinGlow = map[string]color.RGBA{
		"blau": {95, 205, 228, 255}, "violett": {168, 101, 184, 255}, "gruen": {95, 209, 138, 255},
	}
)

// decorGlow plant Balken, Glut und Schein (Aufruf aus decor).
func (p *planner) decorGlow(w *sim.World, k view.Camera, clock float64) {
	p.mineBeams(w, k)
	p.embers(w, k, clock)
	p.crystalGlow(w, k, clock)
}

// mineBeams: alle beamStep Units ein Stützbalken-Rahmen; verfallen, wenn mix % 4 < Wert (B-904).
func (p *planner) mineBeams(w *sim.World, k view.Camera) {
	v, ok := trait(w, "decay")
	if !ok {
		return
	}
	from, to, ok := span(w, k, beamStep, beamHalf+2)
	if !ok {
		return
	}
	for x := from; x < to; x += beamStep {
		if math.Abs(x-w.HubX) < beamHub {
			continue
		}
		if cellHash(x, beamStep, 307)%4 < uint32(v) {
			p.brokenBeam(x)
		} else {
			p.beam(x-beamHalf, 0, x-beamHalf, beamHeight, beamWood)
			p.beam(x+beamHalf, 0, x+beamHalf, beamHeight, beamWood)
			p.beam(x-beamHalf-0.3, beamHeight, x+beamHalf+0.3, beamHeight, beamWood)
		}
	}
}

// brokenBeam: ein Pfosten schief, der Querbalken hängt schräg herab, darunter drei Brocken Geröll.
func (p *planner) brokenBeam(x float64) {
	p.beam(x-beamHalf, 0, x-beamHalf+0.7, beamHeight-0.4, beamWood)
	p.beam(x+beamHalf, 0, x+beamHalf, beamHeight, beamWood)
	p.beam(x+beamHalf+0.3, beamHeight, x-beamHalf+0.2, 1.4, beamWood)
	for i, dx := range [3]float64{-0.6, -0.1, 0.35} {
		s := 0.22 + 0.08*float64(i%2)
		p.rect(&p.body, x+dx, 0, s, s*0.8, rubbleStone)
	}
}

// beam legt eine Balken-Linie an.
func (p *planner) beam(x, y, x2, y2 float64, clr color.RGBA) {
	p.body = append(p.body, item{shape: shapeLine, x: x, y: y, x2: x2, y2: y2, size: beamWidth, clr: clr})
}

// embers: Hitze-Schleier am Boden und Glutpunkte (mix % 8 < 2 · Wert), Deckkraft pulsiert mit der Uhr (B-905).
func (p *planner) embers(w *sim.World, k view.Camera, clock float64) {
	v, ok := trait(w, "glow")
	if !ok || v <= 0 {
		return
	}
	from, to, ok := span(w, k, emberStep, 1)
	if !ok {
		return
	}
	p.body = append(p.body, item{shape: shapeRect, x: (from + to) / 2, w: to - from, h: hazeHeight, clr: hazeColor,
		alpha: 0.06 * float32(v)})
	for x := from; x < to; x += emberStep {
		h := cellHash(x, emberStep, 401)
		if h%8 >= uint32(2*v) {
			continue
		}
		r := 0.08 + float64(h>>4%11)/100
		y := 0.05 + float64(h>>8%36)/100
		phase := float64(h>>12%628) / 100
		a := float32(0.55 + 0.4*math.Sin(clock*2.3+phase))
		p.body = append(p.body, item{shape: shapeCircle, x: x + float64(h>>16%10)/10, y: y, w: r, clr: emberColor, alpha: a})
	}
}

// crystalGlow: Schein in der Farbe jeder sichtbaren Kristall-Ader, Radius und Deckkraft nach Wert (B-906); er liegt
// vor der Ader in der Zeichenliste, also hinter ihr.
func (p *planner) crystalGlow(w *sim.World, k view.Camera, clock float64) {
	v, ok := trait(w, "brightness")
	if !ok {
		return
	}
	r := 0.8 + 0.6*float64(v)
	for _, n := range w.Nodes {
		if n.Kind != "crystalVein" || !visible(k, n.X, r) {
			continue
		}
		clr, ok := veinGlow[n.Variant]
		if !ok {
			clr = veinGlow["blau"]
		}
		pulse := 1 + 0.1*math.Sin(clock*1.5+float64(mix(n.X)%628)/100)
		a := float32(0.12 * float64(v) * pulse)
		p.body = append(p.body, item{shape: shapeCircle, x: n.X, y: veinGlowMid, w: r, clr: clr, alpha: a},
			item{shape: shapeCircle, x: n.X, y: veinGlowMid, w: r * 0.55, clr: clr, alpha: a})
	}
}
