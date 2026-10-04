package sim

import (
	"fmt"
	"math"
	"slices"
)

// Monarch: Schlag, Fund-Pool, Verteilen, Respec und Presets (docs/rules/monarch.md §§ 1–3, B-118). Ein Punkt kauft
// einen Skill oder ein Passiv, jeden höchstens einmal. Verfügbar sind Pool der Insel minus gelernte Skills; wer spät
// beitritt, hat damit alle bisherigen Punkte.

// stepAttack: Schlag (Taste X) auf den nächsten lebenden Gegner in Reichweite, beide Seiten. Ohne Gegner kein Effekt
// und keine Abklingzeit. Nur für lebende Monarchen aufgerufen.
func stepAttack(w *World, p *Player, cmd PlayerCommand, dt float64) {
	p.AttackCooldown = math.Max(0, p.AttackCooldown-dt)
	if !cmd.Attack || p.AttackCooldown > 0 {
		return
	}
	a := monarch.Attack
	e := nearest(w.Enemies, func(e *Enemy) float64 { return e.X }, p.X, a.Range, func(e *Enemy) bool { return e.HP > 0 })
	if e == nil {
		return
	}
	p.AttackCooldown = a.Cooldown
	emit(w, "strike", Event{"from": p.ID, "x": unitX(p.X)})
	applyDamage(w, e.ID, a.Damage)
}

// poolOf ist der Fund-Pool: der Insel, ohne Insel der Zähler der Welt.
func poolOf(w *World) int {
	if w.island != nil {
		return w.island.SkillPool
	}
	return w.SkillPoints
}

// addPoolPoints ist der einzige Weg, Pool-Punkte zu vergeben (Verstecke, Truhen; später Bosse und Meilensteine).
// Alle Stufen der Insel spiegeln den Pool nach World.SkillPoints. Bis der Pool im Spielstand steht (S1.4), senkt das
// Spiegeln keinen höheren Zähler einer Stufe aus einem geladenen Stand (dort gab es nur Zähler je Stufe).
func addPoolPoints(w *World, n int) {
	isl := w.island
	if isl == nil {
		w.SkillPoints += n
		return
	}
	isl.SkillPool += n
	for _, s := range isl.Stages {
		s.SkillPoints = max(s.SkillPoints, isl.SkillPool)
	}
}

// chestSkillPoint zählt eine geöffnete Truhe der Insel; jede n-te (skillPointSources.chestEvery) gibt einen Punkt.
func chestSkillPoint(w *World, p *Player) {
	isl, every := w.island, monarch.SkillPointSources.ChestEvery
	if isl == nil || every <= 0 {
		return
	}
	isl.ChestsOpened++
	if isl.ChestsOpened%every == 0 {
		addPoolPoints(w, 1)
		emit(w, "skillPoint", Event{"player": p.Index, "total": w.SkillPoints})
	}
}

// AvailablePoints sind die Punkte, die der Spieler noch verteilen kann.
func AvailablePoints(w *World, p *Player) int { return poolOf(w) - len(p.Skills) }

// lineCount zählt die gelernten Skills einer Linie.
func lineCount(p *Player, line string) int {
	n := 0
	for _, id := range p.Skills {
		if s, ok := skillByID(id); ok && s.Line == line {
			n++
		}
	}
	return n
}

// tierNeed: gelernte Skills der Linie, die Tier `tier` verlangt.
func tierNeed(tier int) int {
	if tier < 1 || tier > len(monarch.TierPoints) {
		return math.MaxInt
	}
	return monarch.TierPoints[tier-1]
}

// LearnSkill lernt einen Skill für einen Punkt. Aktive Skills belegen den ersten freien Slot 1 bis 4; ohne freien Slot
// bleibt der Skill gelernt ohne Slot.
func LearnSkill(w *World, p *Player, id string) error {
	s, ok := skillByID(id)
	switch {
	case !ok:
		return fmt.Errorf("skill %q unbekannt", id)
	case slices.Contains(p.Skills, id):
		return fmt.Errorf("skill %q schon gelernt", id)
	case AvailablePoints(w, p) <= 0:
		return fmt.Errorf("skill %q: keine Punkte frei", id)
	case lineCount(p, s.Line) < tierNeed(s.Tier):
		return fmt.Errorf("skill %q: Tier %d verlangt %d gelernte Skills der Linie %s", id, s.Tier, tierNeed(s.Tier), s.Line)
	}
	p.Skills = append(p.Skills, id)
	if s.Kind != "active" {
		return nil
	}
	if i := slices.Index(p.Slots, ""); i >= 0 {
		p.Slots[i] = id
	} else if len(p.Slots) < 4 {
		p.Slots = append(p.Slots, id)
	}
	return nil
}

// Respec setzt alle Skills zurück, kostenlos, nur am Tag und an der Burg.
func Respec(w *World, p *Player) error {
	if w.Cycle.Phase != "day" {
		return fmt.Errorf("respec nur am Tag (jetzt %s)", w.Cycle.Phase)
	}
	if math.Abs(p.X-w.Castle.X) > hub.CastleRadiusUnits {
		return fmt.Errorf("respec nur an der Burg")
	}
	p.Skills, p.Slots = nil, nil
	return nil
}

// ApplyPreset lernt die activeSkills eines Presets in ihrer Reihenfolge, soweit Pool-Punkte frei sind. Skills, die
// nicht lernbar sind (z. B. Dieb-Skills ohne Katalog-Eintrag), werden übersprungen.
func ApplyPreset(w *World, p *Player, id string) error {
	pr, ok := monarch.Presets[id]
	if !ok {
		return fmt.Errorf("preset %q unbekannt", id)
	}
	for _, s := range pr.ActiveSkills {
		if AvailablePoints(w, p) <= 0 {
			break
		}
		_ = LearnSkill(w, p, s)
	}
	return nil
}

// presetFor ist das Standard-Preset beim Beitritt (monarch.md § 2): Tank für Spieler 1, Zauberer für Spieler 2, sonst Heiler.
func presetFor(index int) string {
	switch index {
	case 0:
		return "tank"
	case 1:
		return "mage"
	}
	return "healer"
}
