package sim

import "math"

// Zauberer-Linie (docs/rules/monarch.md § 3): Fireball, Ice Wall, Lightning Storm, Meteor. Werte aus monarch.json ›
// skills. Zielpunkt der Flächenzauber ist der nächste lebende Gegner in Range; ohne ihn keine Wirkung (keine
// Abklingzeit). Schaden geht über applyDamage, damit Tod, Gold-Drop und Ereignisse wie beim Schlag laufen.

// spellCenter: Lage des nächsten lebenden Gegners in Range um den Wirkenden; ok false, wenn keiner da ist.
func spellCenter(w *World, p *Player, e skillEffect) (float64, bool) {
	en := nearest(w.Enemies, func(en *Enemy) float64 { return en.X }, p.X, e.Range, func(en *Enemy) bool { return en.HP > 0 })
	if en == nil {
		return 0, false
	}
	return en.X, true
}

// areaEffects: effect.type der Flächenzauber, die um spellCenter wirken (Ort und Radius im Ereignis skillCast, B-674).
var areaEffects = map[string]bool{"areaDamage": true, "slow": true, "damageOverTime": true}

// castPlace: Wirkort (Units) und Radius für das Ereignis skillCast, vor dem Wirken bestimmt (stirbt das Ziel im selben
// Takt, bleibt der Ort). Flächenzauber am Ziel mit ihrem Radius, alle anderen Skills am Wirkenden mit Radius 0.
func castPlace(w *World, p *Player, e skillEffect) (float64, float64) {
	if areaEffects[e.Type] {
		if x, ok := spellCenter(w, p, e); ok {
			return x, e.Radius
		}
	}
	return p.X, 0
}

// enemiesAround ruft f für jeden lebenden Gegner im Radius r um x auf (Reihenfolge von w.Enemies).
func enemiesAround(w *World, x, r float64, f func(*Enemy)) {
	for _, en := range w.Enemies {
		if en.HP > 0 && math.Abs(en.X-x) <= r {
			f(en)
		}
	}
}

// castAreaDamage (Fireball, Meteor): Damage an alle Gegner im Radius um das Ziel.
func castAreaDamage(w *World, p *Player, e skillEffect) bool {
	x, ok := spellCenter(w, p, e)
	if ok {
		enemiesAround(w, x, e.Radius, func(en *Enemy) { applyDamage(w, en.ID, e.Damage) })
	}
	return ok
}

// castSlow (Ice Wall): Gegner im Radius um das Ziel laufen Duration Sekunden mit Factor der Geschwindigkeit.
func castSlow(w *World, p *Player, e skillEffect) bool {
	x, ok := spellCenter(w, p, e)
	if ok {
		enemiesAround(w, x, e.Radius, func(en *Enemy) { en.Slow, en.SlowFor = e.Factor, e.Duration })
	}
	return ok
}

// castDamageOverTime (Lightning Storm): ein Sturm um das Ziel, der Duration Sekunden lang PerSecond Schaden macht.
func castDamageOverTime(w *World, p *Player, e skillEffect) bool {
	x, ok := spellCenter(w, p, e)
	if ok {
		w.Storms = append(w.Storms, &Storm{X: x, Radius: e.Radius, PerSecond: e.PerSecond, Left: e.Duration, Owner: p.ID})
	}
	return ok
}

// stepStorms: Jeder Sturm trifft die Gegner in seinem Radius mit PerSecond × dt (im letzten Tick nur der Rest) und
// endet nach seiner Dauer. Reihenfolge: Wirk-Reihenfolge.
func stepStorms(w *World, dt float64) {
	kept := w.Storms[:0]
	for _, s := range w.Storms {
		damage := s.PerSecond * math.Min(dt, s.Left)
		enemiesAround(w, s.X, s.Radius, func(en *Enemy) { applyDamage(w, en.ID, damage) })
		if s.Left -= dt; s.Left > 1e-9 {
			kept = append(kept, s)
		}
	}
	w.Storms = kept
}
