package sim

import "testing"

// BenchmarkIslandStep3Stages4Players misst einen Tick der Insel mit 3 aktiven Stufen und 4 Spielern in der Nacht
// (Ziel aus B-042: Tick-Dauer p99 unter 10 ms; gemessen wird hier nur auf dem Entwickler-Rechner).
func BenchmarkIslandStep3Stages4Players(b *testing.B) {
	isl, err := CreateIsland("bench", []int{0, 1, 2}, islandSpeed)
	if err != nil {
		b.Fatal(err)
	}
	for _, st := range []int{0, 1, 2, 0} {
		AddIslandPlayer(isl, st)
	}
	cmds := []PlayerCommand{{MoveX: 1}, {MoveX: -1}, {Pay: true}, {}}
	for tm := 0.0; tm < 30; tm += dt { // Aufwärmen bis in die Nacht mit Gegnern
		StepIsland(isl, cmds, dt)
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		StepIsland(isl, cmds, dt)
	}
}
