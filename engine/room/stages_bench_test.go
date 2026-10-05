package room

import (
	"encoding/json"
	"slices"
	"testing"

	"k3c/engine/sim"
)

// eventBudgetBytes ist das Budget der Ereignisse je Tick und Gerät im Mittel (Beschluss Q08): 200 Byte × 30 Ticks/s
// = 6 KB/s. Ein Gerät bekommt die Ereignisse aller Stufen seiner Spieler, also gilt es für die Summe (B-176/AC-04).
const eventBudgetBytes = 200

// stufenTicks ist die Länge der Messung: 1800 Ticks (1 min), wie TestEventsBudgetJeTick in engine/sim.
const stufenTicks = 1800

// countPeer zählt je Tick die JSON-Bytes, die das Gerät bekommt, summiert über seine Stufen: die Ereignisse und den
// ganzen Zustand. Der ganze Zustand ist die Obergrenze (snap); das Delta baut engine/net, das der Raum nicht kennt.
type countPeer struct {
	peer
	tick          int
	events, state []int
}

func (p *countPeer) State(tick, _ int, s any) {
	w := s.(*sim.World)
	ev, _ := json.Marshal(w.Events)
	all, _ := json.Marshal(w)
	if len(p.events) == 0 || tick != p.tick {
		p.tick, p.events, p.state = tick, append(p.events, 0), append(p.state, 0)
	}
	p.events[len(p.events)-1] += len(ev)
	p.state[len(p.state)-1] += len(all)
}

// stufenAufstellung: ein Gerät, zwei Spieler in Stufe 0 und 1 (Seed = Name "bench"), schneller Zyklus wie in
// engine/sim (60), aufgewärmt bis in die Nacht; Eingaben wie dort (laufen, bezahlen). Danach misst der Peer
// stufenTicks Ticks; nights zählt die Nacht-Ticks.
func stufenAufstellung(tb testing.TB) (p *countPeer, nights int) {
	tb.Helper()
	f := newFixture()
	p = &countPeer{}
	r, err := f.m.Create("bench", p, "bench", true, 0, []int{0, 1}, Options{})
	if err != nil {
		tb.Fatal(err)
	}
	moveToExit(tb, r, 0)
	for _, w := range r.isl.Stages {
		w.CycleSpeed = 60
	}
	for i := 0; r.isl.StageOf(0) != 1 || r.isl.Stages[0].Cycle.Phase != "night"; i++ {
		if i > 30*120 {
			tb.Fatalf("keine Nacht in zwei Stufen nach 2 min: Stufe %d, Phase %q", r.isl.StageOf(0), r.isl.Stages[0].Cycle.Phase)
		}
		_ = r.Tick()
	}
	if err := r.Input("bench", p, map[int]sim.PlayerCommand{0: {MoveX: 1}, 1: {Pay: true}}); err != nil {
		tb.Fatal(err)
	}
	p.events, p.state = nil, nil
	for range stufenTicks {
		_ = r.Tick()
		if r.isl.Stages[0].Cycle.Phase == "night" {
			nights++
		}
	}
	if len(p.events) != stufenTicks || r.isl.StageOf(0) == r.isl.StageOf(1) {
		tb.Fatalf("%d Ticks gezählt, Stufen %v", len(p.events), r.depths())
	}
	return p, nights
}

// meanP99 liefert Mittel und 99. Perzentil.
func meanP99(v []int) (mean float64, p99 int) {
	sum := 0
	for _, x := range v {
		sum += x
	}
	s := slices.Sorted(slices.Values(v))
	return float64(sum) / float64(len(s)), s[len(s)*99/100]
}

// BenchmarkStufenZweiSpieler misst die Bytes je Tick eines Geräts mit zwei Spielern in verschiedenen Stufen
// (B-176/AC-04): Ereignisse aller Stufen und ganze Zustände (Obergrenze snap), Mittel und p99. Ein Durchlauf ist die
// ganze Aufstellung samt 1800 Ticks.
func BenchmarkStufenZweiSpieler(b *testing.B) {
	var p *countPeer
	for b.Loop() {
		p, _ = stufenAufstellung(b)
	}
	evMean, evP99 := meanP99(p.events)
	stMean, stP99 := meanP99(p.state)
	b.ReportMetric(evMean, "eventB/tick")
	b.ReportMetric(float64(evP99), "eventB-p99/tick")
	b.ReportMetric(stMean, "stateB/tick")
	b.ReportMetric(float64(stP99), "stateB-p99/tick")
}

// B-176/AC-04: Das Mittel der Ereignis-Bytes je Tick und Gerät (Summe beider Stufen) bleibt im Budget aus Q08.
func TestStufenEreignisBudgetJeGeraet(t *testing.T) {
	p, nights := stufenAufstellung(t)
	mean, p99 := meanP99(p.events)
	t.Logf("Ereignisse je Tick und Gerät (2 Stufen): Mittel %.1f Byte, p99 %d Byte (Ticks in der Nacht: %d)", mean, p99, nights)
	if nights == 0 {
		t.Fatal("der Lauf erreicht keine Nacht")
	}
	if mean > eventBudgetBytes {
		t.Fatalf("Ereignis-Budget je Gerät überschritten: Mittel %.1f Byte je Tick > %d (p99 %d)", mean, eventBudgetBytes, p99)
	}
}
