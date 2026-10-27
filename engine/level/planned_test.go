package level

import (
	"encoding/json"
	"strings"
	"testing"
)

func p(slot int, kind string, objects ...PlannedObject) PlannedChunk {
	return PlannedChunk{Slot: slot, Kind: kind, Objects: objects}
}

func obj(kind string, at float64) PlannedObject { return PlannedObject{Kind: kind, At: at} }

func plain(kind string, slots ...int) []PlannedChunk {
	out := make([]PlannedChunk, len(slots))
	for i, s := range slots {
		out[i] = p(s, kind)
	}
	return out
}

func TestValidatePlanGueltig(t *testing.T) {
	forest, crystal := biome(t, "forest"), biome(t, "crystal")
	cases := map[string]struct {
		b    Biome
		plan []PlannedChunk
	}{
		"keine Planung":                  {forest, nil},
		"Chunk-Seed":                     {forest, []PlannedChunk{{Slot: -3, Kind: "clearing", Seed: "lichtung"}}},
		"Portal in der Mitte":            {forest, []PlannedChunk{p(4, "portal")}},
		"Portal genau am Mindestabstand": {forest, []PlannedChunk{p(3, "portal", obj("portal", 0))}},
		"Portal in 25er-Biom bei 10":     {crystal, []PlannedChunk{p(4, "portal", obj("portal", 10))}},
		"feste Objekte":                  {forest, []PlannedChunk{p(2, "clearing", obj("tree", 10), obj("bush", 30), obj("chest", 45))}},
	}
	for name, c := range cases {
		if err := ValidatePlan(c.b, c.plan, DefaultWallLines()); err != nil {
			t.Errorf("%s: %v", name, err)
		}
	}
}

func TestValidatePlanLehntAb(t *testing.T) {
	forest, crystal, mine := biome(t, "forest"), biome(t, "crystal"), biome(t, "mine")
	bigPlan := plain("forest", 1, -1, 2, -2, 3, -3, 4, -4)
	noPortalRoom := plain("forest", 3, -3, 4, -4, 5, -5, 6, -6, 7, -7)
	cases := map[string]struct {
		b    Biome
		plan []PlannedChunk
		want string
	}{
		"Platz 0":                 {forest, plain("forest", 0), "keinen Platz 0"},
		"Hub":                     {forest, []PlannedChunk{p(1, "hub")}, "nicht planbar"},
		"Ausgang":                 {forest, []PlannedChunk{p(1, "exit")}, "nicht planbar"},
		"Rand":                    {forest, []PlannedChunk{p(1, "edge")}, "nicht planbar"},
		"zu weit rechts":          {forest, plain("forest", 8), "jenseits der kürzesten Länge"},
		"zu weit links":           {forest, plain("forest", -8), "jenseits der kürzesten Länge"},
		"Mine kürzer":             {mine, []PlannedChunk{p(5, "shaft")}, "jenseits der kürzesten Länge"},
		"doppelter Platz":         {forest, plain("forest", 2, 2), "doppelt geplant"},
		"unbekannte Chunk-Art":    {forest, []PlannedChunk{p(1, "volcano")}, "unbekannte Chunk-Art"},
		"unbekannte Objekt-Art":   {forest, []PlannedChunk{p(1, "forest", obj("diamond", 10))}, "unbekannte Objekt-Art"},
		"kein Skill-Punkt":        {forest, []PlannedChunk{p(1, "forest", obj("skillPoint", 10))}, "unbekannte Objekt-Art"},
		"Wald ohne Adern":         {forest, []PlannedChunk{p(1, "forest", obj("stoneVein", 10))}, "unbekannte Objekt-Art"},
		"kind@x doppelt":          {forest, []PlannedChunk{p(1, "forest", obj("tree", 10), obj("tree", 10))}, "doppelt"},
		"zu nah an der Grenze":    {forest, []PlannedChunk{p(1, "forest", obj("tree", 1))}, "Abstand zur Chunk-Grenze"},
		"zu nah an der Grenze 2":  {forest, []PlannedChunk{p(1, "forest", obj("chest", 49))}, "Abstand zur Chunk-Grenze"},
		"Portal zu nah am Hub":    {forest, []PlannedChunk{p(2, "portal")}, "mindestens"},
		"Portal in 25er-Biom":     {crystal, []PlannedChunk{p(4, "portal", obj("portal", 9))}, "mindestens"},
		"Portal Platz 3 in 25er":  {crystal, []PlannedChunk{p(3, "portal")}, "mindestens"},
		"zu viele Portale":        {forest, []PlannedChunk{p(3, "portal"), p(-3, "portal"), p(4, "portal")}, "feste Portale"},
		"zu viele Truhen":         {forest, []PlannedChunk{p(1, "chest"), p(-1, "chest"), p(2, "chest"), p(-2, "chest")}, "feste chest"},
		"zu viele Camps":          {forest, []PlannedChunk{p(1, "recruitCamp"), p(-1, "recruitCamp"), p(2, "recruitCamp")}, "feste recruitCamp"},
		"zu viele Adern":          {mine, []PlannedChunk{p(3, "shaft", obj("copperVein", 10)), p(4, "shaft", obj("copperVein", 10)), p(-3, "shaft", obj("copperVein", 10))}, "feste Adern"},
		"kein Platz mehr":         {forest, bigPlan, "kein Platz mehr"},
		"kein Platz für Portale":  {forest, noPortalRoom, "weitere Portale"},
		"Mine ohne Platz":         {mine, plain("shaft", 1, -1), "kein Platz mehr"},
		"Portal hinter dem Chunk": {forest, []PlannedChunk{p(1, "portal", obj("portal", 100))}, "außerhalb des Chunks"},
		"kind@x über Chunks":      {forest, []PlannedChunk{p(3, "recruitCamp", obj("recruitCamp", 50)), p(4, "recruitCamp", obj("recruitCamp", 0))}, "kind@x"},
	}
	for name, c := range cases {
		err := ValidatePlan(c.b, c.plan, DefaultWallLines())
		if err == nil || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%s: Fehler %v, erwartet mit %q", name, err, c.want)
		}
	}
}

