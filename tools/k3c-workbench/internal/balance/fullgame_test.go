//go:build fullgame

package balance

import (
	"os"
	"strconv"
	"strings"
	"testing"

	"k3c/engine/sim"
	"k3c/tools/k3c-workbench/internal/enginetools"
)

// Vollständiges Spiel mit einem Bot (BS3.4, 🧑-Wunsch): Ein einzelner Bot spielt die Insel 1 Stufe für Stufe bis zum Sieg
// über den Endboss. Die Tests sichern, dass die Simulation durchspielbar ist, unabhängig von der Balance der Münzskala.
//
// Aufbau: Jede Stufe ist ein eigener Test (TestFullGame1…), jede baut auf der vorigen auf und verlangt etwas mehr. Es läuft
// immer nur der aktuelle (`task balance:fullgame STUFE=3`, Standard in Taskfile.yml); neue Stufen kommen unten dazu.
// Bewusst NICHT Teil von `task check:diff` (Build-Tag `fullgame`, läuft Minuten).
//
// „boss“ (Weg D aus B-647) setzt zusätzlich die HP des Endbosses auf 300, sobald er erscheint (er steht am Bau, die Truppen
// erreichen ihn nicht), und heilt die Monarchen jeden Tick (ein Bot ohne Skills regeneriert nicht).
// Dev-Hilfen (Umgebung K3C_FULLGAME): „hilfen“ (Dev-Baukosten, voller Insel-Vorrat, Grad dev), „voll“ (zusätzlich
// alle Stufen voll ausgebaut, Dev-Bau Stufe 5), „boss“ (Standard, alles davor) oder „ohne“
// (Grad normal, nichts geschenkt: der offene Meilenstein der Balance, Fehler gelten dort nur als Hinweis im Log).

const fullgameDays = 40

// fullgame spielt einen Bot, bis done wahr ist oder fullgameDays vorbei sind; den Stand liefert die Insel.
func fullgame(t *testing.T, done func(*sim.Island) bool) (isl *sim.Island, day int, helps bool) {
	t.Helper()
	mode := os.Getenv("K3C_FULLGAME")
	if mode == "" {
		mode = "boss" // alle Hilfen: der Nachweis, dass die Simulation durchspielbar ist (B-647 Weg D)
	}
	helps = mode != "ohne"
	grade := ""
	if helps {
		grade = "dev"
	}
	isl, err := newIsland(Scenario{Seed: "k3c-dev", Players: 1, Days: fullgameDays, World: "standard", Grade: grade})
	if err != nil {
		t.Fatal(err)
	}
	if helps {
		sim.DevSetCheapBuild(isl, true)
		sim.DevFillStock(isl)
		if mode == "voll" || mode == "boss" { // zusätzlich alle Stufen der Insel voll ausgebaut (Hub 5, Mauern, Türme)
			sim.DevBuildAll(isl, 5)
		}
	}
	bot := withBossFight(withDescent(wirtschaft, isl, deepestOf(isl)), isl, deepestOf(isl))
	cmds := make([]sim.PlayerCommand, 1)
	last := 0
	for tick := 1; tick <= fullgameDays*maxTicksPerDay && !done(isl); tick++ {
		for _, w := range isl.Stages {
			for _, p := range w.Players {
				cmds[p.Index] = bot(w, p)
			}
		}
		sim.StepIsland(isl, cmds, 1.0/enginetools.TickHz)
		if tick%enginetools.TickHz == 0 {
			botLearn(isl)
		}
		if mode == "boss" {
			weakenEndboss(isl)
			healMonarchs(isl)
		}
		if d := isl.Stages[0].Cycle.Day; d != last {
			last = d
			logFullgameDay(t, isl, d)
		}
		if isl.Stages[0].Cycle.Day > fullgameDays {
			break
		}
	}
	return isl, isl.Stages[0].Cycle.Day, helps
}

// verdict meldet ein Ergebnis: mit Hilfen ein Fehler, ohne nur ein Hinweis im Log.
func verdict(t *testing.T, ok bool, helps bool, msg string) {
	t.Helper()
	switch {
	case ok:
		t.Log("✅ " + msg)
	case helps:
		t.Error("❌ " + msg)
	default:
		t.Log("⚪ " + msg + " (ohne Hilfen: offener Meilenstein, kein Fehler)")
	}
}

