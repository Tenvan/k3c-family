package menu

import (
	"testing"

	"k3c/native/k3c-game/view"
)

// Optionen › Steuerung öffnet mit A die Unterseite; Links/Rechts auf der Zeile ändert nichts.
func TestOptionsOpenControls(t *testing.T) {
	sc := newOptions(newMenus(), false)
	s := &fakeSession{}
	sc.sel = optControls
	if st := sc.Update(press(pad1, view.ActionRight), s); st.Kind != view.StepStay || len(s.settings) != 0 {
		t.Fatalf("rechts auf Steuerung: %+v, %d Mal gespeichert", st, len(s.settings))
	}
	st := sc.Update(press(pad1, view.ActionConfirm), s)
	if _, ok := st.Next.(*controlsScreen); st.Kind != view.StepPush || !ok {
		t.Errorf("A auf Steuerung: %+v", st)
	}
}

// B-893/AC-03: Umschalten wirkt sofort (SetSettings mit dem neuen Profil); AC-02: Spieler 2 bekommt die Tastatur
// nicht, solange Spieler 1 sie hat, wohl aber, nachdem Spieler 1 auf Gamepad gewechselt hat.
func TestControlsSwitchProfiles(t *testing.T) {
	sc := newControls(newMenus(), false)
	s := &fakeSession{}
	sc.Update(press(pad1, view.ActionRight), s) // Spieler 1: WASD → Pfeile
	if len(s.settings) != 1 || s.settings[0].Controls[0] != view.ProfileArrows {
		t.Fatalf("Spieler 1 nach rechts: %+v", s.settings)
	}
	sc.Update(press(pad1, view.ActionDown), s)
	sc.Update(press(pad1, view.ActionRight), s) // Spieler 2: Gamepad, Tastatur vergeben → bleibt Gamepad
	if len(s.settings) != 1 || sc.cur.Controls[1] != view.ProfileGamepad {
		t.Fatalf("Spieler 2 bekommt die Tastatur: %v (%d Mal gespeichert)", sc.cur.Controls, len(s.settings))
	}
	sc.Update(press(pad1, view.ActionUp), s)
	sc.Update(press(pad1, view.ActionRight), s) // Spieler 1: Pfeile → Gamepad
	sc.Update(press(pad1, view.ActionDown), s)
	sc.Update(press(pad1, view.ActionRight), s) // Spieler 2: Gamepad → WASD
	if got := sc.cur.Controls; got[0] != view.ProfileGamepad || got[1] != view.ProfileWASD {
		t.Errorf("nach Tausch: %v", got)
	}
	for _, set := range s.settings {
		if set.Controls[0] != view.ProfileGamepad && set.Controls[1] != view.ProfileGamepad {
			t.Errorf("zwei Tastatur-Spieler gespeichert: %v", set.Controls)
		}
	}
}

// Zurück mit A auf der letzten Zeile, mit B überall; die Zeilen zeigen die Profile.
func TestControlsBackAndTexts(t *testing.T) {
	sc := newControls(newMenus(), false)
	s := &fakeSession{}
	sc.Update(press(pad1, view.ActionUp), s)
	if sc.sel != ctlBack {
		t.Fatalf("hoch springt ans Ende: %d", sc.sel)
	}
	if st := sc.Update(press(pad1, view.ActionConfirm), s); st.Kind != view.StepPop {
		t.Errorf("A auf Zurück: %+v", st)
	}
	sc = newControls(newMenus(), false)
	if st := sc.Update(press(pad1, view.ActionBack), s); st.Kind != view.StepPop {
		t.Errorf("B: %+v", st)
	}
	if got := profileText("de", view.ProfileArrows); got != "Tastatur Pfeile" {
		t.Errorf("Profil-Text %q", got)
	}
	if got := profileText("en", view.ProfileGamepad); got != "Gamepad" {
		t.Errorf("profile text %q", got)
	}
}
