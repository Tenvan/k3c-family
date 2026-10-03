package sim

import "testing"

// BenchmarkIslandStep3Stages4Players misst einen Tick der Insel mit 3 aktiven Stufen und 4 Spielern in der Nacht
// (Ziel aus B-042: Tick-Dauer p99 unter 10 ms; gemessen wird hier nur auf dem Entwickler-Rechner).
// Dazu Bytes je Stufe und Tick (F4/AC-03): Ereignisse und ganzer Zustand als JSON, Mittel und p99; ein Client sieht
// eine Stufe. Gemessen in den ersten 1800 Ticks (1 min, wie TestEventsBudgetJeTick), außerhalb der gemessenen Zeit.
func BenchmarkIslandStep3Stages4Players(b *testing.B) {
	isl, cmds := benchIsland(b)
	var events, state []int
	for b.Loop() {
		StepIsland(isl, cmds, dt)
		if len(events) < 1800*len(isl.Stages) {
			b.StopTimer()
			tickBytes(b, isl, &events, &state)
			b.StartTimer()
		}
	}
	evMean, evP99 := meanP99(events)
	stMean, stP99 := meanP99(state)
	b.ReportMetric(evMean, "eventB/tick")
	b.ReportMetric(float64(evP99), "eventB-p99/tick")
	b.ReportMetric(stMean, "stateB/tick")
	b.ReportMetric(float64(stP99), "stateB-p99/tick")
}
