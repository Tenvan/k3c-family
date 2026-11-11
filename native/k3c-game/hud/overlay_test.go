package hud

import (
	"errors"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// session erfüllt view.Session für das Skill-Menü: Lernen und Respec in der Welt, der Rest ist unbenutzt.
type session struct {
	view.Session
	w    *sim.World
	sent []string
}

func (s *session) Learn(p int, id string) error {
	s.sent = append(s.sent, "learn "+id)
	return sim.LearnSkill(s.w, s.w.Players[p], id)
}

func (s *session) Respec(p int) error {
	s.sent = append(s.sent, "respec")
	if err := sim.Respec(s.w, s.w.Players[p]); err != nil {
		return errors.New("respec abgelehnt")
	}
	return nil
}

// twoPlayers baut eine Insel mit zwei Monarchen und freien Skill-Punkten; Spieler 1 an der Tastatur, 2 am Pad.
func twoPlayers(t *testing.T, points int) (*sim.World, *view.Frame) {
	t.Helper()
	isl, err := sim.CreateIsland("wald", []int{0}, 1, sim.CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	w := isl.Stages[0]
	sim.AddPlayer(w)
	sim.AddPlayer(w)
	isl.SkillPool += points
	w.SkillPoints = isl.SkillPool
	f := &view.Frame{
		Stages: []view.Stage{{Index: 0, Level: w.Biome.ID, World: w}},
		Seats:  []view.Seat{{Player: 0, Device: keys1, Monarch: 0, Stage: 0}, {Player: 1, Device: pad, Monarch: 1, Stage: 0}},
	}
	return w, f
}

func input(a, b []view.Action) view.Input {
	return view.Input{Devices: []view.DeviceInput{{Device: keys1, Actions: a}, {Device: pad, Actions: b}}}
}

// Zwei Spieler: jeder öffnet sein Menü, wählt und lernt; der andere spielt weiter (skillMenuLogic.test.ts).
func TestOverlayZweiSpieler(t *testing.T) {
	w, f := twoPlayers(t, 2)
	h := New(nil).(*hud)
	s := &session{w: w}
	st := h.Update(input(acts(view.ActionSkillMenu), nil), f, s)
	if !st.Consumed[0] || st.Consumed[1] || !h.seats[0].menu.open || h.seats[1].menu.open {
		t.Fatalf("Spieler 1 im Menü, Spieler 2 spielt: %+v", st.Consumed)
	}
	h.Update(input(acts(view.ActionRight), nil), f, s)
	h.Update(input(nil, nil), f, s)
	if h.seats[0].menu.cursor != 1 {
		t.Errorf("ein Schritt je Flanke: %d", h.seats[0].menu.cursor)
	}
	learnable := -1
	for i, e := range h.seats[0].entries {
		if e.learnable && learnable < 0 {
			learnable = i
		}
	}
	h.seats[0].menu.cursor = learnable
	st = h.Update(input(acts(view.ActionConfirm), nil), f, s)
	want := "learn " + h.seats[0].entries[learnable].id
	if len(s.sent) != 1 || s.sent[0] != want || !hasCue(st.Cues, view.CueConfirm, 0) {
		t.Errorf("gelernt: %v, Cues %v", s.sent, st.Cues)
	}
	secondPlayer(t, h, f, s)
}

// secondPlayer: Spieler 2 öffnet sein Menü mit dem Steuerkreuz, B schließt beide, danach ist B im Spiel unbelegt.
func secondPlayer(t *testing.T, h *hud, f *view.Frame, s *session) {
	t.Helper()
	st := h.Update(input(nil, acts(view.ActionDown, view.ActionSkillMenu)), f, s)
	if !st.Consumed[1] || !h.seats[1].menu.open {
		t.Error("Spieler 2 öffnet mit dem Steuerkreuz")
	}
	st = h.Update(input(acts(view.ActionBack), acts(view.ActionBack)), f, s)
	if !st.Consumed[0] || !st.Consumed[1] || h.seats[0].menu.open || h.seats[1].menu.open {
		t.Error("B schließt beide; das Update gilt noch als verbraucht")
	}
	if st = h.Update(input(acts(view.ActionBack), nil), f, s); st.Consumed[0] {
		t.Error("B im Spiel ist unbelegt")
	}
}

func TestOverlayNichtLernbarUndOhneEingabe(t *testing.T) {
	w, f := twoPlayers(t, 0)
	h := New(nil).(*hud)
	s := &session{w: w}
	h.Update(input(acts(view.ActionSkillMenu), nil), f, s)
	st := h.Update(input(acts(view.ActionConfirm), nil), f, s)
	if len(s.sent) != 0 || !hasCue(st.Cues, view.CueDenied, 0) {
		t.Errorf("nicht lernbar: nichts gesendet, abgelehnt: %v %v", s.sent, st.Cues)
	}
	h.Update(view.Input{Devices: []view.DeviceInput{{Device: pad}}}, f, s)
	if h.seats[0].menu.open {
		t.Error("Spieler ohne Eingabe: Menü zu")
	}
	h.Update(input(acts(view.ActionSkillMenu), nil), f, s)
	f.Seats = f.Seats[1:]
	h.Update(input(nil, nil), f, s)
	if h.seats[0].menu.open {
		t.Error("Spieler gegangen: Menü zu")
	}
}

func hasCue(cues []view.Cue, k view.CueKind, p int) bool {
	for _, c := range cues {
		if c.Kind == k && c.Player == p {
			return true
		}
	}
	return false
}
