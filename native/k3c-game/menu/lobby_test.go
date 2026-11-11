package menu

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/view"
)

// AC-02: Beitritt von 1 und 2 Spielern per Pad, Austritt mit B, Wahl von Welt und Seed, Start über die Session.

func TestLobbyStartsWithOnePlayer(t *testing.T) {
	sc := newLobby(newMenus(), pad1)
	s := &fakeSession{}
	sc.row = lobbyStart
	st := sc.Update(press(pad1, view.ActionConfirm), s)
	if st.Kind != view.StepPlay || len(s.news) != 1 {
		t.Fatalf("Start: %+v", st)
	}
	g := s.news[0]
	want := view.NewGame{Grade: "normal", Goal: "endboss", Defeat: "stage", World: "standard", Devices: []view.DeviceID{pad1}}
	if g.Grade != want.Grade || g.Goal != want.Goal || g.Defeat != want.Defeat || g.World != want.World || g.Seed != "" ||
		!slices.Equal(g.Devices, want.Devices) || g.Name != "" {
		t.Errorf("NewGame %+v, erwartet %+v", g, want)
	}
}

func TestLobbyJoinAndLeave(t *testing.T) {
	cases := []struct {
		name  string
		ins   []view.Input
		devs  []view.DeviceID
		kind  view.StepKind
		cue   view.CueKind
		noted bool
	}{
		{"zweites Pad tritt mit A bei", []view.Input{press(pad2, view.ActionConfirm)}, []view.DeviceID{pad1, pad2}, view.StepStay, view.CueJoined, false},
		{"Tastatur tritt bei", []view.Input{press(keys1, view.ActionConfirm)}, []view.DeviceID{pad1, keys1}, view.StepStay, view.CueJoined, false},
		{"drittes Gerät abgelehnt", []view.Input{press(pad2, view.ActionConfirm), press(pad3, view.ActionConfirm)}, []view.DeviceID{pad1, pad2}, view.StepStay, view.CueDenied, true},
		{"Spieler 2 geht mit B", []view.Input{press(pad2, view.ActionConfirm), press(pad2, view.ActionBack)}, []view.DeviceID{pad1}, view.StepStay, view.CueLeft, false},
		{"B von Spieler 1 bricht ab", []view.Input{press(pad2, view.ActionConfirm), press(pad1, view.ActionBack)}, []view.DeviceID{pad1, pad2}, view.StepPop, view.CueBack, false},
		{"fremdes Gerät bedient nicht", []view.Input{press(pad2, view.ActionDown), press(pad2, view.ActionBack)}, []view.DeviceID{pad1}, view.StepStay, -1, false},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			sc := newLobby(newMenus(), pad1)
			st := stepThrough(sc, &fakeSession{}, c.ins...)
			if got := sc.devs[:sc.n]; !slices.Equal(got, c.devs) {
				t.Errorf("Spieler %v, erwartet %v", got, c.devs)
			}
			if st.Kind != c.kind || (c.cue >= 0 && !hasCue(st, c.cue)) || sc.note.set != c.noted {
				t.Errorf("Schritt %+v, Meldung %+v", st, sc.note)
			}
			if sc.row != 0 {
				t.Errorf("Zeile %d bewegt", sc.row)
			}
		})
	}
}

func TestLobbyStartsWithTwoPlayersInJoinOrder(t *testing.T) {
	sc := newLobby(newMenus(), pad2)
	s := &fakeSession{}
	in := view.Input{Devices: []view.DeviceInput{
		{Device: keys1, Actions: []view.Action{view.ActionConfirm}},
		{Device: pad2, Actions: []view.Action{view.ActionDown}},
	}}
	sc.Update(in, s)
	sc.row = lobbyStart
	if st := sc.Update(press(keys1, view.ActionConfirm), s); st.Kind != view.StepPlay {
		t.Fatalf("Spieler 2 startet nicht: %+v", st)
	}
	if got := s.news[0].Devices; !slices.Equal(got, []view.DeviceID{pad2, keys1}) {
		t.Errorf("Geräte %v", got)
	}
}

