package balance

import "k3c/engine/sim"

// Skills lernen (BS3.4, B-647 Weg B): Ein Mensch lernt Skills im Menü (Aktion `learn`, `sim.LearnSkill`), das ist keine
// Eingabe des Monarchen (`sim.PlayerCommand`). Der Tester gibt dem Bot dieselbe Aktion: Hat ein Monarch einen freien
// Skill-Punkt (Pool aus Bossen und Truppen-Funden, `monarch.md` § 3), lernt er den nächsten Skill der Reihenfolge
// learnOrder, den die Regeln erlauben. Die Reihenfolge ist die der Tank-Linie: erst die beiden Aktiven (öffnen Tier 2), dann
// Regeneration, mehr HP und Verteidigung; der Bot überlebt damit längere Kämpfe, ohne dass die Spielregeln geändert werden.

// learnOrder ist die Rangfolge der Skills, die ein Bot lernt.
var learnOrder = []string{"taunt", "shieldBash", "regeneration", "thickSkin", "fortified", "armorAura", "guardian", "ironWall", "lastStand"}

// botLearn lässt jeden Monarchen der Insel mit freien Punkten Skills lernen; deterministisch, ohne Zufall.
func botLearn(isl *sim.Island) {
	for _, p := range isl.Players() {
		w := isl.Stages[isl.StageOf(p.Index)]
		for sim.AvailablePoints(w, p) > 0 {
			if !learnNext(w, p) {
				break
			}
		}
	}
}

// learnNext lernt den ersten erlaubten Skill aus learnOrder; false, wenn keiner mehr geht.
func learnNext(w *sim.World, p *sim.Player) bool {
	for _, id := range learnOrder {
		if sim.CanLearn(w, p, id) == nil {
			return sim.LearnSkill(w, p, id) == nil
		}
	}
	return false
}
