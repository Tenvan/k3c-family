package actors

import (
	"testing"

	"k3c/engine/sim"
)

func TestBannerFlagPointsAwayFromBack(t *testing.T) {
	for _, left := range []bool{false, true} {
		pole, flag := bannerBoxes(60, left)
		away := pole.cx > 0 // hinter dem Rücken: bei Blick nach links rechts davon
		if (flag.cx > pole.cx) != away {
			t.Errorf("left=%v: Flagge zeigt zum Monarchen hin", left)
		}
		if flag.cy+flag.h/2 > pole.cy+pole.h/2 {
			t.Errorf("left=%v: Flagge ragt über den Stock", left)
		}
	}
}

func TestCrownCarrier(t *testing.T) {
	w := &sim.World{Players: []*sim.Player{
		{ID: 1, Index: 0, Crown: &sim.Crown{State: sim.CrownPlayer, Carrier: 2}},
		{ID: 2, Index: 1},
	}}
	if crownCarrier(w, w.Players[0]) || !crownCarrier(w, w.Players[1]) {
		t.Error("Träger der fremden Krone falsch erkannt")
	}
}

func TestCrownBoxesSymmetric(t *testing.T) {
	b := crownBoxes(0, 10)
	if b[1].cx != -b[3].cx || b[1].cy != b[3].cy {
		t.Errorf("Zacken nicht symmetrisch: %+v", b)
	}
}
