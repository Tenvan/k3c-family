package rig

// Tests gegen die Sollwerte aus tools/paper (B-778/AC-02, B-789/AC-03): Teil-Matrizen und Stützpunkte je Frame,
// Toleranz 0,01, für alle exportierten Figuren. Die Sollwerte schreibt `task native:rig:export` nach
// assets/soll/<gruppe>/<id>/<animation>.json, beim Reittier dazu reiten-<gangart>.json (Reiter: der Monarch).

import (
	"encoding/json"
	"io/fs"
	"math"
	"os"
	"path"
	"path/filepath"
	"slices"
	"sort"
	"strings"
	"testing"
)

const (
	tolerance = 0.01
	riderDir  = "figuren/monarch"
)

type figureFrame struct {
	Angles map[string]float64 `json:"winkel"`
	Offset Point              `json:"versatz"`
	Wind   float64            `json:"wind"`
	World  map[string]M       `json:"matrizen"`
	Hidden []string           `json:"ausgeblendet"`
	Cloak  []Point            `json:"umhang"`
}

type gaitFrame struct {
	Ride struct {
		Dy    float64 `json:"dy"`
		Pitch float64 `json:"pitch"`
	} `json:"ride"`
	Wind   float64            `json:"wind"`
	Angles map[string]float64 `json:"winkel"`
	World  map[string]M       `json:"matrizen"`
	Legs   map[string][]Point `json:"beine"`
}

type rideFrame struct {
	gaitFrame
	RiderWorld map[string]M `json:"reiterMatrizen"`
	Cloak      []Point      `json:"umhang"`
	Reins      struct {
		Hand  Point `json:"hand"`
		Mouth Point `json:"maul"`
	} `json:"zuegel"`
}

func assets(t *testing.T, parts ...string) string {
	t.Helper()
	return filepath.Join(append([]string{"..", "assets"}, parts...)...)
}

func loadRig(t *testing.T, dir string) *Rig {
	t.Helper()
	r, err := Load(assets(t, filepath.FromSlash(dir)))
	if err != nil {
		t.Fatal(err)
	}
	return r
}

// rigDirs nennt die Ordner aller Rigs (relativ zu assets/, mit /, sortiert); ohne Rigs schlägt der Test fehl.
func rigDirs(t *testing.T) []string {
	t.Helper()
	root := assets(t)
	var dirs []string
	err := filepath.WalkDir(root, func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() || d.Name() != "rig.json" {
			return err
		}
		rel, err := filepath.Rel(root, filepath.Dir(p))
		dirs = append(dirs, filepath.ToSlash(rel))
		return err
	})
	if err != nil || len(dirs) == 0 {
		t.Fatalf("keine Rigs unter %s (erst `task native:rig:export`): %v", root, err)
	}
	sort.Strings(dirs)
	return dirs
}

// animationNames: Animationen des Rigs, sortiert (stabile Reihenfolge der Subtests).
func animationNames(r *Rig) []string {
	names := make([]string, 0, len(r.Animations))
	for n := range r.Animations {
		names = append(names, n)
	}
	sort.Strings(names)
	return names
}

