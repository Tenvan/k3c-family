package sim

import (
	"testing"

	"k3c/engine/level"
)

// BenchmarkSimRun100k bildet `sim_run` nach (Wald, Seed k3c-dev, ein Monarch ohne Eingaben, 100 000 Ticks bei dt=1/30)
// und meldet Ticks/s. Messreihe RS1 (B-725): `-benchtime 1x`; gleicher Aufbau vor und nach jeder Änderung.
func BenchmarkSimRun100k(b *testing.B) {
	const ticks = 100000
	biome, err := level.LoadBiome("forest")
	if err != nil {
		b.Fatal(err)
	}
	for b.Loop() {
		w, err := CreateWorld(biome, "k3c-dev", Options{})
		if err != nil {
			b.Fatal(err)
		}
		AddPlayer(w)
		cmds := make([]PlayerCommand, 1)
		for range ticks {
			Step(w, cmds, 1.0/30)
		}
	}
	b.ReportMetric(float64(ticks)*float64(b.N)/b.Elapsed().Seconds(), "ticks/s")
}
