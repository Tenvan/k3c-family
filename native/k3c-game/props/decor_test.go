package props

import (
	"slices"
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// biomeWorld baut eine Stufe des Bioms id (Daten aus data/biomes) mit 400 Units Breite und den Werten traits.
func biomeWorld(t *testing.T, id string, traits map[string]int) *sim.World {
	t.Helper()
	b, err := level.LoadBiome(id)
	if err != nil {
		t.Fatal(err)
	}
	w := &sim.World{Biome: b, WidthUnits: 400}
	for k, v := range traits {
		w.Level.Nature.SetTrait(k, v)
	}
	return w
}

// planDecor plant nur die Darstellung der Natur-Eigenschaften (ganze Welt).
func planDecor(w *sim.World) []item {
	p := newPlanner(nil)
	p.decor(w, view.Camera{})
	return p.body
}

// ceilingY ist die Unterkante des Decken-Rechtecks; ok = es gibt eins.
func ceilingY(items []item) (float64, bool) {
	for _, it := range items {
		if it.shape == shapeRect && it.clr == ceilingRock {
			return it.y, true
		}
	}
	return 0, false
}

// B-902/AC-01: Decke nur in Biomen mit ceilingHeight.
func TestDeckeNurMitEigenschaft(t *testing.T) {
	if _, ok := ceilingY(planDecor(biomeWorld(t, "cave", nil))); !ok {
		t.Error("Höhle ohne Decke")
	}
	for _, id := range []string{"forest", "mine", "ironhold", "crystal"} {
		if _, ok := ceilingY(planDecor(biomeWorld(t, id, nil))); ok {
			t.Errorf("%s mit Decke", id)
		}
	}
}

// B-902/AC-02: Die Unterkante steigt mit dem Wert; ohne Wert gilt der Default (2).
func TestDeckeFolgtDemWert(t *testing.T) {
	var ys []float64
	for v := 1; v <= 3; v++ {
		y, _ := ceilingY(planDecor(biomeWorld(t, "cave", map[string]int{"ceilingHeight": v})))
		ys = append(ys, y)
	}
	if !(ys[0] < ys[1] && ys[1] < ys[2]) || ys[2] >= viewTop {
		t.Errorf("Unterkanten %v", ys)
	}
	if y, _ := ceilingY(planDecor(biomeWorld(t, "cave", nil))); y != ys[1] {
		t.Errorf("ohne Wert %v, Default %v", y, ys[1])
	}
	if y, _ := ceilingY(planDecor(biomeWorld(t, "cave", map[string]int{"ceilingHeight": 9}))); y != ys[2] {
		t.Errorf("Wert über dem Bereich %v, will %v", y, ys[2])
	}
}

// stalXs sind die Orte der Tropfsteine.
func stalXs(items []item) []float64 {
	var out []float64
	for _, it := range items {
		if it.shape == shapeTri && it.clr == stalColor {
			out = append(out, it.x)
		}
	}
	return out
}

// B-903/AC-01, AC-02: Wert 0 und Biome ohne Eigenschaft ohne Tropfsteine; Anzahl steigt streng, Orte bleiben.
func TestTropfsteineFolgenDemWert(t *testing.T) {
	if n := len(stalXs(planDecor(biomeWorld(t, "cave", map[string]int{"stalactites": 0})))); n != 0 {
		t.Errorf("Wert 0: %d Tropfsteine", n)
	}
	if n := len(stalXs(planDecor(biomeWorld(t, "mine", nil)))); n != 0 {
		t.Errorf("Mine: %d Tropfsteine", n)
	}
	var prev []float64
	for v := 1; v <= 3; v++ {
		xs := stalXs(planDecor(biomeWorld(t, "cave", map[string]int{"stalactites": v})))
		if len(xs) <= len(prev) {
			t.Errorf("Wert %d: %d nicht mehr als %d", v, len(xs), len(prev))
		}
		for _, x := range prev {
			if !slices.Contains(xs, x) {
				t.Errorf("Wert %d: Tropfstein bei %v fehlt", v, x)
			}
		}
		if again := stalXs(planDecor(biomeWorld(t, "cave", map[string]int{"stalactites": v}))); !slices.Equal(xs, again) {
			t.Errorf("Wert %d nicht gleich", v)
		}
		prev = xs
	}
}

// Die Darstellung bleibt im sichtbaren Teil der Welt (TestPlanNurSichtbares bleibt leer).
func TestDekoNurSichtbar(t *testing.T) {
	w := biomeWorld(t, "cave", map[string]int{"stalactites": 3})
	p := newPlanner(nil)
	p.decor(w, camera(-10000))
	if len(p.body) != 0 {
		t.Errorf("außerhalb der Welt %d Einträge", len(p.body))
	}
	p.decor(w, camera(200))
	l, r := camera(200).Span()
	for _, it := range p.body {
		if it.shape == shapeTri && (it.x < l-3 || it.x > r+3) {
			t.Errorf("Tropfstein bei %v außerhalb %v…%v", it.x, l, r)
		}
	}
}
