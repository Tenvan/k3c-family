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
// Dev-Hilfen (Umgebung K3C_FULLGAME): „hilfen“ (Standard: Dev-Baukosten, voller Insel-Vorrat, Grad dev) oder „ohne“
// (Grad normal, nichts geschenkt: der offene Meilenstein der Balance, Fehler gelten dort nur als Hinweis im Log).

const fullgameDays = 40

// fullgame spielt einen Bot, bis done wahr ist oder fullgameDays vorbei sind; den Stand liefert die Insel.
func fullgame(t *testing.T, done func(*sim.Island) bool) (isl *sim.Island, day int, helps bool) {
	t.Helper()
	helps = os.Getenv("K3C_FULLGAME") != "ohne"
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
	}
	t.Log(line + " | Bosse " + strings.Join(isl.DefeatedBosses, ", "))
}
