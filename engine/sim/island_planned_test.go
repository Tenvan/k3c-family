package sim

import (
	"strings"
	"testing"
)

// islandsWithPlan liest eine Test-Welt mit einer Insel „a“ und der Planung plan (island_worlds_test.go).
func islandsWithPlan(plan string) ([]IslandDef, error) {
	w, err := parseTestWorld(`,"plannedChunks":` + plan)
	return w.Islands, err
}

func TestInselPlanungWirdBeimLadenGeprueft(t *testing.T) {
	good := `{"0":[{"slot":-3,"kind":"clearing","seed":"lichtung"}],"1":[{"slot":3,"kind":"cavern","objects":[{"kind":"stoneVein","at":40}]}]}`
	dst, err := islandsWithPlan(good)
	if err != nil {
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
		out, err := islandsWithPlan(plan)
		if err == nil || out != nil || !strings.Contains(err.Error(), "data/worlds/test.json") {
			t.Errorf("%s: Fehler %v, Ergebnis %v", name, err, out)
		}
	}
}

func TestInselPlanungLeerIstKeinePlanung(t *testing.T) {
	for _, plan := range []string{`{}`, `null`, `{"0":[]}`} {
		if _, err := islandsWithPlan(plan); err != nil {
			t.Errorf("%s: %v", plan, err)
		}
	}
	for _, d := range StandardWorld().Islands {
		if len(d.PlannedChunks) != 0 {
			t.Errorf("%s: eingebettete Daten enthalten eine Planung", d.ID)
		}
	}
}
