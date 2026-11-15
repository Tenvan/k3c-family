package sim

import (
	"errors"
	"fmt"
	"maps"
	"slices"
)

// Etappen und Zielwahl am Hafen (B-961, docs/rules/welten.md › Inselwechsel): Inseln einer Etappe sind in beliebiger
// Reihenfolge anfahrbar, die nächste Etappe öffnet erst, wenn alle Inseln der laufenden abgeschlossen sind. Abgeschlossen
// ist eine Insel mit besiegtem Endboss beim Ablegen (Island.Completed, Insel-IDs). Der Raum reicht die Wahl über
// ChooseIsland durch; mehrere Wahlen im selben Takt entscheidet der höchste Spieler-Index (applyChoices).

// SwitchTarget ist ein Ziel am Hafen (Zustand `islandSwitch.targets`).
type SwitchTarget struct {
	ID     string `json:"id"`
	Name   string `json:"name"`
	Etappe int    `json:"etappe"`
}

// etappeOf: Etappe der Insel am Listenplatz i; ohne Angabe (0) gilt Listenplatz + 1.
func (w WorldDef) etappeOf(i int) int {
	if e := w.Islands[i].Etappe; e > 0 {
		return e
	}
	return i + 1
}

// done: Die Insel am Listenplatz i gilt als abgeschlossen (früher abgeschlossen oder die laufende nach dem Endboss).
func (isl *Island) done(i int) bool {
	id := isl.world.Islands[i].ID
	return slices.Contains(isl.Completed, id) || (i == isl.Number && isl.EndbossDefeated)
}

// targetIndexes sind die Listenplätze der Ziele: offene Inseln der Etappe der aktuellen Insel, sonst die Inseln der
// nächsten Etappe; leer = letzte Insel der Welt.
func (isl *Island) targetIndexes() []int { return isl.world.targetsFrom(isl.Number, isl.done) }

// targetsFrom wie targetIndexes für die Insel am Listenplatz cur; done sagt je Listenplatz, ob die Insel abgeschlossen
// gilt (reine Funktion, auch für die Wendepunkte der Geschichte, story_points.go).
func (w WorldDef) targetsFrom(cur int, done func(int) bool) []int {
	n := len(w.Islands)
	if cur < 0 || cur >= n {
		return nil
	}
	etappe := w.etappeOf(cur)
	var same, next []int
	for i := 0; i < n; i++ {
		if i == cur || done(i) {
			continue
		}
		switch w.etappeOf(i) {
		case etappe:
			same = append(same, i)
		case etappe + 1:
			next = append(next, i)
		}
	}
	if len(same) > 0 {
		return same
	}
	return next
}

// Targets sind die Ziele am Hafen in Listenreihenfolge.
func (isl *Island) Targets() []SwitchTarget {
	var out []SwitchTarget
	for _, i := range isl.targetIndexes() {
		d := isl.world.Islands[i]
		out = append(out, SwitchTarget{ID: d.ID, Name: d.Name, Etappe: isl.world.etappeOf(i)})
	}
	return out
}

// chosenIndex ist der Listenplatz des Ziels: die gewählte Insel, sonst das erste Ziel; ok ist false ohne Ziele.
func (isl *Island) chosenIndex() (int, bool) {
	idx := isl.targetIndexes()
	if len(idx) == 0 {
		return 0, false
	}
	for _, i := range idx {
		if isl.Chosen != "" && isl.world.Islands[i].ID == isl.Chosen {
			return i, true
		}
	}
	return idx[0], true
}

// ChosenTarget ist die ID des Ziels, das der Wechsel anfährt (Wahl oder erstes Ziel); leer ohne Ziele.
func (isl *Island) ChosenTarget() string {
	if i, ok := isl.chosenIndex(); ok {
		return isl.world.Islands[i].ID
	}
	return ""
}

// ChooseIsland ist die Wahl-Aktion eines gekrönten Spielers am geöffneten Wechselpunkt; nur aus den Zielen. Die Wahl
// wirkt im nächsten Inselschritt (stepIslandSwitch); eine Ablehnung lässt das Ziel unverändert.
func ChooseIsland(w *World, p *Player, islandID string) error {
	isl := w.island
	switch {
	case isl == nil || !isl.GateOpen || isl.SwitchReady:
		return errors.New("insel: Wechselpunkt nicht offen")
	case !crowned(p) || p.Free:
		return errors.New("insel: ohne Krone keine Wahl")
	case !slices.ContainsFunc(isl.Targets(), func(t SwitchTarget) bool { return t.ID == islandID }):
		return fmt.Errorf("insel: %q ist kein Ziel", islandID)
	}
	if isl.pendingChoice == nil {
		isl.pendingChoice = map[int]string{}
	}
	isl.pendingChoice[p.Index] = islandID
	return nil
}

// applyChoices übernimmt die Wahlen des Takts: bei mehreren gewinnt der höchste Spieler-Index (deterministisch).
func (isl *Island) applyChoices() {
	if len(isl.pendingChoice) == 0 {
		return
	}
	keys := slices.Sorted(maps.Keys(isl.pendingChoice))
	isl.Chosen = isl.pendingChoice[keys[len(keys)-1]]
	clear(isl.pendingChoice)
}
