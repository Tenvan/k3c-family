package room

import (
	"testing"
	"time"
)

// determinismTicks ist N aus B-138/AC-02: so viele Ticks laufen beide Räume.
const determinismTicks = 600

// B-138/AC-02: Ein Raum, dessen Takt künstlich stockt (Pausen zwischen einzelnen Ticks), rechnet nach 600 Ticks
// dieselbe Welt wie ein Raum, der ohne Pause getaktet wird. Die Simulation hängt nur von Ticks und Eingaben ab, nicht
// von der Wanduhr. Drei Räume mit 2, 3 und 4 Spielern und festen Eingaben (setup aus run_test.go).
func TestVerlangsamterTaktGleicheWelt(t *testing.T) {
	fast := setup(t, newFixture())
	slow := setup(t, newFixture())
	for i := range fast {
		ticks(fast[i], determinismTicks)
	}
	for n := range determinismTicks {
		for _, r := range slow {
			r.Tick()
		}
		if n%60 == 0 { // 10 Pausen à 50 ms, gesamt 0,5 s
			time.Sleep(50 * time.Millisecond)
		}
	}
	for i := range fast {
		if fast[i].tick != determinismTicks || slow[i].tick != determinismTicks {
			t.Fatalf("Raum %s: %d und %d Ticks", fast[i].Code, fast[i].tick, slow[i].tick)
		}
		if world(t, fast[i]) != world(t, slow[i]) {
			t.Errorf("Raum %s: Welt nach %d Ticks mit verlangsamtem Takt weicht ab", fast[i].Code, determinismTicks)
		}
	}
}
