package level

import (
	"encoding/json"
	"reflect"
	"testing"
)

// natureCounts zählt Bäume und Felsen und die Bäume je Baumart.
func natureCounts(l Layout) (trees, rocks int, species map[string]int) {
	species = map[string]int{}
	for _, e := range l.Entities {
		switch e.Kind {
		case "tree":
			trees++
			species[e.Variant]++
		case "rock":
			rocks++
		}
	}
	return trees, rocks, species
}

func generate(t *testing.T, b Biome, seed string, fixed NatureValues) Layout {
	t.Helper()
	l, err := GenerateWith(b, seed, Options{Nature: fixed})
	if err != nil {
		t.Fatal(err)
	}
	return l
}

// AC-01: gleicher Seed, gleiche Werte und gleiche Natur; die Werte liegen in den Bereichen des Bioms.
func TestNaturGleicherSeedGleicheNatur(t *testing.T) {
	b := biome(t, "forest")
	a, c := generate(t, b, "eiche", NatureValues{}), generate(t, b, "eiche", NatureValues{})
	if !reflect.DeepEqual(a.Nature, c.Nature) || !reflect.DeepEqual(a.Entities, c.Entities) {
		t.Fatal("gleicher Seed ergibt andere Natur")
	}
	for _, e := range b.Traits {
		if v := a.Nature.Traits[e.Key]; v < e.Value.Min || v > e.Value.Max {
			t.Errorf("%s = %d außerhalb %d–%d", e.Key, v, e.Value.Min, e.Value.Max)
		}
	}
	for _, e := range a.Entities {
		if (e.Kind == "tree") != (e.Variant != "") {
			t.Fatalf("Variante falsch gesetzt: %+v", e)
		}
	}
}

// AC-02: Zwei Seeds unterscheiden sich im selben Biom in Walddichte oder Mischung.
func TestNaturSeedsUnterscheidenSich(t *testing.T) {
	b := biome(t, "forest")
	first := generate(t, b, "seed-0", NatureValues{}).Nature
	for _, seed := range []string{"seed-1", "seed-2", "seed-3", "seed-4"} {
		if !reflect.DeepEqual(generate(t, b, seed, NatureValues{}).Nature, first) {
			return
		}
	}
	t.Fatal("fünf Seeds ergeben dieselben Natur-Werte")
}

// AC-02: Die Walddichte ändert die Zahl der Bäume messbar (1 gegen 5).
func TestNaturWalddichteSkaliertBaeume(t *testing.T) {
	b := biome(t, "forest")
	low, _, _ := natureCounts(generate(t, b, "dichte", NatureValues{Traits: map[string]int{"forestDensity": 1}}))
	high, _, _ := natureCounts(generate(t, b, "dichte", NatureValues{Traits: map[string]int{"forestDensity": 5}}))
	if high < 3*low {
		t.Fatalf("Walddichte 5 ergibt %d Bäume, Walddichte 1 %d: erwartet etwa Faktor 5", high, low)
	}
	_, rocks, _ := natureCounts(generate(t, b, "dichte", NatureValues{Traits: map[string]int{"rockDensity": 0}}))
	if rocks != 0 {
		t.Fatalf("Steindichte 0 ergibt %d Felsen", rocks)
	}
}

// AC-03: Ein fest gesetzter Wert gilt unabhängig vom Seed.
func TestNaturFesterWertGilt(t *testing.T) {
	b := biome(t, "forest")
	fixed := NatureValues{Traits: map[string]int{"forestDensity": 4}, Weights: map[string]map[string]int{"treeSpecies": {"birke": 2}}}
	for _, seed := range []string{"a", "b", "c", "d"} {
		n := generate(t, b, seed, fixed).Nature
		if n.Traits["forestDensity"] != 4 || n.Weights["treeSpecies"]["birke"] != 2 {
			t.Fatalf("Seed %s: fester Wert überschrieben: %+v", seed, n)
		}
	}
}

