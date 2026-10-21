package level

import (
	"encoding/json"
	"os"
	"reflect"
	"strings"
	"testing"
)

func ruin(kind string, at float64) []PlannedBuilding { return []PlannedBuilding{{Kind: kind, At: at}} }

func withRuin(c PlannedChunk, kind string, at float64) PlannedChunk {
	c.Buildings = ruin(kind, at)
	return c
}

func TestRuinenWerteAusDaten(t *testing.T) {
	r := DefaultRuinRules()
	if r.HPShare != 0.25 || len(r.Kinds) != 8 {
		t.Fatalf("data/ruins.json: Anteil %v, %d Arten", r.HPShare, len(r.Kinds))
	}
	if got := RuinHP("smithy"); got != 63 {
		t.Errorf("Schmiede: %v HP, erwartet 63 (25 %% von 250, aufgerundet)", got)
	}
}

func TestRuinenWerteLehntAb(t *testing.T) {
	buildings := map[string]buildingInfo{"smithy": {HP: 250}, "castle": {HP: 1000}, "stairsUp": {HP: 500}}
	cases := map[string]struct {
		r    RuinRules
		want string
	}{
		"Anteil 0":       {RuinRules{HPShare: 0, Kinds: []string{"smithy"}}, "hpShare"},
		"Anteil über 1":  {RuinRules{HPShare: 1.5, Kinds: []string{"smithy"}}, "hpShare"},
		"keine Art":      {RuinRules{HPShare: 0.25}, "keine Art"},
		"unbekannte Art": {RuinRules{HPShare: 0.25, Kinds: []string{"volcano"}}, "unbekannte Gebäudeart"},
		"Burg":           {RuinRules{HPShare: 0.25, Kinds: []string{"castle"}}, "gehört dem Hub"},
		"Treppe":         {RuinRules{HPShare: 0.25, Kinds: []string{"stairsUp"}}, "gehört dem Hub"},
		"Anteil genau 1": {RuinRules{HPShare: 1, Kinds: []string{"smithy"}}, ""},
		"gültige Ruinen": {RuinRules{HPShare: 0.25, Kinds: []string{"smithy"}}, ""},
	}
	for name, c := range cases {
		err := checkRuinRules(c.r, buildings)
		if c.want == "" && err != nil || c.want != "" && (err == nil || !strings.Contains(err.Error(), c.want)) {
			t.Errorf("%s: Fehler %v, erwartet %q", name, err, c.want)
		}
	}
}

func TestRuineGueltig(t *testing.T) {
	forest := biome(t, "forest")
	cases := map[string][]PlannedChunk{
		"Schmiede":            {withRuin(p(3, "forest"), "smithy", 10)},
		"Schmiede am Rand":    {withRuin(p(3, "forest"), "smithy", 2)},
		"Farm mit Plantage":   {withRuin(p(3, "forest"), "farm", 3)},
		"neben festem Objekt": {withRuin(p(3, "clearing", obj("tree", 10)), "barracks", 14)},
		"mit Chunk-Seed":      {withRuin(PlannedChunk{Slot: -3, Kind: "clearing", Seed: "lichtung"}, "storage", 25)},
		"an einer Mauerlinie": {withRuin(p(2, "forest"), "tavern", 25)},  // innerhalb der Mauern, kein Abstand zu Bauplätzen
		"links am Tor":        {withRuin(p(-1, "forest"), "healer", 10)}, // 90 Units vom Hub, im Tor-Bereich von Linie 3
	}
	for name, plan := range cases {
		if err := ValidatePlan(forest, plan, DefaultWallLines()); err != nil {
			t.Errorf("%s: %v", name, err)
		}
	}
}

func TestRuineLehntAb(t *testing.T) {
	forest, cave := biome(t, "forest"), biome(t, "cave")
	two := p(3, "forest")
	two.Buildings = []PlannedBuilding{{Kind: "smithy", At: 10}, {Kind: "farm", At: 30}}
	cases := map[string]struct {
		b    Biome
		plan []PlannedChunk
		want string
	}{
		"zwei Ruinen":          {forest, []PlannedChunk{two}, "höchstens eine Ruine"},
		"Burg":                 {forest, []PlannedChunk{withRuin(p(3, "forest"), "castle", 10)}, "nicht erlaubt"},
		"Mauer":                {forest, []PlannedChunk{withRuin(p(3, "forest"), "wall", 10)}, "nicht erlaubt"},
		"unbekannte Art":       {forest, []PlannedChunk{withRuin(p(3, "forest"), "volcano", 10)}, "nicht erlaubt"},
		"am Rand":              {forest, []PlannedChunk{withRuin(p(3, "forest"), "smithy", 1)}, "Chunk-Grenze"},
		"hinter dem Chunk":     {forest, []PlannedChunk{withRuin(p(3, "forest"), "smithy", 49)}, "Chunk-Grenze"},
		"Plantage passt nicht": {forest, []PlannedChunk{withRuin(p(3, "forest"), "farm", 2)}, "Chunk-Grenze"},
		"Portal im Nachbar":    {forest, []PlannedChunk{p(3, "portal", obj("portal", 50)), withRuin(p(4, "forest"), "smithy", 2)}, "portal auf Platz +3"},
		"Camp im Nachbar":      {forest, []PlannedChunk{withRuin(p(-3, "forest"), "smithy", 2), p(-4, "recruitCamp", obj("recruitCamp", 50))}, "recruitCamp auf Platz -4"},
		"am Farm-Weltplatz":    {forest, []PlannedChunk{withRuin(p(1, "forest"), "smithy", 2)}, "Farm-Weltplatz"},
		"am festen Objekt":     {forest, []PlannedChunk{withRuin(p(3, "clearing", obj("tree", 10)), "smithy", 12)}, "tree"},
		"an der Truhe":         {forest, []PlannedChunk{withRuin(p(3, "chest"), "smithy", 23)}, "chest"},
		"am Portal":            {forest, []PlannedChunk{withRuin(p(4, "portal", obj("portal", 25)), "smithy", 27)}, "portal"},
		"an der Ader":          {cave, []PlannedChunk{withRuin(p(3, "cavern", obj("stoneVein", 20)), "smithy", 22)}, "stoneVein"},
	}
	for name, c := range cases {
		err := ValidatePlan(c.b, c.plan, DefaultWallLines())
		if err == nil || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%s: Fehler %v, erwartet mit %q", name, err, c.want)
		}
	}
}

