package props

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func TestCombatPlan(t *testing.T) {
	w := &sim.World{Portals: []float64{10, 30}, PortalHP: []float64{50, 100}, PortalMaxHP: 100, PortalsDown: []float64{50}}
	w.Players = []*sim.Player{{Index: 0, Crown: &sim.Crown{State: sim.CrownGround, X: 12}}}
	p := newPlanner(nil)
	p.plan(w, nil, view.Camera{}, 0)
	if len(p.top) == 0 {
		t.Fatal("keine Balken und keine Krone gezeichnet")
	}
	if !portalDown(w, 50) || portalDown(w, 10) {
		t.Error("portalDown erkennt zerstörte Portale falsch")
	}
}