// AC-04: Monokultur erzeugt nur die eine Art.
func TestNaturMonokultur(t *testing.T) {
	b := biome(t, "forest")
	mono := NatureValues{Weights: map[string]map[string]int{"treeSpecies": {"tanne": 0, "pappel": 3, "birke": 0}}}
	trees, _, species := natureCounts(generate(t, b, "mono", mono))
	if trees == 0 || species["pappel"] != trees {
		t.Fatalf("Monokultur Pappel: %d Bäume, Arten %v", trees, species)
	}
}

// AC-04: Alle Baumarten fest auf 0 lehnt die Prüfung ab; ebenso ein Wert außerhalb des Bereichs und ein unbekannter Schlüssel.
func TestNaturFesteWerteAbgelehnt(t *testing.T) {
	b := biome(t, "forest")
	for name, v := range map[string]NatureValues{
		"alle 0":    {Weights: map[string]map[string]int{"treeSpecies": {"tanne": 0, "pappel": 0, "birke": 0}}},
		"Bereich":   {Traits: map[string]int{"forestDensity": 9}},
		"unbekannt": {Traits: map[string]int{"pilze": 1}},
		"Art":       {Weights: map[string]map[string]int{"treeSpecies": {"eiche": 3}}},
		"Gruppe":    {Weights: map[string]map[string]int{"pilze": {"a": 1}}},
	} {
		if _, err := GenerateWith(b, "x", Options{Nature: v}); err == nil {
			t.Errorf("%s: nicht abgelehnt", name)
		}
	}
}

// Datenprüfung des Bioms: alle Gewichte 0, Bereich verkehrt, Skalierung einer fremden Art.
func TestNaturDatenpruefung(t *testing.T) {
	tr := func(body string) string { return `{"traits": {"d": {"name": "D", ` + body + `}}}` }
	for name, raw := range map[string]string{
		"Gewichte 0":     `{"weights": {"g": {"name": "G", "kind": "tree", "options": {"a": {"min": 0, "max": 0}}}}}`,
		"Gewicht 6":      `{"weights": {"g": {"name": "G", "kind": "tree", "options": {"a": {"min": 0, "max": 6}}}}}`,
		"fremde Art":     `{"weights": {"g": {"name": "G", "kind": "gold", "options": {"a": {"min": 0, "max": 2}}}}}`,
		"ohne Varianten": `{"weights": {"g": {"name": "G", "kind": "tree", "options": {}}}}`,
		"Bereich":        tr(`"min": 3, "max": 1, "default": 2, "scales": ["tree"]`),
		"default 0":      tr(`"min": 0, "max": 1, "default": 0, "scales": ["tree"]`),
		"fremde Res":     tr(`"min": 1, "max": 3, "default": 2, "scales": ["gold"]`),
		"fremder Chunk":  tr(`"min": 1, "max": 3, "default": 2, "chunks": ["moor"]`),
		"zwei Wirkungen": tr(`"min": 1, "max": 3, "default": 2, "scales": ["tree"], "display": true`),
		"ohne Wirkung":   tr(`"min": 1, "max": 3, "default": 2`),
		"ohne Name":      `{"traits": {"d": {"min": 1, "max": 3, "default": 2, "scales": ["tree"]}}}`,
		"doppelt":        `{"traits": {"d": {"name": "D", "min": 1, "max": 3, "default": 2, "scales": ["tree"]}, "e": {"name": "E", "min": 1, "max": 3, "default": 2, "scales": ["tree"]}}}`,
	} {
		b := Biome{ID: "test"}
		if err := json.Unmarshal([]byte(`{"resourcesPerChunk": {"a": {"tree": [1, 2]}}, "chunkWeights": {"a": 1}}`), &b); err != nil {
			t.Fatal(err)
		}
		if err := json.Unmarshal([]byte(raw), &b); err != nil {
			t.Fatal(err)
		}
		if err := b.checkNature(); err == nil {
			t.Errorf("%s: nicht abgelehnt", name)
		}
	}
	for _, id := range biomeIDs(t) {
		if err := biome(t, id).checkNature(); err != nil {
			t.Errorf("%s: %v", id, err)
		}
	}
}

