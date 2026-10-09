package net

import (
	"encoding/json"
	"slices"
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// K4.2, B-154/AC-05: Bytes je Tick einer Bosswelle mit 4 Spielern, ganzer Zustand (`snap`) und `delta` mit Ereignissen
// wie in stateData. Nur gemessen (t.Log), keine Schwelle; das Budget gilt für Ereignisse (TestEventsBudgetJeTick).

// bossWelt ist Stufe 0 einer Kristallhöhle mit 4 Spielern am Bau; der Endboss ist ausgelöst, in Phase 2 und zeigt
// seinen ersten Warnkreis.
func bossWelt(t *testing.T) (*sim.Island, *sim.World) {
	t.Helper()
	isl, err := sim.CreateIsland("bossbench", []int{4}, 1)
	if err != nil {
		t.Fatal(err)
	}
	for range 4 {
		sim.AddIslandPlayer(isl, 0)
	}
	w := isl.Stages[0]
	i := slices.IndexFunc(w.Level.Entities, func(e level.Entity) bool { return e.Kind == "lair" })
	if i < 0 {
		t.Fatal("Kristallhöhle ohne Bau")
	}
	for k, p := range w.Players {
		p.X = w.Level.Entities[i].X + 4 + float64(k)
	}
	for range 600 {
		bossStep(isl, w)
		for _, e := range w.Enemies {
			if e.Boss && e.Warn != nil {
				return isl, w
			}
			if e.Boss && e.HP > e.MaxHP*0.6 {
				e.HP = e.MaxHP * 0.59
			}
		}
	}
	t.Fatal("kein Warnkreis")
	return nil, nil
}

// bossStep hält die Spieler am Leben und rechnet einen Takt.
func bossStep(isl *sim.Island, w *sim.World) {
	for _, p := range w.Players {
		p.HP, p.LastStandFor = p.MaxHP, 1
	}
	sim.StepIsland(isl, nil, 1.0/30)
}

func TestKampfBytesBosswelle(t *testing.T) {
	isl, w := bossWelt(t)
	var snaps, deltas []int
	prev := stateOf(w, 0, false)
	for range 1800 {
		bossStep(isl, w)
		cur := stateOf(w, 0, false)
		snap, _ := json.Marshal(cur)
		d := deltaOf(prev, cur)
		if len(w.Events) > 0 {
			d["events"] = w.Events
		}
		delta, _ := json.Marshal(stateMsg{"delta", 0, 0, 0, d})
		snaps, deltas = append(snaps, len(snap)), append(deltas, len(delta))
		prev = cur
	}
	sm, sp := bytesMeanP99(snaps)
	dm, dp := bytesMeanP99(deltas)
	t.Logf("Bosswelle, 4 Spieler, 1800 Ticks: snap Mittel %.0f B, p99 %d B; delta Mittel %.0f B, p99 %d B", sm, sp, dm, dp)
}

func bytesMeanP99(v []int) (float64, int) {
	sum := 0
	for _, x := range v {
		sum += x
	}
	s := slices.Sorted(slices.Values(v))
	return float64(sum) / float64(len(s)), s[len(s)*99/100]
}
