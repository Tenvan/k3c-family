package level

import (
	"encoding/json"
	"os"
	"strconv"
	"testing"
)

// BenchmarkGenerate misst die Level-Erzeugung je Biom ohne Planung (WZ8.3, AC-13: Baseline vor den Änderungen am
// Generator, danach höchstens +10 %). Aufruf: go test ./engine/level -run '^$' -bench Generate -count 5
func BenchmarkGenerate(b *testing.B) {
	for _, id := range []string{"forest", "cave", "mine", "ironhold", "crystal"} {
		biome, err := LoadBiome(id)
		if err != nil {
			b.Fatal(err)
		}
		b.Run(id, func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				if _, err := Generate(biome, strconv.Itoa(i%64)); err != nil {
					b.Fatal(err)
				}
			}
		})
	}
}

// BenchmarkGenerateGeplant misst die Erzeugung mit der Test-Planung (testdata/planned/forest.json); Vergleich mit
// BenchmarkGenerate/forest: höchstens +10 % (WZ8.3, AC-13).
func BenchmarkGenerateGeplant(b *testing.B) {
	raw, err := os.ReadFile("../../testdata/planned/forest.json")
	if err != nil {
		b.Fatal(err)
	}
	var c plannedCase
	if err := json.Unmarshal(raw, &c); err != nil {
		b.Fatal(err)
	}
	biome, err := LoadBiome(c.Biome)
	if err != nil {
		b.Fatal(err)
	}
	for i := 0; i < b.N; i++ {
		if _, err := GenerateWith(biome, strconv.Itoa(i%64), Options{Planned: c.Planned}); err != nil {
			b.Fatal(err)
		}
	}
}
