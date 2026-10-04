package sim

import "math"

// Tank-Linie (docs/rules/monarch.md § 3): Taunt, Shield Bash, Iron Wall, Last Stand. Werte aus monarch.json › skills.

// castTaunt: Lebende Gegner im Radius zielen Duration Sekunden auf den Wirkenden, sofern sie ihn erreichen
// (chooseTarget).
func castTaunt(w *World, p *Player, e skillEffect) bool {
	for _, en := range w.Enemies {
		if en.HP > 0 && math.Abs(en.X-p.X) <= e.Radius {
			en.TauntID, en.TauntFor = p.ID, e.Duration
		}
	}
	return true
}

// castStun (Shield Bash): Der nächste lebende Gegner in Range ist Duration Sekunden betäubt. Ohne Gegner keine Wirkung.
func castStun(w *World, p *Player, e skillEffect) bool {
	en := nearest(w.Enemies, func(en *Enemy) float64 { return en.X }, p.X, e.Range, func(en *Enemy) bool { return en.HP > 0 })
	if en == nil {
		return false
	}
	en.Stun = e.Duration
	return true
}

// castShield (Iron Wall): Barriere aus HP Schild-Punkten auf dem Wirkenden für Duration Sekunden.
func castShield(_ *World, p *Player, e skillEffect) bool {
	p.Shield, p.ShieldFor = e.HP, e.Duration
	return true
}

// castLastStand: Duration Sekunden lang lässt ein tödlicher Treffer den Wirkenden mit 1 HP stehen (applyDamage).
func castLastStand(_ *World, p *Player, e skillEffect) bool {
	p.LastStandFor = e.Duration
	return true
}