func readExpected[F any](t *testing.T, dir, name string) []F {
	t.Helper()
	text, err := os.ReadFile(assets(t, "soll", filepath.FromSlash(dir), name+".json"))
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
		t.Fatalf("%s/%s: keine Frames", dir, name)
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

// checkWorld vergleicht die Matrizen aller Teile des Rigs; das Schwert des Monarchen ist im Rig ein Element am Platz `waffe`.
func checkWorld(t *testing.T, what string, r *Rig, got []M, want map[string]M) {
	t.Helper()
	seen := 0
	for id, m := range want {
		var g M
		switch i := r.Index(id); {
		case i >= 0:
			g = got[i]
		case id == "schwert" && r.Slot("waffe") != nil:
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

// checkAngles vergleicht die Winkel; die Posen idle und lauf nennen auch Teile, die eine Figur nicht hat (krone), die
// zählen nicht (die Matrizen prüft checkWorld für alle Teile).
func checkAngles(t *testing.T, what string, r *Rig, got []float64, want map[string]float64) {
	t.Helper()
	for id, w := range want {
		if i := r.Index(id); i >= 0 && !near(got[i], w) {
			t.Errorf("%s Winkel %s: Soll %v (Index %d, Ist %v)", what, id, w, i, got)
		}
	}
}

// checkHidden vergleicht die ausgeblendeten Teile bei Phase p (Keyframes mit `sichtbar`).
func checkHidden(t *testing.T, what string, r *Rig, name string, p float64, want []string) {
	t.Helper()
	got := []string{}
	for _, part := range r.Parts {
		if r.KeyframeHidden(name, part.ID, p) {
			got = append(got, part.ID)
		}
	}
	sort.Strings(got)
	if !slices.Equal(got, want) {
		t.Errorf("%s ausgeblendet: %v, Soll %v", what, got, want)
	}
}

// checkGait vergleicht Ride, Wind, Winkel, Welt-Matrizen und Beine eines Vierbeiners mit einem Soll-Frame.
func checkGait(t *testing.T, what string, r *Rig, p *GaitPose, world []M, f *gaitFrame) {
	t.Helper()
	if !near(p.Dy, f.Ride.Dy) || !near(p.Pitch, f.Ride.Pitch) || !near(p.Wind, f.Wind) {
		t.Errorf("%s Ride %v/%v Wind %v, Soll %+v Wind %v", what, p.Dy, p.Pitch, p.Wind, f.Ride, f.Wind)
	}
	checkAngles(t, what, r, p.Angles, f.Angles)
	checkWorld(t, what, r, world, f.World)
	for leg, name := range LegNames {
		checkPoints(t, what+" Bein "+name, r.LegPoints(leg, p.Angles, nil)[:6], f.Legs[name])
	}
}

func checkFigure(t *testing.T, r *Rig, dir, name string) {
	frames := readExpected[figureFrame](t, dir, name)
	freq, err := r.FigureFreq(name)
	if err != nil {
		t.Fatal(err)
	}
	var cloak [][]Point
	if r.Cloak != nil {
		if cloak, err = r.FigureCloakFrames(name); err != nil {
			t.Fatal(err)
		}
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
		checkWorld(t, name, r, r.World(pose.Angles, Translate(pose.Offset[0], pose.Offset[1]), nil), f.World)
		checkHidden(t, name, r, name, float64(n)/float64(len(frames)), f.Hidden)
		if cloak != nil {
			checkPoints(t, name+" Umhang", cloak[n], f.Cloak)
		}
	}
}

func checkGaitAnimation(t *testing.T, r *Rig, dir, name string) {
	frames := readExpected[gaitFrame](t, dir, name)
	var pose GaitPose
	for n := range frames {
		if err := r.GaitPose(r.Animations[name].Gait, float64(n)/float64(len(frames)), &pose); err != nil {
			t.Fatal(err)
		}
		checkGait(t, name, r, &pose, r.World(pose.Angles, RideMatrix(pose.Dy, pose.Pitch, r.Scale), nil), &frames[n])
	}
}

// Jede Animation jeder Figur rechnet Go wie tools/paper: figur mit Posen, Keyframes, Sichtbarkeit und Umhang,
// vierbeiner mit seiner Gangart.
func TestAnimationsMatchPaper(t *testing.T) {
	for _, dir := range rigDirs(t) {
		r := loadRig(t, dir)
		for _, name := range animationNames(r) {
			t.Run(dir+"/"+name, func(t *testing.T) {
				switch r.Kind {
				case "figur":
					checkFigure(t, r, dir, name)
				case "vierbeiner":
					checkGaitAnimation(t, r, dir, name)
				default:
					t.Fatalf("Rig-Typ %s unbekannt", r.Kind)
				}
			})
		}
	}
}

// Jedes Reittier (Platz `sattel`) trägt den Monarchen in allen Gangarten, für die es Sollwerte gibt.
func TestRideMatchesPaper(t *testing.T) {
	rider, mounts := loadRig(t, riderDir), 0
	for _, dir := range rigDirs(t) {
		mount := loadRig(t, dir)
		if mount.Kind != "vierbeiner" || mount.Slot("sattel") == nil {
			continue
		}
		files, _ := filepath.Glob(assets(t, "soll", filepath.FromSlash(dir), "reiten-*.json"))
		if len(files) == 0 {
			t.Errorf("%s: keine Reit-Sollwerte", dir)
		}
		for _, file := range files {
			gaitName := strings.TrimPrefix(strings.TrimSuffix(filepath.Base(file), ".json"), "reiten-")
			t.Run(path.Join(dir, gaitName), func(t *testing.T) { checkRide(t, mount, rider, dir, gaitName) })
		}
		mounts++
	}
	if mounts == 0 {
		t.Fatal("kein Reittier mit Platz sattel")
	}
}

func checkRide(t *testing.T, mount, rider *Rig, dir, gaitName string) {
	frames := readExpected[rideFrame](t, dir, "reiten-"+gaitName)
	cloak, err := RideCloakFrames(mount, rider, gaitName, len(frames))
	if err != nil {
		t.Fatal(err)
	}
	var s Ride
	for n, f := range frames {
		if err := s.Set(mount, rider, gaitName, float64(n)/float64(len(frames)), Identity); err != nil {
			t.Fatal(err)
		}
		checkGait(t, gaitName, mount, &s.Pose, s.MountWorld, &f.gaitFrame)
		checkWorld(t, gaitName+" Reiter", rider, s.RiderWorld, f.RiderWorld)
		checkPoints(t, gaitName+" Umhang", cloak[n], f.Cloak)
		hand, _, mouth, ok := ReinPoints(mount, s.MountWorld, rider, s.RiderWorld, s.Base)
		if !ok {
			t.Fatal("Zügel fehlen (Plätze hand und maul)")
		}
		inv := s.Base.Invert()
		checkPoint(t, "Zügel Hand", inv.Apply(hand), f.Reins.Hand)
		checkPoint(t, "Zügel Maul", inv.Apply(mouth), f.Reins.Mouth)
	}
}
