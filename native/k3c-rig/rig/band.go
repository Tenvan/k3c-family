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
	out = emit(out[:0], extend(st[0], pts[0], pts[1], -(st[0].v-bandInset)/density, bandInset))
	for i := range n - 1 {
		s0, s1, s2, s3 := st[max(i-1, 0)], st[i], st[i+1], st[min(i+2, n-1)]
		for k := range BandSub {
			out = emit(out, between(s0, s1, s2, s3, float64(k)/BandSub))
		}
	}
	out = emit(out, st[n-1])
	return emit(out, extend(st[n-1], pts[n-2], pts[n-1], (height-bandInset-st[n-1].v)/density, height-bandInset))
}

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

// between: Querschnitt zwischen s1 und s2 bei t = 0..1; Kanten und Stellen im Bild mit derselben Kurve.
func between(s0, s1, s2, s3 station, t float64) station {
	if t == 0 {
		return s1
	}
	return station{
		l:  Point{catmull(s0.l[0], s1.l[0], s2.l[0], s3.l[0], t), catmull(s0.l[1], s1.l[1], s2.l[1], s3.l[1], t)},
		r:  Point{catmull(s0.r[0], s1.r[0], s2.r[0], s3.r[0], t), catmull(s0.r[1], s1.r[1], s2.r[1], s3.r[1], t)},
		ul: catmull(s0.ul, s1.ul, s2.ul, s3.ul, t), ur: catmull(s0.ur, s1.ur, s2.ur, s3.ur, t),
		v: catmull(s0.v, s1.v, s2.v, s3.v, t),
	}
}

func emit(out []Vertex, s station) []Vertex {
	return append(out, Vertex{s.l[0], s.l[1], s.ul, s.v}, Vertex{s.r[0], s.r[1], s.ur, s.v})
}
