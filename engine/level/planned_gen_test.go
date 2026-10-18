package level

import (
	"slices"
	"strconv"
	"testing"
)

// examplePlan ist eine gültige Planung für jedes Biom: ein Chunk mit Chunk-Seed, ein festes Portal, eine feste Truhe
// und, wo das Biom Adern hat, eine feste Ader hinter dem äußersten Tor.
func examplePlan(t *testing.T, b Biome) []PlannedChunk {
	t.Helper()
	g := shortest(b)
	first := b.ChunkWeights[0].Key
	plan := []PlannedChunk{
		{Slot: -3, Kind: first, Seed: "lichtung"},
		{Slot: 4, Kind: "portal"},
		{Slot: -1, Kind: "chest"},
	}
	if b.Veins.Count > 0 {
		slot := 1
		for g.dist(slot, edgeGapUnits) < 136 {
			slot++
		}
		plan = append(plan, PlannedChunk{Slot: slot, Kind: first, Objects: []PlannedObject{{Kind: b.Veins.Kind, At: 10}}})
	}
	if err := ValidatePlan(b, plan, DefaultWallLines()); err != nil {
		t.Fatalf("Beispiel-Planung für %s ungültig: %v", b.ID, err)
	}
	return plan
}

func gen2(t *testing.T, b Biome, seed string, plan []PlannedChunk) Layout {
	t.Helper()
	l, err := GenerateWith(b, seed, Options{Planned: plan})
	if err != nil {
		t.Fatalf("Biom %s, Seed %s: %v", b.ID, seed, err)
	}
	return l
}

func TestGenerateWithNullwertIstGenerate(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		for _, seed := range []string{"a", "7", "Käse🧀"} {
			want, _ := Generate(b, seed)
			got, err := GenerateWith(b, seed, Options{})
			explicit := DefaultWallLines()
			again, err2 := GenerateWith(b, seed, Options{Lines: &explicit})
			if err != nil || err2 != nil || !slices.Equal(want.Entities, got.Entities) || !slices.Equal(want.Entities, again.Entities) || !slices.Equal(want.Lines, again.Lines) {
				t.Fatalf("Biom %s, Seed %s: GenerateWith(Options{}) weicht von Generate ab (%v, %v)", id, seed, err, err2)
			}
		}
	}
}

func TestGeplanteChunksSindDeterministischUndGueltig(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		plan := examplePlan(t, b)
		for seed := range 500 {
			s := strconv.Itoa(seed)
			l := gen2(t, b, s, plan)
			if errs := Validate(l, b); len(errs) > 0 {
				t.Fatalf("Biom %s, Seed %s: %v", id, s, errs)
			}
			if again := gen2(t, b, s, plan); !slices.Equal(l.Entities, again.Entities) || !slices.Equal(l.Chunks, again.Chunks) {
				t.Fatalf("Biom %s, Seed %s: nicht deterministisch", id, s)
			}
		}
	}
}

func TestFestesPortalLiegtAmErwartetenOrt(t *testing.T) {
	b := biome(t, "forest")
	plan := []PlannedChunk{{Slot: 6, Kind: "portal"}} // Hub + 300 … + 350, Mitte bei + 325
	for seed := range 20 {
		l := gen2(t, b, strconv.Itoa(seed), plan)
		var xs []float64
		for _, e := range l.Entities {
			if e.Kind == "portal" {
				xs = append(xs, e.X-l.HubCenterUnits)
			}
		}
		slices.Sort(xs)
		if len(xs) != 2 || xs[1] != 325 || xs[0] >= 0 {
			t.Fatalf("Seed %d: Portale bei %v, erwartet eines bei +325 und das zweite links", seed, xs)
		}
	}
}

// AC-03: Der Chunk-Seed macht den Inhalt in jedem Level gleich, gemessen am Abstand zum Hub.
func TestChunkSeedIstUeberLevelSeedsStabil(t *testing.T) {
	b := biome(t, "forest")
	plan := []PlannedChunk{{Slot: -3, Kind: "forest", Seed: "lichtung"}}
	contents := func(seed string) []Entity {
		l := gen2(t, b, seed, plan)
		start := l.HubCenterUnits - 50 - 3*b.ChunkWidthUnits
		var out []Entity
		for _, e := range l.Entities {
			if e.X >= start && e.X < start+b.ChunkWidthUnits {
				out = append(out, Entity{Kind: e.Kind, X: e.X - l.HubCenterUnits})
			}
		}
		return out
	}
	want := contents("0")
	if len(want) == 0 {
		t.Fatal("der Chunk enthält nichts")
	}
	for seed := 1; seed < 100; seed++ {
		got := contents(strconv.Itoa(seed))
		// Skill-Punkte würfeln im Haupt-Strom über freie Chunks und liegen nie im geplanten Chunk.
		if !slices.Equal(want, got) {
			t.Fatalf("Seed %d: Inhalt %v, erwartet %v", seed, got, want)
		}
	}
}

// AC-04: Der Inhalt eines geplanten Chunks würfelt nur aus seinem eigenen Strom; alle anderen Chunks bleiben gleich.
func TestInhaltGeplanterChunksStoertDenHauptStromNicht(t *testing.T) {
	b := biome(t, "forest")
	variants := [][]PlannedChunk{
		{{Slot: -3, Kind: "forest", Seed: "a"}},
		{{Slot: -3, Kind: "forest", Seed: "b"}},
		{{Slot: -3, Kind: "forest", Objects: []PlannedObject{{Kind: "tree", At: 10}}}},
		{{Slot: -3, Kind: "forest", Objects: []PlannedObject{{Kind: "rock", At: 20}, {Kind: "bush", At: 30}}}},
	}
	outside := func(l Layout) []Entity {
		start := l.HubCenterUnits - 50 - 3*b.ChunkWidthUnits
		return slices.DeleteFunc(slices.Clone(l.Entities), func(e Entity) bool { return e.X >= start && e.X < start+b.ChunkWidthUnits })
	}
	for seed := range 50 {
		s := strconv.Itoa(seed)
		want := outside(gen2(t, b, s, variants[0]))
		for i, plan := range variants[1:] {
			if got := outside(gen2(t, b, s, plan)); !slices.Equal(want, got) {
				t.Fatalf("Seed %s, Variante %d: ein geplanter Chunk verschiebt andere Objekte", s, i+1)
			}
		}
	}
}

func TestGenerateWithLehntUngueltigePlanungAb(t *testing.T) {
	if _, err := GenerateWith(biome(t, "forest"), "x", Options{Planned: []PlannedChunk{{Slot: 0, Kind: "forest"}}}); err == nil {
		t.Error("ungültige Planung muss abgelehnt werden")
	}
}

// AC-03: Die Linien sind ein Parameter; verschiedene Linien nacheinander liefern je ihr Layout.
func TestGenerateWithLinienParameterWirkenBeiPlanung(t *testing.T) {
	b := biome(t, "cave")
	wide := DefaultWallLines()
	wide.WallUnits = []float64{44, 64, 84, 104, 126}
	a, _ := GenerateWith(b, "x", Options{Lines: &wide})
	def, _ := GenerateWith(b, "x", Options{})
	again, _ := GenerateWith(b, "x", Options{Lines: &wide})
	if slices.Equal(a.Lines, def.Lines) || !slices.Equal(a.Lines, again.Lines) {
		t.Error("die Linien-Parameter wirken nicht oder nicht wiederholbar")
	}
	if got := a.Lines[4].Wall - a.HubCenterUnits; got < -130 || got > -126 {
		t.Errorf("Linie 5 links bei %v, erwartet −126 … −130", got)
	}
}
