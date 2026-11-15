package sim

import "math"

// Heiler-Linie (docs/rules/monarch.md § 3, kampf.md § 5): Heal und Group Heal heilen Truppen (Monarchen haben keine HP),
// Divine Shield ist der Schild aus skills_tank.go (castShield), Resurrection bringt gefallene Kronen zurück. Werte aus
// monarch.json › skills. Heilung geht nie über MaxHP.

// castHeal: Die Truppe mit dem kleinsten HP-Anteil in Range (Gleichstand: kleinere ID) bekommt Amount HP.
func castHeal(w *World, p *Player, e skillEffect) bool {
	var best *Troop
	for _, t := range w.Troops {
		if t.HP <= 0 || t.HP >= t.MaxHP || math.Abs(t.X-p.X) > e.Range {
			continue
		}
		if best == nil || t.HP/t.MaxHP < best.HP/best.MaxHP || (t.HP/t.MaxHP == best.HP/best.MaxHP && t.ID < best.ID) {
			best = t
		}
	}
	if best == nil {
		return false
	}
	best.HP = math.Min(best.MaxHP, best.HP+e.Amount)
	return true
}

// castGroupHeal: Alle Truppen im Radius um den Wirkenden bekommen Amount HP.
func castGroupHeal(w *World, p *Player, e skillEffect) bool {
	for _, t := range w.Troops {
		if t.HP > 0 && math.Abs(t.X-p.X) <= e.Radius {
			t.HP = math.Min(t.MaxHP, t.HP+e.Amount)
		}
	}
	return true
}

// castReviveCrown (Resurrection, Annahme aus kampf.md § 9): Jede gefallene Krone im Radius, die nicht ein Gegner trägt,
// sitzt sofort wieder auf ihrem Monarchen. Ohne Krone im Radius keine Wirkung.
func castReviveCrown(w *World, p *Player, e skillEffect) bool {
	hit := false
	for _, q := range w.Players {
		if c := q.Crown; c != nil && c.State != CrownEnemy && math.Abs(c.X-p.X) <= e.Radius {
			restoreCrown(w, q, p)
			hit = true
		}
	}
	return hit
}
