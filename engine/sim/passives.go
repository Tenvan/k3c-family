package sim

import "math"

// Passive Skills (docs/rules/monarch.md § 3, B-119, S1.2c): Gelernte Passive wirken immer (kein Slot, keine
// Abklingzeit). Werte aus monarch.json › skills › effect, alle vorläufig. Gleiche Effekte mehrerer Besitzer addieren sich
// je Besitzer. Eigenschaften werden berechnet, nicht im Zustand gehalten; ohne gelernte Passive liefern die Funktionen
// genau die Basiswerte (Golden-Läufe unverändert). Nur lebende Besitzer wirken.

// eachPassive ruft f für jedes gelernte Passiv jedes lebenden Spielers: Spieler nach Index, Skills in Lernreihenfolge.
func eachPassive(w *World, f func(owner *Player, e skillEffect)) {
	for _, o := range w.Players {
		if !isAlive(o) {
			continue
		}
		for _, id := range o.Skills {
			if s, ok := skillByID(id); ok && s.Kind == "passive" {
				f(o, s.Effect)
			}
		}
	}
}

// statSum addiert field über alle stat-Passive, die den Monarchen p erreichen (Aura um den Besitzer, ohne Aura nur
// der Besitzer selbst).
func statSum(w *World, p *Player, field func(skillEffect) float64) float64 {
	sum := 0.0
	eachPassive(w, func(o *Player, e skillEffect) {
		if e.Type == "stat" && (o == p || e.Aura > 0 && math.Abs(o.X-p.X) <= e.Aura) {
			sum += field(e)
		}
	})
	return sum
}

// defenseOf: Verteidigung des Monarchen (Basis + Armor Aura, Fortified).
func defenseOf(w *World, p *Player) float64 {
	return monarch.Base.Defense + statSum(w, p, func(e skillEffect) float64 { return e.Defense })
}

// maxHPOf: maximale HP des Monarchen (Basis × (1 + Thick Skin)).
func maxHPOf(w *World, p *Player) float64 {
	return float64(monarch.Base.HP * (1 + statSum(w, p, func(e skillEffect) float64 { return e.MaxHPBonus })))
}

// speedOf: Grundtempo des Monarchen (Basis × (1 + Blessings)).
func speedOf(w *World, p *Player) float64 {
	return float64(monarch.Base.Speed * (1 + statSum(w, p, func(e skillEffect) float64 { return e.SpeedBonus })))
}

// damageMultOf: Faktor auf Schlag- und Zauberschaden (Blessings); spell = Zauber, dann zusätzlich Arcane Power.
func damageMultOf(w *World, p *Player, spell bool) float64 {
	return 1 + statSum(w, p, func(e skillEffect) float64 {
		if spell {
			return e.DamageBonus + e.SpellBonus
		}
		return e.DamageBonus
	})
}

// cooldownMultOf: Faktor auf die Abklingzeit der Zauberer-Skills (Elemental Mastery).
func cooldownMultOf(w *World, p *Player) float64 {
	return 1 + statSum(w, p, func(e skillEffect) float64 { return e.CooldownBonus })
}

// troopDamage: Schaden an einer Truppe nach Verteidigung aus Auren (Armor Aura, mindestens 1) und Guardian (je
// Besitzer im Radius Reduction weniger).
func troopDamage(w *World, t *Troop, damage float64) float64 {
	def, cut := 0.0, 0.0
	eachPassive(w, func(o *Player, e skillEffect) {
		d := math.Abs(o.X - t.X)
		switch {
		case e.Type == "stat" && e.Aura > 0 && d <= e.Aura:
			def += e.Defense
		case e.Type == "guardian" && d <= e.Radius:
			cut += e.Reduction
		}
	})
	if def > 0 {
		damage = math.Max(1, damage-def)
	}
	return float64(damage * math.Max(0, 1-cut))
}

// stepPassives (je Tick nach stepPlayers): MaxHP folgt maxHPOf (Heilung deckelt daran), dann Regeneration, Healing
// Aura und Holy Ground. Reihenfolge: Besitzer nach Index, je Besitzer Spieler nach Index, dann Truppen.
func stepPassives(w *World, dt float64) {
	for _, p := range w.Players {
		p.MaxHP = maxHPOf(w, p)
		p.HP = math.Min(p.HP, p.MaxHP)
	}
	eachPassive(w, func(o *Player, e skillEffect) {
		if e.Type != "regen" {
			return
		}
		heal := float64(e.PerSecond * dt)
		for _, q := range w.Players {
			if isAlive(q) && regenReaches(w, o, e, q.X, q == o) {
				q.HP = math.Min(q.MaxHP, q.HP+heal)
			}
		}
		for _, t := range w.Troops {
			if t.HP > 0 && regenReaches(w, o, e, t.X, false) {
				t.HP = math.Min(t.MaxHP, t.HP+heal)
			}
		}
	})
}

// regenReaches: Castle = alle im Burgradius (Holy Ground, Auslegung), Aura = im Radius um den Besitzer, sonst nur
// der Besitzer.
func regenReaches(w *World, o *Player, e skillEffect, x float64, self bool) bool {
	switch {
	case e.Castle:
		return math.Abs(x-w.Castle.X) <= hub.CastleRadiusUnits
	case e.Aura > 0:
		return math.Abs(x-o.X) <= e.Aura
	}
	return self
}

// frostArmorHit: Trifft Gegner en den Monarchen p im Nahkampf, verlangsamt Frost Armor ihn (Feld wie Ice Wall). Eine
// stärkere laufende Verlangsamung bleibt.
func frostArmorHit(p *Player, en *Enemy) {
	for _, id := range p.Skills {
		s, ok := skillByID(id)
		if !ok || s.Kind != "passive" || s.Effect.Type != "frostArmor" {
			continue
		}
		if en.SlowFor == 0 || s.Effect.Factor <= en.Slow {
			en.Slow, en.SlowFor = s.Effect.Factor, s.Effect.Duration
		}
	}
}

// spellEchoes: Wirkt ein Zauber ein zweites Mal? Würfelt nur, wenn der Wirkende Spell Echo gelernt hat; sonst wird
// kein Wert aus w.rng verbraucht (Golden-Läufe unverändert).
func spellEchoes(w *World, p *Player) bool {
	chance := 0.0
	for _, id := range p.Skills {
		if s, ok := skillByID(id); ok && s.Kind == "passive" && s.Effect.Type == "spellEcho" {
			chance += s.Effect.Chance
		}
	}
	return chance > 0 && w.rng.Next() < chance
}
