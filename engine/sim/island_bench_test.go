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

// bossBenchIsland: Kristallhöhle mit 4 Spielern am Bau, Endboss ausgelöst und unter 60 % HP (Phase 2, Warnkreis alle
// 6 s); gewartet wird bis zum ersten Warnkreis (K4.2, B-154/AC-05).
func bossBenchIsland(b *testing.B) *Island {
	b.Helper()
	isl, err := CreateIsland("bossbench", []int{4}, 1)
	if err != nil {
		b.Fatal(err)
	}
	for range 4 {
		AddIslandPlayer(isl, 0)
	}
	w := isl.Stages[0]
	x, ok := lairX(w)
	if !ok {
		b.Fatal("Kristallhöhle ohne Bau")
	}
	for i, p := range w.Players {
		p.X = x + 4 + float64(i)
	}
	for range 600 {
		protect(w)
		StepIsland(isl, nil, dt)
		for _, e := range w.Enemies {
			if e.Boss && e.Warn != nil {
				return isl
			}
			if e.Boss && e.HP > e.MaxHP*0.6 {
				e.HP = e.MaxHP * 0.59
			}
		}
	}
	b.Fatal("kein Warnkreis")
	return nil
}

// BenchmarkIslandStepBoss4Players misst einen Tick der Bosswelle (Endboss Phase 2 mit Warnkreis, 4 Spieler) und die
// Bytes je Tick wie BenchmarkIslandStep3Stages4Players, gemessen in den ersten 1800 Ticks (K4.2, B-154/AC-05).
func BenchmarkIslandStepBoss4Players(b *testing.B) {
	isl := bossBenchIsland(b)
	var events, state []int
	for b.Loop() {
		protect(isl.Stages[0])
		StepIsland(isl, nil, dt)
		if len(events) < 1800 {
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
