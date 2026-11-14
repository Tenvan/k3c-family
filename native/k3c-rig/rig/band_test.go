package rig

import (
	"math"
	"testing"
)

// restPoints: Stützpunkte eines Bands in Ruhelage (senkrecht nach unten auf der Mittellinie).
func restPoints(b *Band) []Point {
	pts := make([]Point, len(b.Rest))
	for i, y := range b.Rest {
		pts[i] = Point{b.Center, y}
	}
	return pts
}

// In Ruhelage bildet das Band sein Bild unverzerrt ab: Ort in Einheiten · Dichte = Stelle im Bild.
func TestStripRestIsIdentity(t *testing.T) {
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		for i := range r.Bands {
			b := &r.Bands[i]
			verts := b.Strip(restPoints(b), r.Density, nil)
			if len(verts) != 2*((len(b.Rest)-1)*BandSub+3) {
				t.Fatalf("%s: %d Ecken", b.ID, len(verts))
			}
			for _, v := range verts {
				u, w := (v.X-b.Offset[0])*r.Density, (v.Y-b.Offset[1])*r.Density
				if math.Abs(u-v.U) > 0.01 || math.Abs(w-v.V) > 0.01 {
					t.Errorf("%s: Ecke (%.2f, %.2f) zeigt auf (%.2f, %.2f) statt (%.2f, %.2f)", b.ID, v.X, v.Y, v.U, v.V, u, w)
				}
				if v.U < 0 || v.U > float64(b.Size[0]) || v.V < 0 || v.V > float64(b.Size[1]) {
					t.Errorf("%s: Stelle (%.2f, %.2f) liegt außerhalb des Bilds %v", b.ID, v.U, v.V, b.Size)
				}
			}
		}
	}
}

// knownFolds: Vierbeiner, deren Bein-Band bekannt umklappt. Seit B-791 leer: Die Tangenten-Grenze hält die Kurve am
// Band-Ende (Elefant), unfold die innere Kante an engen Beugungen (Wölfe im Galopp).
var knownFolds = map[string]string{}

// Ein Bein-Band klappt in keiner Gangart um: Alle Dreiecke behalten ihren Umlaufsinn (sonst zeigt das Bild Streifen).
func TestLegStripNeverFolds(t *testing.T) {
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		if r.Kind != "vierbeiner" {
			continue
		}
		t.Run(dir, func(t *testing.T) {
			if why, ok := knownFolds[dir]; ok {
				t.Skip("bekannt: " + why)
			}
			for _, name := range animationNames(r) {
				checkLegFolds(t, dir, r, r.Animations[name].Gait)
			}
		})
	}
}

func checkLegFolds(t *testing.T, dir string, r *Rig, gaitName string) {
	var pose GaitPose
	for step := range 64 {
		if err := r.GaitPose(gaitName, float64(step)/64, &pose); err != nil {
			t.Fatal(err)
		}
		for leg, name := range LegNames {
			if i := foldAt(r.Bands[leg].Strip(r.LegPoints(leg, pose.Angles, nil), r.Density, nil)); i >= 0 {
				t.Errorf("%s %s Phase %d/64, Bein %s: Band klappt bei Querschnitt %d um", dir, gaitName, step, name, i)
			}
		}
	}
}

// foldAt liefert den ersten Querschnitt, an dem das Band umklappt, oder -1.
// foldEps: numerische Null der doppelten Dreiecksfläche (Rig-Einheiten²); unfold legt Ecken per Lot genau auf die
// Linie, gerundet bleibt ein Rest um 1e-15.
const foldEps = 1e-9

func foldAt(verts []Vertex) int {
	for i := 0; i+3 < len(verts); i += 2 {
		// Viereck aus zwei Querschnitten: (links, rechts) oben und unten
		if area(verts[i], verts[i+1], verts[i+2]) < -foldEps || area(verts[i+1], verts[i+3], verts[i+2]) < -foldEps {
			return i / 2
		}
	}
	return -1
}

// Mit Faktor 1 ist hermite genau Catmull-Rom; ein kleiner Faktor hält die Kurve im Abschnitt.
func TestHermiteLimitsOvershoot(t *testing.T) {
	for _, tt := range []float64{0, 0.25, 0.5, 0.75, 1} {
		if a, b := hermite(1, 2, 5, 9, tt, 1, 1), catmull(1, 2, 5, 9, tt); math.Abs(a-b) > 1e-12 {
			t.Errorf("t=%.2f: hermite %.4f, catmull %.4f", tt, a, b)
		}
	}
	// langer Abschnitt davor (−11), kurzer Abschnitt 0 … 1,2: ungebremst schießt die Kurve über 1,2 hinaus
	if v := catmull(-11, 0, 1.2, 1.2, 0.75); v <= 1.2 {
		t.Fatalf("Testfall schießt nicht über: %.3f", v)
	}
	lim := make([]float64, 4)
	tangentLimits([]Point{{0, -11}, {0, 0}, {0, 1.2}, {0, 1.2}}, lim)
	for k := range 9 {
		if v := hermite(-11, 0, 1.2, 1.2, float64(k)/8, lim[1], lim[2]); v < 0 || v > 1.2+1e-9 {
			t.Errorf("t=%d/8: %.3f liegt außerhalb 0 … 1,2", k, v)
		}
	}
}

