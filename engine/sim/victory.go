package sim

// Siegvarianten der Raum-Optionen (B-102, docs/rules/stufen.md § 3): Das Ziel der Insel (Options.Goal) beendet das
// Spiel mit dem Ereignis `victory`, genau einmal (Island.Won). Danach rechnet die Simulation weiter; was der Raum
// anzeigt, entscheidet das Protokoll (K4). Auf mehreren Inseln wirkt das Ziel mit K2.3a; bis dahin gilt: erfüllt → Sieg.

// checkVictory prüft das Ziel der Insel nach dem Tick aller Stufen und meldet den Sieg in Stufe 0.
func checkVictory(isl *Island) {
	if isl.Won || len(isl.Stages) == 0 || !goalReached(isl) {
		return
	}
	isl.Won = true
	w := isl.Stages[0]
	emit(w, "victory", Event{"goal": isl.Options.Goal, "day": w.Cycle.Day})
}

func goalReached(isl *Island) bool {
	switch isl.Options.Goal {
	case "endboss":
		return isl.EndbossDefeated
	case "gold":
		return isl.GoldCollected >= goals.Gold
	case "days":
		return isl.Stages[0].Cycle.Day > goals.Days
	case "mineAll":
		return allMined(isl)
	case "buildAll":
		return allBuilt(isl)
	}
	return false
}

// allMined: Kein endliches Ressourcenobjekt steht mehr. Endlich sind Objekte aus dem Level, nicht Adern (vein.go) und
// nicht Plantage-Bäume (plantation.go). Eine Insel, auf der nie eines stand, gewinnt nie (hadFinite).
func allMined(isl *Island) bool {
	for _, w := range isl.Stages {
		planted := plantedTrees(w)
		for _, n := range w.Nodes {
			if !isVein(n) && !planted[n] {
				isl.hadFinite = true
				return false
			}
		}
	}
	return isl.hadFinite
}

// plantedTrees sind die Bäume der Plantagen einer Stufe.
func plantedTrees(w *World) map[*ResourceNode]bool {
	m := map[*ResourceNode]bool{}
	for _, s := range w.Sites {
		for _, p := range s.plots {
			if p.tree != nil {
				m[p.tree] = true
			}
		}
	}
	return m
}

// allBuilt: Alle Bauplätze aller Stufen sind gebaut; eine Insel ohne Bauplatz gewinnt nie.
func allBuilt(isl *Island) bool {
	n := 0
	for _, w := range isl.Stages {
		for _, s := range w.Sites {
			if s.State != "built" {
				return false
			}
			n++
		}
	}
	return n > 0
}