// Ohne Eigenschaften bleibt das Level wie zuvor: neutrale Werte ändern keinen Wurf.
func TestNaturNeutralerWertAendertNichts(t *testing.T) {
	b := biome(t, "forest")
	neutral := NatureValues{Traits: map[string]int{}}
	for _, e := range b.Traits {
		neutral.Traits[e.Key] = e.Value.Default
	}
	plain := b
	plain.Traits, plain.Weights = nil, nil
	want, got := generate(t, plain, "neutral", NatureValues{}), generate(t, b, "neutral", neutral)
	if len(want.Entities) != len(got.Entities) {
		t.Fatalf("neutrale Werte ändern das Level: %d gegen %d Entitäten", len(want.Entities), len(got.Entities))
	}
	for i := range want.Entities {
		if want.Entities[i].Kind != got.Entities[i].Kind || want.Entities[i].X != got.Entities[i].X {
			t.Fatalf("Entität %d weicht ab: %+v gegen %+v", i, want.Entities[i], got.Entities[i])
		}
	}
}

func TestNaturScaleCountRundet(t *testing.T) {
	for _, c := range []struct{ n, v, def, want int }{{22, 3, 3, 22}, {22, 1, 3, 7}, {30, 5, 3, 50}, {3, 0, 2, 0}, {1, 1, 2, 1}} {
		if got := scaleCount(c.n, c.v, c.def); got != c.want {
			t.Errorf("scaleCount(%d, %d, %d) = %d, erwartet %d", c.n, c.v, c.def, got, c.want)
		}
	}
	if MergeNature(NatureValues{Traits: map[string]int{"a": 1}}, NatureValues{Traits: map[string]int{"a": 2}}).Traits["a"] != 2 {
		t.Error("MergeNature: over gewinnt nicht")
	}
}

// Chunk-Eigenschaften verschieben die Chunk-Arten: Lichtungen 0 und Bachläufe 0 lassen keine dieser Chunks übrig.
func TestNaturChunkGewichte(t *testing.T) {
	b := biome(t, "forest")
	l := generate(t, b, "chunks", NatureValues{Traits: map[string]int{"clearings": 0, "streams": 0}})
	for _, c := range l.Chunks {
		if c.Kind == "clearing" || c.Kind == "stream" {
			t.Fatalf("Chunk %d ist %s trotz Gewicht 0", c.Index, c.Kind)
		}
	}
	// Alle Chunk-Gewichte 0 (Kristallhöhle): Die Gewichte des Bioms bleiben, das Level entsteht.
	generate(t, biome(t, "crystal"), "chunks", NatureValues{Traits: map[string]int{"geodes": 0, "grottoShare": 0}})
}

// Adern-Eigenschaften ändern die Zahl der Adern, die Prüfung zählt mit den Natur-Werten der Stufe.
func TestNaturAdern(t *testing.T) {
	b := biome(t, "cave")
	for v, want := range map[int]int{1: 1, 2: 2, 3: 3} {
		l := generate(t, b, "adern", NatureValues{Traits: map[string]int{"stoneVeins": v}})
		if n := countEntities(l, "stoneVein"); n != want {
			t.Errorf("Steinadern %d: %d Adern, erwartet %d", v, n, want)
		}
		if errs := Validate(l, b); len(errs) > 0 {
			t.Errorf("Steinadern %d: %v", v, errs)
		}
	}
}

// Kristallfarben: Gewichts-Gruppe auf Adern, jede Ader bekommt eine Farbe.
func TestNaturKristallfarben(t *testing.T) {
	l := generate(t, biome(t, "crystal"), "farben", NatureValues{Weights: map[string]map[string]int{"crystalColors": {"blau": 0, "violett": 4, "gruen": 0}}})
	for _, e := range l.Entities {
		if e.Kind == "crystalVein" && e.Variant != "violett" {
			t.Fatalf("Ader %+v ist nicht violett", e)
		}
	}
}
