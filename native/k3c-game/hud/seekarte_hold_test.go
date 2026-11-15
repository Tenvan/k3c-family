package hud

import (
	"errors"
	"slices"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// payInput: Eingabe beider Spieler mit gehaltener Bezahl-Taste (pay1, pay2) und den Flanken a, b.
func payInput(pay1, pay2 bool, a, b []view.Action) view.Input {
	in := input(a, b)
	in.Devices[0].Command.Pay, in.Devices[1].Command.Pay = pay1, pay2
	return in
}

// heldSea: Seekarte offen, beide am Steg, Spieler 1 hat eben bestätigt und hält A.
func heldSea(t *testing.T, err error) (*sim.World, *view.Frame, *hud, *seaSession) {
	t.Helper()
	w, f := seaWorld(t)
	w.IslandSwitch = openSwitch("b")
	h := New(nil).(*hud)
	h.Listen(f)
	s := &seaSession{picks: make([]string, 0, 4), err: err}
	if st := h.Update(payInput(true, true, acts(view.ActionConfirm), nil), f, s); !st.Consumed[0] || st.Consumed[1] {
		t.Fatalf("Spieler 1 bestätigt (verbraucht), Spieler 2 hält A und zahlt: %v", st.Consumed)
	}
	return w, f, h, s
}

// B-1006/AC-01: Nach dem Bestätigen bleibt die Eingabe verbraucht, solange A gehalten ist; ein Update ohne A beendet
// die Sperre, erneutes Halten zahlt wieder. Gilt auch bei abgelehnter Wahl.
func TestSeekarteGehaltenesAZahltNicht(t *testing.T) {
	for _, err := range []error{nil, errors.New("abgelehnt")} {
		_, f, h, s := heldSea(t, err)
		for i := range 3 {
			if st := h.Update(payInput(true, false, nil, nil), f, s); !st.Consumed[0] {
				t.Fatalf("err=%v: gehaltenes A zahlt im Folgetakt %d", err, i+1)
			}
		}
		if st := h.Update(payInput(false, false, nil, nil), f, s); st.Consumed[0] {
			t.Errorf("err=%v: A losgelassen, Sperre endet", err)
		}
		if st := h.Update(payInput(true, false, nil, nil), f, s); st.Consumed[0] {
			t.Errorf("err=%v: erneut gehaltenes A zahlt wie überall", err)
		}
		if len(s.picks) != 1 {
			t.Errorf("genau eine Wahl: %v", s.picks)
		}
	}
}

// B-1006/AC-02: Die Sperre gilt je Spieler; der Mitspieler am selben Steg zahlt und wählt unabhängig weiter.
func TestSeekarteSperreJeSpieler(t *testing.T) {
	_, f, h, s := heldSea(t, nil)
	if st := h.Update(payInput(true, true, nil, nil), f, s); !st.Consumed[0] || st.Consumed[1] {
		t.Errorf("Sperre von Spieler 1 trifft Spieler 2 nicht: %v", st.Consumed)
	}
	if st := h.Update(payInput(true, true, nil, acts(view.ActionConfirm)), f, s); !st.Consumed[0] || !st.Consumed[1] {
		t.Errorf("Spieler 2 wählt unabhängig: %v", st.Consumed)
	}
	if st := h.Update(payInput(false, true, nil, nil), f, s); st.Consumed[0] || !st.Consumed[1] {
		t.Errorf("Spieler 1 lässt los, Spieler 2 hält weiter: %v", st.Consumed)
	}
	if !slices.Equal(s.picks, []string{"0:b", "1:b"}) {
		t.Errorf("Wahlen: %v", s.picks)
	}
}

// Die Sperre endet auch beim Verlassen des Stegs, bei Kronenverlust, getrenntem Gerät und geschlossener Karte.
func TestSeekarteSperreEndet(t *testing.T) {
	cases := map[string]func(w *sim.World, in *view.Input){
		"Steg verlassen": func(w *sim.World, _ *view.Input) { w.Players[0].X = 80 },
		"Krone verloren": func(w *sim.World, _ *view.Input) { w.Players[0].Crown = &sim.Crown{} },
		"Gerät getrennt": func(_ *sim.World, in *view.Input) { in.Devices = in.Devices[1:] },
		"Karte zu":       func(w *sim.World, _ *view.Input) { w.IslandSwitch = nil },
	}
	for name, change := range cases {
		w, f, h, s := heldSea(t, nil)
		in := payInput(true, false, nil, nil)
		change(w, &in)
		h.Listen(f)
		if st := h.Update(in, f, s); st.Consumed[0] || h.seats[0].sea.hold {
			t.Errorf("%s: Sperre endet", name)
		}
	}
}

// Ein offenes Skill-Menü führt die Sperre weiter: gehalten bleibt sie, im Menü losgelassen endet sie.
func TestSeekarteSperreImSkillMenue(t *testing.T) {
	_, f, h, s := heldSea(t, nil)
	if st := h.Update(payInput(true, false, acts(view.ActionSkillMenu), nil), f, s); !st.Consumed[0] || !h.seats[0].sea.hold {
		t.Error("Skill-Menü offen, A gehalten: Sperre bleibt")
	}
	if h.Update(payInput(false, false, nil, nil), f, s); h.seats[0].sea.hold {
		t.Error("im Skill-Menü losgelassen: Sperre endet")
	}
}

// B-1006/AC-03: 0 Allokationen je Update mit aktiver Sperre.
func TestSeekarteSperreOhneAllokationen(t *testing.T) {
	_, f, h, s := heldSea(t, nil)
	in := payInput(true, true, nil, nil)
	n := testing.AllocsPerRun(50, func() {
		if st := h.Update(in, f, s); !st.Consumed[0] || st.Consumed[1] {
			t.Fatal("Sperre aktiv für Spieler 1, Spieler 2 zahlt")
		}
	})
	if n != 0 {
		t.Errorf("Update mit aktiver Sperre: %v Allokationen", n)
	}
}
