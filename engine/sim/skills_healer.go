package sim

import "math"

// Heiler-Linie (docs/rules/monarch.md § 3): Heal, Group Heal, Divine Shield, Resurrection. Werte aus monarch.json ›
// skills. Divine Shield ist der Schild aus skills_tank.go (castShield). Heilung geht nie über MaxHP.

// castHeal: Der lebende Monarch mit den wenigsten HP in Range (auch der Wirkende; Gleichstand: kleinerer Index)
// bekommt Amount HP.
func castHeal(w *World, p *Player, e skillEffect) bool {
	var best *Player
	for _, q := range w.Players {
		if !isAlive(q) || math.Abs(q.X-p.X) > e.Range {
			continue
		}
		if best == nil || q.HP < best.HP || (q.HP == best.HP && q.Index < best.Index) {
			best = q
		}
	}
	if best == nil {
		return false
	}
	best.HP = math.Min(best.MaxHP, best.HP+e.Amount)
	return true
}

// castGroupHeal: Alle lebenden Monarchen und Truppen im Radius um den Wirkenden bekommen Amount HP.
func castGroupHeal(w *World, p *Player, e skillEffect) bool {
	for _, q := range w.Players {
		if isAlive(q) && math.Abs(q.X-p.X) <= e.Radius {
			q.HP = math.Min(q.MaxHP, q.HP+e.Amount)
		}
	}
	for _, t := range w.Troops {
		if t.HP > 0 && math.Abs(t.X-p.X) <= e.Radius {
			t.HP = math.Min(t.MaxHP, t.HP+e.Amount)
		}
	}
	return true
}

// castReviveTroops (Resurrection, Auslegung S1.2b): Truppen im Radius haben danach mindestens HPFraction ihrer MaxHP.
// Gefallene Truppen gibt es nicht mehr als Objekt (Step entfernt sie). Ohne Truppe im Radius keine Wirkung.
func castReviveTroops(w *World, p *Player, e skillEffect) bool {
	hit := false
	for _, t := range w.Troops {
		if t.HP > 0 && math.Abs(t.X-p.X) <= e.Radius {
			t.HP = math.Max(t.HP, t.MaxHP*e.HPFraction)
			hit = true
		}
	}
	return hit
}
