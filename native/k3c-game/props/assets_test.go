package props

import (
	"encoding/json"
	"slices"
	"testing"

	"k3c/data"
	"k3c/native/k3c-game/art"
)

// AC-03: Jeder Natur- und Objekt-Typ aus data/ hat eine Grafik, und jede Grafik, die props wählen kann, liegt als
// Manifest mit der Animation unter public/grafik (gerendert von `task gen:paper`).

// biome liest die Arten eines Bioms aus data/biomes (Ressourcen je Chunk, Ereignis-Chunks, Adern).
type biome struct {
	ID                string
	ResourcesPerChunk map[string]map[string][2]int
	EventChunks       map[string]any
	Veins             struct{ Kind string }
}

func biomes(t *testing.T) []biome {
	t.Helper()
	entries, err := data.Files.ReadDir("biomes")
	if err != nil {
		t.Fatal(err)
	}
	var out []biome
	for _, e := range entries {
		raw, err := data.Files.ReadFile("biomes/" + e.Name())
		if err != nil || slices.Contains([]string{"codemap.md"}, e.Name()) {
			continue
		}
		var b biome
		if err := json.Unmarshal(raw, &b); err != nil {
			t.Fatalf("%s: %v", e.Name(), err)
		}
		out = append(out, b)
	}
	return out
}

// nodeKinds sind die Knoten-Arten eines Bioms: Sammel-Knoten und Adern aus data/economy.json, Ressourcen des Bioms.
func nodeKinds(t *testing.T, b biome) []string {
	t.Helper()
	var eco struct{ Gatherables, Veins map[string]any }
	raw, _ := data.Files.ReadFile("economy.json")
	if err := json.Unmarshal(raw, &eco); err != nil {
		t.Fatal(err)
	}
	var kinds []string
	for k := range eco.Gatherables {
		kinds = append(kinds, k)
	}
	for k := range eco.Veins {
		kinds = append(kinds, k)
	}
	for _, res := range b.ResourcesPerChunk {
		for k := range res {
			kinds = append(kinds, k)
		}
	}
	return kinds
}

func TestJederTypAusDenDatenHatEineGrafik(t *testing.T) {
	for _, b := range biomes(t) {
		for _, k := range nodeKinds(t, b) {
			if !nodeLook(k, 3, b.ID).ok() {
				t.Errorf("%s: Knoten %s ohne Grafik", b.ID, k)
			}
		}
		for k := range b.EventChunks {
			_, _, entity := entityLook(k, b.ID)
			if pk, _ := pickupLook(k); !entity && !pk.ok() {
				t.Errorf("%s: Ereignis %s ohne Grafik", b.ID, k)
			}
		}
	}
	for _, k := range []string{"portal", "exit", "harbor", "recruitCamp", "bush"} {
		if _, _, ok := entityLook(k, "forest"); !ok {
			t.Errorf("Entität %s ohne Grafik", k)
		}
	}
	for _, k := range []string{"chest", "skillPoint"} {
		if l, _ := pickupLook(k); !l.ok() {
			t.Errorf("Pickup %s ohne Grafik", k)
		}
	}
}

// chosen sind alle Grafiken, die die Auswahl-Regeln in einem Biom liefern können.
func chosen(biome string) []look {
	out := []look{coinLook, chestOpenLook, seedlingLook, plantTreeLook, hubScaffold}
	for kind := range siteSize {
		out = append(out, scaffoldLook(kind))
		for s := 1; s <= 5; s++ {
			out = append(out, siteLook(kind, "built", s))
		}
	}
	for lv := 1; lv <= 5; lv++ {
		out = append(out, backWallLook(lv))
		for step := 1; step <= 3; step++ {
			h := hubLookOf(lv, step, "")
			out = append(out, h.main, h.overlay)
		}
	}
	for _, kind := range []string{"tree", "rock", "copperOre", "bush", "stoneVein", "copperVein", "ironVein", "crystalVein"} {
		out = append(out, mineSignLook(kind))
		for x := range 60 {
			out = append(out, nodeLook(kind, float64(x), biome))
		}
	}
	for _, k := range []string{"portal", "exit", "harbor", "recruitCamp", "chest", "skillPoint"} {
		l, _, _ := entityLook(k, biome)
		p, _ := pickupLook(k)
		out = append(out, l, p)
	}
	return out
}

func TestJedeGewaehlteGrafikLiegtVorUndIstGemeldet(t *testing.T) {
	for _, biome := range []string{"forest", "cave", "mine", "ironhold", "crystal"} {
		listed := levelAssets(biome)
		for _, l := range chosen(biome) {
			if !l.ok() {
				continue
			}
			if !slices.Contains(listed, l.asset) {
				t.Errorf("%s: %s nicht in Assets gemeldet", biome, l.asset)
			}
			m, err := art.ReadManifest(grafikDir, l.asset)
			if err != nil {
				t.Errorf("%s: %v", biome, err)
				continue
			}
			if _, ok := m.Anims[l.anim]; !ok {
				t.Errorf("%s: Animation %s fehlt in %s", biome, l.anim, l.asset)
			}
		}
		if _, err := art.PlanLevel(grafikDir, biome, listed); err != nil {
			t.Errorf("%s: Level-Plan: %v", biome, err)
		}
	}
}

func TestAssetsJeLevelOhneDoppelte(t *testing.T) {
	wald, hoehle := levelAssets("forest"), levelAssets("cave")
	if !slices.IsSorted(wald) || len(slices.Compact(slices.Clone(wald))) != len(wald) {
		t.Error("sortiert, jedes einmal")
	}
	if !slices.Contains(wald, "objekte/tanne") || slices.Contains(hoehle, "objekte/tanne") || !slices.Contains(hoehle, "objekte/wurzel") {
		t.Error("Natur nach dem Look des Bioms")
	}
	var d drawer
	if got := d.Assets("forest", nil); !slices.Equal(got, wald) {
		t.Error("AssetSource liefert levelAssets")
	}
}
