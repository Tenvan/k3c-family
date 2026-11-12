package room

import "testing"

// Victory meldet den Tick, in dem die Insel gewonnen wird, genau einmal (B-813: das Ereignis liegt in Stufe 0).
func TestVictoryEinmal(t *testing.T) {
	f := newFixture()
	r := need(f.m.Create("x", &peer{}, "stand", true, 0, []int{0}, Options{Goal: "gold"}))(t)
	r.Tick()
	if r.Victory() {
		t.Fatal("Sieg ohne erreichtes Ziel")
	}
	r.mu.Lock()
	r.isl.GoldCollected = 1 << 20
	r.mu.Unlock()
	r.Tick()
	if !r.Victory() {
		t.Fatal("Sieg im Tick des Ziels nicht gemeldet")
	}
	r.Tick()
	if r.Victory() {
		t.Error("Sieg im Folgetick erneut gemeldet")
	}
}
