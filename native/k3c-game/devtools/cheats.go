package devtools

// Cheat-Menü als view.Overlay (F9 bzw. View + Y, B-888): je Spieler ein eigenes Menü in seiner Zelle; ein offenes Menü
// verbraucht die Eingabe seines Spielers (Monarch steht), die anderen spielen weiter. F9 kommt von der Tastatur;
// spielt sie nicht, gilt der erste Spieler. Die Aktion geht
// über view.DevSession an den Raum (dort 🐛-Log mit Spieler und Wert); nicht im Aufnahme-Modus.

import (
	"log/slog"
	"slices"

	"k3c/native/k3c-game/view"
)

// Update bedient die Cheat-Menüs aller Spieler mit der Eingabe in und dem Frame f des vorigen Updates.
func (o *overlay) Update(in view.Input, f *view.Frame, s view.Session) view.OverlayStep {
	var out view.OverlayStep
	o.cues = o.cues[:0]
	if f.Recording {
		return out
	}
	if len(f.Seats) == 0 { // kein Spiel: die Schalter des Raums sind wieder aus
		clear(o.toggles)
	}
	var seated [view.MaxPlayers]bool
	for _, seat := range f.Seats {
		p := seat.Player
		if p < 0 || p >= view.MaxPlayers {
			continue
		}
		seated[p] = true
		acts := o.actionsOf(in, f.Seats, seat)
		st := &o.menus[p]
		r := st.step(acts, cheatGroups)
		o.apply(r, st, p, s)
		out.Consumed[p] = st.open || r.closed
	}
	for p := range o.menus {
		if !seated[p] {
			o.menus[p] = cheatState{}
		}
	}
	out.Cues = o.cues
	return out
}

// actionsOf liefert die Aktionen des Spielers seat: die seines Geräts, dazu F9 eines Geräts ohne Platz, wenn er der
// Ersatz ist (keyboardSeat).
func (o *overlay) actionsOf(in view.Input, seats []view.Seat, seat view.Seat) []view.Action {
	di, _ := in.Of(seat.Device)
	acts := di.Actions
	for _, d := range in.Devices {
		if d.Device == seat.Device || !slices.Contains(d.Actions, view.ActionCheats) || seatOf(seats, d.Device) >= 0 {
			continue
		}
		if keyboardSeat(seats) == seat.Player {
			o.acts = append(append(o.acts[:0], acts...), view.ActionCheats)
			return o.acts
		}
	}
	return acts
}

// keyboardSeat ist der Spieler, für den F9 gilt, wenn die Tastatur keinen Platz hat: der erste; -1 ohne Spieler. Hat
// die Tastatur einen Platz, öffnet F9 das Menü dieses Spielers über sein eigenes Gerät (eine Tastatur, B-893).
func keyboardSeat(seats []view.Seat) int {
	if len(seats) > 0 {
		return seats[0].Player
	}
	return -1
}

func seatOf(seats []view.Seat, d view.DeviceID) int {
	for _, s := range seats {
		if s.Device == d {
			return s.Player
		}
	}
	return -1
}

// apply meldet Cues und führt einen ausgelösten Eintrag über die Session aus.
func (o *overlay) apply(r cheatResult, st *cheatState, player int, s view.Session) {
	cue := func(k view.CueKind, text string) {
		o.cues = append(o.cues, view.Cue{Kind: k, Player: player, Text: text})
	}
	switch {
	case r.opened:
		cue(view.CueConfirm, "")
		slog.Info("🐛 Cheat-Menü offen", "player", player)
	case r.closed:
		cue(view.CueBack, "")
	case r.moved:
		cue(view.CueMove, "")
	}
	if r.fire == nil {
		return
	}
	ds, ok := s.(view.DevSession)
	a := o.toggles.action(r.fire)
	var err error
	if !ok {
		err = errNoDev
	} else {
		err = ds.Dev(player, a)
	}
	if err != nil {
		st.status, st.bad = err.Error(), true
		cue(view.CueDenied, err.Error())
		return
	}
	if r.fire.toggle != "" {
		o.toggles[r.fire.toggle] = *a.On
	}
	st.status, st.bad = "Erledigt: "+o.toggles.label(r.fire), false
	cue(view.CueConfirm, "")
}

// errNoDev meldet eine Session ohne Entwickler-Rolle.
var errNoDev = devError("Cheats gehen nur im lokalen Spiel")

type devError string

func (e devError) Error() string { return string(e) }
