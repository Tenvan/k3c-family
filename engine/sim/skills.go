package sim

import (
	"math"
	"slices"
)

// Aktive Skills (docs/rules/monarch.md § 3, B-119): Der Befehl `Skill` löst den Skill in Slot 1 bis 4 aus. Die Wirkung
// steht in monarch.json › skills › effect; effect.type wählt die Funktion. Abklingzeiten laufen je Spieler und Slot.

// skillEffects: Wirkung je effect.type, nur zum Nachschlagen (nie darüber iterieren). Rückgabe false = ohne Wirkung,
// dann startet keine Abklingzeit.
var skillEffects = map[string]func(w *World, p *Player, e skillEffect) bool{
	"taunt":     castTaunt,
	"stun":      castStun,
	"shield":    castShield,
	"lastStand": castLastStand,
	// Zauberer (skills_caster.go) und Heiler (skills_healer.go); Divine Shield nutzt "shield".
	"areaDamage":     castAreaDamage,
	"slow":           castSlow,
	"damageOverTime": castDamageOverTime,
	"heal":           castHeal,
	"groupHeal":      castGroupHeal,
	"reviveTroops":   castReviveTroops,
}

// stepSkills senkt Abklingzeiten und Skill-Effekte des Spielers und löst den Slot aus cmd.Skill aus. Nur für lebende
// Monarchen aufgerufen.
func stepSkills(w *World, p *Player, cmd PlayerCommand, dt float64) {
	for i, cd := range p.Cooldowns {
		p.Cooldowns[i] = math.Max(0, cd-dt)
	}
	p.ShieldFor = math.Max(0, p.ShieldFor-dt)
	if p.ShieldFor == 0 {
		p.Shield = 0
	}
	p.LastStandFor = math.Max(0, p.LastStandFor-dt)
	if cmd.Skill >= 1 && cmd.Skill <= 4 {
		castSkill(w, p, cmd.Skill-1)
	}
}

// castSkill wirkt den Skill in Slot slot (0 bis 3): Slot belegt, Skill gelernt und Abklingzeit 0, sonst nichts.
func castSkill(w *World, p *Player, slot int) {
	if slot >= len(p.Slots) || !slices.Contains(p.Skills, p.Slots[slot]) {
		return
	}
	if slot < len(p.Cooldowns) && p.Cooldowns[slot] > 0 {
		return
	}
	s, ok := skillByID(p.Slots[slot])
	cast := skillEffects[s.Effect.Type]
	e, cd := s.Effect, s.Cooldown
	mult := damageMultOf(w, p, true) // Passive (passives.go): Zauberschaden, Abklingzeit, Spell Echo
	e.Damage, e.PerSecond = float64(e.Damage*mult), float64(e.PerSecond*mult)
	if !ok || cast == nil {
		return
	}
	if !cast(w, p, e) {
		emit(w, "castFailed", Event{"from": p.ID, "slot": slot, "x": unitX(p.X)}) // kein Ziel: keine Abklingzeit (B-321)
		return
	}
	if s.Line == "mage" {
		cd = float64(cd * cooldownMultOf(w, p))
		if spellEchoes(w, p) {
			cast(w, p, e)
		}
	}
	if p.Cooldowns == nil {
		p.Cooldowns = make([]float64, 4)
	}
	p.Cooldowns[slot] = cd
}
