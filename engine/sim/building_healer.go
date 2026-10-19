package sim

import "math"

// Heilplatz (B-116, Q32, Q63): Ein gebauter Heilplatz heilt Bürger und lebende Spieler im Radius, immer, auch im
// Kampf, bis MaxHP (data/buildings.json › healer.heal). Sonst gibt es keine Regeneration (buerger.md § 3).

func stepHealing(w *World, dt float64) {
	h := buildings["healer"].Heal
	amount := h.HPPerSecond * dt
	for _, s := range w.Sites {
		if s.Kind != "healer" || s.State != "built" {
			continue
		}
		for _, t := range w.Troops {
			if math.Abs(t.X-s.X) <= h.RadiusUnits {
				t.HP = math.Min(t.MaxHP, t.HP+amount)
			}
		}
		for _, p := range w.Players {
			if isAlive(p) && math.Abs(p.X-s.X) <= h.RadiusUnits {
				p.HP = math.Min(p.MaxHP, p.HP+amount)
			}
		}
	}
}
