package level

import (
	"encoding/json"
	"io/fs"
	"os"
	"path"
	"slices"
	"strconv"
	"strings"
	"testing"

	"k3c/data"
	"k3c/engine/internal/golden"
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

func TestGolden(t *testing.T) {
	for _, id := range biomeIDs(t) {
		t.Run(id, func(t *testing.T) {
			path := "../../testdata/golden/level-" + id + ".json"
			raw, err := os.ReadFile(path)
			if err != nil {
				t.Fatalf("Golden-Daten fehlen für Biom %s: %v", id, err)
			}
			var g goldenFile
			if err := json.Unmarshal(raw, &g); err != nil || g.Biome != id || len(g.Levels) == 0 {
				t.Fatalf("level-%s.json unbrauchbar (Biom %q, %d Level): %v", id, g.Biome, len(g.Levels), err)
			}
			b := biome(t, id)
			var all []Layout // mit -update: alle Level in der Reihenfolge der Datei
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
				all = append(all, got)
				if d := golden.Diff("level", want, golden.Tree(t, got)); d != "" && !*golden.Update {
					t.Errorf("Biom %s, Seed %q: %s", id, seed, d)
				}
			}
			if *golden.Update {
				f := golden.ReadFile(t, path)
				f.Set(t, "levels", all)
				f.Write(t, path)
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

// W2.1 (B-114/AC-01): Biome mit Adern haben über 500 Seeds genau so viele, gleicher Seed gleiche Adern; die Adern
// kommen nach allen übrigen Würfen, alle anderen Objekte bleiben gleich.
func TestAdernJeStufe(t *testing.T) {
	withVeins := 0
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		if b.Veins.Count == 0 {
			continue
		}
		withVeins++
		bare := b
		bare.Veins.Count = 0
		for seed := range 500 {
			l, _ := Generate(b, strconv.Itoa(seed))
			if n := countEntities(l, b.Veins.Kind); n != b.Veins.Count {
				t.Fatalf("Biom %s, Seed %d: %d Adern, erwartet %d", id, seed, n, b.Veins.Count)
			}
			again, _ := Generate(b, strconv.Itoa(seed))
			without, _ := Generate(bare, strconv.Itoa(seed))
			rest := slices.DeleteFunc(slices.Clone(l.Entities), func(e Entity) bool { return e.Kind == b.Veins.Kind })
			if !slices.Equal(l.Entities, again.Entities) || !slices.Equal(rest, without.Entities) {
				t.Fatalf("Biom %s, Seed %d: Adern nicht reproduzierbar oder verschieben andere Objekte", id, seed)
			}
		}
	}
	if withVeins < 2 || biome(t, "forest").Veins.Count != 0 {
		t.Errorf("%d Biome mit Adern, Wald %d Adern", withVeins, biome(t, "forest").Veins.Count)
	}
}

// density sind die endlichen Objekte (Bäume, Felsen, Erz, Truhen, Camps) je 100 Units, gemittelt über 100 Seeds.
func density(t *testing.T, id string) float64 {
	t.Helper()
	finite := map[string]bool{"tree": true, "rock": true, "copperOre": true, "chest": true, "recruitCamp": true}
	objects, width := 0, 0.0
	for seed := range 100 {
		l, _ := Generate(biome(t, id), strconv.Itoa(seed))
		for _, e := range l.Entities {
			if finite[e.Kind] {
				objects++
			}
		}
		width += l.WidthUnits
	}
	return 100 * float64(objects) / width
}

// W2.2 (B-115/AC-02, Sprint W2 AC-04, Beschluss Q28): Breiten laut materialien-gebaeude.md § 1, Dichte unter Tage
// nicht abnehmend und ab der Mine mindestens 2,8 endliche Objekte je 100 Units.
func TestBreiteUndDichte(t *testing.T) {
	widths := map[string]Range{"forest": {900, 1100}, "cave": {700, 900}, "mine": {550, 700}, "ironhold": {480, 560}, "crystal": {400, 480}}
	for id, want := range widths {
		if got := biome(t, id).LengthUnits; got != want {
			t.Errorf("%s: Breite %v, laut Regel %v", id, got, want)
		}
	}
	prev := 0.0
	for i, id := range []string{"cave", "mine", "ironhold", "crystal"} {
		d := density(t, id)
		t.Logf("%s: %.2f endliche Objekte je 100 Units", id, d)
		if d < prev || (i > 0 && d < 2.8) {
			t.Errorf("%s: Dichte %.2f (davor %.2f), erwartet nicht abnehmend und ≥ 2,8", id, d, prev)
		}
		prev = d
	}
	t.Logf("forest: %.2f endliche Objekte je 100 Units", density(t, "forest"))
}
