package studio

import (
	"encoding/json"
	"reflect"
	"strings"
	"testing"

	"k3c/data"
	"k3c/engine/level"
)

// WZ7.2 (B-460/AC-01): Die Chunk-Vorschau liefert das Level des Spiels ohne Spielserver.

func TestWorldLevelIstDasLevelDesSpiels(t *testing.T) {
	view := (&App{}).WorldLevel("abc", WorldSource{Biome: "forest"})
	if view.Error != "" {
		t.Fatal(view.Error)
	}
	biome, err := level.LoadBiome("forest")
	if err != nil {
		t.Fatal(err)
	}
	want, err := level.Generate(biome, "abc")
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(view.Layout, want) {
		t.Error("Layout weicht von level.Generate(biome, seed) ab")
	}
	if len(view.Chunks) == 0 || len(view.Entities) == 0 {
		t.Errorf("Layout leer: %d Abschnitte, %d Objekte", len(view.Chunks), len(view.Entities))
	}
}

// Für Seed k3c und jedes eingebettete Biom dasselbe Layout und dieselben Warnungen wie level.Generate/level.Validate.
func TestWorldLevelJedesBiomWieGenerateUndValidate(t *testing.T) {
	files, err := data.Files.ReadDir("biomes")
	if err != nil || len(files) < 5 {
		t.Fatalf("Biome: %d Dateien, %v", len(files), err)
	}
	a := &App{}
	for _, f := range files {
		id := strings.TrimSuffix(f.Name(), ".json")
		biome, err := level.LoadBiome(id)
		if err != nil {
			t.Fatal(err)
		}
		layout, err := level.Generate(biome, "k3c")
		if err != nil {
			t.Fatal(err)
		}
		warnings := level.Validate(layout, biome)
		if warnings == nil {
			warnings = []string{}
		}
		first, second := a.WorldLevel("k3c", WorldSource{Biome: id}), a.WorldLevel("k3c", WorldSource{Biome: id})
		if first.Error != "" || !reflect.DeepEqual(first.Layout, layout) || !reflect.DeepEqual(first.Warnings, warnings) {
			t.Errorf("%s: Layout oder Warnungen weichen von Generate/Validate ab (Error %q)", id, first.Error)
		}
		if !reflect.DeepEqual(first, second) {
			t.Errorf("%s: zwei Aufrufe liefern verschiedene Level", id)
		}
	}
}

func TestWorldLevelJSONHatDieFelderDerSeite(t *testing.T) {
	raw, err := json.Marshal((&App{}).WorldLevel("abc", WorldSource{Biome: "forest"}))
	if err != nil {
		t.Fatal(err)
	}
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatal(err)
	}
	for _, key := range []string{"seed", "biomeId", "widthUnits", "chunkWidthUnits", "hubCenterUnits", "chunks", "entities", "warnings", "error"} {
		if _, has := m[key]; !has {
			t.Errorf("Feld %q fehlt im JSON %s", key, raw)
		}
	}
	if _, ok := m["warnings"].([]any); !ok {
		t.Errorf("warnings ist %v, erwartet ein Array (nie null)", m["warnings"])
	}
}

func TestWorldLevelLehntUngueltigesAb(t *testing.T) {
	a := &App{}
	cases := map[string]struct {
		seed string
		src  WorldSource
	}{
		"Seed leer":        {"", WorldSource{Biome: "forest"}},
		"Seed zu lang":     {strings.Repeat("a", 65), WorldSource{Biome: "forest"}},
		"Biom unbekannt":   {"abc", WorldSource{Biome: "nirgendwo"}},
		"Biom mit Pfad":    {"abc", WorldSource{Biome: "../hub"}},
		"Quelle fehlt":     {"abc", WorldSource{}},
		"Insel noch nicht": {"abc", WorldSource{Island: "k3c", Stage: 1}},
		"Insel mit Biom":   {"abc", WorldSource{Biome: "forest", Island: "k3c"}},
	}
	for name, c := range cases {
		view := a.WorldLevel(c.seed, c.src)
		if view.Error == "" || len(view.Chunks) != 0 || view.Warnings == nil {
			t.Errorf("%s: Error %q, %d Abschnitte, Warnungen %v", name, view.Error, len(view.Chunks), view.Warnings)
		}
	}
}
