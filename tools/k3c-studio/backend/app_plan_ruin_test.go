package backend

import (
	"encoding/json"
	"slices"
	"strings"
	"testing"

	"k3c/engine/level"
)

// WZ14.7 (B-468/AC-05): Der Editor setzt die Ruine je geplantem Chunk (Feld buildings). Schreiben byte-genau mit fester
// Schlüssel-Reihenfolge (slot, kind, seed, objects, buildings), Prüfung im Go-Generator, Arten aus data/ruins.json.

var ruinPlan = []level.PlannedChunk{{Slot: 3, Kind: "clearing", Objects: []level.PlannedObject{{Kind: "tree", At: 10}}, Buildings: []level.PlannedBuilding{{Kind: "smithy", At: 30}}}}

func TestPlanLoadNenntRuinenArten(t *testing.T) {
	app, _ := planApp(t, islandsFixture)
	view := app.PlanLoad()
	if !slices.Equal(view.RuinKinds, level.DefaultRuinRules().Kinds) || !slices.Contains(view.RuinKinds, "smithy") {
		t.Errorf("Ruinen-Arten %v", view.RuinKinds)
	}
}

func TestRuineSpeichernByteGenau(t *testing.T) {
	app, path := planApp(t, islandsFixture)
	saved := app.PlanSave("island1", 0, ruinPlan, 0, app.PlanLoad().Stand)
	if saved.Error != "" || saved.Hint != planRegHint {
		t.Fatalf("Speichern: %+v", saved)
	}
	content := readFile(t, path)
	want := `"plannedChunks": {"0": [{"slot":3,"kind":"clearing","objects":[{"kind":"tree","at":10}],"buildings":[{"kind":"smithy","at":30}]}]}`
	if !strings.Contains(content, want) {
		t.Fatalf("Datei:\n%s\nerwartet darin:\n%s", content, want)
	}
	if !strings.Contains(content, `{"id":"island2","name":"Zwei","depths":[0,1],"plannedChunks":{"1":[{"slot":3,"kind":"cavern"}]}}`) {
		t.Error("die zweite Insel hat sich verändert")
	}
	view := app.PlanLoad()
	if got := view.Islands[0].Planned["0"]; len(got) != 1 || !slices.Equal(got[0].Buildings, ruinPlan[0].Buildings) {
		t.Fatalf("gelesen: %+v", got)
	}
	// Rundreise ohne Änderung: die Datei bleibt byte-gleich.
	if s := app.PlanSave("island1", 0, view.Islands[0].Planned["0"], 0, view.Stand); s.Error != "" || readFile(t, path) != content {
		t.Errorf("Rundreise: %+v, Datei geändert %v", s, readFile(t, path) != content)
	}
	// Entfernen der Ruine schreibt den Chunk ohne buildings.
	plain := []level.PlannedChunk{{Slot: 3, Kind: "clearing", Objects: ruinPlan[0].Objects}}
	if s := app.PlanSave("island1", 0, plain, 0, app.PlanLoad().Stand); s.Error != "" || strings.Contains(readFile(t, path), "buildings") {
		t.Errorf("Ruine entfernen: %+v\n%s", s, readFile(t, path))
	}
}

func TestRuinePruefungNenntDenPlatz(t *testing.T) {
	app, path := planApp(t, islandsFixture)
	if v := app.PlanCheck("island1", 0, ruinPlan, 0); !v.OK {
		t.Fatalf("gültige Ruine abgelehnt: %+v", v)
	}
	bad := []level.PlannedChunk{{Slot: 3, Kind: "clearing", Buildings: []level.PlannedBuilding{{Kind: "tower", At: 30}}}}
	v := app.PlanCheck("island1", 0, bad, 0)
	if v.OK || v.Slot != 3 || v.Error == "" {
		t.Fatalf("Turm als Ruine: %+v", v)
	}
	before := readFile(t, path)
	if s := app.PlanSave("island1", 0, bad, 0, app.PlanLoad().Stand); s.Error == "" || readFile(t, path) != before {
		t.Errorf("ungültige Ruine gespeichert: %+v", s)
	}
}

// Die Live-Vorschau gibt die Ruine des Entwurfs weiter: ein Platz mit ruin an der geplanten Art.
func TestVorschauZeigtRuineDesEntwurfs(t *testing.T) {
	plan := ruinPlan
	view := (&App{}).WorldPreview(WorldPreviewArgs{Seed: "ruine-1", Source: WorldSource{Biome: "forest", Planned: &plan}})
	if view.Error != "" {
		t.Fatal(view.Error)
	}
	raw, err := json.Marshal(view.State["sites"])
	if err != nil {
		t.Fatal(err)
	}
	var sites []struct {
		Kind string `json:"kind"`
		Ruin bool   `json:"ruin"`
	}
	if err := json.Unmarshal(raw, &sites); err != nil {
		t.Fatal(err)
	}
	n := 0
	for _, s := range sites {
		if s.Ruin {
			n++
			if s.Kind != "smithy" {
				t.Errorf("Ruine %s, geplant smithy", s.Kind)
			}
		}
	}
	if n != 1 {
		t.Errorf("%d Ruinen in der Vorschau, erwartet 1", n)
	}
}
