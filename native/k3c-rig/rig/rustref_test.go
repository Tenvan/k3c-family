package rig

// Referenzwerte für den Rust-Port (rust/crates/k3c-rig, RU5): Dieser Test schreibt mit `go test ./rig -run RustRef
// -update` JSON nach testdata/rust/rig/; die Rust-Tests lesen sie und vergleichen mit Toleranz 1e-9. Ohne -update
// schreibt er nichts. Der Go-Code selbst bleibt unverändert. Die Einträge sind `map[string]any`, damit die Datei kurz
// bleibt; die Schlüssel lesen die Rust-Tests in rust/crates/k3c-rig/src/rig/rustref_*_tests.rs.

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

type rec = map[string]any

// writeRef schreibt v als JSON nach testdata/rust/rig/<name>.json (nur mit -update).
func writeRef(t *testing.T, name string, v any) {
	t.Helper()
	if !*updateRef {
		t.Skip("Referenzwerte schreibt nur -update")
	}
	dir := filepath.Join("..", "..", "..", "testdata", "rust", "rig")
	text, err := json.Marshal(v)
	if err == nil {
		err = os.MkdirAll(dir, 0o755)
	}
	if err == nil {
		err = os.WriteFile(filepath.Join(dir, name+".json"), append(text, '\n'), 0o644)
	}
	if err != nil {
		t.Fatal(err)
	}
}

func errText(err error) string {
	if err == nil {
		return ""
	}
	return err.Error()
}

func vertsOf(v []Vertex) [][4]float64 {
	out := make([][4]float64, len(v))
	for i, e := range v {
		out[i] = [4]float64{e.X, e.Y, e.U, e.V}
	}
	return out
}

// TestRustRefCurve: Kurven und IK an festen Stützstellen.
func TestRustRefCurve(t *testing.T) {
	var cat, her, spl, iks []rec
	for _, tt := range []float64{0, 0.1, 0.25, 0.5, 0.75, 0.9, 1} {
		for _, p := range [][4]float64{{1, 2, 5, 9}, {-11, 0, 1.2, 1.2}, {0, 3, -2, 4.5}} {
			cat = append(cat, rec{"args": []float64{p[0], p[1], p[2], p[3], tt}, "v": []float64{catmull(p[0], p[1], p[2], p[3], tt)}})
			for _, f := range [][2]float64{{1, 1}, {0.3, 0.7}} {
				her = append(her, rec{"args": []float64{p[0], p[1], p[2], p[3], tt, f[0], f[1]}, "v": []float64{hermite(p[0], p[1], p[2], p[3], tt, f[0], f[1])}})
			}
		}
	}
	for _, pts := range [][]Point{
		{{-1, 0}, {-1.35, 0.45}, {-0.7, 1}, {0.3, 0.95}, {1.35, 0.55}, {1.15, 0.12}, {1, 0}},
		{{-1, 0}, {-1.45, 0.35}, {-0.9, 0.85}, {0.2, 0.75}, {1.1, 0.35}, {1, 0}},
	} {
		for _, k := range []float64{0, 0.07, 0.3, 0.5, 0.77, 0.9999, 1} {
			p := spline(pts, k)
			spl = append(spl, rec{"pts": pts, "k": k, "v": []float64{p[0], p[1]}})
		}
	}
	for _, target := range [][2]float64{{0, 40}, {10, 45}, {-12, 30}, {5, 5}, {30, 60}, {-20, 52}} {
		for i, front := range []bool{false, true} {
			a, b := ik(target[0], target[1], front, 25, 31)
			iks = append(iks, rec{"args": []float64{target[0], target[1], float64(i), 25, 31}, "v": []float64{a, b}})
		}
	}
	writeRef(t, "curve", rec{"catmull": cat, "hermite": her, "spline": spl, "ik": iks})
}

