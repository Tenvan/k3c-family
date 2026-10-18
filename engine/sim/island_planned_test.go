package sim

import (
	"strings"
	"testing"
	"testing/fstest"
)

func islandsWithPlan(plan string) fstest.MapFS {
	raw := `{"islands":[{"id":"a","depths":[0,1,2,3,4],"depthScaling":{"hp":1,"damage":1,"speed":1},"plannedChunks":` + plan + `}]}`
	return fstest.MapFS{"islands.json": {Data: []byte(raw)}}
}

func TestInselPlanungWirdBeimLadenGeprueft(t *testing.T) {
	good := `{"0":[{"slot":-3,"kind":"clearing","seed":"lichtung"}],"1":[{"slot":3,"kind":"cavern","objects":[{"kind":"stoneVein","at":40}]}]}`
	var dst []IslandDef
	if err := loadIslands(islandsWithPlan(good), &dst); err != nil {
		t.Fatalf("gültige Planung abgelehnt: %v", err)
	}
	if len(dst[0].PlannedChunks["0"]) != 1 || dst[0].PlannedChunks["0"][0].Seed != "lichtung" {
		t.Errorf("Planung nicht geladen: %+v", dst[0].PlannedChunks)
	}
	bad := map[string]string{
		"Tiefe unbekannt":   `{"7":[{"slot":1,"kind":"forest"}]}`,
		"Tiefe kein Wert":   `{"x":[{"slot":1,"kind":"forest"}]}`,
		"Tiefe 00":          `{"00":[{"slot":1,"kind":"forest"}]}`,
		"Feld unbekannt":    `{"0":[{"slot":1,"kind":"forest","object":[]}]}`,
		"Platz 0":           `{"0":[{"slot":0,"kind":"forest"}]}`,
		"Art unbekannt":     `{"0":[{"slot":1,"kind":"volcano"}]}`,
		"Chunk der Höhle":   `{"0":[{"slot":1,"kind":"cavern"}]}`,
		"Teil 2 reserviert": `{"0":[{"slot":1,"kind":"forest","buildings":[{"kind":"smithy"}]}]}`,
	}
	for name, plan := range bad {
		var out []IslandDef
		err := loadIslands(islandsWithPlan(plan), &out)
		if err == nil || out != nil || !strings.Contains(err.Error(), "data/islands.json") {
			t.Errorf("%s: Fehler %v, Ergebnis %v", name, err, out)
		}
	}
}

func TestInselPlanungLeerIstKeinePlanung(t *testing.T) {
	for _, plan := range []string{`{}`, `null`, `{"0":[]}`} {
		var dst []IslandDef
		if err := loadIslands(islandsWithPlan(plan), &dst); err != nil {
			t.Errorf("%s: %v", plan, err)
		}
	}
	for _, d := range islandDefs {
		if len(d.PlannedChunks) != 0 {
			t.Errorf("%s: eingebettete Daten enthalten eine Planung", d.ID)
		}
	}
}
