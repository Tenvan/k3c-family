package level

import (
	"bytes"
	"encoding/json"
	"os"
	"slices"
	"strconv"
	"testing"

	"k3c/engine/core"
	"k3c/engine/internal/golden"
)

// Vergleich Go ↔ Rust (RS1.5, B-725, AC-06): Der Rust-Kern (rust/k3c-core/src/level, über engine/core ohne cgo) erzeugt
// jedes Golden-Level bitgleich und dasselbe Layout wie GenerateWith, inklusive Plätzen, Baugrund, Ruinen, Wachen und
// Fehlertexten. Fehlt die Bibliothek (task rust:build), werden die Tests übersprungen.

// rustOpts ist level.Options als JSON für den Rust-Kern; WithSim hängt die Felder der Sim an.
type rustOpts struct {
	Planned    []PlannedChunk `json:"planned,omitempty"`
	ChunkCount int            `json:"chunkCount,omitempty"`
	WithSim    bool           `json:"withSim,omitempty"`
}

// rustSim sind die Felder der Sim in der Ausgabe mit WithSim (WallSite, Ruin, Guard haben keine Tags: Go liest die
// Namen ohne Rücksicht auf Groß- und Kleinschreibung).
type rustSim struct {
	Sites      []WallSite `json:"sites"`
	FreeGround []float64  `json:"freeGround"`
	Ruins      []Ruin     `json:"ruins"`
	Guards     []Guard    `json:"guards"`
}

type rustGen struct {
	gen    *core.LevelGen
	inputs map[string][]byte
	out    []byte
}

func newRustGen(t testing.TB) *rustGen {
	t.Helper()
	lib, err := core.Find()
	if err != nil {
		t.Skip("Rust-Kern nicht gebaut (task rust:build): ", err)
	}
	g, err := lib.LevelGen()
	if err != nil {
		t.Skip("Rust-Kern ohne k3c_level_generate (task rust:build): ", err)
	}
	return &rustGen{gen: g, inputs: map[string][]byte{}, out: make([]byte, 0, 64<<10)}
}

// input ist die Eingabe für ein Biom: json.Marshal(Biome) plus die Regeln aus data/ (core.LevelInput).
func (r *rustGen) input(t testing.TB, b Biome) []byte {
	t.Helper()
	key := string(golden.Raw(t, b))
	if in, ok := r.inputs[key]; ok {
		return in
	}
	in, err := core.LevelInput([]byte(key))
	if err != nil {
		t.Fatal(err)
	}
	r.inputs[key] = in
	return in
}

func (r *rustGen) generate(t testing.TB, b Biome, seed string, o rustOpts) ([]byte, error) {
	t.Helper()
	out, err := r.gen.Generate(r.input(t, b), seed, golden.Raw(t, o), r.out)
	if err == nil {
		r.out = out[:0]
	}
	return out, err
}

// TestRustGoldenBitgleich: alle Level aus testdata/golden/level-*.json über Rust; Baum gleich dem Golden und Bytes
// gleich json.Marshal des Go-Layouts.
func TestRustGoldenBitgleich(t *testing.T) {
	r := newRustGen(t)
	plannedCase := loadPlannedCase(t)
	files := map[string]rustOpts{"level-planned.json": {Planned: plannedCase.Planned}}
	for _, id := range biomeIDs(t) {
		files["level-"+id+".json"] = rustOpts{}
	}
	for file, o := range files {
		raw, err := os.ReadFile("../../testdata/golden/" + file)
		if err != nil {
			t.Fatal(err)
		}
		var g goldenFile
		if err := json.Unmarshal(raw, &g); err != nil || len(g.Levels) == 0 {
			t.Fatalf("%s unbrauchbar: %v", file, err)
		}
		b := biome(t, g.Biome)
		for _, rawLevel := range g.Levels {
			var want map[string]any
			if err := json.Unmarshal(rawLevel, &want); err != nil {
				t.Fatal(err)
			}
			seed, _ := want["seed"].(string)
			got, err := r.generate(t, b, seed, o)
			if err != nil {
				t.Fatalf("%s, Seed %q: %v", file, seed, err)
			}
			var tree any
			if err := json.Unmarshal(got, &tree); err != nil {
				t.Fatalf("%s, Seed %q: %v", file, seed, err)
			}
			if d := golden.Diff("level", any(want), tree); d != "" {
				t.Errorf("%s, Seed %q: Rust weicht vom Golden ab: %s", file, seed, d)
			}
			goLayout, err := GenerateWith(b, seed, Options{Planned: o.Planned})
			if err != nil {
				t.Fatal(err)
			}
			if wantBytes := golden.Raw(t, goLayout); !bytes.Equal(got, wantBytes) {
				t.Errorf("%s, Seed %q: Bytes ungleich json.Marshal (Go %d, Rust %d Bytes)", file, seed, len(wantBytes), len(got))
			}
		}
	}
}

