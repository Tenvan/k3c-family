package menu

import (
	"testing"

	"k3c/native/k3c-game/view"
)

// AC-01: Auswahl, B zurück und Übergänge des Hauptmenüs; AC-03: „Weiter“ hängt am neuesten ladbaren Stand.

func TestMainSelectionStartsAtContinueOnlyWithLoadableSave(t *testing.T) {
	cases := []struct {
		name  string
		saves []view.SaveInfo
		want  int
	}{
		{"keine Stände", nil, mainNew},
		{"nur kaputte", []view.SaveInfo{save("alt", 1, false)}, mainNew},
		{"ladbarer Stand", []view.SaveInfo{save("alt", 1, false), save("spiel-1", 4, true)}, mainContinue},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			sc := newMain(newMenus())
			sc.Update(view.Input{}, &fakeSession{saves: c.saves})
			if sc.sel != c.want {
				t.Fatalf("Auswahl %d, erwartet %d", sc.sel, c.want)
			}
		})
	}
}

func TestMainMovesAndSkipsDisabledContinue(t *testing.T) {
	cases := []struct {
		name  string
		saves []view.SaveInfo
		keys  []view.Action
		want  int
	}{
		{"runter", nil, []view.Action{view.ActionDown}, mainLoad},
		{"hoch über ausgegrautes Weiter bleibt", nil, []view.Action{view.ActionUp}, mainNew},
		{"unten bleibt am Rand", nil, []view.Action{view.ActionDown, view.ActionDown, view.ActionDown, view.ActionDown}, mainQuit},
		{"hoch zu Weiter mit Stand", []view.SaveInfo{save("a", 1, true)}, []view.Action{view.ActionDown, view.ActionUp}, mainContinue},
		{"Home springt zum Anfang", nil, []view.Action{view.ActionDown, view.ActionDown, view.ActionHome}, mainNew},
		{"B im Hauptmenü bleibt", nil, []view.Action{view.ActionDown, view.ActionBack}, mainLoad},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			sc := newMain(newMenus())
			s := &fakeSession{saves: c.saves}
			for _, a := range c.keys {
				if st := sc.Update(press(pad1, a), s); st.Kind != view.StepStay {
					t.Fatalf("Taste %v verlässt das Hauptmenü: %+v", a, st)
				}
			}
			if sc.sel != c.want {
				t.Errorf("Auswahl %d, erwartet %d", sc.sel, c.want)
			}
		})
	}
}

func TestMainConfirmTransitions(t *testing.T) {
	cases := []struct {
		name string
		sel  int
		kind view.StepKind
		next string
	}{
		{"Weiter spielt", mainContinue, view.StepPlay, ""},
		{"Neues Spiel öffnet die Lobby", mainNew, view.StepPush, "lobby"},
		{"Laden öffnet die Liste", mainLoad, view.StepPush, "saves"},
		{"Optionen", mainOptions, view.StepPush, "options"},
		{"Beenden", mainQuit, view.StepQuit, ""},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			sc := newMain(newMenus())
			s := &fakeSession{saves: []view.SaveInfo{save("spiel-1", 2, true)}}
			sc.Update(view.Input{}, s)
			sc.sel = c.sel
			st := sc.Update(press(pad2, view.ActionConfirm), s)
			if st.Kind != c.kind || screenName(st.Next) != c.next || !hasCue(st, view.CueConfirm) {
				t.Fatalf("Schritt %+v (%s), erwartet %v %q", st, screenName(st.Next), c.kind, c.next)
			}
		})
	}
}

func TestMainContinueUsesConfirmingDevice(t *testing.T) {
	sc := newMain(newMenus())
	s := &fakeSession{saves: []view.SaveInfo{save("spiel-2", 5, true)}}
	st := sc.Update(press(pad2, view.ActionConfirm), s)
	if st.Kind != view.StepPlay || s.conts != 1 || len(s.devs) != 1 || s.devs[0][0] != pad2 {
		t.Fatalf("Weiter: %+v, Geräte %v", st, s.devs)
	}
	if sc.latest.Name != "spiel-2" {
		t.Errorf("neuester Stand %q", sc.latest.Name)
	}
}

func TestMainContinueErrorStaysWithMessage(t *testing.T) {
	sc := newMain(newMenus())
	s := &fakeSession{saves: []view.SaveInfo{save("a", 1, true)}}
	sc.Update(view.Input{}, s)
	s.saves = nil // inzwischen gelöscht: Continue scheitert
	st := sc.Update(press(pad1, view.ActionConfirm), s)
	if st.Kind != view.StepStay || !hasCue(st, view.CueDenied) || !sc.note.bad || sc.note.raw == "" {
		t.Fatalf("Fehler: %+v, Meldung %+v", st, sc.note)
	}
	sc.Update(view.Input{}, s)
	if sc.sel != mainNew || !sc.off(mainContinue) {
		t.Errorf("nach dem Fehler neu gelesen: Auswahl %d", sc.sel)
	}
}

func TestMainListErrorIsShown(t *testing.T) {
	sc := newMain(newMenus())
	sc.Update(view.Input{}, &fakeSession{listErr: errFake})
	if !sc.note.bad || sc.note.raw != errFake.Error() {
		t.Errorf("Meldung %+v", sc.note)
	}
}

func TestMainRereadsListAfterLoadMenu(t *testing.T) {
	m := newMenus()
	sc := newMain(m)
	s := &fakeSession{saves: []view.SaveInfo{save("a", 1, true)}}
	sc.Update(view.Input{}, s)
	sc.sel = mainLoad
	sc.Update(press(pad1, view.ActionConfirm), s)
	s.saves = nil // in der Liste gelöscht
	sc.Update(view.Input{}, s)
	if !sc.off(mainContinue) {
		t.Error("Weiter bleibt nach Löschen wählbar")
	}
}

func TestMainShowsNoticeOnce(t *testing.T) {
	m := newMenus()
	m.notice = note{key: txtLost, set: true, bad: true}
	if sc := m.Main().(*mainScreen); sc.note.key != txtLost {
		t.Fatalf("Meldung fehlt: %+v", sc.note)
	}
	if sc := m.Main().(*mainScreen); sc.note.set {
		t.Errorf("Meldung erscheint zweimal: %+v", sc.note)
	}
}

// screenName nennt die Art eines Screens für die Tabellen.
func screenName(sc view.Screen) string {
	switch sc.(type) {
	case *mainScreen:
		return "main"
	case *lobbyScreen:
		return "lobby"
	case *savesScreen:
		return "saves"
	case *optionsScreen:
		return "options"
	case *pauseScreen:
		return "pause"
	}
	return ""
}
