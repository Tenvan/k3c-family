package actors

import (
	"testing"

	"k3c/engine/sim"
)

func TestBannerFlagPointsAwayFromBack(t *testing.T) {
	for _, left := range []bool{false, true} {
		// Der Wimpel zeigt im Bild nach rechts; gespiegelt, wenn der Stock links der Mitte steht.
		if flip := bannerFlip(left); flip != (LoadOffset(left)*bannerOffset < 0) {
			t.Errorf("left=%v: Wimpel zeigt zum Monarchen hin (flip=%v)", left, flip)
		}
	}
}

func TestVariantAnim(t *testing.T) {
	if variantAnim(0) != "idle" || variantAnim(1) != "idle@spieler1" || variantAnim(4) != "idle@spieler4" || variantAnim(5) != "idle" {
		t.Error("Variante je Spieler 1 bis 4, sonst Grundvariante")
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