func TestValidatePlanNenntDenPlatz(t *testing.T) {
	err := ValidatePlan(biome(t, "forest"), []PlannedChunk{p(5, "volcano")}, DefaultWallLines())
	if err == nil || !strings.Contains(err.Error(), "Platz +5") {
		t.Fatalf("Fehler %v, erwartet mit Platz +5", err)
	}
}

func TestValidatePlanMineMitFreiemPlatz(t *testing.T) {
	if err := ValidatePlan(biome(t, "mine"), plain("shaft", 1), DefaultWallLines()); err != nil {
		t.Errorf("ein geplanter Chunk in der Mine lässt Platz: %v", err)
	}
}

func TestValidatePlanLinienAlsParameter(t *testing.T) {
	cave := biome(t, "cave")
	plan := []PlannedChunk{p(2, "cavern", obj("stoneVein", 36))} // Hub + 136
	lines124, lines126 := DefaultWallLines(), DefaultWallLines()
	lines124.WallUnits = []float64{44, 64, 84, 104, 124}
	lines126.WallUnits = []float64{44, 64, 84, 104, 126}
	for i, c := range []struct {
		lines WallLines
		ok    bool
	}{{lines124, true}, {lines126, false}, {lines124, true}, {lines126, false}} {
		err := ValidatePlan(cave, plan, c.lines)
		if (err == nil) != c.ok {
			t.Errorf("Aufruf %d: Fehler %v, gültig erwartet: %v", i, err, c.ok)
		}
	}
	if err := ValidatePlan(cave, plan, lines126); err == nil || !strings.Contains(err.Error(), "Bauplatz") {
		t.Errorf("Ablehnung nennt den Bauplatz nicht: %v", err)
	}
}

func TestValidatePlanAderAmFarmPlatz(t *testing.T) {
	cave := biome(t, "cave")
	// Hub + 52 ist der Farm-Weltplatz: Slot 1 bei 2 Units liegt bei 52.
	if err := ValidatePlan(cave, []PlannedChunk{p(1, "cavern", obj("stoneVein", 2))}, DefaultWallLines()); err == nil {
		t.Error("Ader auf dem Farm-Platz muss abgelehnt werden")
	}
}

func TestLeereFelderSindKeinePlanung(t *testing.T) {
	for _, raw := range []string{`null`, `[]`} {
		var plan []PlannedChunk
		if err := json.Unmarshal([]byte(raw), &plan); err != nil {
			t.Fatal(err)
		}
		if err := ValidatePlan(biome(t, "forest"), plan, DefaultWallLines()); err != nil {
			t.Errorf("%s: %v", raw, err)
		}
	}
}

func TestPlatzGrenzenJeBiom(t *testing.T) {
	want := map[string]int{"forest": 7, "cave": 5, "mine": 4, "ironhold": 7, "crystal": 9}
	for id, n := range want {
		if got := geometryOf(biome(t, id), 0).usable; got != n {
			t.Errorf("%s: %d planbare Plätze je Seite, erwartet %d", id, got, n)
		}
	}
}

func TestPlannedChunkLiestStreng(t *testing.T) {
	var c PlannedChunk
	if err := json.Unmarshal([]byte(`{"slot":1,"kind":"forest","object":[]}`), &c); err == nil {
		t.Error("unbekanntes Feld object muss abgelehnt werden")
	}
	if err := json.Unmarshal([]byte(`{"slot":1,"kind":"forest","objects":[{"kind":"tree","att":3}]}`), &c); err == nil {
		t.Error("unbekanntes Feld in einem Objekt muss abgelehnt werden")
	}
	if err := json.Unmarshal([]byte(`{"slot":1,"kind":"forest","seed":"x","objects":[{"kind":"tree","at":3}]}`), &c); err != nil {
		t.Errorf("gültiges Chunk-Format: %v", err)
	}
}
