package rig

// Referenzwerte für den Rust-Port (rust/crates/k3c-rig, RU5): Dieser Test schreibt mit `go test ./rig -run RustRef
// -update` JSON nach testdata/rust/rig/; die Rust-Tests lesen sie und vergleichen mit Toleranz 1e-9. Ohne -update
// schreibt er nichts. Der Go-Code selbst bleibt unverändert.

import (
	"encoding/json"
	"flag"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"testing"
)

var updateRef = flag.Bool("update", false, "Referenzwerte für Rust nach testdata/rust/rig schreiben")

// writeRef schreibt v als JSON nach testdata/rust/rig/<name>.json (nur mit -update).
func writeRef(t *testing.T, name string, v any) {
	t.Helper()
	if !*updateRef {
		t.Skip("Referenzwerte schreibt nur -update")
	}
	dir := filepath.Join("..", "..", "..", "testdata", "rust", "rig")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	text, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, name+".json"), append(text, '\n'), 0o644); err != nil {
		t.Fatal(err)
	}
}

// TestRustRefCurve: Kurven und IK an festen Stützstellen.
func TestRustRefCurve(t *testing.T) {
	type sample struct {
		Args []float64 `json:"args"`
		V    []float64 `json:"v"`
	}
	var out struct {
		Catmull []sample `json:"catmull"`
		Hermite []sample `json:"hermite"`
		Spline  []struct {
			Pts []Point   `json:"pts"`
			K   float64   `json:"k"`
			V   []float64 `json:"v"`
		} `json:"spline"`
		IK []sample `json:"ik"`
	}
	for _, tt := range []float64{0, 0.1, 0.25, 0.5, 0.75, 0.9, 1} {
		for _, p := range [][4]float64{{1, 2, 5, 9}, {-11, 0, 1.2, 1.2}, {0, 3, -2, 4.5}} {
			out.Catmull = append(out.Catmull, sample{[]float64{p[0], p[1], p[2], p[3], tt}, []float64{catmull(p[0], p[1], p[2], p[3], tt)}})
			for _, f := range [][2]float64{{1, 1}, {0.3, 0.7}} {
				out.Hermite = append(out.Hermite, sample{[]float64{p[0], p[1], p[2], p[3], tt, f[0], f[1]},
					[]float64{hermite(p[0], p[1], p[2], p[3], tt, f[0], f[1])}})
			}
		}
	}
	paths := [][]Point{
		{{-1, 0}, {-1.35, 0.45}, {-0.7, 1}, {0.3, 0.95}, {1.35, 0.55}, {1.15, 0.12}, {1, 0}},
		{{-1, 0}, {-1.45, 0.35}, {-0.9, 0.85}, {0.2, 0.75}, {1.1, 0.35}, {1, 0}},
	}
	for _, pts := range paths {
		for _, k := range []float64{0, 0.07, 0.3, 0.5, 0.77, 0.9999, 1} {
			p := spline(pts, k)
			out.Spline = append(out.Spline, struct {
				Pts []Point   `json:"pts"`
				K   float64   `json:"k"`
				V   []float64 `json:"v"`
			}{pts, k, []float64{p[0], p[1]}})
		}
	}
	for _, target := range [][2]float64{{0, 40}, {10, 45}, {-12, 30}, {5, 5}, {30, 60}, {-20, 52}} {
		for _, front := range []bool{false, true} {
			a, b := ik(target[0], target[1], front, 25, 31)
			f := 0.0
			if front {
				f = 1
			}
			out.IK = append(out.IK, sample{[]float64{target[0], target[1], f, 25, 31}, []float64{a, b}})
		}
	}
	writeRef(t, "curve", out)
}

// refStrip ist ein Band-Fall: Eingabe-Stützpunkte, Dichte und die Ecken (x, y, u, v).
type refStrip struct {
	Rig     string       `json:"rig"`
	Band    string       `json:"band"`
	Kind    string       `json:"kind"`
	Points  []Point      `json:"points"`
	Density float64      `json:"density"`
	Verts   [][4]float64 `json:"verts"`
	Limits  []float64    `json:"limits"`
}

func vertsOf(v []Vertex) [][4]float64 {
	out := make([][4]float64, len(v))
	for i, e := range v {
		out[i] = [4]float64{e.X, e.Y, e.U, e.V}
	}
	return out
}