// Doppeltes kind@x ist mit höchstens einer Ruine je Chunk und getrennten Arten im Level nicht erreichbar; markSpots
// lehnt es trotzdem ab, auch zwischen Objekten und Ruinen.
func TestMarkSpotsLehntDoppeltAb(t *testing.T) {
	a := placed{kind: "smithy", slot: 3, at: 10, dist: 160}
	if err := markSpots(map[spotKey]bool{}, []placed{a}, []placed{a}); err == nil || !strings.Contains(err.Error(), "doppelt") {
		t.Errorf("Fehler %v, erwartet doppelt", err)
	}
	b := placed{kind: "smithy", slot: -3, at: 40, dist: 160}
	if err := markSpots(map[spotKey]bool{}, []placed{a}, []placed{b}); err != nil {
		t.Errorf("links und rechts sind verschiedene Lagen: %v", err)
	}
}

func TestRuineFehlerNenntDenPlatz(t *testing.T) {
	err := ValidatePlan(biome(t, "forest"), []PlannedChunk{withRuin(p(-4, "forest"), "castle", 10)}, DefaultWallLines())
	if err == nil || !strings.Contains(err.Error(), "Platz -4") {
		t.Fatalf("Fehler %v, erwartet mit Platz -4", err)
	}
}

func TestRuineLiestStreng(t *testing.T) {
	var c PlannedChunk
	if err := json.Unmarshal([]byte(`{"slot":3,"kind":"forest","buildings":[{"kind":"smithy","att":3}]}`), &c); err == nil {
		t.Error("unbekanntes Feld in einer Ruine muss abgelehnt werden")
	}
	if err := json.Unmarshal([]byte(`{"slot":3,"kind":"forest","buildings":[{"kind":"smithy","at":3}]}`), &c); err != nil || len(c.Buildings) != 1 {
		t.Errorf("gültige Ruine: %v %+v", err, c.Buildings)
	}
	out, err := json.Marshal(PlannedChunk{Slot: 3, Kind: "forest"})
	if err != nil || strings.Contains(string(out), "buildings") {
		t.Errorf("ohne Ruine kein Feld buildings: %s %v", out, err)
	}
}

// testdata/planned/forest-ruin.json ist gültig und trägt je Seed alle drei Ruinen (Fixture für die Sim, WZ14.3).
func TestRuinenFixture(t *testing.T) {
	raw, err := os.ReadFile("../../testdata/planned/forest-ruin.json")
	if err != nil {
		t.Fatal(err)
	}
	var c plannedCase
	if err := json.Unmarshal(raw, &c); err != nil {
		t.Fatal(err)
	}
	for _, seed := range c.Seeds {
		l, err := GenerateWith(biome(t, c.Biome), seed, Options{Planned: c.Planned})
		if err != nil {
			t.Fatalf("Seed %q: %v", seed, err)
		}
		if len(l.Ruins) != 3 {
			t.Errorf("Seed %q: %d Ruinen, erwartet 3", seed, len(l.Ruins))
		}
	}
}

// Die Ruine würfelt nichts: Level mit und ohne Ruine sind bis auf Layout.Ruins gleich.
func TestRuineImLevelOhneWuerfe(t *testing.T) {
	forest := biome(t, "forest")
	plain := []PlannedChunk{p(3, "clearing", obj("tree", 10))}
	with := []PlannedChunk{withRuin(p(3, "clearing", obj("tree", 10)), "smithy", 30)}
	for _, seed := range []string{"a", "ruine", "Käse🧀"} {
		a, err := GenerateWith(forest, seed, Options{Planned: plain})
		if err != nil {
			t.Fatal(err)
		}
		b, err := GenerateWith(forest, seed, Options{Planned: with})
		if err != nil {
			t.Fatal(err)
		}
		want := []Ruin{{Kind: "smithy", X: b.HubCenterUnits + 50 + 2*50 + 30}}
		if !reflect.DeepEqual(b.Ruins, want) {
			t.Errorf("Seed %q: Ruinen %+v, erwartet %+v", seed, b.Ruins, want)
		}
		if a.Ruins != nil {
			t.Errorf("Seed %q: ohne Ruine %+v", seed, a.Ruins)
		}
		b.Ruins = nil
		if !reflect.DeepEqual(a, b) {
			t.Errorf("Seed %q: die Ruine ändert das Level", seed)
		}
	}
}
