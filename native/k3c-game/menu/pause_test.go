package menu

import (
	"testing"

	"k3c/native/k3c-game/view"
)

// AC-04: Pause-Menü und Spielende.

// twoSeats ist ein Frame mit zwei Spielern: Spieler 0 an pad1, Spieler 1 an keys1.
func twoSeats() *view.Frame {
	return &view.Frame{Seats: []view.Seat{{Player: 0, Device: pad1}, {Player: 1, Device: keys1}}, Settings: view.DefaultSettings()}
}

func TestPauseTransitions(t *testing.T) {
	cases := []struct {
		name string
		keys []view.Action
		kind view.StepKind
		next string
		cue  view.CueKind
	}{
		{"B setzt fort", []view.Action{view.ActionBack}, view.StepPop, "", view.CueResume},
		{"Menu setzt fort", []view.Action{view.ActionMenu}, view.StepPop, "", view.CueResume},
		{"Fortsetzen", []view.Action{view.ActionConfirm}, view.StepPop, "", view.CueResume},
		{"Optionen", []view.Action{view.ActionDown, view.ActionConfirm}, view.StepPush, "options", view.CueConfirm},
		{"Spiel verlassen", []view.Action{view.ActionDown, view.ActionDown, view.ActionDown, view.ActionConfirm}, view.StepReplace, "main", view.CueConfirm},
		{"Home bleibt im Pause-Menü", []view.Action{view.ActionDown, view.ActionHome}, view.StepStay, "", -1},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			m := newMenus()
			sc := m.Pause()
			s := &fakeSession{active: true}
			var st view.Step
			for _, a := range c.keys {
				st = sc.Update(press(pad1, a), s)
			}
			if st.Kind != c.kind || screenName(st.Next) != c.next || (c.cue >= 0 && !hasCue(st, c.cue)) {
				t.Errorf("Schritt %+v (%s)", st, screenName(st.Next))
			}
		})
	}
}

func TestPauseLeaveClosesAndReportsError(t *testing.T) {
	m := newMenus()
	s := &fakeSession{active: true, closeErr: errFake}
	sc := newPause(m)
	sc.sel = pauseLeave
	st := sc.Update(press(pad1, view.ActionConfirm), s)
	if s.closed != 1 || s.active || st.Kind != view.StepReplace {
		t.Fatalf("Verlassen: %+v, geschlossen %d", st, s.closed)
	}
	if main := st.Next.(*mainScreen); main.note.raw != errFake.Error() {
		t.Errorf("Hauptmenü zeigt den Fehler nicht: %+v", main.note)
	}
}

func TestPauseRemovePlayer(t *testing.T) {
	cases := []struct {
		name  string
		frame *view.Frame
		dev   view.DeviceID
		err   error
		left  []int
		kind  view.StepKind
		cue   view.CueKind
	}{
		{"Spieler 2 meldet sich ab", twoSeats(), keys1, nil, []int{1}, view.StepPop, view.CueResume},
		{"Session lehnt ab", twoSeats(), pad1, errFake, []int{0}, view.StepStay, view.CueDenied},
		{"Gerät ohne Platz", twoSeats(), pad3, nil, nil, view.StepStay, view.CueDenied},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			m := newMenus()
			sc := newPause(m)
			s := &fakeSession{active: true, leaveErr: c.err, seats: c.frame.Seats}
			st := stepThrough(sc, s, press(c.dev, view.ActionDown), press(c.dev, view.ActionDown), press(c.dev, view.ActionDown),
				press(c.dev, view.ActionConfirm))
			if sc.sel != pauseRemove || st.Kind != c.kind || !hasCue(st, c.cue) || len(s.left) != len(c.left) {
				t.Errorf("Auswahl %d, Schritt %+v, abgemeldet %v", sc.sel, st, s.left)
			}
		})
	}
}

func TestPauseRemoveDisabledWithOnePlayer(t *testing.T) {
	sc := newPause(newMenus())
	s := &fakeSession{active: true, seats: []view.Seat{{Player: 0, Device: pad1}}}
	stepThrough(sc, s, press(pad1, view.ActionDown), press(pad1, view.ActionDown), press(pad1, view.ActionDown))
	if sc.sel != pauseLeave {
		t.Errorf("Auswahl %d, Abmelden nicht übersprungen", sc.sel)
	}
}

// Spielende: Das Hauptmenü zeigt den Ausgang aus dem Frame; eine eigene Meldung (Fehler beim Verlassen) geht vor.
func TestMainShowsOutcomeOfFrame(t *testing.T) {
	cases := []struct {
		name string
		o    view.Outcome
		key  textKey
		bad  bool
	}{
		{"Niederlage", view.OutcomeLost, txtLost, true},
		{"Sieg", view.OutcomeWon, txtWon, false},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			sc := newMenus().Main().(*mainScreen)
			sc.ended = c.o
			sc.fill("de")
			if want := tr("de", c.key); sc.pg.note != want || sc.pg.bad != c.bad {
				t.Errorf("Meldung %q (rot %v), erwartet %q", sc.pg.note, sc.pg.bad, want)
			}
			sc.note = note{raw: "eigene", set: true}
			if sc.fill("de"); sc.pg.note != "eigene" {
				t.Errorf("eigene Meldung verdrängt: %q", sc.pg.note)
			}
		})
	}
	sc := newMenus().Main().(*mainScreen)
	if sc.fill("de"); sc.pg.note != "" {
		t.Errorf("ohne Ausgang: %q", sc.pg.note)
	}
}

// menus ist kein Overlay mehr (Spielende liegt in game) und liefert die Untermenüs des Aufnahme-Modus.
func TestMenusPagesAndFallbackFont(t *testing.T) {
	ms := New(nil, nil)
	if _, ok := ms.(view.Overlay); ok {
		t.Error("menus ist ein Overlay")
	}
	pages := ms.(view.MenuPages)
	for name, want := range map[string]string{"saves": "saves", "options": "options", "lobby": "lobby", "x": ""} {
		if got := screenName(pages.Page(name, false, pad1)); got != want {
			t.Errorf("%s: %q", name, got)
		}
	}
	if !ms.(*menus).paint.font.Fallback {
		t.Error("ohne Grafik keine Debug-Schrift")
	}
}
