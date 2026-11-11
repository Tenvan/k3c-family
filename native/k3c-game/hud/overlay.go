package hud

// Skill-Menü als view.Overlay: game ruft Update in jedem Update im Spiel vor Pause, Beitritt und Befehlen. Jeder
// Spieler hat sein eigenes Menü; ein offenes Menü verbraucht die Eingabe seines Spielers (Monarch steht, kein
// Pause-Menü), die anderen spielen weiter. Lernen und Respec gehen über die Session; die Wirkung zeigt der nächste
// Frame.

import (
	"log/slog"

	"k3c/native/k3c-game/view"
)

// Update bedient die Skill-Menüs aller Spieler mit der Eingabe in und dem Frame f des vorigen Updates.
func (h *hud) Update(in view.Input, f *view.Frame, s view.Session) view.OverlayStep {
	var out view.OverlayStep
	h.cueBuf = h.cueBuf[:0]
	var seated [view.MaxPlayers]bool
	for _, seat := range f.Seats {
		p := seat.Player
		if p < 0 || p >= view.MaxPlayers {
			continue
		}
		seated[p] = true
		out.Consumed[p] = h.updateSeat(in, f, s, seat)
	}
	for p := range h.seats {
		if !seated[p] {
			h.seats[p].menu = menuState{}
		}
	}
	out.Cues = h.cueBuf
	return out
}

// updateSeat bedient das Menü eines Spielers; Rückgabe: Eingabe verbraucht (Menü offen oder eben geschlossen).
func (h *hud) updateSeat(in view.Input, f *view.Frame, s view.Session, seat view.Seat) bool {
	st := &h.seats[seat.Player]
	di, ok := in.Of(seat.Device)
	m := f.Monarch(seat.Player)
	stage, okStage := f.Stage(seat.Stage)
	if !ok || seat.Lost || m == nil || !okStage || stage.World == nil {
		st.menu = menuState{} // Spieler ohne Eingabe oder ohne Monarch: Menü zu
		return false
	}
	if !st.menu.open && !hasAction(di.Actions, view.ActionSkillMenu) {
		return false
	}
	st.refreshEntries(stage.World, m)
	r := st.menu.step(di.Actions, st.entries)
	h.apply(r, seat.Player, s)
	return st.menu.open || r.closed
}

// apply führt Lernen und Respec über die Session aus und meldet die Cues.
func (h *hud) apply(r menuResult, player int, s view.Session) {
	cue := func(k view.CueKind, text string) {
		h.cueBuf = append(h.cueBuf, view.Cue{Kind: k, Player: player, Text: text})
	}
	switch {
	case r.opened:
		cue(view.CueConfirm, "")
	case r.closed:
		cue(view.CueBack, "")
	case r.moved:
		cue(view.CueMove, "")
	}
	var err error
	switch {
	case r.denied:
		cue(view.CueDenied, "")
		return
	case r.learn != "":
		err = s.Learn(player, r.learn)
	case r.respec:
		err = s.Respec(player)
	default:
		return
	}
	if err != nil {
		slog.Info("🚫 Skill-Menü abgelehnt", "player", player, "err", err)
		cue(view.CueDenied, err.Error())
		return
	}
	cue(view.CueConfirm, "")
}

func hasAction(acts []view.Action, a view.Action) bool {
	for _, x := range acts {
		if x == a {
			return true
		}
	}
	return false
}