// TestRustRefBand: Bänder aller Rigs aus Ruhelage, verbogenen Stützpunkten und (Vierbeiner) Bein-Stützpunkten.
func TestRustRefBand(t *testing.T) {
	var strips, legs []rec
	add := func(dir string, b *Band, kind string, pts []Point, density float64) {
		lim := make([]float64, min(len(pts), maxStations))
		tangentLimits(pts[:len(lim)], lim)
		strips = append(strips, rec{"rig": dir, "band": b.ID, "kind": kind, "points": pts, "density": density,
			"verts": vertsOf(b.Strip(pts, density, nil)), "limits": lim})
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
			add(dir, b, "rest", rest, r.Density)
			add(dir, b, "bent", bent, r.Density)
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
				legs = append(legs, rec{"rig": dir, "leg": leg, "angles": angles, "points": pts})
				add(dir, &r.Bands[leg], fmt.Sprintf("leg%d@%g", leg, phase), pts, r.Density)
			}
		}
	}
	writeRef(t, "band", rec{"strips": strips, "legs": legs})
}

// TestRustRefFigure: Figur-Posen (Winkel, Versatz, Wind), Frequenzen, Fehlertexte und der Umhang je Rig.
func TestRustRefFigure(t *testing.T) {
	var poses, cloaks []rec
	state := func(k int) (Point, float64) {
		wind := 0.0
		if (k+1000)%200 >= 100 {
			wind = 25
		}
		return Point{2 * math.Sin(float64(k)/60), -50 + 3*math.Cos(float64(k)/45)}, wind
	}
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		for _, name := range append(animationNames(r), "nichtda") {
			for _, tm := range []float64{0, 0.13, 0.5, 0.77, -0.3, 2.6} {
				var fp FigurePose
				err := r.FigurePose(name, tm, &fp)
				freq, ferr := r.FigureFreq(name)
				p := rec{"rig": dir, "anim": name, "t": tm, "offset": fp.Offset, "wind": fp.Wind, "freq": freq}
				if err != nil {
					p["err"] = err.Error()
				} else {
					p["angles"] = fp.Angles
				}
				if ferr != nil {
					p["freq_err"] = ferr.Error()
				}
				poses = append(poses, p)
			}
		}
		if r.Cloak == nil {
			continue
		}
		for _, floor := range []float64{0, rideFloor} {
			steps := FrameSteps(1.3, 6)
			cloaks = append(cloaks, rec{"rig": dir, "floor": floor, "steps": steps, "frames": CloakRun(r.Cloak, steps, state, floor)})
		}
	}
	writeRef(t, "figure", rec{"poses": poses, "cloaks": cloaks})
}

// TestRustRefGait: Gangart-Posen der Vierbeiner (alle Gangarten, mehrere Phasen), Reiter-Winkel und Frequenzen.
func TestRustRefGait(t *testing.T) {
	var gaits []rec
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		for _, name := range []string{"stand", "schritt", "trab", "galopp", "nichtda"} {
			freq, _ := GaitFreq(name)
			for _, ph := range []float64{0, 0.07, 0.25, 0.5, 0.61, 0.93, 1} {
				var p GaitPose
				g := rec{"rig": dir, "gait": name, "phase": ph, "freq": freq}
				if err := r.GaitPose(name, ph, &p); err != nil {
					g["err"] = err.Error()
				} else {
					g["angles"], g["dy"], g["pitch"], g["wind"], g["rider_y"], g["lean"] = p.Angles, p.Dy, p.Pitch, p.Wind, p.RiderY, p.Lean
					g["rider"], g["rider_angles"] = p.Rider[:], r.RiderAngles(&p, nil)
				}
				gaits = append(gaits, g)
			}
		}
	}
	writeRef(t, "gait", rec{"gaits": gaits})
}

