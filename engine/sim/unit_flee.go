package sim

import "math"

// Flucht der Bürger (docs/rules/kampf.md § 4, B-928): Ein getroffener Bürger lässt seine Arbeit liegen und läuft zum Schutzplatz
// an der Burg, ohne Statusverlust. Er arbeitet wieder, sobald er dort ist und kein Gegner in Hörweite ist.

// enemyWithin: Steht ein lebender Gegner höchstens dist Units von x entfernt (Wachen eines Lagers zählen mit)?
func enemyWithin(w *World, x, dist float64) bool {
	for _, e := range w.Enemies {
		if e.HP > 0 && math.Abs(e.X-x) <= dist {
			return true
		}
	}
	return false
}

// flee läuft zum Schutzplatz; true, solange der Bürger noch flieht (der Aufrufer macht dann nichts weiter).
func flee(w *World, t *Troop, dt float64) bool {
	if t.Job == nil || t.Job.Type != "carry" { // getragenes Material bleibt beim Bauern, er trägt es nach der Flucht weiter
		releaseJob(w, t)
	}
	if walkTo(w, t, shelterX(w, t), dt) && !enemyWithin(w, t.X, w.values().Troops["peasant"].FetchEnemyDistance) {
		t.fleeing, t.shelter = false, 0
		return false
	}
	return true
}
