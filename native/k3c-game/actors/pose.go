package actors

// Pose der Rig-Figuren im festen Takt: Listen schreibt einmal je Frame Animation und Umhang aller Rig-Figuren fort,
// mit dem Zeitschritt aus Frame.Clock (ein Takt = 1/30 s), unabhängig davon, ob und wie oft Draw die Figur zeichnet.
// So ist das Bild eines Frames bildgleich, egal wie viele Draws je Update laufen oder welche Figur vorher sichtbar war.

// maxPoseStep begrenzt den Zeitschritt zwischen zwei Posen; ein größerer Sprung lässt den Umhang neu einschwingen.
const maxPoseStep = 0.25

// Einschwingen des Umhangs einer neuen Figur: settleSteps Schritte zu je settleStep Sekunden. settleStep entspricht
// der Obergrenze der Umhang-Schritte je Update im Actor (8 × 1/120 s), so geht keine Zeit verloren; 8 Schritte statt
// 30 begrenzen die Spitze, wenn viele Figuren zugleich erscheinen (Laden, Welle).
const (
	settleSteps = 8
	settleStep  = 8.0 / 120
)

// advanceAll schreibt die Pose aller Rig-Figuren fort (Reihenfolge egal: jede Figur rechnet nur für sich).
func (d *drawer) advanceAll() {
	for _, fg := range d.figs {
		if fg.actor == nil || fg.broken {
			continue
		}
		if err := d.advance(fg); err != nil {
			fg.posed = false
			d.warn(warnKey{fg.look.Asset, fg.pose.Anim}, err)
		}
	}
}

// advance rechnet die Pose des Actors für die Darstellungszeit d.now: höchstens einmal je Zeit und Pose; der Umhang
// läuft mit der vergangenen Zeit seit der letzten Pose. Beim ersten Mal (oder nach einem großen Sprung) schwingt er
// mit settleSteps festen Schritten ein, damit er nicht senkrecht hängt.
func (d *drawer) advance(fg *figure) error {
	if fg.posed && !fg.dirty && fg.posedAt == d.now {
		return nil
	}
	a, target := fg.actor, AnimTime(fg.pose, d.now, fg.shift)
	dt, had := d.now-fg.posedAt, fg.posed
	fg.posed, fg.dirty, fg.posedAt = true, false, d.now
	if had && dt >= 0 && dt <= maxPoseStep {
		a.T = target - dt*a.Tempo
		return a.Update(dt)
	}
	a.ResetCloak()
	a.T = target - settleSteps*settleStep*a.Tempo
	for range settleSteps {
		if err := a.Update(settleStep); err != nil {
			return err
		}
	}
	return nil
}