// TestRustRefFrames: Umhang je Frame zu Fuß und beim Reiten, Ride-Zustand und Keyframe-Sichtbarkeit.
func TestRustRefFrames(t *testing.T) {
	var cloaks, rides, rideCloaks, hidden []rec
	var times []float64
	for _, n := range []int{0, 1, 5} {
		times = append(times, FrameTime(n, 8, 1.6), FrameTime(n, 4, 0.5))
	}
	rider := loadRig(t, riderDir)
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		for _, name := range animationNames(r) {
			if r.Cloak != nil {
				f, err := r.FigureCloakFrames(name)
				cloaks = append(cloaks, rec{"rig": dir, "anim": name, "err": errText(err), "frames": f})
			}
			for _, ph := range []float64{0, 0.3, 0.8} {
				parts := []string{}
				for _, part := range r.Parts {
					if r.KeyframeHidden(name, part.ID, ph) {
						parts = append(parts, part.ID)
					}
				}
				hidden = append(hidden, rec{"rig": dir, "anim": name, "phase": ph, "parts": parts})
			}
		}
		if r.Kind != "vierbeiner" {
			continue
		}
		for _, gname := range []string{"stand", "schritt", "trab", "galopp"} {
			if rider.Cloak != nil {
				f, err := RideCloakFrames(r, rider, gname, 6)
				rideCloaks = append(rideCloaks, rec{"mount": dir, "gait": gname, "err": errText(err), "frames": f})
			}
			for _, ph := range []float64{0, 0.35, 0.8} {
				var s Ride
				if s.Set(r, rider, gname, ph, Translate(10, -3)) != nil {
					continue
				}
				ref := rec{"mount": dir, "gait": gname, "phase": ph, "base": s.Base, "mount_world": s.MountWorld, "rider_world": s.RiderWorld}
				if rider.Cloak != nil {
					ref["anchor"] = s.CloakAnchor(rider)
				}
				if hand, ctrl, mouth, ok := ReinPoints(r, s.MountWorld, rider, s.RiderWorld, s.Base); ok {
					ref["rein"] = []float64{hand[0], hand[1], ctrl[0], ctrl[1], mouth[0], mouth[1]}
				}
				rides = append(rides, ref)
			}
		}
	}
	writeRef(t, "frames", rec{"cloaks": cloaks, "rides": rides, "ride_cloaks": rideCloaks, "hidden": hidden, "frame_times": times})
}

// TestRustRefLoad: Ladevorgang je Rig-Datei: Index, Eltern, Topo-Reihenfolge, Faktoren, Zeichenreihenfolge, Beine.
func TestRustRefLoad(t *testing.T) {
	var rigs []rec
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		var order []rec
		for _, it := range r.Order {
			order = append(order, rec{"kind": int(it.Kind), "index": it.Index, "layer": it.Layer})
		}
		var names, files []string
		var ids [][]string
		for _, name := range animationNames(r) {
			names, ids = append(names, name), append(ids, append([]string{}, r.Animations[name].ids...))
		}
		for _, p := range r.Parts {
			files = append(files, r.ImageFile(p.ID, p.File, "x")+"|"+r.ImageFile(p.ID, p.File, ""))
		}
		rigs = append(rigs, rec{"rig": dir, "scale": r.Scale, "parent": r.parent, "topo": r.topo, "factor": r.factor,
			"hide": r.hide, "order": order, "rider": r.rider[:], "upper": r.legs.upper[:], "lower": r.legs.lower[:],
			"hoof": r.legs.hoof[:], "neck_tail_back": []int{r.legs.neck, r.legs.tail, r.legs.back},
			"l": []float64{r.legs.l1, r.legs.l2}, "anim_names": names, "anim_ids": ids, "files": files})
	}
	writeRef(t, "load", rec{"rigs": rigs})
}

// TestRustRefWorld: Knochen-Matrizen (World) und Platz-Matrizen je Rig und Pose (Figur-Posen, Gangarten).
func TestRustRefWorld(t *testing.T) {
	var worlds []rec
	add := func(dir string, r *Rig, pose string, tm float64, angles []float64, base M) {
		world := r.World(angles, base, nil)
		var slots []M
		for i := range r.Slots {
			slots = append(slots, r.SlotWorld(&r.Slots[i], world))
		}
		hidden := make([]bool, len(r.Parts))
		for i := range hidden {
			hidden[i] = r.Hidden(i)
		}
		worlds = append(worlds, rec{"rig": dir, "pose": pose, "t": tm, "world": world, "slots": slots, "hidden": hidden,
			"inv": Mul(base, Rotation(33, 4, -7)).Invert()})
	}
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		for _, name := range animationNames(r) {
			for _, tm := range []float64{0, 0.3, 0.77} {
				var fp FigurePose
				var gp GaitPose
				switch {
				case r.Kind == "figur" && r.FigurePose(name, tm, &fp) == nil:
					add(dir, r, name, tm, fp.Angles, Translate(fp.Offset[0], fp.Offset[1]))
				case r.Kind != "figur" && r.GaitPose(r.Animations[name].Gait, tm, &gp) == nil:
					add(dir, r, name, tm, gp.Angles, RideMatrix(gp.Dy, gp.Pitch, r.Scale))
				}
			}
		}
	}
	writeRef(t, "world", rec{"worlds": worlds})
}