// Stufe 1: Der Miniboss des Waldes (Goblin-Anführer) fällt.
func TestFullGame1MinibossWald(t *testing.T) {
	isl, day, helps := fullgame(t, func(i *sim.Island) bool { return bossDown(i, "goblinLeader") })
	verdict(t, bossDown(isl, "goblinLeader"), helps, "Goblin-Anführer besiegt, Tag "+strconv.Itoa(day))
}

// Stufe 2: Der Spieler erreicht die tiefste Stufe (Kristallhöhle).
func TestFullGame2TiefsteStufe(t *testing.T) {
	isl, day, helps := fullgame(t, func(i *sim.Island) bool { return i.StageOf(0) == len(i.Stages)-1 })
	verdict(t, isl.StageOf(0) == len(isl.Stages)-1, helps, "tiefste Stufe erreicht, Tag "+strconv.Itoa(day))
}

// Stufe 3: Alle vier Minibosse der Insel fallen.
func TestFullGame3AlleMinibosse(t *testing.T) {
	minis := []string{"goblinLeader", "trollKing", "ratQueen", "lavaGolem"}
	all := func(i *sim.Island) bool {
		for _, id := range minis {
			if !bossDown(i, id) {
				return false
			}
		}
		return true
	}
	isl, day, helps := fullgame(t, all)
	verdict(t, all(isl), helps, "alle Minibosse besiegt, Tag "+strconv.Itoa(day)+" (Bosse: "+strings.Join(isl.DefeatedBosses, ", ")+")")
}

// Stufe 4: Der Endboss der Insel 1 fällt (komplettes Spiel).
func TestFullGame4Endboss(t *testing.T) {
	isl, day, helps := fullgame(t, func(i *sim.Island) bool { return i.EndbossDefeated })
	verdict(t, isl.EndbossDefeated, helps, "Endboss besiegt, Tag "+strconv.Itoa(day)+" (Bosse: "+strings.Join(isl.DefeatedBosses, ", ")+")")
}

func bossDown(isl *sim.Island, id string) bool {
	for _, b := range isl.DefeatedBosses {
		if b == id {
			return true
		}
	}
	return false
}

// logFullgameDay schreibt zu Tagesbeginn, wo der Spieler steht und wie die Stufen aussehen.
func logFullgameDay(t *testing.T, isl *sim.Island, day int) {
	t.Helper()
	line := "Tag " + strconv.Itoa(day) + ": Spieler auf Stufe " + strconv.Itoa(isl.StageOf(0))
	for i, w := range isl.Stages {
		e := sim.EconomyOf(w)
		line += " | S" + strconv.Itoa(i) + " Hub " + strconv.Itoa(e.HubLevel) + " Kämpfer " + strconv.Itoa(e.Fighters)
		if w.Aggression != nil {
			line += " (Pool " + strconv.Itoa(int(*w.Aggression)) + " Welle " + strconv.Itoa(w.Wave) + " Gegner " + strconv.Itoa(len(w.Enemies)) + ")"
		}
		for _, en := range w.Enemies {
			if en.Boss && en.HP > 0 {
				line += " [" + en.Kind + " " + strconv.Itoa(int(en.HP)) + "/" + strconv.Itoa(int(en.MaxHP)) + " x" + strconv.Itoa(int(en.X)) + "]"
			}
		}
	}
	for _, w := range isl.Stages {
		for _, p := range w.Players {
			line += " | Monarch HP " + strconv.Itoa(int(p.HP)) + " Gold " + strconv.Itoa(p.Gold) + " x" + strconv.Itoa(int(p.X)) + " (Hub x" + strconv.Itoa(int(w.HubX)) + ")"
		}
	}
	t.Log(line + " | Bosse " + strings.Join(isl.DefeatedBosses, ", "))
}

// weakenEndboss setzt den erschienenen Endboss auf 300 HP: Ohne das trifft ihn nur der Monarch (10 Schaden), er steht am Bau
// außerhalb der Reichweite der Truppen (B-647). Prüft so den Rest der Kette: Sieg, Belohnung, Insel-Zustand.
func weakenEndboss(isl *sim.Island) {
	for _, w := range isl.Stages {
		for _, e := range w.Enemies {
			if e.Boss && e.MaxHP > 300 && !bossDownKind(e.Kind) {
				e.HP, e.MaxHP = 300, 300
			}
		}
	}
}

// bossDownKind: Miniboss-IDs bleiben unberührt (nur der Endboss wird abgeschwächt).
func bossDownKind(kind string) bool { return miniBossKind[kind] }

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
