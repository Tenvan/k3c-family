package props

import (
	"image/color"
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
	p.decor(w, view.Camera{}, 0)
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
	p.decor(w, camera(-10000), 0)
	if len(p.body) != 0 {
		t.Errorf("außerhalb der Welt %d Einträge", len(p.body))
	}
	p.decor(w, camera(200), 0)
	l, r := camera(200).Span()
	for _, it := range p.body {
		if it.shape == shapeTri && (it.x < l-3 || it.x > r+3) {
			t.Errorf("Tropfstein bei %v außerhalb %v…%v", it.x, l, r)
		}
	}
}

// countOf zählt die Einträge der Form s in der Farbe clr.
func countOf(items []item, s shape, clr color.RGBA) int {
	n := 0
	for _, it := range items {
		if it.shape == s && it.clr == clr {
			n++
		}
	}
	return n
}

// B-904/AC-01, AC-02: Balken nur mit decay, Wert 0 ohne Verfall, verfallene Rahmen (je drei Brocken) steigen streng.
func TestStuetzbalkenVerfallenNachWert(t *testing.T) {
	if n := countOf(planDecor(biomeWorld(t, "cave", nil)), shapeLine, beamWood); n != 0 {
		t.Errorf("Höhle mit %d Balken", n)
	}
	intact := planDecor(biomeWorld(t, "mine", map[string]int{"decay": 0}))
	if countOf(intact, shapeLine, beamWood) == 0 || countOf(intact, shapeRect, rubbleStone) != 0 {
		t.Error("Wert 0: Balken fehlen oder Verfall da")
	}
	prev := 0
	for v := 1; v <= 3; v++ {
		n := countOf(planDecor(biomeWorld(t, "mine", map[string]int{"decay": v})), shapeRect, rubbleStone) / 3
		if n <= prev {
			t.Errorf("Wert %d: %d verfallene Rahmen, vorher %d", v, n, prev)
		}
		prev = n
	}
}

// hazeAlpha ist die Deckkraft des Hitze-Schleiers (0 = keiner).
func hazeAlpha(items []item) float32 {
	for _, it := range items {
		if it.shape == shapeRect && it.clr == hazeColor {
			return it.alpha
		}
	}
	return 0
}

// B-905/AC-01, AC-02: Glut nur mit glow > 0, Anzahl steigt streng, Schleier wird deckender.
func TestGlutNachWert(t *testing.T) {
	for _, w := range []*sim.World{biomeWorld(t, "ironhold", map[string]int{"glow": 0}), biomeWorld(t, "mine", nil)} {
		items := planDecor(w)
		if countOf(items, shapeCircle, emberColor) != 0 || hazeAlpha(items) != 0 {
			t.Errorf("%s: Glut ohne Wert", w.Biome.ID)
		}
	}
	prevN, prevA := 0, float32(0)
	for v := 1; v <= 3; v++ {
		items := planDecor(biomeWorld(t, "ironhold", map[string]int{"glow": v}))
		n, a := countOf(items, shapeCircle, emberColor), hazeAlpha(items)
		if n <= prevN || a <= prevA {
			t.Errorf("Wert %d: %d Glutpunkte (vorher %d), Schleier %v (vorher %v)", v, n, prevN, a, prevA)
		}
		prevN, prevA = n, a
	}
}

// crystalWorld ist eine Kristallhöhle mit je einer Ader je Farbe (rot = unbekannt), einem Fels und Leuchtstärke v.
func crystalWorld(t *testing.T, v int) *sim.World {
	w := biomeWorld(t, "crystal", map[string]int{"brightness": v})
	for i, c := range []string{"blau", "violett", "gruen", "rot"} {
		w.Nodes = append(w.Nodes, &sim.ResourceNode{ID: i, Kind: "crystalVein", X: float64(20 + 10*i), Variant: c})
	}
	w.Nodes = append(w.Nodes, &sim.ResourceNode{ID: 9, Kind: "rock", X: 80})
	return w
}

// B-906/AC-01: je Kristall-Ader ein Schein in ihrer Farbe vor der Ader in der Zeichenliste; ohne brightness keiner.
func TestScheinJeKristallAder(t *testing.T) {
	p := newPlanner(nil) // ohne Grafik: Ader als Platzhalter-Kreis (volle Deckkraft, alpha 0)
	w := crystalWorld(t, 2)
	p.plan(w, nil, view.Camera{}, 0)
	for i, c := range []string{"blau", "violett", "gruen", "blau"} {
		x := float64(20 + 10*i)
		glow := slices.IndexFunc(p.body, func(it item) bool { return it.shape == shapeCircle && it.x == x && it.alpha > 0 })
		vein := slices.IndexFunc(p.body, func(it item) bool { return it.shape == shapeCircle && it.x == x && it.alpha == 0 })
		if glow < 0 || p.body[glow].clr != veinGlow[c] || vein < glow {
			t.Errorf("Ader %d (%s): Schein %d, Ader %d", i, c, glow, vein)
		}
	}
	if slices.ContainsFunc(p.body, func(it item) bool { return it.shape == shapeCircle && it.x == 80 && it.alpha > 0 }) {
		t.Error("Schein am Fels")
	}
	w.Biome.Traits = nil
	if n := len(planDecor(w)); n != 0 {
		t.Errorf("ohne brightness %d Einträge", n)
	}
}

// B-906/AC-02: Radius und Deckkraft des Scheins steigen mit dem Wert.
func TestScheinNachWert(t *testing.T) {
	prevR, prevA := 0.0, float32(0)
	for v := 1; v <= 3; v++ {
		items := planDecor(crystalWorld(t, v))
		i := slices.IndexFunc(items, func(it item) bool { return it.shape == shapeCircle })
		if i < 0 || items[i].w <= prevR || items[i].alpha <= prevA {
			t.Fatalf("Wert %d: Schein %+v", v, items)
		}
		prevR, prevA = items[i].w, items[i].alpha
	}
}
