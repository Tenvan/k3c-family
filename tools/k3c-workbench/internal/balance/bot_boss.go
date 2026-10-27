package balance

import (
	"math"

	"k3c/engine/sim"
)

// Bosskampf (BS11.3, B-642): Im Insel-Lauf schlagen die Bots jeden Gegner in Reichweite und gehen auf den Boss los. Der
// Miniboss kommt mit seiner Welle zum Hub (die Truppen und Monarchen dort kämpfen); den Endboss weckt nur, wer sich dem Bau
// der tiefsten Stufe nähert (boss_endboss.go): Am hellen Tag geht ein gesunder Monarch zum Bau und schlägt auf den Boss ein.
// Fällt die Gesundheit unter retreatHP (Anteil), zieht er sich zur Burg zurück. Ist der Endboss besiegt
// (Island.EndbossDefeated), bleibt er bei seinem Profil.

const (
	retreatHP   = 0.35 // darunter zieht sich der Bot zur Burg zurück
	fightHP     = 0.7  // ab diesem Anteil geht er zum Bau des Endbosses
	bossHubRange = 40.0 // so nah am Hub gilt ein Boss als Verteidigungsfall, weiter weg geht der Bot nur mit Heer hin
	strikeRange = 1.4  // etwas unter monarch.json › attack.range (1,5): der Schlag trifft sicher
)

// withBossFight legt den Bosskampf über ein Profil; deepest ist die tiefste Tiefe der Insel.
func withBossFight(inner Bot, isl *sim.Island, deepest int) Bot {
	return func(w *sim.World, p *sim.Player) sim.PlayerCommand {
		cmd := inner(w, p)
		if p.HP < retreatHP*p.MaxHP {
			cmd = goTo(p, w.HubX, false)
		} else if x, ok := fightX(w, p, deepest, isl.EndbossDefeated); ok {
			cmd = goTo(p, x, false)
		}
		cmd.Attack = enemyWithin(w, p.X, strikeRange)
		return cmd
	}
}

// fightX: der Ort, zu dem der Bot zum Kampf läuft: ein lebender Boss der Stufe, sonst der Bau des Endbosses der
// tiefsten Stufe, solange der Endboss nicht besiegt ist und es hell ist.
func fightX(w *sim.World, p *sim.Player, deepest int, won bool) (float64, bool) {
	boss := nearestBoss(w, p)
	// Ein Boss weit vom Hub (Endboss am Bau, B-645): allein dorthin zu laufen kostet das Leben und das Gold (Kristallhöhle, Tag 34:
	// der Monarch starb im Minutentakt am Bau). Ohne Heer verteidigt er den Hub; der Boss kommt mit der Welle zu ihm.
	if boss != nil && w.Biome.Depth == deepest && (math.Abs(boss.X-w.HubX) <= bossHubRange || sim.EconomyOf(w).Fighters >= fighterWant) {
		return boss.X - math.Copysign(0.3, boss.X-p.X), true
	}
	// Erst die Armee: Seit dem Begleitschutz (unit_escort.go) kämpfen die Truppen am Boss mit, also baut der Bot sie vorher auf.
	if w.Biome.Depth != deepest || won || w.Cycle.Phase != "day" || p.HP < fightHP*p.MaxHP || sim.EconomyOf(w).Fighters < fighterWant {
		return 0, false
	}
	for _, e := range w.Level.Entities {
		if e.Kind == "lair" {
			return e.X, true
		}
	}
	return 0, false
}

// nearestBoss: der nächste lebende Boss der Stufe (nil: keiner).
func nearestBoss(w *sim.World, p *sim.Player) *sim.Enemy {
	var boss *sim.Enemy
	for _, e := range w.Enemies {
		if e.Boss && e.HP > 0 && (boss == nil || math.Abs(e.X-p.X) < math.Abs(boss.X-p.X)) {
			boss = e
		}
	}
	return boss
}

// enemyWithin: ein lebender Gegner liegt höchstens dist von x entfernt.
func enemyWithin(w *sim.World, x, dist float64) bool {
	for _, e := range w.Enemies {
		if e.HP > 0 && math.Abs(e.X-x) <= dist {
			return true
		}
	}
	return false
}
