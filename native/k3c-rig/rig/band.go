package rig

import "math"

// Vertex ist eine Ecke eines Bands: Ort in Einheiten und Stelle im Bild in Pixeln.
type Vertex struct{ X, Y, U, V float64 }

// station ist ein Querschnitt des Bands: linke und rechte Kante (Ort) und ihre Stelle im Bild (Pixel).
type station struct {
	l, r      Point
	ul, ur, v float64
}

// maxStations begrenzt die Stützpunkte eines Bands (Umhang 8, Bein 7); mehr werden abgeschnitten.
const maxStations = 16

// BandSub ist die Zahl der Zwischenschritte je Abschnitt zwischen zwei Stützpunkten (Catmull-Rom).
const BandSub = 4

// bandInset hält die Ecken einen Pixel vom Bildrand fern, damit der lineare Filter nichts von außen liest.
const bandInset = 1

// LegPoints liefert die Stützpunkte eines Beins im Rahmen des Rumpf-Teils (tools/paper/rig/beinpfad.ts › beinPunkte):
// Ansatz 16 über der Hüfte, Hüfte, Oberbein-Mitte, Knie, Unterbein-Mitte, Fessel und die Spitze des Fußes.
func (r *Rig) LegPoints(leg int, angles []float64, out []Point) []Point {
	x := &r.legs
	hip := r.Parts[x.upper[leg]].Pivot
	kneeY, fetlockY := r.Parts[x.lower[leg]].Pivot[1], r.Parts[x.hoof[leg]].Pivot[1]
	a1 := angles[x.upper[leg]]
	d1, d2 := direction(a1), direction(a1+angles[x.lower[leg]])
	upper, lower := kneeY-hip[1], fetlockY-kneeY
	knee := along(hip, d1, upper)
	fetlock := along(knee, d2, lower)
	tip := 0.0
	if r.Leg != nil {
		tip = r.Leg.Tip
	}
	return append(out[:0], Point{hip[0], hip[1] - 16}, hip, along(hip, d1, upper*0.48), knee, along(knee, d2, lower*0.5),
		fetlock, along(fetlock, d2, tip))
}

// direction: Richtung eines hängenden Knochens bei Winkel deg (0 = senkrecht nach unten).
func direction(deg float64) Point {
	rad := deg * math.Pi / 180
	return Point{-math.Sin(rad), math.Cos(rad)}
}

func along(p, d Point, l float64) Point { return Point{p[0] + d[0]*l, p[1] + d[1]*l} }

// Strip biegt das Bild des Bands entlang der Stützpunkte pts (einer je Rest-Wert) und liefert die Ecken paarweise
// (links, rechts) von oben nach unten. Je Stützpunkt liegen die Kanten Width neben der Mittellinie (die Papierkante
// samt Rand für den Schatten), nicht am Bildrand: So faltet sich das Band an einem stark gebeugten Knie nicht. Die
// Kanten laufen als Catmull-Rom-Kurve durch die Stützpunkte, wie die Umrisse in tools/paper; oben und unten verlängert
// der Rand des Bilds das Band gerade. density = Pixel je Einheit. In Ruhelage ergibt das genau das Bild.
func (b *Band) Strip(pts []Point, density float64, out []Vertex) []Vertex {
	var buf [maxStations]station
	n := min(len(pts), len(b.Rest), len(b.Width), maxStations)
	if n < 2 {
		return out[:0]
	}
	st, pts := buf[:n], pts[:n]
	width, height := float64(b.Size[0]), float64(b.Size[1])
	// größter Abstand von der Mittellinie, der noch im Bild liegt
	maxL, maxR := b.Center-b.Offset[0]-bandInset/density, b.Offset[0]+width/density-b.Center-bandInset/density
	for i, p := range pts {
		nx, ny := normal(pts[max(i-1, 0)], pts[min(i+1, n-1)])
		l, r := math.Min(b.Width[i][0], maxL), math.Min(b.Width[i][1], maxR)
		st[i] = station{
			l: Point{p[0] - nx*l, p[1] - ny*l}, r: Point{p[0] + nx*r, p[1] + ny*r},
			ul: (b.Center - l - b.Offset[0]) * density, ur: (b.Center + r - b.Offset[0]) * density,
			v: (b.Rest[i] - b.Offset[1]) * density,
		}
	}
	var lim [maxStations]float64
	tangentLimits(pts, lim[:n])
	out = emit(out[:0], extend(st[0], pts[0], pts[1], -(st[0].v-bandInset)/density, bandInset))
	for i := range n - 1 {
		s0, s1, s2, s3 := st[max(i-1, 0)], st[i], st[i+1], st[min(i+2, n-1)]
		for k := range BandSub {
			out = emit(out, between(s0, s1, s2, s3, float64(k)/BandSub, lim[i], lim[i+1]))
		}
	}
	out = emit(out, st[n-1])
	out = emit(out, extend(st[n-1], pts[n-2], pts[n-1], (height-bandInset-st[n-1].v)/density, height-bandInset))
	unfold(out)
	return out
}

// tangentReach: höchstens so viele Längen des kürzeren anliegenden Abschnitts darf die Tangente eines Stützpunkts
// lang sein (Fritsch-Carlson: darüber schießt die Hermite-Kurve über das Abschnittsende hinaus).
const tangentReach = 3

