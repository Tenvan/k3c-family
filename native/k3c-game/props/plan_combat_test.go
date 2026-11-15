package props

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func TestPayKeyIs(t *testing.T) {
	cases := []struct {
		key    string
		spot   sim.CombatSpot
		player int
		want   bool
	}{
		{"combat:shield:0", sim.CombatSpot{Kind: "shield"}, 0, true},
		{"combat:trigger:-1", sim.CombatSpot{Kind: "trigger", Side: -1}, 1, true},
		{"combat:trigger:1", sim.CombatSpot{Kind: "trigger", Side: -1}, 1, false},
		{"combat:banner:1:1", sim.CombatSpot{Kind: "banner", Side: 1}, 1, true},
		{"combat:banner:0:1", sim.CombatSpot{Kind: "banner", Side: 1}, 1, false},
		{"site:3", sim.CombatSpot{Kind: "shield"}, 0, false},
	}
	for _, c := range cases {
		if got := payKeyIs(c.key, c.spot, c.player); got != c.want {
			t.Errorf("payKeyIs(%q, %+v, %d) = %v, want %v", c.key, c.spot, c.player, got, c.want)
		}
	}
}

func TestCombatPlan(t *testing.T) {
	w := &sim.World{Portals: []float64{10, 30}, PortalHP: []float64{50, 100}, PortalMaxHP: 100, PortalsDown: []float64{50}}
	w.Players = []*sim.Player{{Index: 0, Crown: &sim.Crown{State: view.CrownGround, X: 12}}}
	p := newPlanner(nil)
	p.plan(w, nil, view.Camera{}, 0)
	if len(p.top) == 0 {
		t.Fatal("keine Balken und keine Krone gezeichnet")
	}
	if !portalDown(w, 50) || portalDown(w, 10) {
		t.Error("portalDown erkennt zerstörte Portale falsch")
	}
}
