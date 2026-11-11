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
	for _, id := range []string{"monarch", "pferd"} {
		r := loadRig(t, id)
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

// Ein Bein-Band klappt in keiner Gangart um: Alle Dreiecke behalten ihren Umlaufsinn (sonst zeigt das Bild Streifen).
func TestLegStripNeverFolds(t *testing.T) {
	r := loadRig(t, "pferd")
	var pose GaitPose
	for _, gaitName := range []string{"stand", "schritt", "trab", "galopp"} {
		for step := range 64 {
			if err := r.GaitPose(gaitName, float64(step)/64, &pose); err != nil {
				t.Fatal(err)
			}
			for leg, name := range LegNames {
				verts := r.Bands[leg].Strip(r.LegPoints(leg, pose.Angles, nil), r.Density, nil)
				for i := 0; i+3 < len(verts); i += 2 {
					// Viereck aus zwei Querschnitten: (links, rechts) oben und unten
					if area(verts[i], verts[i+1], verts[i+2]) < 0 || area(verts[i+1], verts[i+3], verts[i+2]) < 0 {
						t.Errorf("%s Phase %d/64, Bein %s: Band klappt bei Querschnitt %d um", gaitName, step, name, i/2)
						break
					}
				}
			}
		}
	}
}
