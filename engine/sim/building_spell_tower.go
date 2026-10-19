package sim

import "math"

// Zaubertum (B-116, Q31): Ein Turm ab Stufe spell.fromLevel schießt selbst, im Takt intervalSeconds auf den nächsten
// Gegner in rangeUnits, und trifft jeden Gegner im Radius radiusUnits um das Ziel mit damage
// (data/buildings.json › tower.spell). Er trägt keine Schützen (freeTower, stepArcher).

// isSpellTower: Ist der Platz ein gebauter Turm ab der Zaubertum-Stufe?
func isSpellTower(w *World, s *Site) bool {
	return s.Kind == "tower" && s.State == "built" && levelOf(w, s) >= buildings["tower"].Spell.FromLevel
}

func stepSpellTowers(w *World, dt float64) {
	sp := buildings["tower"].Spell
	for _, s := range w.Sites {
		if !isSpellTower(w, s) {
			continue
		}
		if s.spellIn = math.Max(0, s.spellIn-dt); s.spellIn > 0 {
			continue
		}
		var target *Enemy
		for _, e := range w.Enemies {
			if math.Abs(e.X-s.X) <= sp.RangeUnits && (target == nil || math.Abs(e.X-s.X) < math.Abs(target.X-s.X)) {
				target = e
			}
		}
		if target == nil {
			continue
		}
		hit := []int{}
		for _, e := range w.Enemies {
			if math.Abs(e.X-target.X) <= sp.RadiusUnits {
				hit = append(hit, e.ID)
			}
		}
		for _, id := range hit {
			applyDamage(w, id, sp.Damage)
		}
		s.spellIn = sp.IntervalSeconds
	}
}
