package balance

import (
	"math"

	"k3c/engine/sim"
)

// crownRescueUnits: so weit läuft ein Bot mit Krone zur Krone eines Mitspielers.
const crownRescueUnits = 40.0

// retrieveCrown (B-931): Ein Bot ohne Krone läuft zur nächsten liegenden Krone (eigene oder fremde) und hebt sie so auf;
// trägt ein Bot eine fremde Krone, bringt er sie zum Besitzer. Sonst gilt sein Profil.
func retrieveCrown(w *sim.World, p *sim.Player, cmd sim.PlayerCommand) sim.PlayerCommand {
	var target *sim.Crown
	for _, q := range w.Players {
		if c := q.Crown; c != nil && c.State == "player" && c.Carrier == p.ID {
			return goTo(p, q.X, false) // Mitspieler trägt: zum Besitzer
		}
	}
	for _, q := range w.Players {
		if c := q.Crown; c != nil && c.State == "ground" && (target == nil || math.Abs(c.X-p.X) < math.Abs(target.X-p.X)) {
			target = c
		}
	}
	switch {
	case target == nil:
		if p.Crown != nil {
			return sim.PlayerCommand{}
		}
		return cmd
	case p.Crown != nil && math.Abs(target.X-p.X) > crownRescueUnits: // ein Bot mit Krone rettet nur nahe fremde Kronen
		return cmd
	}
	return goTo(p, target.X, false)
}
