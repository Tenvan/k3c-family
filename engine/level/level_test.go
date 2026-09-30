package level

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path"
	"slices"
	"strconv"
	"strings"
	"testing"

	"k3c/data"
)

type goldenFile struct {
	Biome  string            `json:"biome"`
	Levels []json.RawMessage `json:"levels"`
}

// biomeIDs liefert alle Biome aus data/biomes/.
func biomeIDs(t *testing.T) []string {
	t.Helper()
	files, err := fs.Glob(data.Files, "biomes/*.json")
	if err != nil || len(files) == 0 {
		t.Fatalf("keine Biome in data/biomes/: %v", err)
	}
	ids := make([]string, len(files))
	for i, f := range files {
		ids[i] = strings.TrimSuffix(path.Base(f), ".json")
	}
	return ids
}

func biome(t *testing.T, id string) Biome {
	t.Helper()
	b, err := LoadBiome(id)
	if err != nil {
		t.Fatal(err)
	}
	return b
}

// diff liefert den Pfad der ersten Abweichung zwischen zwei JSON-Bäumen ("" = gleich). Zahlen als float64.
func diff(at string, want, got any) string {
	switch w := want.(type) {
	case map[string]any:
		g, ok := got.(map[string]any)
		if !ok {
			return at
		}
		keys := make([]string, 0, len(w)+len(g))
		for k := range w {
			keys = append(keys, k)
		}
		for k := range g {
			keys = append(keys, k)
		}
		slices.Sort(keys)
		for _, k := range slices.Compact(keys) {
			if d := diff(at+"."+k, w[k], g[k]); d != "" {
				return d
			}
		}
	case []any:
		g, ok := got.([]any)
		if !ok || len(g) != len(w) {
			return fmt.Sprintf("%s (Länge)", at)
		}
		for i := range w {
			if d := diff(at+"["+strconv.Itoa(i)+"]", w[i], g[i]); d != "" {
				return d
			}
		}
	default:
		if want != got {
			return fmt.Sprintf("%s = %v, erwartet %v", at, got, want)
		}
	}
	return ""
}

func tree(t *testing.T, v any) any {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	var out any
	if err := json.Unmarshal(raw, &out); err != nil {
		t.Fatal(err)
	}
	return out
}

func TestGolden(t *testing.T) {
	for _, id := range biomeIDs(t) {
		t.Run(id, func(t *testing.T) {
			raw, err := os.ReadFile("../../testdata/golden/level-" + id + ".json")
			if err != nil {
				t.Fatalf("Golden-Daten fehlen für Biom %s: %v", id, err)
			}
			var g goldenFile
			if err := json.Unmarshal(raw, &g); err != nil || g.Biome != id || len(g.Levels) == 0 {
				t.Fatalf("level-%s.json unbrauchbar (Biom %q, %d Level): %v", id, g.Biome, len(g.Levels), err)
			}
			b := biome(t, id)
			for _, rawLevel := range g.Levels {
				var want any
				if err := json.Unmarshal(rawLevel, &want); err != nil {
					t.Fatal(err)
				}
				seed, _ := want.(map[string]any)["seed"].(string)
				got, err := Generate(b, seed)
				if err != nil {
					t.Errorf("Biom %s, Seed %q: %v", id, seed, err)
					continue
				}
				if d := diff("level", want, tree(t, got)); d != "" {
					t.Errorf("Biom %s, Seed %q: %s", id, seed, d)
				}
			}
		})
	}
}

func TestGoldenMitUmlautUndEmoji(t *testing.T) {
	raw, err := os.ReadFile("../../testdata/golden/level-forest.json")
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(string(raw), `"seed":"Käse🧀"`) {
		t.Error("level-forest.json deckt Seed Käse🧀 nicht ab")
	}
}

func TestValidate500Seeds(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		for seed := range 500 {
			l, err := Generate(b, strconv.Itoa(seed))
			if err != nil {
				t.Fatalf("Biom %s, Seed %d: %v", id, seed, err)
			}
			if errs := Validate(l, b); len(errs) > 0 {
				t.Fatalf("Biom %s, Seed %d: %v", id, seed, errs)
			}
		}
	}
}

func TestValidateMeldetKaputtesLevel(t *testing.T) {
	b := biome(t, "forest")
	l, err := Generate(b, "kaputt")
	if err != nil {
		t.Fatal(err)
	}
	for i, c := range l.Chunks {
		if c.Kind == "exit" {
			l.Chunks[i].Kind = "forest"
		}
	}
	l.Entities = append(l.Entities, Entity{Kind: "castle", X: l.WidthUnits + 1})
	want := []string{"Genau ein Tiefen-Eingang erwartet", "Genau eine Burg erwartet", "Entity außerhalb des Levels"}
	if got := Validate(l, b); !slices.Equal(got, want) {
		t.Errorf("Validate = %q, erwartet %q", got, want)
	}
}

func TestHubInDerMitte(t *testing.T) {
	for _, id := range biomeIDs(t) {
		l, err := Generate(biome(t, id), "7")
		if err != nil {
			t.Fatal(err)
		}
		if l.HubCenterUnits != l.WidthUnits/2 || l.Entities[slices.IndexFunc(l.Entities, func(e Entity) bool { return e.Kind == "castle" })].X != l.HubCenterUnits {
			t.Errorf("Biom %s: Hub nicht in der Mitte", id)
		}
	}
}
