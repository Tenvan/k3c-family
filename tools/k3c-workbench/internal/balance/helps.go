package balance

import (
	"fmt"

	"k3c/engine/sim"
)

// Hilfen (BS3.4): Der Stufen-Test fullgame_test.go und die Balancing-Seite spielen dieselbe Insel mit demselben Bot; was der
// Test an Dev-Hilfen einschaltet, kann die Seite im Feld „Hilfen“ ebenfalls wählen (vorher unterschieden sich beide genau darin).
// Leer = keine Hilfen (Grad normal, nichts geschenkt: der offene Meilenstein der Balance). Hilfen gelten nur mit World.
const (
	HelpsAids = "hilfen" // Grad dev (falls keiner gesetzt), Dev-Baukosten, voller Insel-Vorrat
	HelpsFull = "voll"   // zusätzlich alle Stufen voll ausgebaut (Dev-Bau Stufe 5)
	HelpsBoss = "boss"   // zusätzlich schwacher Endboss (300 HP) und jeden Tick geheilte Monarchen (B-647 Weg D)
)

// HelpsNames sind die gültigen Werte von Scenario.Helps (ohne den leeren).
func HelpsNames() []string { return []string{HelpsAids, HelpsFull, HelpsBoss} }

// checkHelps prüft Wert und Zusammenhang mit der Welt.
func checkHelps(helps, world string) error {
	switch helps {
	case "":
		return nil
	case HelpsAids, HelpsFull, HelpsBoss:
		if world == "" {
			return fmt.Errorf("hilfen %q brauchen eine Welt (ganze Insel)", helps)
		}
		return nil
	}
	return fmt.Errorf("unbekannte hilfen %q (bekannt: %v)", helps, HelpsNames())
}

// gradeOf ist der Grad des Laufs: gesetzt wie angegeben, mit Hilfen sonst dev.
func gradeOf(sc Scenario) string {
	if sc.Grade == "" && sc.Helps != "" {
		return "dev"
	}
	return sc.Grade
}

// applyHelps schaltet die Dev-Hilfen des Szenarios auf der frischen Insel ein.
func applyHelps(isl *sim.Island, sc Scenario) {
	if sc.Helps == "" {
		return
	}
	sim.DevSetCheapBuild(isl, true)
	sim.DevFillStock(isl)
	if sc.Helps == HelpsFull || sc.Helps == HelpsBoss {
		sim.DevBuildAll(isl, 5)
	}
}

// afterStep läuft nach jedem Tick einer Insel-Welt: einmal je Sekunde lernt der Bot Skills (wie ein Mensch im Menü, bot_learn.go),
// bei Hilfen „boss“ wird der Endboss geschwächt und werden die Monarchen geheilt.
func afterStep(isl *sim.Island, sc Scenario, tick, tickHz int) {
	if sc.World == "" {
		return
	}
	if tick%tickHz == 0 {
		botLearn(isl)
	}
	if sc.Helps == HelpsBoss {
		weakenEndboss(isl)
		healMonarchs(isl)
	}
}

// weakenEndboss setzt den erschienenen Endboss auf 300 HP: Ohne das trifft ihn nur der Monarch (10 Schaden), er steht am Bau
// außerhalb der Reichweite der Truppen (B-647). Prüft so den Rest der Kette: Sieg, Belohnung, Insel-Zustand.
func weakenEndboss(isl *sim.Island) {
	for _, w := range isl.Stages {
		for _, e := range w.Enemies {
			if e.Boss && e.MaxHP > 300 && !miniBossKind[e.Kind] {
				e.HP, e.MaxHP = 300, 300
			}
		}
	}
}

// miniBossKind: Miniboss-IDs bleiben unberührt (nur der Endboss wird abgeschwächt).
var miniBossKind = map[string]bool{"goblinLeader": true, "trollKing": true, "ratQueen": true, "shardTitan": true, "lavaGolem": true}

// healMonarchs stellt die HP lebender Monarchen voll her (Dev-Hilfe ohne Skills, B-647 Weg D).
func healMonarchs(isl *sim.Island) {
	for _, w := range isl.Stages {
		for _, p := range w.Players {
			if p.HP > 0 {
				p.HP = p.MaxHP
			}
		}
	}
}