// unfold hält eine rückwärts laufende Kante an: Danach hat kein Dreieck eine negative Fläche.
func TestUnfoldStopsBackwardEdge(t *testing.T) {
	v := []Vertex{{X: 0, Y: 0}, {X: 4, Y: 0}, {X: 0, Y: 2}, {X: 4, Y: -1}, {X: 0, Y: 4}, {X: 4, Y: 4}}
	if foldAt(v) < 0 {
		t.Fatal("Testfall faltet nicht")
	}
	unfold(v)
	if i := foldAt(v); i >= 0 {
		t.Errorf("faltet weiter bei %d: %+v", i, v)
	}
	if a := area(v[1], v[3], v[2]); math.Abs(a) > foldEps { // per Lot auf die Diagonale: Dreieck wird zur Linie
		t.Errorf("rechte Kante nicht auf der Diagonale angehalten: %+v (Fläche %g)", v[3], a)
	}
}

// unfold bewegt eine Ecke stetig: Wandert die Ecke eines Querschnitts in kleinen Schritten über die Linie des vorigen
// Querschnitts, springt sie nicht (vorher: Sprung auf die Ecke davor, B-791-Review).
func TestUnfoldIsContinuous(t *testing.T) {
	var last Vertex
	for k := 0; k <= 40; k++ {
		y := 0.01 - float64(k)*0.0005 // von +0,01 (ungefaltet) bis -0,01 (gefaltet)
		v := []Vertex{{X: 0, Y: 0}, {X: 1, Y: 0}, {X: 0.5, Y: y}, {X: 1, Y: 1}}
		unfold(v)
		if a := area(v[0], v[1], v[2]); a < -foldEps {
			t.Fatalf("y=%.4f: Dreieck bleibt gefaltet (%g)", y, a)
		}
		if k > 0 && math.Hypot(v[2].X-last.X, v[2].Y-last.Y) > 0.001 {
			t.Errorf("y=%.4f: Ecke springt von (%.4f, %.4f) nach (%.4f, %.4f)", y, last.X, last.Y, v[2].X, v[2].Y)
		}
		last = v[2]
	}
}

// Zwischen benachbarten Phasen (1/512) bewegt sich keine Ecke eines Bein-Bands sprunghaft: höchstens legJump mal so
// weit wie der am weitesten bewegte Stützpunkt des Beins (plus eine kleine Grenze für Phasen fast ohne Bewegung).
func TestLegStripContinuous(t *testing.T) {
	const steps, legJump, minJump = 512, 4.0, 0.05
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		if r.Kind != "vierbeiner" {
			continue
		}
		for _, name := range animationNames(r) {
			checkLegJumps(t, dir, r, r.Animations[name].Gait, steps, legJump, minJump)
		}
	}
}

func checkLegJumps(t *testing.T, dir string, r *Rig, gaitName string, steps int, legJump, minJump float64) {
	var pose GaitPose
	prevPts, prevStrip := make([][]Point, len(LegNames)), make([][]Vertex, len(LegNames))
	for step := range steps + 1 {
		if err := r.GaitPose(gaitName, float64(step%steps)/float64(steps), &pose); err != nil {
			t.Fatal(err)
		}
		for leg, name := range LegNames {
			pts := r.LegPoints(leg, pose.Angles, nil)
			strip := r.Bands[leg].Strip(pts, r.Density, nil)
			if step > 0 && len(strip) == len(prevStrip[leg]) {
				limit := max(legJump*maxMove(prevPts[leg], pts), minJump)
				if j := maxVertexMove(prevStrip[leg], strip); j > limit {
					t.Errorf("%s %s Phase %d/%d, Bein %s: Ecke springt %.3f (Grenze %.3f)", dir, gaitName, step, steps, name, j, limit)
				}
			}
			prevPts[leg], prevStrip[leg] = pts, strip
		}
	}
}

func maxMove(a, b []Point) float64 {
	m := 0.0
	for i := range min(len(a), len(b)) {
		m = max(m, math.Hypot(b[i][0]-a[i][0], b[i][1]-a[i][1]))
	}
	return m
}

func maxVertexMove(a, b []Vertex) float64 {
	m := 0.0
	for i := range min(len(a), len(b)) {
		m = max(m, math.Hypot(b[i].X-a[i].X, b[i].Y-a[i].Y))
	}
	return m
}
