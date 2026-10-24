package sim

import "math"

// Dieb-Linie (docs/rules/monarch.md § 3, B-577): Backstab und Smoke Bomb. Werte aus monarch.json › skills.

// castBackstab: Damage am nächsten lebenden Gegner in Range, BackDamage, wenn der Wirkende in dessen Rücken steht
// (behindEnemy). Ohne Gegner keine Wirkung.
func castBackstab(w *World, p *Player, e skillEffect) bool {
	en := nearest(w.Enemies, func(en *Enemy) float64 { return en.X }, p.X, e.Range, func(en *Enemy) bool { return en.HP > 0 })
	if en == nil {
		return false
	}
	damage := e.Damage
	if behindEnemy(w, en, p.X) {
		damage = e.BackDamage
	}
	applyDamage(w, en.ID, damage)
	return true
}

// enemyFacing ist die Blickrichtung eines Gegners (1 rechts, -1 links): Fliehende blicken zu ihrem Portal, alle übrigen
// (auch Wachen und Betäubte) zur Burg, in Richtung ihres Marschs.
func enemyFacing(w *World, en *Enemy) float64 {
	to := w.HubX
	if en.Fleeing {
		to = en.HomeX
	}
	if d := sign(to - en.X); d != 0 {
		return d
	}
	return 1
}

// behindEnemy: x liegt hinter dem Gegner, entgegen seiner Blickrichtung; dieselbe Stelle zählt als vorn.
func behindEnemy(w *World, en *Enemy, x float64) bool {
	return (x-en.X)*enemyFacing(w, en) < 0
}

// castStealth (Smoke Bomb): Duration Sekunden unsichtbar (StealthFor, shielded): Gegner wählen den Wirkenden nicht als
// Ziel und verlieren ihn, kein Schaden. Ein eigener Schlag oder ein anderer Skill beendet es vorzeitig (stepAttack,
// castSkill).
func castStealth(_ *World, p *Player, e skillEffect) bool {
	p.StealthFor = math.Max(p.StealthFor, e.Duration)
	return true
}
