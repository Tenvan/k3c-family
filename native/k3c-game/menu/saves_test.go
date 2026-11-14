package menu

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/view"
)

// AC-03: Liste, Laden, Löschen mit Rückfrage und nicht ladbare Stände gegen eine Schein-Session.

func threeSaves() []view.SaveInfo {
	return []view.SaveInfo{save("spiel-3", 7, true), save("kaputt", 0, false), save("spiel-1", 2, true)}
}

func TestSavesLoad(t *testing.T) {
	cases := []struct {
		name   string
		saves  []view.SaveInfo
		keys   []view.Action
		kind   view.StepKind
		loaded []string
		cue    view.CueKind
	}{
		{"neuester zuerst", threeSaves(), nil, view.StepPlay, []string{"spiel-3"}, view.CueConfirm},
		{"nicht ladbar gesperrt", threeSaves(), []view.Action{view.ActionDown}, view.StepStay, nil, view.CueDenied},
		{"dritter Stand", threeSaves(), []view.Action{view.ActionDown, view.ActionDown, view.ActionDown}, view.StepPlay, []string{"spiel-1"}, view.CueConfirm},
		{"leere Liste", nil, []view.Action{view.ActionDown}, view.StepStay, nil, -1},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			sc := newSaves(newMenus())
			s := &fakeSession{saves: c.saves}
			for _, a := range c.keys {
				sc.Update(press(pad2, a), s)
			}
			st := sc.Update(press(pad2, view.ActionConfirm), s)
			if st.Kind != c.kind || !slices.Equal(s.loads, c.loaded) || (c.cue >= 0 && !hasCue(st, c.cue)) {
				t.Fatalf("Schritt %+v, geladen %v", st, s.loads)
			}
			if len(s.loads) > 0 && !slices.Equal(s.devs[0], []view.DeviceID{pad2}) {
				t.Errorf("Geräte %v", s.devs[0])
			}
		})
	}
}

func TestSavesNotLoadableShowsProblem(t *testing.T) {
	sc := newSaves(newMenus())
	s := &fakeSession{saves: threeSaves()}
	sc.Update(press(pad1, view.ActionDown), s)
	sc.Update(press(pad1, view.ActionConfirm), s)
	if sc.note.raw != "Spielstand nicht ladbar: Version 3" || !sc.note.bad {
		t.Errorf("Meldung %+v", sc.note)
	}
	sc.fill(view.DefaultLanguage)
	if r := sc.pg.rows[1]; !r.off || r.value != "Spielstand nicht ladbar: Version 3" {
		t.Errorf("Zeile %+v nicht markiert", r)
	}
}

func TestSavesLoadErrorStays(t *testing.T) {
	sc := newSaves(newMenus())
	s := &fakeSession{saves: threeSaves(), loadErr: errFake}
	st := sc.Update(press(pad1, view.ActionConfirm), s)
	if st.Kind != view.StepStay || !hasCue(st, view.CueDenied) || sc.note.raw != errFake.Error() {
		t.Errorf("Fehler: %+v, %+v", st, sc.note)
	}
}

func TestSavesDeleteWithQuestion(t *testing.T) {
	cases := []struct {
		name    string
		keys    []view.Action
		deleted []string
		asking  bool
		kind    view.StepKind
	}{
		{"View fragt", []view.Action{view.ActionView}, nil, true, view.StepStay},
		{"A auf Abbrechen", []view.Action{view.ActionView, view.ActionConfirm}, nil, false, view.StepStay},
		{"Löschen wählen und bestätigen", []view.Action{view.ActionView, view.ActionRight, view.ActionConfirm}, []string{"spiel-3"}, false, view.StepStay},
		{"zweimal wechseln ist Abbrechen", []view.Action{view.ActionView, view.ActionDown, view.ActionUp, view.ActionConfirm}, nil, false, view.StepStay},
		{"B bricht ab und bleibt in der Liste", []view.Action{view.ActionView, view.ActionRight, view.ActionBack}, nil, false, view.StepStay},
		{"View noch einmal bricht ab", []view.Action{view.ActionView, view.ActionView}, nil, false, view.StepStay},
		{"Home verlässt die Liste", []view.Action{view.ActionView, view.ActionHome}, nil, false, view.StepPop},
		{"kaputten Stand löschen", []view.Action{view.ActionDown, view.ActionView, view.ActionLeft, view.ActionConfirm}, []string{"kaputt"}, false, view.StepStay},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			sc := newSaves(newMenus())
			s := &fakeSession{saves: threeSaves()}
			var st view.Step
			for _, a := range c.keys {
				st = sc.Update(press(pad1, a), s)
			}
			if !slices.Equal(s.deleted, c.deleted) || sc.asking != c.asking || st.Kind != c.kind {
				t.Errorf("gelöscht %v, Rückfrage %v, Schritt %+v", s.deleted, sc.asking, st)
			}
		})
	}
}