// tangentLimits schreibt je Stützpunkt den Faktor (0..1) auf die Catmull-Rom-Tangente: Folgt auf einen langen
// Abschnitt ein kurzer (Elefant: 11 Einheiten Unterbein, 1,2 Fußspitze), schießt die Kurve sonst über das Band-Ende
// hinaus (B-791). Ein Faktor je Stützpunkt für alle Größen hält die Kurve glatt und die Ruhelage unverzerrt.
func tangentLimits(pts []Point, out []float64) {
	n := len(out)
	for i := range out {
		p0, p2 := pts[max(i-1, 0)], pts[min(i+1, n-1)]
		m := math.Hypot(p2[0]-p0[0], p2[1]-p0[1]) / 2
		chord := math.Inf(1)
		if i > 0 {
			chord = dist(pts[i-1], pts[i])
		}
		if i < n-1 {
			chord = math.Min(chord, dist(pts[i], pts[i+1]))
		}
		out[i] = 1
		if m > tangentReach*chord {
			out[i] = tangentReach * chord / m
		}
	}
}

func dist(a, b Point) float64 { return math.Hypot(b[0]-a[0], b[1]-a[1]) }

// unfold hält eine Kante an, wo sie gegen die Laufrichtung des Bands zurückliefe: An einer Beugung, deren Radius
// kleiner ist als die Breite (Hüfte des Wolfs im Galopp), läge die innere Kante sonst rückwärts und das Dreieck
// klappte um. Die Ecke kommt per Lot auf die Linie des vorigen Querschnitts bzw. der vorigen Diagonale: Das Dreieck
// wird zur Linie (Fläche 0), das Bild staucht sich in der Ecke, und weil das Lot eines Punkts auf der Linie er selbst
// ist, bewegt sich die Kante beim Vorzeichenwechsel stetig (kein Sprung zwischen zwei Bildern).
func unfold(v []Vertex) {
	for i := 0; i+3 < len(v); i += 2 {
		if area(v[i], v[i+1], v[i+2]) < 0 {
			v[i+2].X, v[i+2].Y = foot(v[i], v[i+1], v[i+2])
		}
		if area(v[i+1], v[i+3], v[i+2]) < 0 {
			v[i+3].X, v[i+3].Y = foot(v[i+1], v[i+2], v[i+3])
		}
	}
}

// foot ist der Fußpunkt des Lots von p auf die Gerade durch a und b (a, wenn a und b zusammenfallen).
func foot(a, b, p Vertex) (x, y float64) {
	dx, dy := b.X-a.X, b.Y-a.Y
	d := dx*dx + dy*dy
	if d == 0 {
		return a.X, a.Y
	}
	t := ((p.X-a.X)*dx + (p.Y-a.Y)*dy) / d
	return a.X + t*dx, a.Y + t*dy
}

// area: doppelte Fläche des Dreiecks mit Vorzeichen (Umlaufsinn; positiv = Band läuft ungefaltet).
func area(a, b, c Vertex) float64 { return (b.X-a.X)*(c.Y-a.Y) - (c.X-a.X)*(b.Y-a.Y) }

// normal: Einheits-Normale zur Richtung a → b (zeigt nach rechts, wenn die Richtung nach unten läuft).
func normal(a, b Point) (float64, float64) {
	tx, ty := b[0]-a[0], b[1]-a[1]
	d := math.Hypot(tx, ty)
	if d == 0 {
		d = 1
	}
	return ty / d, -tx / d
}

// extend verschiebt einen Querschnitt um dist entlang der Richtung a → b (negativ = zurück) auf die Bildhöhe v.
func extend(s station, a, b Point, dist, v float64) station {
	tx, ty := b[0]-a[0], b[1]-a[1]
	d := math.Hypot(tx, ty)
	if d == 0 {
		d = 1
	}
	dx, dy := tx/d*dist, ty/d*dist
	s.l, s.r, s.v = Point{s.l[0] + dx, s.l[1] + dy}, Point{s.r[0] + dx, s.r[1] + dy}, v
	return s
}

// between: Querschnitt zwischen s1 und s2 bei t = 0..1; Kanten und Stellen im Bild mit derselben Kurve, die
// Tangenten in s1 und s2 mit den Faktoren f1 und f2 (tangentLimits; 1 = Catmull-Rom).
func between(s0, s1, s2, s3 station, t, f1, f2 float64) station {
	if t == 0 {
		return s1
	}
	c := func(p0, p1, p2, p3 float64) float64 { return hermite(p0, p1, p2, p3, t, f1, f2) }
	return station{
		l:  Point{c(s0.l[0], s1.l[0], s2.l[0], s3.l[0]), c(s0.l[1], s1.l[1], s2.l[1], s3.l[1])},
		r:  Point{c(s0.r[0], s1.r[0], s2.r[0], s3.r[0]), c(s0.r[1], s1.r[1], s2.r[1], s3.r[1])},
		ul: c(s0.ul, s1.ul, s2.ul, s3.ul), ur: c(s0.ur, s1.ur, s2.ur, s3.ur),
		v: c(s0.v, s1.v, s2.v, s3.v),
	}
}

func emit(out []Vertex, s station) []Vertex {
	return append(out, Vertex{s.l[0], s.l[1], s.ul, s.v}, Vertex{s.r[0], s.r[1], s.ur, s.v})
}
