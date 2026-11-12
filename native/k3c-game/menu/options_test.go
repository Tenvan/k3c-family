package menu

import (
	"testing"

	"k3c/native/k3c-game/view"
)

// AC-04: Werte und Übergänge der Optionen (Lautstärke Musik und Effekte, Vollbild, Sprache).

func TestApplyOption(t *testing.T) {
	d := view.DefaultSettings()
	half := view.Settings{Music: 0.5, Effects: 0.05, Language: view.LanguageEnglish}
	cases := []struct {
		name string
		from view.Settings
		opt  int
		k    key
		want view.Settings
	}{
		{"Musik leiser", d, optMusic, keyLeft, view.Settings{Music: 0.9, Effects: 1, Language: "de"}},
		{"Musik über 100 bleibt", d, optMusic, keyRight, d},
		{"Musik A ändert nichts", d, optMusic, keyOK, d},
		{"Effekte rundet auf 10 %", half, optEffects, keyRight, view.Settings{Music: 0.5, Effects: 0.2, Language: "en"}},
		{"Effekte nicht unter 0", half, optEffects, keyLeft, view.Settings{Music: 0.5, Effects: 0, Language: "en"}},
		{"Vollbild an", d, optFullscreen, keyOK, view.Settings{Music: 1, Effects: 1, Fullscreen: true, Language: "de"}},
		{"Vollbild mit links", d, optFullscreen, keyLeft, view.Settings{Music: 1, Effects: 1, Fullscreen: true, Language: "de"}},
		{"Sprache englisch", d, optLanguage, keyRight, view.Settings{Music: 1, Effects: 1, Language: "en"}},
		{"Sprache zurück", half, optLanguage, keyOK, view.Settings{Music: 0.5, Effects: 0.05, Language: "de"}},
		{"Zurück ändert nichts", d, optBack, keyRight, d},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := applyOption(c.from, c.opt, c.k); got != c.want {
				t.Errorf("%+v, erwartet %+v", got, c.want)
			}
		})
	}
}

func TestOptionsWritesThroughSession(t *testing.T) {
	m := newMenus()
	sc := newOptions(m, false)
	s := &fakeSession{}
	stepThrough(sc, s, press(pad1, view.ActionLeft), press(pad1, view.ActionLeft), press(pad1, view.ActionDown), press(pad1, view.ActionLeft))
	if len(s.settings) != 3 {
		t.Fatalf("SetSettings %d-mal", len(s.settings))
	}
	if last := s.settings[2]; last.Music != 0.8 || last.Effects != 0.9 || s.Settings() != last {
		t.Errorf("zuletzt %+v, Session %+v", last, s.Settings())
	}
}

func TestOptionsUnchangedValueIsNotWritten(t *testing.T) {
	s := &fakeSession{}
	stepThrough(newOptions(newMenus(), false), s, press(pad1, view.ActionRight), press(pad1, view.ActionConfirm))
	if len(s.settings) != 0 {
		t.Errorf("unveränderte Werte gespeichert: %v", s.settings)
	}
}

func TestOptionsStartFromKnownSettings(t *testing.T) {
	s := &fakeSession{cur: view.Settings{Music: 0.3, Effects: 0.4, Fullscreen: true, Language: "en"}}
	sc := newOptions(newMenus(), true)
	sc.Update(press(pad1, view.ActionRight), s)
	if got := s.settings[0]; got.Music != 0.4 || got.Effects != 0.4 || !got.Fullscreen || got.Language != "en" {
		t.Errorf("Basis nicht von der Session: %+v", got)
	}
}

func TestOptionsSaveErrorKeepsValues(t *testing.T) {
	sc := newOptions(newMenus(), false)
	s := &fakeSession{setErr: errFake}
	sc.Update(press(pad1, view.ActionLeft), s)
	if sc.cur.Music != 0.9 || sc.note.raw != errFake.Error() || !sc.note.bad {
		t.Errorf("Wert %v, Meldung %+v", sc.cur.Music, sc.note)
	}
}

