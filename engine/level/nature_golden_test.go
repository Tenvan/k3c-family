package level

import (
	"encoding/json"
	"testing"

	"k3c/engine/internal/golden"
)

// natureGoldenPath hält je Biom und Seed die geltenden Natur-Werte, die Zahl der Objekte je Art und der Varianten
// (AC-01, B-891); `task go:golden` schreibt sie neu.
const natureGoldenPath = "../../testdata/golden/nature.json"

type natureCase struct {
	Biome    string         `json:"biome"`
	Seed     string         `json:"seed"`
	Nature   NatureValues   `json:"nature"`
	Kinds    map[string]int `json:"kinds"`
	Variants map[string]int `json:"variants"`
}

// AC-01: Gleicher Seed und gleiche Welt ergeben dieselben Werte und dieselbe Natur (Golden).
func TestGoldenNatur(t *testing.T) {
	var got []natureCase
	for _, id := range biomeIDs(t) {
		for _, seed := range []string{"golden", "Käse🧀"} {
			l := generate(t, biome(t, id), seed, NatureValues{})
			c := natureCase{Biome: id, Seed: seed, Nature: l.Nature, Kinds: map[string]int{}, Variants: map[string]int{}}
			for _, e := range l.Entities {
				c.Kinds[e.Kind]++
				if e.Variant != "" {
					c.Variants[e.Kind+"."+e.Variant]++
				}
			}
			got = append(got, c)
		}
	}
	f := golden.ReadFile(t, natureGoldenPath)
	if *golden.Update {
		f.Set(t, "cases", got)
		f.Write(t, natureGoldenPath)
		return
	}
	var want map[string]any
	if err := json.Unmarshal(golden.Read(t, natureGoldenPath), &want); err != nil {
		t.Fatal(err)
	}
	if d := golden.Diff("natur", want["cases"], golden.Tree(t, got)); d != "" {
		t.Fatal(d)
	}
}
