package sim

import (
	"encoding/json"
	"slices"
	"testing"
)

// eventBudgetBytes ist das Budget der Feedback-Ereignisse je Tick und Client im Mittel (Beschluss Q08, F4/AC-04):
// 200 Byte × 30 Ticks/s = 6 KB/s je Client. Ein Client sieht genau eine Stufe, also gilt es je Stufe und Tick.
const eventBudgetBytes = 200

// benchIsland ist die Aufstellung für Benchmark und Budget: 3 Stufen, 4 Spieler, schneller Zyklus, aufgewärmt bis in
// die Nacht mit Gegnern.
func benchIsland(tb testing.TB) (*Island, []PlayerCommand) {
	tb.Helper()
	isl, err := CreateIsland("bench", []int{0, 1, 2}, islandSpeed)
	if err != nil {
		tb.Fatal(err)
	}
	for _, st := range []int{0, 1, 2, 0} {
		AddIslandPlayer(isl, st)
	}
	cmds := []PlayerCommand{{MoveX: 1}, {MoveX: -1}, {Pay: true}, {}}
	for tm := 0.0; tm < 30; tm += dt {
		StepIsland(isl, cmds, dt)
	}
	return isl, cmds
}

// tickBytes sammelt je Stufe die JSON-Bytes der Ereignisse und des ganzen Zustands dieses Ticks.
func tickBytes(tb testing.TB, isl *Island, events, state *[]int) {
	tb.Helper()
	for _, w := range isl.Stages {
		ev, err := json.Marshal(w.Events)
		if err != nil {
			tb.Fatal(err)
		}
		all, err := json.Marshal(w)
		if err != nil {
			tb.Fatal(err)
		}
		*events, *state = append(*events, len(ev)), append(*state, len(all))
	}
}

// meanP99 liefert Mittel und 99. Perzentil.
func meanP99(v []int) (mean float64, p99 int) {
	if len(v) == 0 {
		return 0, 0
	}
	sum := 0
	for _, x := range v {
		sum += x
	}
	s := slices.Clone(v)
	slices.Sort(s)
	return float64(sum) / float64(len(s)), s[len(s)*99/100]
}

// F4/AC-04 (B-140/AC-04): 1800 Ticks (1 min) mit festem Seed; das Mittel der Ereignis-Bytes je Stufe und Tick bleibt
// im Budget. Gekürzt wird bei Überschreitung im Server (Obergrenze K, events.go), nicht hier.
func TestEventsBudgetJeTick(t *testing.T) {
	isl, cmds := benchIsland(t)
	var events, state []int
	nights := 0
	for range 1800 {
		StepIsland(isl, cmds, dt)
		tickBytes(t, isl, &events, &state)
		if isl.Stages[0].Cycle.Phase == "night" {
			nights++
		}
	}
	mean, p99 := meanP99(events)
	t.Logf("Ereignisse je Stufe und Tick: Mittel %.1f Byte, p99 %d Byte (Ticks in der Nacht: %d)", mean, p99, nights)
	if nights == 0 {
		t.Fatal("der Lauf erreicht keine Nacht")
	}
	if mean > eventBudgetBytes {
		t.Fatalf("Ereignis-Budget überschritten: Mittel %.1f Byte je Tick > %d (p99 %d)", mean, eventBudgetBytes, p99)
	}
}
