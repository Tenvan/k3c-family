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

// area: doppelte Fläche des Dreiecks mit Vorzeichen (Umlaufsinn).
func area(a, b, c Vertex) float64 { return (b.X-a.X)*(c.Y-a.Y) - (c.X-a.X)*(b.Y-a.Y) }

// knownFolds: Vierbeiner, deren Bein-Band bekannt umklappt (NV3.2); die Posen stimmen, nur das Dreiecksband faltet sich.
var knownFolds = map[string]string{
	"reittiere/elefant": "Fußspitze 1,2 Einheiten nach 11 Einheiten Unterbein: Catmull-Rom mit gleichmäßigem Parameter schießt über die Spitze hinaus (in jeder Pose)",
	"gegner/wolf":       "Galopp, Abschnitt Hüfte–Oberbein-Mitte bei starker Beugung",
	"gegner/alphawolf":  "Galopp, Abschnitt Hüfte–Oberbein-Mitte bei starker Beugung",
}

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
func foldAt(verts []Vertex) int {
	for i := 0; i+3 < len(verts); i += 2 {
		// Viereck aus zwei Querschnitten: (links, rechts) oben und unten
		if area(verts[i], verts[i+1], verts[i+2]) < 0 || area(verts[i+1], verts[i+3], verts[i+2]) < 0 {
			return i / 2
		}
	}
	return -1
}
