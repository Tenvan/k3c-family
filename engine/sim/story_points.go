package sim

import (
	"slices"
	"strconv"
)

// Wendepunkte der Geschichte (B-965/R6, docs/rules/welten.md § 6.2): reine Funktionen, die zu einem Moment der Kampagne die
// Zeilen nennen. Ohne Zustand, ohne Zufall, ohne neue Ereignisse; das Spiel ruft sie bei den vorhandenen Ereignissen
// (`islandSwitch`, `bossSpawned`, `bossDefeated`) und blendet das Ergebnis ein. Das Laden eines Spielstands ruft sie nicht.
// Die Zeilen gelten für alle Spieler der Insel gleich. Ergebnis: alle Zeilen in Reihenfolge (Einträge hintereinander),
// leer = keine Zeile; die Slices sind frisch, der Aufrufer darf sie ändern.

// StoryStart: Neue Kampagne, `start` der Welt, danach `arrival` der Startinsel (erste Insel der Welt).
func (w WorldDef) StoryStart() []string {
	if len(w.Islands) == 0 {
		return slices.Clone(w.Story.Start)
	}
	return concat(w.Story.Start, w.Islands[0].Story.Arrival)
}

// StoryArrival: Ankunft nach einem Inselwechsel, `arrival` der Insel islandID.
func (w WorldDef) StoryArrival(islandID string) []string {
	if i := w.islandIndex(islandID); i >= 0 {
		return slices.Clone(w.Islands[i].Story.Arrival)
	}
	return nil
}

// StoryBoss: Endboss der Insel islandID ausgelöst (`bossSpawned` des Endbosses), `boss` der Insel. Miniboss: keine Zeile.
func (w WorldDef) StoryBoss(islandID string) []string {
	if i := w.islandIndex(islandID); i >= 0 {
		return slices.Clone(w.Islands[i].Story.Boss)
	}
	return nil
}

// StoryDefeated: Endboss der Insel islandID besiegt (`bossDefeated` des Endbosses). completed sind die IDs der früher
// abgeschlossenen Inseln (Island.Completed); die Insel islandID zählt schon mit. Ergebnis: `done` der Insel; schließt der
// Sieg die Etappe ab, danach `etappen[<Nr>]`; ist es die letzte Insel der Welt, statt dessen `end`.
func (w WorldDef) StoryDefeated(islandID string, completed []string) []string {
	cur := w.islandIndex(islandID)
	if cur < 0 {
		return nil
	}
	done := func(i int) bool { return i == cur || slices.Contains(completed, w.Islands[i].ID) }
	out := slices.Clone(w.Islands[cur].Story.Done)
	switch {
	case len(w.targetsFrom(cur, done)) == 0:
		return concat(out, w.Story.End)
	case w.etappeDone(cur, done):
		return concat(out, w.Story.Etappen[strconv.Itoa(w.etappeOf(cur))])
	}
	return out
}

// StoryVictory: Sieg einer Variante ohne Endboss (z. B. Tage überleben) auf der Insel islandID. Nur auf der letzten Insel
// der Welt gilt es: `done` der Insel, danach `end`; auf früheren Inseln bleibt es beim Endboss, dann keine Zeile.
func (w WorldDef) StoryVictory(islandID string, completed []string) []string {
	cur := w.islandIndex(islandID)
	if cur < 0 {
		return nil
	}
	done := func(i int) bool { return i == cur || slices.Contains(completed, w.Islands[i].ID) }
	if len(w.targetsFrom(cur, done)) > 0 {
		return nil
	}
	return concat(w.Islands[cur].Story.Done, w.Story.End)
}

// StoryArrival: Zeilen der aktuellen Insel bei der Ankunft (Welt der Insel, ID der Insel).
func (isl *Island) StoryArrival() []string { return isl.world.StoryArrival(isl.islandID) }

// StoryBoss: Zeilen der aktuellen Insel, wenn ihr Endboss ausgelöst wird.
func (isl *Island) StoryBoss() []string { return isl.world.StoryBoss(isl.islandID) }

// StoryDefeated: Zeilen der aktuellen Insel nach dem Sieg über ihren Endboss (nach den früher abgeschlossenen Inseln).
func (isl *Island) StoryDefeated() []string { return isl.world.StoryDefeated(isl.islandID, isl.Completed) }

// StoryVictory: Zeilen der aktuellen Insel beim Sieg einer Variante ohne Endboss.
func (isl *Island) StoryVictory() []string { return isl.world.StoryVictory(isl.islandID, isl.Completed) }

// islandIndex ist der Listenplatz der Insel id, -1 ohne Treffer.
func (w WorldDef) islandIndex(id string) int {
	return slices.IndexFunc(w.Islands, func(d IslandDef) bool { return d.ID == id })
}

// etappeDone: Alle Inseln der Etappe der Insel cur gelten als abgeschlossen.
func (w WorldDef) etappeDone(cur int, done func(int) bool) bool {
	e := w.etappeOf(cur)
	for i := range w.Islands {
		if w.etappeOf(i) == e && !done(i) {
			return false
		}
	}
	return true
}

// concat hängt Einträge zu einer frischen Liste zusammen.
func concat(parts ...[]string) []string {
	var out []string
	for _, p := range parts {
		out = append(out, p...)
	}
	return out
}
