package hud

// Eingabe der Seekarte als Teil des Overlays (overlay.go). Links/Rechts bewegen im Spiel den Monarchen, darum zählen
// die Flanken nur, solange der Monarch am Steg steht (harborReach), und die Eingabe wird nicht verbraucht: Der Monarch
// läuft mit, die Karte bleibt in jeder Zelle sichtbar. B bleibt unbelegt.

import (
	"log/slog"
	"math"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

const (
	// shipKind ist die Art des Schiffs am Steg (sim ship.go); der Steg liegt am Schiff.
	shipKind = "ship"
	// harborReach: so nah in Units muss der Monarch am Schiff stehen, damit Links/Rechts/Bestätigen die Seekarte bedienen
	// (etwas weiter als hub.travel.rangeUnits = 3, nur Eingabe-Filter, die Regel für den Wechsel bleibt in sim).
	harborReach = 4.0
)

// atHarbor meldet, ob der Monarch m in der Stufe w am Schiff steht.
func atHarbor(w *sim.World, m *sim.Player) bool {
	for _, s := range w.Sites {
		if s.Kind == shipKind && math.Abs(m.X-s.X) <= harborReach {
			return true
		}
	}
	return false
}

// updateSea bedient die Seekarte des Spielers seat; Monarch ohne Krone, getrenntes Gerät oder fern vom Steg: keine Eingabe.
// Läuft nach dem Skill-Menü (ein offenes Skill-Menü verbraucht die Eingabe schon vorher).
func (h *hud) updateSea(in view.Input, f *view.Frame, s view.Session, seat view.Seat) {
	st := &h.seats[seat.Player]
	di, ok := in.Of(seat.Device)
	m := f.Monarch(seat.Player)
	stage, okStage := f.Stage(seat.Stage)
	if !st.sea.open || !ok || seat.Lost || m == nil || m.Crown != nil || !okStage || stage.World == nil || !atHarbor(stage.World, m) {
		return
	}
	r := st.sea.step(di.Actions)
	cue := func(k view.CueKind, text string) {
		h.cueBuf = append(h.cueBuf, view.Cue{Kind: k, Player: seat.Player, Text: text})
	}
	if r.moved {
		cue(view.CueMove, "")
	}
	if r.pick == "" {
		return
	}
	if err := s.ChooseIsland(seat.Player, r.pick); err != nil {
		slog.Info("🚫 Seekarte abgelehnt", "player", seat.Player, "err", err)
		cue(view.CueDenied, err.Error())
		return
	}
	cue(view.CueConfirm, "")
}
