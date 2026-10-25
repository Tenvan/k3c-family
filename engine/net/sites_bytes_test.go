package net

import (
	"encoding/json"
	"testing"

	"k3c/engine/sim"
)

// NT1.2, B-263/AC-01: Bytes je Tick einer Stufe mit allen Plätzen (seit W0 39: Linien und Hub), 4 Spieler, Seed
// `bench`, schneller Zyklus mit Nacht, 1800 Ticks; `snap` und `delta` wie stateData. Schwelle nur für das beschlossene
// Budget der Ereignisse (Q08, 200 Byte je Tick und Client), der ganze Zustand wird gemessen (docs/protocol.md ›
// Snapshot-Größe).
func TestSnapshotBytesAllePlaetze(t *testing.T) {
	isl, err := sim.CreateIsland("bench", []int{0}, 60, sim.CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	for range 4 {
		sim.AddIslandPlayer(isl, 0)
	}
	w, cmds := isl.Stages[0], []sim.PlayerCommand{{MoveX: 1}, {MoveX: -1}, {Pay: true}, {}}
	if len(w.Sites) < 39 {
		t.Fatalf("%d Plätze, erwartet mindestens 39", len(w.Sites))
	}
	for range 900 { // 30 s aufwärmen, bis in die Nacht
		sim.StepIsland(isl, cmds, 1.0/30)
	}
	var snaps, deltas, events []int
	prev := stateOf(w, 0, false)
	for range 1800 {
		sim.StepIsland(isl, cmds, 1.0/30)
		cur := stateOf(w, 0, false)
		snap, _ := json.Marshal(stateMsg{"snap", 0, 0, 0, cur})
		delta, _ := json.Marshal(stateMsg{"delta", 0, 0, 0, deltaOf(prev, cur)})
		ev, _ := json.Marshal(cur["events"])
		snaps, deltas, events = append(snaps, len(snap)), append(deltas, len(delta)), append(events, len(ev))
		prev = cur
	}
	sm, sp := bytesMeanP99(snaps)
	dm, dp := bytesMeanP99(deltas)
	em, ep := bytesMeanP99(events)
	sites, _ := json.Marshal(prev["sites"])
	t.Logf("%d Plätze (%d B im letzten snap), 4 Spieler, 1800 Ticks: snap Mittel %.0f B, p99 %d B; delta Mittel %.0f B, "+
		"p99 %d B; events Mittel %.1f B, p99 %d B", len(w.Sites), len(sites), sm, sp, dm, dp, em, ep)
	if em > 200 {
		t.Fatalf("Ereignis-Budget (Q08) überschritten: Mittel %.1f Byte je Tick", em)
	}
}
