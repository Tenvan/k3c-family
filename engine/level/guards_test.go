package level

import (
	"reflect"
	"strconv"
	"strings"
	"testing"
)

func withGuards(c PlannedChunk, guards ...PlannedEnemy) PlannedChunk {
	c.Enemies = guards
	return c
}

func guard(kind string, at float64) PlannedEnemy { return PlannedEnemy{Kind: kind, At: at} }

func TestWachenWerteAusDaten(t *testing.T) {
	r := DefaultGuardRules()
	if r.AggroRadiusUnits != 10 || r.MaxPerChunk != 4 || r.MinGapUnits != 1.5 {
		t.Fatalf("data/guards.json: %+v", r)
	}
}

func TestWachenWerteLehntAb(t *testing.T) {
	cases := map[string]struct {
		r    GuardRules
		want string
	}{
		"Radius 0":    {GuardRules{0, 4, 1}, "aggroRadiusUnits"},
		"keine Wache": {GuardRules{10, 0, 1}, "maxPerChunk"},
		"Abstand 0":   {GuardRules{10, 4, 0}, "minGapUnits"},
		"gültig":      {GuardRules{10, 4, 1}, ""},
	}
	for name, c := range cases {
		err := checkGuardRules(c.r)
		if c.want == "" && err != nil || c.want != "" && (err == nil || !strings.Contains(err.Error(), c.want)) {
			t.Errorf("%s: Fehler %v, erwartet %q", name, err, c.want)
		}
	}
}

func TestGegnerlagerGueltig(t *testing.T) {
	forest := biome(t, "forest")
	cases := map[string][]PlannedChunk{
		"eine Wache":      {withGuards(p(4, "forest"), guard("goblin", 20))},
		"vier Wachen":     {withGuards(p(4, "forest"), guard("greed", 10), guard("goblin", 20), guard("goblinArcher", 30), guard("goblin", 40))},
		"links":           {withGuards(p(-4, "forest"), guard("greed", 25))},
		"mit Ruine":       {withGuards(withRuin(p(4, "forest"), "smithy", 10), guard("goblin", 14))},
		"mit Chunk-Seed":  {withGuards(PlannedChunk{Slot: 4, Kind: "forest", Seed: "lager"}, guard("goblin", 25))},
		"Abstand genau 2": {withGuards(p(4, "forest"), guard("goblin", 2), guard("goblin", 3.5))},
	}
	for name, plan := range cases {
		if err := ValidatePlan(forest, plan); err != nil {
			t.Errorf("%s: %v", name, err)
		}
	}
}

func TestGegnerlagerLehntAb(t *testing.T) {
	forest := biome(t, "forest")
	cases := map[string]struct {
		plan []PlannedChunk
		want string
	}{
		"zu viele Wachen":    {[]PlannedChunk{withGuards(p(4, "forest"), guard("greed", 5), guard("greed", 10), guard("greed", 15), guard("greed", 20), guard("greed", 25))}, "höchstens 4 Wachen"},
		"Art nicht im Biom":  {[]PlannedChunk{withGuards(p(4, "forest"), guard("skeleton", 20))}, "nicht erlaubt"},
		"Nachtgegner":        {[]PlannedChunk{withGuards(p(4, "forest"), guard("wolf", 20))}, "nicht erlaubt"},
		"unbekannte Art":     {[]PlannedChunk{withGuards(p(4, "forest"), guard("drache", 20))}, "nicht erlaubt"},
		"zu nah an Grenze":   {[]PlannedChunk{withGuards(p(4, "forest"), guard("goblin", 1))}, "Abstand zur Chunk-Grenze"},
		"zu nah an Grenze 2": {[]PlannedChunk{withGuards(p(4, "forest"), guard("goblin", 49))}, "Abstand zur Chunk-Grenze"},
		"innerhalb der Verteidigungszone": {[]PlannedChunk{withGuards(p(1, "forest"), guard("goblin", 20))}, "innerhalb der Verteidigungszone"},
		"Wachen zu nah":      {[]PlannedChunk{withGuards(p(4, "forest"), guard("goblin", 20), guard("greed", 21))}, "weniger als"},
	}
	for name, c := range cases {
		err := ValidatePlan(forest, c.plan)
		if err == nil || !strings.Contains(err.Error(), c.want) || !strings.Contains(err.Error(), "auf Platz") {
			t.Errorf("%s: Fehler %v, erwartet mit Platz und %q", name, err, c.want)
		}
	}
}

// Die Wachen liegen im Layout; sie würfeln nichts: Level mit und ohne Lager sind bis auf Layout.Guards gleich.
func TestGegnerlagerImLevelOhneWuerfe(t *testing.T) {
	forest := biome(t, "forest")
	plain := []PlannedChunk{p(3, "forest")}
	with := []PlannedChunk{withGuards(p(3, "forest"), guard("goblin", 30), guard("greed", 35))}
	for _, seed := range []string{"a", "lager", "Käse🧀"} {
		a, b := gen2(t, forest, seed, plain), gen2(t, forest, seed, with)
		start := b.HubCenterUnits + 50 + 2*50
		want := []Guard{{Kind: "goblin", X: start + 30}, {Kind: "greed", X: start + 35}}
		if !reflect.DeepEqual(b.Guards, want) {
			t.Errorf("Seed %q: Wachen %+v, erwartet %+v", seed, b.Guards, want)
		}
		if a.Guards != nil {
			t.Errorf("Seed %q: ohne Lager %+v", seed, a.Guards)
		}
		b.Guards = nil
		if !reflect.DeepEqual(a, b) {
			t.Errorf("Seed %q: das Lager ändert das Level", seed)
		}
	}
}

func TestGegnerlagerAlleBiomeGeneriertDeterministisch(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		if len(b.Enemies.Portal) == 0 {
			continue
		}
		plan := []PlannedChunk{withGuards(p(-3, string(firstChunkKind(t, b))), guard(b.Enemies.Portal[0], 25))}
		if err := ValidatePlan(b, plan); err != nil {
			continue // Plätze nahe am Hub liegen in manchen Biomen innerhalb des Tors; dort gilt die Ablehnung
		}
		for seed := range 20 {
			s := strconv.Itoa(seed)
			if a, b2 := gen2(t, b, s, plan), gen2(t, b, s, plan); !reflect.DeepEqual(a, b2) || len(a.Guards) != 1 {
				t.Fatalf("Biom %s, Seed %s: %d Wachen oder nicht deterministisch", id, s, len(a.Guards))
			}
		}
	}
}

func firstChunkKind(t *testing.T, b Biome) string {
	t.Helper()
	if len(b.ChunkWeights) == 0 {
		t.Fatalf("Biom %s ohne chunkWeights", b.ID)
	}
	return b.ChunkWeights[0].Key
}