func TestOptionsNavigation(t *testing.T) {
	cases := []struct {
		name string
		keys []view.Action
		sel  int
		kind view.StepKind
	}{
		{"hoch springt ans Ende", []view.Action{view.ActionUp}, optBack, view.StepStay},
		{"runter im Kreis", []view.Action{view.ActionUp, view.ActionDown}, optMusic, view.StepStay},
		{"Zurück mit A", []view.Action{view.ActionUp, view.ActionConfirm}, optBack, view.StepPop},
		{"Zurück mit rechts bleibt", []view.Action{view.ActionUp, view.ActionRight}, optBack, view.StepStay},
		{"B", []view.Action{view.ActionBack}, optMusic, view.StepPop},
		{"Menu", []view.Action{view.ActionMenu}, optMusic, view.StepPop},
		{"Home", []view.Action{view.ActionHome}, optMusic, view.StepPop},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			sc := newOptions(newMenus(), false)
			var st view.Step
			for _, a := range c.keys {
				st = sc.Update(press(pad1, a), &fakeSession{})
			}
			if sc.sel != c.sel || st.Kind != c.kind {
				t.Errorf("Auswahl %d, Schritt %+v", sc.sel, st)
			}
		})
	}
}

func TestOptionValueTexts(t *testing.T) {
	s := view.Settings{Music: 0.7, Effects: 0, Fullscreen: true, Language: "en"}
	want := []string{"< 70 % >", "< 0 % >", "an", "< English >", "an", "an", ""}
	for i, w := range want {
		if got := optionValue(view.DefaultLanguage, s, i); got != w {
			t.Errorf("Zeile %d: %q, erwartet %q", i, got, w)
		}
	}
}

// Blitze und Anleitung (B-809 AC-07): A legt die Schalter um, die Session bekommt den neuen Stand; Anleitung wieder an
// setzt „Anleitung gesehen“ zurück.
func TestOptionsFlashesAndGuide(t *testing.T) {
	s := &fakeSession{cur: view.Settings{Music: 1, Effects: 1, GuideSeen: 0b101}}
	sc := newOptions(newMenus(), false)
	sc.sel = optFlashes
	sc.Update(press(pad1, view.ActionConfirm), s)
	if !s.cur.FlashesOff || optionValue("de", s.cur, optFlashes) != "aus" {
		t.Errorf("Blitze aus: %+v", s.cur)
	}
	sc.sel = optGuide
	sc.Update(press(pad1, view.ActionConfirm), s)
	if !s.cur.GuideOff || s.cur.GuideSeen != 0b101 {
		t.Errorf("Anleitung aus, gesehen bleibt: %+v", s.cur)
	}
	sc.Update(press(pad1, view.ActionRight), s)
	if s.cur.GuideOff || s.cur.GuideSeen != 0 || !s.cur.FlashesOff {
		t.Errorf("Anleitung wieder an: alle Hinweise erneut, Blitze bleiben aus: %+v", s.cur)
	}
}

// F2/F3 bei offenen Optionen (Befund Welle 4a): Ändert sich Settings.Stats oder River in der Session, nachdem das Menü
// geöffnet ist, schreibt der nächste Optionswechsel nicht den alten Stand zurück.
func TestOptionsKeepStatsAndRiver(t *testing.T) {
	s := &fakeSession{cur: view.DefaultSettings()}
	sc := newOptions(newMenus(), true)
	sc.Update(view.Input{}, s)
	s.cur.Stats, s.cur.River = true, view.RiverTiles
	sc.Update(press(pad1, view.ActionLeft), s)
	if got := s.settings[len(s.settings)-1]; !got.Stats || got.River != view.RiverTiles || got.Music != 0.9 {
		t.Errorf("F2/F3 verloren: %+v", got)
	}
}