func TestSavesEscCancelsQuestionOnce(t *testing.T) {
	sc := newSaves(newMenus())
	s := &fakeSession{saves: threeSaves()}
	sc.Update(press(keys1, view.ActionView), s)
	// Esc liefert Back und Menu im selben Update: das zählt einmal (Rückfrage zu, Liste bleibt)
	if st := sc.Update(press(keys1, view.ActionBack, view.ActionMenu), s); st.Kind != view.StepStay || sc.asking {
		t.Errorf("Esc: %+v, Rückfrage %v", st, sc.asking)
	}
	if st := sc.Update(press(keys1, view.ActionBack, view.ActionMenu), s); st.Kind != view.StepPop {
		t.Errorf("zweites Esc verlässt die Liste nicht: %+v", st)
	}
}

func TestSavesDeleteRefreshesAndKeepsSelectionInList(t *testing.T) {
	sc := newSaves(newMenus())
	s := &fakeSession{saves: threeSaves()}
	stepThrough(sc, s, press(pad1, view.ActionDown), press(pad1, view.ActionDown),
		press(pad1, view.ActionView), press(pad1, view.ActionRight), press(pad1, view.ActionConfirm))
	if len(sc.list) != 2 || sc.sel != 1 || sc.note.key != txtDeleted || sc.note.arg != "spiel-1" {
		t.Errorf("Liste %d, Auswahl %d, Meldung %+v", len(sc.list), sc.sel, sc.note)
	}
}

func TestSavesDeleteError(t *testing.T) {
	sc := newSaves(newMenus())
	s := &fakeSession{saves: threeSaves(), delErr: errFake}
	st := stepThrough(sc, s, press(pad1, view.ActionView), press(pad1, view.ActionRight), press(pad1, view.ActionConfirm))
	if !hasCue(st, view.CueDenied) || sc.note.raw != errFake.Error() || len(sc.list) != 3 {
		t.Errorf("Fehler: %+v, %+v", st, sc.note)
	}
}

func TestSavesEmptyListAndBack(t *testing.T) {
	sc := newSaves(newMenus())
	s := &fakeSession{}
	if st := sc.Update(press(pad1, view.ActionView), s); st.Kind != view.StepStay || sc.asking {
		t.Errorf("Löschen in leerer Liste: %+v", st)
	}
	sc.fill(view.DefaultLanguage)
	if sc.pg.sub != "Keine Spielstände" || sc.pg.n != 0 {
		t.Errorf("leere Liste: %q, %d Zeilen", sc.pg.sub, sc.pg.n)
	}
	if st := sc.Update(press(pad1, view.ActionBack), s); st.Kind != view.StepPop || !hasCue(st, view.CueBack) {
		t.Errorf("B: %+v", st)
	}
}

func TestSavesListErrorIsShown(t *testing.T) {
	sc := newSaves(newMenus())
	sc.Update(view.Input{}, &fakeSession{listErr: errFake})
	if sc.note.raw != errFake.Error() {
		t.Errorf("Meldung %+v", sc.note)
	}
}

func TestSavesRowsScrollAroundSelection(t *testing.T) {
	sc := newSaves(newMenus())
	s := &fakeSession{}
	for i := range 12 {
		s.saves = append(s.saves, save("s"+string(rune('a'+i)), i, true))
	}
	for range 9 {
		sc.Update(press(pad1, view.ActionDown), s)
	}
	sc.fill(view.DefaultLanguage)
	if sc.pg.n != maxRows || sc.pg.rows[sc.pg.sel].label != "sj" {
		t.Errorf("%d Zeilen, Auswahl %d = %q", sc.pg.n, sc.pg.sel, sc.pg.rows[sc.pg.sel].label)
	}
}

func TestSaveRowText(t *testing.T) {
	e := view.SaveInfo{Name: "spiel-1", Day: 4, Depths: []int{0, 2}, Loadable: true}
	if r := saveRow(view.DefaultLanguage, e); r.value != "Tag 4 · Stufe 0,2" || r.off {
		t.Errorf("Zeile %+v", r)
	}
	if r := saveRow(view.LanguageEnglish, e); r.value != "Day 4 · Level 0,2" {
		t.Errorf("englisch %+v", r)
	}
}