func TestLobbyOptions(t *testing.T) {
	cases := []struct {
		name string
		keys []view.Action
		want view.NewGame
	}{
		{"Grad rechts bringt Standard-Niederlage", []view.Action{view.ActionRight}, view.NewGame{Grade: "hard", Goal: "endboss", Defeat: "stage"}},
		{"Grad Ultra: komplett verloren", []view.Action{view.ActionRight, view.ActionRight}, view.NewGame{Grade: "ultra", Goal: "endboss", Defeat: "lost"}},
		{"Grad links: Leicht", []view.Action{view.ActionLeft, view.ActionLeft}, view.NewGame{Grade: "easy", Goal: "endboss", Defeat: "resources"}},
		{"A im Kreis", []view.Action{view.ActionConfirm, view.ActionConfirm, view.ActionConfirm}, view.NewGame{Grade: "easy", Goal: "endboss", Defeat: "resources"}},
		{"gewählte Niederlage bleibt beim Grad", []view.Action{view.ActionDown, view.ActionDown, view.ActionLeft, view.ActionUp, view.ActionUp, view.ActionRight}, view.NewGame{Grade: "hard", Goal: "endboss", Defeat: "resources"}},
		{"Ziel", []view.Action{view.ActionDown, view.ActionRight, view.ActionRight}, view.NewGame{Grade: "normal", Goal: "days", Defeat: "stage"}},
		{"Welt bleibt Standard", []view.Action{view.ActionDown, view.ActionDown, view.ActionDown, view.ActionRight}, view.NewGame{Grade: "normal", Goal: "endboss", Defeat: "stage"}},
		{"Seed 2", []view.Action{view.ActionDown, view.ActionDown, view.ActionDown, view.ActionDown, view.ActionRight, view.ActionRight}, view.NewGame{Grade: "normal", Goal: "endboss", Defeat: "stage", Seed: "2"}},
		{"Seed links bleibt Name", []view.Action{view.ActionDown, view.ActionDown, view.ActionDown, view.ActionDown, view.ActionLeft}, view.NewGame{Grade: "normal", Goal: "endboss", Defeat: "stage"}},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			sc := newLobby(newMenus(), pad1)
			s := &fakeSession{}
			for _, a := range c.keys {
				sc.Update(press(pad1, a), s)
			}
			g := sc.newGame()
			if g.Grade != c.want.Grade || g.Goal != c.want.Goal || g.Defeat != c.want.Defeat || g.Seed != c.want.Seed || g.World != "standard" {
				t.Errorf("NewGame %+v, erwartet %+v", g, c.want)
			}
		})
	}
}

func TestLobbySeedWrapsWithConfirm(t *testing.T) {
	sc := newLobby(newMenus(), pad1)
	sc.row, sc.seed = lobbySeed, maxSeed
	sc.Update(press(pad1, view.ActionConfirm), &fakeSession{})
	if sc.seed != 0 || sc.newGame().Seed != "" {
		t.Errorf("Seed %d nach A am Ende", sc.seed)
	}
}

func TestLobbyRowsAndCancel(t *testing.T) {
	sc := newLobby(newMenus(), pad1)
	s := &fakeSession{}
	for range lobbyRows + 2 {
		sc.Update(press(pad1, view.ActionDown), s)
	}
	if sc.row != lobbyCancel {
		t.Fatalf("Zeile %d am unteren Rand", sc.row)
	}
	if st := sc.Update(press(pad1, view.ActionRight), s); st.Kind != view.StepStay {
		t.Errorf("Rechts auf Abbrechen: %+v", st)
	}
	if st := sc.Update(press(pad1, view.ActionConfirm), s); st.Kind != view.StepPop || len(s.news) != 0 {
		t.Errorf("Abbrechen: %+v", st)
	}
	if st := newLobby(newMenus(), pad1).Update(press(pad1, view.ActionHome), s); st.Kind != view.StepPop {
		t.Errorf("Home: %+v", st)
	}
}

func TestLobbyStartErrorStays(t *testing.T) {
	sc := newLobby(newMenus(), pad1)
	s := &fakeSession{newErr: errFake}
	sc.row = lobbyStart
	st := sc.Update(press(pad1, view.ActionConfirm), s)
	if st.Kind != view.StepStay || !hasCue(st, view.CueDenied) || sc.note.raw != errFake.Error() {
		t.Errorf("Fehler: %+v, %+v", st, sc.note)
	}
}

func TestLobbyTextsShowPlayers(t *testing.T) {
	sc := newLobby(newMenus(), pad1)
	sc.Update(press(keys1, view.ActionConfirm), &fakeSession{})
	sc.fill(view.DefaultLanguage)
	if want := "Spieler 1: Pad 1   ·   Spieler 2: Tastatur links"; sc.pg.sub != want {
		t.Errorf("Spieler-Zeile %q, erwartet %q", sc.pg.sub, want)
	}
	if got := sc.pg.rows[lobbyGrade].value; got != "< Normal >" {
		t.Errorf("Grad %q", got)
	}
	sc.fill(view.LanguageEnglish)
	if got := sc.pg.rows[lobbySeed].value; got != "< Name of the save >" {
		t.Errorf("Seed englisch %q", got)
	}
}
