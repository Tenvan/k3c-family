package level

import (
	"encoding/json"
	"os"
	"testing"

	"k3c/engine/internal/golden"
)

// plannedCase ist die Test-Planung unter testdata/planned/ (keine echte Planung; data/islands.json bleibt leer).
type plannedCase struct {
	Biome   string         `json:"biome"`
	Seeds   []string       `json:"seeds"`
	Planned []PlannedChunk `json:"planned"`
}

const plannedGoldenPath = "../../testdata/golden/level-planned.json"

func loadPlannedCase(t *testing.T) plannedCase {
	t.Helper()
	raw, err := os.ReadFile("../../testdata/planned/forest.json")
	if err != nil {
		t.Fatal(err)
	}
	var c plannedCase
	if err := json.Unmarshal(raw, &c); err != nil || len(c.Seeds) == 0 || len(c.Planned) == 0 {
		t.Fatalf("testdata/planned/forest.json unbrauchbar (%d Seeds, %d Chunks): %v", len(c.Seeds), len(c.Planned), err)
	}
	return c
}

func plannedLevels(t *testing.T, c plannedCase) []Layout {
	t.Helper()
	b := biome(t, c.Biome)
	var all []Layout
	for _, seed := range c.Seeds {
		l, err := GenerateWith(b, seed, Options{Planned: c.Planned})
		if err != nil {
			t.Fatalf("Seed %q: %v", seed, err)
		}
		if errs := Validate(l, b); len(errs) > 0 {
			t.Fatalf("Seed %q: %v", seed, errs)
		}
		all = append(all, l)
	}
	return all
}

// TestGoldenPlanned prüft den Golden-Fall mit Test-Planung (WZ8.3, B-461/AC-03): Der Generator setzt geplante Chunks
// deterministisch ein. Die Datei testdata/golden/level-planned.json ist ein eigener Golden-Fall; `-update` schreibt sie neu.
func TestGoldenPlanned(t *testing.T) {
	c := loadPlannedCase(t)
	all := plannedLevels(t, c)
	if *golden.Update {
		f := golden.ReadFile(t, plannedGoldenPath)
		f.Set(t, "levels", all)
		f.Write(t, plannedGoldenPath)
		return
	}
	rawGolden, err := os.ReadFile(plannedGoldenPath)
	if err != nil {
		t.Fatalf("Golden-Daten fehlen: %v", err)
	}
	var g goldenFile
	if err := json.Unmarshal(rawGolden, &g); err != nil || g.Biome != c.Biome || len(g.Levels) != len(c.Seeds) {
		t.Fatalf("level-planned.json unbrauchbar (Biom %q, %d Level, erwartet %d): %v", g.Biome, len(g.Levels), len(c.Seeds), err)
	}
	for i, l := range all {
		var want any
		if err := json.Unmarshal(g.Levels[i], &want); err != nil {
			t.Fatal(err)
		}
		if d := golden.Diff("level", want, golden.Tree(t, l)); d != "" {
			t.Errorf("Seed %q: %s", c.Seeds[i], d)
		}
	}
}
