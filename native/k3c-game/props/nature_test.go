package props

import (
	"slices"
	"testing"
)

// Fälle aus src/scenes/render/natureSprites.test.ts und plantation.test.ts; die Werte von mix stammen aus dem
// Browser (node, natureSprites.ts › mix), damit Baumform und Phase an derselben Stelle gleich sind.

func TestMixWieImBrowser(t *testing.T) {
	want := map[float64]uint32{0: 3413276431, 1.4: 3390830866, -0.5: 3413276431, -137.25: 2787311493, 412: 2386506617,
		99999.9: 2345423595, 7: 2896571549, 3.5: 2761611335}
	for x, v := range want {
		if got := mix(x); got != v {
			t.Errorf("mix(%v) = %d, Browser %d", x, got, v)
		}
	}
	phases := map[float64]int{0: 7, 1.4: 0, -0.5: 7, -137.25: 4, 412: 2, 99999.9: 6, 7: 7, 3.5: 0}
	for x, v := range phases {
		if got := animPhase(x, 8); got != v {
			t.Errorf("animPhase(%v) = %d, Browser %d", x, got, v)
		}
	}
	if animPhase(5, 1) != 0 {
		t.Error("ein Frame: Phase 0")
	}
}

func TestBaumUndFelsAusDerPosition(t *testing.T) {
	trees := map[float64]string{0: "pappel", 1.4: "pappel", -137.25: "tanne", 412: "birke", 99999.9: "tanne", 7: "birke"}
	for x, id := range trees {
		if got := treeLook(x, "forest"); got != object(id) {
			t.Errorf("Baum bei %v: %v, Browser %s", x, got, id)
		}
	}
	if treeLook(412.2, "forest") != treeLook(412.4, "forest") || treeLook(7, "") != treeLook(7, "forest") {
		t.Error("gerundet und leere Biom-ID wie Wald")
	}
	kinds := map[string]bool{}
	for i := range 200 {
		x := float64(i)*3.7 - 40
		kinds[treeLook(x, "forest").asset] = true
		if treeLook(x, "cave") != object("wurzel") || treeLook(x, "mine") != object("wurzel") {
			t.Fatalf("außerhalb des Walds Wurzeln bei %v", x)
		}
		kinds[rockLook(x).asset] = true
	}
	if len(kinds) != 6 {
		t.Errorf("drei Baumarten und drei Fels-Formen erwartet: %v", kinds)
	}
}

func TestDecoRootsWieImBrowser(t *testing.T) {
	cases := []struct {
		x    float64
		n    int
		want []float64
	}{{5, 3, []float64{0.5, 0.9}}, {-137.25, 2, []float64{0.5}}, {412, 4, []float64{0.4, -0.6, 0.6}}}
	for _, c := range cases {
		if got := decoRoots(nil, c.x, "cave", c.n); !slices.Equal(got, c.want) {
			t.Errorf("decoRoots(%v, %d) = %v, Browser %v", c.x, c.n, got, c.want)
		}
	}
	if decoRoots(nil, 5, "forest", 3) != nil || decoRoots(nil, 5, "mine", 1) != nil {
		t.Error("im Wald und mit einer Wurzel je Knoten keine Deko")
	}
	if rootsByBiom["cave"] != 2 || rootsByBiom["mine"] != 1 {
		t.Errorf("rootsPerNode aus data/biomes: %v", rootsByBiom)
	}
}

func TestNatureLookUndSchilder(t *testing.T) {
	for biome, want := range map[string]int{"forest": 1, "": 1, "cave": 2, "mine": 3, "ironhold": 3, "neu": 3} {
		if natureLook(biome) != want {
			t.Errorf("%q: %d", biome, natureLook(biome))
		}
	}
	signs := map[string]string{"tree": "schild-axt", "rock": "schild-spitzhacke", "copperOre": "schild-spitzhacke",
		"ironVein": "schild-spitzhacke", "bush": ""}
	for kind, id := range signs {
		if got := mineSignLook(kind); (id == "" && got.ok()) || (id != "" && got != object(id)) {
			t.Errorf("%s: %v", kind, got)
		}
	}
	if bushLook("forest") != object("busch") || bushLook("cave") != object("hoehlenbusch") {
		t.Error("Busch je Biom")
	}
}

func TestPlantagePlaetze(t *testing.T) {
	want := []float64{97, 98.2, 99.4, 100.6, 101.8, 103}
	for i, x := range want {
		if got := plotX(100, i, 6, 3); abs(got-x) > 1e-9 {
			t.Errorf("Platz %d: %v, erwartet %v", i, got, x)
		}
	}
	if !onPlot(plotX(100, 2, 6, 3), 100, 6, 3) || onPlot(100.2, 100, 6, 3) || plotX(50, 0, 1, 3) != 50 {
		t.Error("Baum auf dem Platz, Waldbaum im Streifen nicht")
	}
	pl := buildings["nursery"].Plantation
	if pl.Slots != 6 || pl.StripUnits != 3 {
		t.Errorf("Plantage aus data/buildings.json: %+v", pl)
	}
}

func abs(v float64) float64 {
	if v < 0 {
		return -v
	}
	return v
}
