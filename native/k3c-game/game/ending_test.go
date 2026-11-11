package game

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// countingOverlay zählt die Aufrufe (nach dem Schließen des Spiels darf keiner mehr kommen).
type countingOverlay struct{ n int }

func (o *countingOverlay) Update(view.Input, *view.Frame, view.Session) view.OverlayStep {
	o.n++
	return view.OverlayStep{}
}

// endedFrame ist ein Frame im Spiel mit Ausgang o.
func endedFrame(g *Game, o view.Outcome) {
	g.frame.Outcome = o
	g.frame.Stages = []view.Stage{{Index: 0, World: &sim.World{}}}
}

// Spielende: view.EndDelay Takte nach Frame.Outcome schließt game das Spiel und legt das Hauptmenü auf; im Takt des
// Schließens läuft nichts mehr (kein Overlay, kein Befehl, kein Beitritt).
func TestEndingClosesAfterDelay(t *testing.T) {
	s := newFakeSim()
	o := &countingOverlay{}
	g := playGame(s, o)
	endedFrame(g, view.OutcomeWon)
	in := view.Input{Devices: []view.DeviceInput{press(pad5, view.Command{}, view.ActionConfirm)}}
	for range view.EndDelay - 1 {
		g.play(in)
	}
	if s.closed != 0 || g.mode != ModePlay {
		t.Fatalf("zu früh geschlossen: %d, Modus %v", s.closed, g.mode)
	}
	joins, calls := len(s.joins), o.n
	g.play(in)
	if s.closed != 1 || g.mode != ModeMenu || g.stack.Len() != 1 || g.endIn != 0 {
		t.Fatalf("nach %d Takten: geschlossen %d, Modus %v, Stapel %d", view.EndDelay, s.closed, g.mode, g.stack.Len())
	}
	if o.n != calls || len(s.joins) != joins {
		t.Errorf("nach dem Schließen weiter gespielt: Overlay %d→%d, Beitritte %d→%d", calls, o.n, joins, len(s.joins))
	}
}

// Review-Befund: Spielende erkannt, dann über die Pause „Spiel verlassen“ und ein neues Spiel: Der Zähler beginnt neu,
// das neue Spiel endet nicht nach wenigen Takten. Der Ausgang des alten Spiels bleibt im Menü-Frame ohne Stufe stehen.
func TestEndingResetsAfterLeavingThroughPause(t *testing.T) {
	s := newFakeSim()
	g := playGame(s)
	endedFrame(g, view.OutcomeLost)
	for range view.EndDelay - 2 {
		g.play(view.Input{})
	}
	g.frame = view.Frame{Outcome: view.OutcomeLost, Seats: g.frame.Seats} // Pause › Spiel verlassen › Hauptmenü
	g.play(view.Input{})
	if g.endIn != 0 {
		t.Fatalf("Zähler nach dem Verlassen %d", g.endIn)
	}
	s.ended, g.mode, g.stack = false, ModePlay, Stack{} // neues Spiel läuft
	g.frame.Stages, g.frame.Outcome = []view.Stage{{Index: 0, World: &sim.World{}}}, view.OutcomeNone
	for range view.EndDelay * 2 {
		g.play(view.Input{})
	}
	if s.closed != 0 || g.mode != ModePlay {
		t.Errorf("neues Spiel beendet: %d, Modus %v", s.closed, g.mode)
	}
}
