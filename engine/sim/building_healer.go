package sim

import "math"

// Heilplatz (B-116, Q32, Q63): Ein gebauter Heilplatz heilt Bürger und Truppen im Radius (Monarchen haben keine HP, B-930), immer, auch im
// Kampf, bis MaxHP (data/buildings.json › healer.heal). Sonst gibt es keine Regeneration (buerger.md § 3).

func stepHealing(w *World, dt float64) {
	h := w.values().Buildings["healer"].Heal
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
	}
}