// rustCases sind Optionen je Biom für den breiten Vergleich: ohne, Inselgrößen, Beispiel-Planung, Ruine und Wachen.
func rustCases(t *testing.T, b Biome) map[string]rustOpts {
	cases := map[string]rustOpts{"ohne": {}, "Beispiel-Planung": {Planned: examplePlan(t, b)}}
	for _, n := range sizeCounts(t) {
		cases["Anzahl "+strconv.Itoa(n)] = rustOpts{ChunkCount: n}
	}
	if b.ID == "forest" {
		cases["Test-Planung"] = rustOpts{Planned: loadPlannedCase(t).Planned}
		cases["Ruine und Lager"] = rustOpts{Planned: []PlannedChunk{withGuards(withRuin(p(4, "forest"), "smithy", 10), guard("goblin", 14), guard("greed", 30))}}
		cases["Ruine mit Chunk-Seed"] = rustOpts{ChunkCount: 40, Planned: []PlannedChunk{withRuin(PlannedChunk{Slot: -9, Kind: "forest", Seed: "ruine"}, "nursery", 25)}}
	}
	return cases
}

// TestRustWieGo: je Biom 40 Seeds je Fall, Layout bytegleich und Sim-Felder gleich.
func TestRustWieGo(t *testing.T) {
	r := newRustGen(t)
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		for name, o := range rustCases(t, b) {
			for s := range 40 {
				seed := strconv.Itoa(s)
				compareRust(t, r, b, seed, name, o)
			}
		}
	}
}

func compareRust(t *testing.T, r *rustGen, b Biome, seed, name string, o rustOpts) {
	t.Helper()
	want, err := GenerateWith(b, seed, Options{Planned: o.Planned, ChunkCount: o.ChunkCount})
	if err != nil {
		t.Fatalf("Biom %s, %s, Seed %s: %v", b.ID, name, seed, err)
	}
	o.WithSim = true
	got, err := r.generate(t, b, seed, o)
	if err != nil {
		t.Fatalf("Biom %s, %s, Seed %s: Rust: %v", b.ID, name, seed, err)
	}
	var sim rustSim
	if err := json.Unmarshal(got, &sim); err != nil {
		t.Fatal(err)
	}
	var tree map[string]any
	if err := json.Unmarshal(got, &tree); err != nil {
		t.Fatal(err)
	}
	for _, k := range []string{"sites", "freeGround", "ruins", "guards"} {
		delete(tree, k)
	}
	if d := golden.Diff("level", golden.Tree(t, want), any(tree)); d != "" {
		t.Fatalf("Biom %s, %s, Seed %s: Layout weicht ab: %s", b.ID, name, seed, d)
	}
	if !slices.Equal(sim.Sites, want.Sites) || !slices.Equal(sim.FreeGround, want.FreeGround) ||
		!slices.Equal(sim.Ruins, want.Ruins) || !slices.Equal(sim.Guards, want.Guards) {
		t.Fatalf("Biom %s, %s, Seed %s: Plätze, Baugrund, Ruinen oder Wachen weichen ab", b.ID, name, seed)
	}
}

