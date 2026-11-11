package rig

// Tests gegen die Sollwerte aus tools/paper (B-778/AC-02): Teil-Matrizen und Stützpunkte je Frame, Toleranz 0,01.
// Die Sollwerte schreibt `task native:rig:export` nach assets/soll/.

import (
	"encoding/json"
	"math"
	"os"
	"path/filepath"
	"testing"
)

const tolerance = 0.01

type figureFrame struct {
	Angles map[string]float64 `json:"winkel"`
	Offset Point              `json:"versatz"`
	Wind   float64            `json:"wind"`
	World  map[string]M       `json:"matrizen"`
	Cloak  []Point            `json:"umhang"`
}

type rideFrame struct {
	Ride struct {
		Dy    float64 `json:"dy"`
		Pitch float64 `json:"pitch"`
	} `json:"ride"`
	Wind       float64            `json:"wind"`
	Angles     map[string]float64 `json:"winkel"`
	World      map[string]M       `json:"matrizen"`
	Legs       map[string][]Point `json:"beine"`
	RiderWorld map[string]M       `json:"reiterMatrizen"`
	Cloak      []Point            `json:"umhang"`
	Reins      struct {
		Hand  Point `json:"hand"`
		Mouth Point `json:"maul"`
	} `json:"zuegel"`
}

func assets(t *testing.T, parts ...string) string {
	t.Helper()
	return filepath.Join(append([]string{"..", "assets"}, parts...)...)
}

func loadRig(t *testing.T, id string) *Rig {
	t.Helper()
	r, err := Load(assets(t, id))
	if err != nil {
		t.Fatal(err)
	}
	return r
}

func readExpected[F any](t *testing.T, name string) []F {
	t.Helper()
	text, err := os.ReadFile(assets(t, "soll", name))
	if err != nil {
		t.Fatalf("Sollwerte fehlen (erst `task native:rig:export`): %v", err)
	}
	var doc struct {
		Frames []F `json:"frames"`
	}
	if err := json.Unmarshal(text, &doc); err != nil {
		t.Fatal(err)
	}
	if len(doc.Frames) == 0 {
		t.Fatalf("%s: keine Frames", name)
	}
	return doc.Frames
}

func near(a, b float64) bool { return math.Abs(a-b) <= tolerance }

func checkPoint(t *testing.T, what string, got, want Point) {
	t.Helper()
	if !near(got[0], want[0]) || !near(got[1], want[1]) {
		t.Errorf("%s: %v, Soll %v", what, got, want)
	}
}

func checkPoints(t *testing.T, what string, got, want []Point) {
	t.Helper()
	if len(got) != len(want) {
		t.Fatalf("%s: %d Punkte, Soll %d", what, len(got), len(want))
	}
	for i := range want {
		checkPoint(t, what, got[i], want[i])
	}
}

// checkWorld vergleicht die Matrizen aller Teile des Rigs; das Schwert ist im Rig ein Element am Platz `waffe`.
func checkWorld(t *testing.T, what string, r *Rig, got []M, want map[string]M) {
	t.Helper()
	seen := 0
	for id, m := range want {
		var g M
		switch i := r.Index(id); {
		case i >= 0:
			g = got[i]
		case id == "schwert":
			s := r.Slot("waffe")
			g = Mul(r.SlotWorld(s, got), Translate(-s.Point[0], -s.Point[1]))
		default:
			t.Fatalf("%s: Teil %s fehlt im Rig %s", what, id, r.ID)
		}
		seen++
		for k := range m {
			if !near(g[k], m[k]) {
				t.Errorf("%s %s: %v, Soll %v", what, id, g, m)
				break
			}
		}
	}
	if seen < len(r.Parts) {
		t.Errorf("%s: Soll nennt %d Teile, das Rig hat %d", what, seen, len(r.Parts))
	}
}

func checkAngles(t *testing.T, what string, r *Rig, got []float64, want map[string]float64) {
	t.Helper()
	for id, w := range want {
		if i := r.Index(id); i < 0 || !near(got[i], w) {
			t.Errorf("%s Winkel %s: Soll %v (Index %d, Ist %v)", what, id, w, i, got)
		}
	}
}

func TestFigureMatchesPaper(t *testing.T) {
	r := loadRig(t, "monarch")
	for _, name := range []string{"idle", "lauf"} {
		t.Run(name, func(t *testing.T) {
			frames := readExpected[figureFrame](t, "monarch-"+name+".json")
			freq, _ := r.FigureFreq(name)
			cloak, err := r.FigureCloakFrames(name)
			if err != nil {
				t.Fatal(err)
			}
			var pose FigurePose
			for n, f := range frames {
				if err := r.FigurePose(name, FrameTime(n, len(frames), freq), &pose); err != nil {
					t.Fatal(err)
				}
				checkAngles(t, name, r, pose.Angles, f.Angles)
				checkPoint(t, name+" Versatz", pose.Offset, f.Offset)
				if !near(pose.Wind, f.Wind) {
					t.Errorf("%s Wind %v, Soll %v", name, pose.Wind, f.Wind)
				}
				world := r.World(pose.Angles, Translate(pose.Offset[0], pose.Offset[1]), nil)
				checkWorld(t, name, r, world, f.World)
				checkPoints(t, name+" Umhang", cloak[n], f.Cloak)
			}
		})
	}
}

func TestRideMatchesPaper(t *testing.T) {
	mount, rider := loadRig(t, "pferd"), loadRig(t, "monarch")
	frames := readExpected[rideFrame](t, "reiten-trab.json")
	cloak, err := RideCloakFrames(mount, rider, "trab", len(frames))
	if err != nil {
		t.Fatal(err)
	}
	var s Ride
	for n, f := range frames {
		if err := s.Set(mount, rider, "trab", float64(n)/float64(len(frames)), Identity); err != nil {
			t.Fatal(err)
		}
		if !near(s.Pose.Dy, f.Ride.Dy) || !near(s.Pose.Pitch, f.Ride.Pitch) || !near(s.Pose.Wind, f.Wind) {
			t.Errorf("Frame %d Ride %v/%v Wind %v, Soll %+v Wind %v", n, s.Pose.Dy, s.Pose.Pitch, s.Pose.Wind, f.Ride, f.Wind)
		}
		checkAngles(t, "trab", mount, s.Pose.Angles, f.Angles)
		checkWorld(t, "trab Pferd", mount, s.MountWorld, f.World)
		checkWorld(t, "trab Reiter", rider, s.RiderWorld, f.RiderWorld)
		for leg, name := range LegNames {
			checkPoints(t, "Bein "+name, mount.LegPoints(leg, s.Pose.Angles, nil)[:6], f.Legs[name])
		}
		checkPoints(t, "trab Umhang", cloak[n], f.Cloak)
		hand, _, mouth, ok := ReinPoints(mount, s.MountWorld, rider, s.RiderWorld, s.Base)
		if !ok {
			t.Fatal("Zügel fehlen (Plätze hand und maul)")
		}
		inv := s.Base.Invert()
		checkPoint(t, "Zügel Hand", inv.Apply(hand), f.Reins.Hand)
		checkPoint(t, "Zügel Maul", inv.Apply(mouth), f.Reins.Mouth)
	}
}
