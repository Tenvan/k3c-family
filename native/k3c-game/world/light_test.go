package world

import (
	"encoding/json"
	"os"
	"slices"
	"testing"

	"k3c/engine/sim"
)

func cycle(phase string, p float64) sim.CycleInfo { return sim.CycleInfo{Phase: phase, Progress: p} }

// testLight sind Lichtwerte wie data/light.json (eigene Zahlen, damit Datenänderungen die Fälle nicht ändern).
func testLight() lightData {
	d := lightData{
		Night: 0.2, Back: 0.05, Glow: "#ffffff",
		Sources: map[string]lightSource{"campfire": {Radius: 12, Strength: 1}, "torch": {Radius: 6, Strength: 0.8}},
		Torches: map[string][]float64{"wall": {0}, "tower": {-0.75, 0.75}},
	}
	d.prepare()
	return d
}

// AC-03 (Fälle aus daylight.test.ts und nightLight.test.ts): Helligkeit je Tageszeit.
func TestBrightnessByPhase(t *testing.T) {
	d := testLight()
	cases := []struct {
		c           sim.CycleInfo
		front, back float64
	}{
		{cycle("day", 0.5), 1, 1},
		{cycle("dusk", 0.5), 0.6, 0.525},
		{cycle("night", 0.95), 0.2, 0.05},
		{cycle("dawn", 0), 0.2, 0.05},
		{cycle("dawn", 0.5), 0.6, 0.525},
		{cycle("eclipse", 0.5), 0.2, 0.05},
	}
	for _, tc := range cases {
		if f, b := d.front(tc.c), d.backdrop(tc.c); !near(f, tc.front) || !near(b, tc.back) {
			t.Errorf("%s %.2f: Vordergrund %v Hintergrund %v, erwartet %v / %v", tc.c.Phase, tc.c.Progress, f, b, tc.front, tc.back)
		}
	}
	if !near(daylight(cycle("dusk", 0.25)), 0.75) {
		t.Error("Dämmerung dunkelt nicht linear ab")
	}
}

// Lichtkarte: außen Nacht-Helligkeit, an der Burg (Lagerfeuer) voll, dazwischen linear, am Tag überall 1;
// eine ausgebaute Burg gibt kein Licht.
func TestLightLevel(t *testing.T) {
	d := testLight()
	w := &sim.World{Castle: sim.Castle{X: 10}, HubX: 10, HubLevel: 1, Cycle: cycle("night", 0)}
	cases := []struct{ x, want float64 }{{40, 0.2}, {10, 1}, {16, 0.6}}
	for _, tc := range cases {
		if got := d.levelAt(tc.x, w); !near(got, tc.want) {
			t.Errorf("x=%v: %v, erwartet %v", tc.x, got, tc.want)
		}
	}
	w.HubLevel = 2
	if got := d.levelAt(10, w); !near(got, 0.2) {
		t.Errorf("Burg leuchtet: %v", got)
	}
	w.HubLevel, w.Cycle = 1, cycle("day", 0)
	if got := d.levelAt(40, w); got != 1 {
		t.Errorf("Tag: %v", got)
	}
}

// Deckkraft des Scheins: am Tag 0, nachts die Stärke, im Morgengrauen fallend, begrenzt auf 0 … 1.
func TestGlowAlpha(t *testing.T) {
	if glowAlpha(0.7, cycle("day", 0.3)) != 0 || !near(glowAlpha(0.7, cycle("night", 0.3)), 0.7) {
		t.Error("Tag/Nacht")
	}
	if a, b := glowAlpha(0.7, cycle("dawn", 0)), glowAlpha(0.7, cycle("dawn", 0.5)); a <= b {
		t.Errorf("Morgengrauen steigt: %v → %v", a, b)
	}
	if glowAlpha(3, cycle("night", 0)) != 1 || glowAlpha(-1, cycle("night", 0)) != 0 {
		t.Error("Begrenzung")
	}
}

// Fackeln stehen wie in der Simulation (testdata/torches.json): Jeder Ort leuchtet, gebaute Plätze ohne Fackel nicht.
func TestTorchesLikeSim(t *testing.T) {
	raw, err := os.ReadFile("../../../testdata/torches.json")
	if err != nil {
		t.Fatal(err)
	}
	var tc struct {
		HubX    float64              `json:"hubX"`
		Offsets map[string][]float64 `json:"offsets"`
		Sites   []*sim.Site          `json:"sites"`
		Torches []float64            `json:"torches"`
	}
	if err := json.Unmarshal(raw, &tc); err != nil {
		t.Fatal(err)
	}
	d := testLight()
	d.Torches = tc.Offsets
	w := &sim.World{HubX: tc.HubX, HubLevel: 2, Sites: tc.Sites, Cycle: cycle("night", 0)}
	for _, x := range tc.Torches {
		if got := d.levelAt(x, w); got <= d.front(w.Cycle) {
			t.Errorf("Fackel bei %v leuchtet nicht (%v)", x, got)
		}
	}
	if got := torchXsOf(&d, w); !slices.Equal(got, tc.Torches) {
		t.Errorf("Fackeln %v, erwartet %v", got, tc.Torches)
	}
	real := loadLight()
	if got, want := torchXsOf(&real, w), sim.TorchXs(w); !slices.Equal(got, want) {
		t.Errorf("mit data/light.json: %v, Simulation %v", got, want)
	}
}

// torchXsOf sammelt die Fackel-Orte wie drawTorches (nur für den Test).
func torchXsOf(d *lightData, w *sim.World) []float64 {
	out := []float64{}
	for _, s := range w.Sites {
		if s.State != "built" {
			continue
		}
		for _, o := range d.Torches[s.Kind] {
			out = append(out, s.X+torchSide(s.X, w.HubX)*o)
		}
	}
	return out
}

// Lichtfarbe: Schein in glowColor, nie über 1; Rechnen ohne Allokation; data/light.json ist lesbar.
func TestLightColorAndData(t *testing.T) {
	d := testLight()
	d.Glow = "#ff8000"
	d.prepare()
	w := &sim.World{Castle: sim.Castle{X: 10}, HubLevel: 1, Cycle: cycle("night", 0)}
	r, g, b := d.colorAt(10, w)
	if r != 1 || !near(g, 0.2+0.8*128.0/255) || !near(b, 0.2) {
		t.Errorf("Farbe %v %v %v", r, g, b)
	}
	if n := testing.AllocsPerRun(100, func() { _, _, _ = d.colorAt(12, w) }); n != 0 {
		t.Errorf("colorAt legt %v an", n)
	}
	real := loadLight()
	if real.Night <= 0 || real.Night >= 1 || !real.campfire.valid() || !real.torch.valid() {
		t.Errorf("data/light.json: %+v", real)
	}
}