// TestRustFehlerWieGo: ungültige Anzahl und Planung lehnt Rust mit demselben Text ab wie GenerateWith.
func TestRustFehlerWieGo(t *testing.T) {
	r := newRustGen(t)
	forest, crystal, mine, cave := biome(t, "forest"), biome(t, "crystal"), biome(t, "mine"), biome(t, "cave")
	two := p(3, "forest")
	two.Buildings = []PlannedBuilding{{Kind: "smithy", At: 10}, {Kind: "nursery", At: 30}}
	cases := []struct {
		b Biome
		o rustOpts
	}{
		{forest, rustOpts{ChunkCount: 17}}, {forest, rustOpts{ChunkCount: 2}},
		{forest, rustOpts{Planned: plain("forest", 0)}}, {forest, rustOpts{Planned: []PlannedChunk{p(1, "hub")}}},
		{forest, rustOpts{Planned: plain("forest", 8)}}, {forest, rustOpts{ChunkCount: 16, Planned: plain("chest", -7)}},
		{forest, rustOpts{Planned: plain("forest", 2, 2)}}, {forest, rustOpts{Planned: []PlannedChunk{p(5, "volcano")}}},
		{forest, rustOpts{Planned: []PlannedChunk{p(1, "forest", obj("diamond", 10))}}},
		{forest, rustOpts{Planned: []PlannedChunk{p(1, "forest", obj("tree", 10), obj("tree", 10))}}},
		{forest, rustOpts{Planned: []PlannedChunk{p(1, "forest", obj("chest", 49))}}},
		{forest, rustOpts{Planned: []PlannedChunk{p(2, "portal")}}},
		{crystal, rustOpts{Planned: []PlannedChunk{p(4, "portal", obj("portal", 9))}}},
		{forest, rustOpts{Planned: []PlannedChunk{p(1, "portal", obj("portal", 100))}}},
		{forest, rustOpts{Planned: []PlannedChunk{p(3, "portal"), p(-3, "portal"), p(4, "portal")}}},
		{forest, rustOpts{Planned: []PlannedChunk{p(1, "chest"), p(-1, "chest"), p(2, "chest"), p(-2, "chest")}}},
		{mine, rustOpts{Planned: []PlannedChunk{p(3, "shaft", obj("copperVein", 10)), p(4, "shaft", obj("copperVein", 10)), p(-3, "shaft", obj("copperVein", 10))}}},
		{forest, rustOpts{Planned: plain("forest", 1, -1, 2, -2, 3, -3, 4, -4)}},
		{forest, rustOpts{Planned: plain("forest", 3, -3, 4, -4, 5, -5, 6, -6, 7, -7)}},
		{forest, rustOpts{Planned: []PlannedChunk{p(3, "recruitCamp", obj("recruitCamp", 50)), p(4, "recruitCamp", obj("recruitCamp", 0))}}},
		{forest, rustOpts{Planned: []PlannedChunk{two}}},
		{forest, rustOpts{Planned: []PlannedChunk{withRuin(p(3, "forest"), "castle", 10)}}},
		{forest, rustOpts{Planned: []PlannedChunk{withRuin(p(3, "forest"), "nursery", 2)}}},
		{forest, rustOpts{Planned: []PlannedChunk{p(3, "portal", obj("portal", 50)), withRuin(p(4, "forest"), "smithy", 2)}}},
		{forest, rustOpts{Planned: []PlannedChunk{withRuin(p(3, "clearing", obj("tree", 10)), "smithy", 12)}}},
		{cave, rustOpts{Planned: []PlannedChunk{withRuin(p(3, "cavern", obj("stoneVein", 20)), "smithy", 22)}}},
		{forest, rustOpts{Planned: []PlannedChunk{withGuards(p(4, "forest"), guard("greed", 5), guard("greed", 10), guard("greed", 15), guard("greed", 20), guard("greed", 25))}}},
		{forest, rustOpts{Planned: []PlannedChunk{withGuards(p(4, "forest"), guard("wolf", 20))}}},
		{forest, rustOpts{Planned: []PlannedChunk{withGuards(p(4, "forest"), guard("goblin", 1))}}},
		{forest, rustOpts{Planned: []PlannedChunk{withGuards(p(1, "forest"), guard("goblin", 20))}}},
		{forest, rustOpts{Planned: []PlannedChunk{withGuards(p(4, "forest"), guard("goblin", 20), guard("greed", 21))}}},
	}
	for i, c := range cases {
		_, goErr := GenerateWith(c.b, "fehler", Options{Planned: c.o.Planned, ChunkCount: c.o.ChunkCount})
		_, rustErr := r.generate(t, c.b, "fehler", c.o)
		if goErr == nil || rustErr == nil {
			t.Fatalf("Fall %d: Go %v, Rust %v; erwartet beide mit Fehler", i, goErr, rustErr)
		}
		if want := "core: Level (Generator): " + goErr.Error(); rustErr.Error() != want {
			t.Errorf("Fall %d:\n Rust %s\n Go   %s", i, rustErr, want)
		}
	}
}

// BenchmarkGenerateRust misst den Rust-Generator je Biom inklusive Brücke (Seed, Optionen und Ausgabe je Aufruf, das
// Biom-JSON einmal vorbereitet; Rust liest es je Aufruf). Vergleich mit BenchmarkGenerate (Go, ohne JSON-Ausgabe):
// go test ./engine/level -run '^$' -bench Generate -benchmem
func BenchmarkGenerateRust(b *testing.B) {
	r := newRustGen(b)
	for _, id := range []string{"forest", "cave", "mine", "ironhold", "crystal"} {
		biome, err := LoadBiome(id)
		if err != nil {
			b.Fatal(err)
		}
		input := r.input(b, biome)
		b.Run(id, func(b *testing.B) {
			out := make([]byte, 0, 64<<10)
			for i := 0; i < b.N; i++ {
				got, err := r.gen.Generate(input, strconv.Itoa(i%64), nil, out)
				if err != nil {
					b.Fatal(err)
				}
				out = got[:0]
			}
		})
	}
}