// TestRustRefBand: Bänder aller Rigs aus Ruhelage, verbogenen Stützpunkten und (Vierbeiner) Bein-Stützpunkten.
func TestRustRefBand(t *testing.T) {
	var out struct {
		Strips []refStrip `json:"strips"`
		Legs   []struct {
			Rig    string    `json:"rig"`
			Leg    int       `json:"leg"`
			Angles []float64 `json:"angles"`
			Points []Point   `json:"points"`
		} `json:"legs"`
	}
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		for i := range r.Bands {
			b := &r.Bands[i]
			rest := restPoints(b)
			bent := make([]Point, len(rest))
			for k, p := range rest {
				bent[k] = Point{p[0] + 2.5*math.Sin(0.9*float64(k)+1), p[1] + 0.4*math.Cos(float64(k))}
			}
			for _, kp := range []struct {
				kind string
				pts  []Point
			}{{"rest", rest}, {"bent", bent}} {
				kind, pts := kp.kind, kp.pts
				lim := make([]float64, min(len(pts), maxStations))
				tangentLimits(pts[:len(lim)], lim)
				out.Strips = append(out.Strips, refStrip{dir, b.ID, kind, pts, r.Density, vertsOf(b.Strip(pts, r.Density, nil)), lim})
			}
		}
		if r.Kind != "vierbeiner" || len(r.Bands) < legCount {
			continue
		}
		for _, phase := range []float64{0, 1.3, 2.9} {
			angles := make([]float64, len(r.Parts))
			for i := range angles {
				angles[i] = 25 * math.Sin(0.7*float64(i)+phase)
			}
			for leg := range legCount {
				pts := r.LegPoints(leg, angles, nil)
				out.Legs = append(out.Legs, struct {
					Rig    string    `json:"rig"`
					Leg    int       `json:"leg"`
					Angles []float64 `json:"angles"`
					Points []Point   `json:"points"`
				}{dir, leg, angles, pts})
				out.Strips = append(out.Strips, refStrip{dir, r.Bands[leg].ID, fmt.Sprintf("leg%d@%g", leg, phase), pts, r.Density,
					vertsOf(r.Bands[leg].Strip(pts, r.Density, nil)), nil})
			}
		}
	}
	writeRef(t, "band", out)
}

// TestRustRefFigure: Figur-Posen (Winkel, Versatz, Wind), Frequenzen, Fehlertexte und der Umhang je Rig.
func TestRustRefFigure(t *testing.T) {
	type pose struct {
		Rig    string    `json:"rig"`
		Anim   string    `json:"anim"`
		T      float64   `json:"t"`
		Err    string    `json:"err,omitempty"`
		Angles []float64 `json:"angles,omitempty"`
		Offset Point     `json:"offset"`
		Wind   float64   `json:"wind"`
		Freq   float64   `json:"freq"`
		FreqEr string    `json:"freq_err,omitempty"`
	}
	type cloak struct {
		Rig    string    `json:"rig"`
		Floor  float64   `json:"floor"`
		Steps  []int     `json:"steps"`
		Frames [][]Point `json:"frames"`
	}
	var out struct {
		Poses  []pose  `json:"poses"`
		Cloaks []cloak `json:"cloaks"`
	}
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		names := append(animationNames(r), "nichtda")
		for _, name := range names {
			for _, tm := range []float64{0, 0.13, 0.5, 0.77, -0.3, 2.6} {
				var fp FigurePose
				p := pose{Rig: dir, Anim: name, T: tm}
				if err := r.FigurePose(name, tm, &fp); err != nil {
					p.Err = err.Error()
				} else {
					p.Angles, p.Offset, p.Wind = fp.Angles, fp.Offset, fp.Wind
				}
				if f, err := r.FigureFreq(name); err != nil {
					p.FreqEr = err.Error()
				} else {
					p.Freq = f
				}
				out.Poses = append(out.Poses, p)
			}
		}
		if r.Cloak == nil {
			continue
		}
		for _, floor := range []float64{0, rideFloor} {
			steps := FrameSteps(1.3, 6)
			state := func(k int) (Point, float64) {
				wind := 0.0
				if (k+1000)%200 >= 100 {
					wind = 25
				}
				return Point{2 * math.Sin(float64(k)/60), -50 + 3*math.Cos(float64(k)/45)}, wind
			}
			out.Cloaks = append(out.Cloaks, cloak{dir, floor, steps, CloakRun(r.Cloak, steps, state, floor)})
		}
	}
	writeRef(t, "figure", out)
}
