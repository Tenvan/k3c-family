package props

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
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

// stubSheets liefert für jedes Bild ein 32x32-Sheet ohne Pixel (nur die Größe zählt).
type stubSheets struct{}

func (stubSheets) Sheet(asset, anim string) *art.Sheet {
	return &art.Sheet{Asset: asset, Anim: anim, FW: 32, FH: 32, OriginY: 1, UnitsPerPx: unitPx}
}

// Das Gestell (B-942) erscheint erst ab Hub-Stufe 2 und trägt dann je gebautem, nicht abgeholtem Schild ein Bild mehr.
func TestShieldRackStack(t *testing.T) {
	for _, tc := range []struct {
		hubLevel, shields, want int
	}{{1, 0, 0}, {2, 0, 1}} { // gebaute Schilde setzt nur die Sim (Test dort: TestGestellStapel…)
		w := &sim.World{HubLevel: tc.hubLevel, HubX: 100}
		p := newPlanner(stubSheets{})
		p.shieldRack(w)
		if len(p.body) != tc.want {
			t.Errorf("Stufe %d, %d Schilde: %d Bilder, erwartet %d", tc.hubLevel, tc.shields, len(p.body), tc.want)
		}
	}
}
