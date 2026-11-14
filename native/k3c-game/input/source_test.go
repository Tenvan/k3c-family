package input

import (
	"slices"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

var (
	keys1 = view.DeviceID{Kind: view.DeviceKeyboard, Index: 0}
	keys2 = view.DeviceID{Kind: view.DeviceKeyboard, Index: 1}
)

func padID(id int) view.DeviceID { return view.DeviceID{Kind: view.DeviceGamepad, Index: id} }

func deviceIDs(in view.Input) []view.DeviceID {
	var out []view.DeviceID
	for _, di := range in.Devices {
		out = append(out, di.Device)
	}
	return out
}

// AC-02: Geräte-Liste bei kein Pad, Pad kommt, zwei Pads, Pad geht und kommt mit neuer ID wieder: Tastatur zuerst,
// dann Pads nach ebiten.GamepadID, nie ein fester Pad-Index; Pads ohne Standard-Belegung fehlen.
func TestDevicesHotPlug(t *testing.T) {
	cases := []struct {
		name string
		pads []PadState
		want []view.DeviceID
	}{
		{"kein Pad", nil, []view.DeviceID{keys1, keys2}},
		{"Pad kommt", []PadState{padState(5)}, []view.DeviceID{keys1, keys2, padID(5)}},
		{"zweites Pad, unsortiert gemeldet", []PadState{padState(5), padState(2)}, []view.DeviceID{keys1, keys2, padID(2), padID(5)}},
		{"Pad 2 geht", []PadState{padState(5)}, []view.DeviceID{keys1, keys2, padID(5)}},
		{"Pad kommt mit neuer ID", []PadState{padState(7), padState(5)}, []view.DeviceID{keys1, keys2, padID(5), padID(7)}},
		{"Pad ohne Standard-Belegung", []PadState{padState(5), {ID: 9, Name: "Lenkrad"}}, []view.DeviceID{keys1, keys2, padID(5)}},
		{"alle Pads weg", nil, []view.DeviceID{keys1, keys2}},
	}
	s := &source{}
	for _, c := range cases {
		if got := deviceIDs(s.poll(keyState(), slices.Clone(c.pads))); !slices.Equal(got, c.want) {
			t.Errorf("%s: %v, erwartet %v", c.name, got, c.want)
		}
	}
	if len(s.pads) != 0 || len(s.warned) != 0 {
		t.Errorf("Zustand getrennter Pads bleibt: %d tracker, %d Meldungen", len(s.pads), len(s.warned))
	}
}

// Ein neu verbundenes Pad löst im ersten Update nichts aus, auch wenn es eine frühere ID wieder bekommt.
func TestReconnectedPadStartsFresh(t *testing.T) {
	s := &source{}
	s.poll(keyState(), []PadState{padState(1)})
	s.poll(keyState(), nil)
	in := s.poll(keyState(), []PadState{padState(1, padA)})
	if di, ok := in.Of(padID(1)); !ok || len(di.Actions) != 0 || !di.Command.Pay {
		t.Errorf("wiederverbundenes Pad: %+v", di)
	}
}

// Pad ohne Standard-Belegung: eine Meldung, solange es verbunden ist; nach Trennen und Wiederkommen wieder eine.
func TestNonStandardPadWarnsOnce(t *testing.T) {
	s := &source{}
	odd := []PadState{{ID: 4, Name: "Lenkrad"}}
	for range 3 {
		s.poll(keyState(), slices.Clone(odd))
	}
	if !slices.Equal(s.warned, []ebiten.GamepadID{4}) {
		t.Errorf("Meldungen: %v", s.warned)
	}
	s.poll(keyState(), nil)
	if len(s.warned) != 0 {
		t.Errorf("Meldung bleibt nach Trennen: %v", s.warned)
	}
}

// pollTwice liefert die Eingabe des zweiten Updates (nach einem leeren ersten Update mit denselben Geräten).
func pollTwice(k *KeyState, pads []PadState) view.Input {
	s := &source{}
	idle := make([]PadState, len(pads))
	for i, p := range pads {
		idle[i] = padState(p.ID)
	}
	s.poll(keyState(), idle)
	return s.poll(k, pads)
}

// Zwei Spieler gleichzeitig ohne Übersprechen: Jedes Gerät bekommt nur seine Tasten, auch wenn alle zugleich drücken.
func TestNoCrosstalk(t *testing.T) {
	right := padState(3, padRT, padX)
	right.X = 1
	in := pollTwice(keyState(ebiten.KeyA, ebiten.KeySpace, ebiten.KeyArrowRight, ebiten.KeyNumpad1),
		[]PadState{padState(1, padA, padLeft), right})
	want := []struct {
		dev     view.DeviceID
		cmd     view.Command
		actions []view.Action
	}{
		{keys1, view.Command{Move: -1, Pay: true}, []view.Action{view.ActionLeft, view.ActionConfirm}},
		{keys2, view.Command{Move: 1, Skill: 1}, []view.Action{view.ActionRight}},
		{padID(1), view.Command{Move: -1, Pay: true}, []view.Action{view.ActionLeft, view.ActionConfirm}},
		{padID(3), view.Command{Move: 1, Sprint: true, Attack: true}, []view.Action{view.ActionRight}},
	}
	for _, w := range want {
		di, ok := in.Of(w.dev)
		if !ok || di.Command != w.cmd || !slices.Equal(di.Actions, w.actions) {
			t.Errorf("%v: %+v, erwartet %+v %v", w.dev, di, w.cmd, w.actions)
		}
	}
}

// Poll legt nach dem ersten Aufruf nichts mehr an, auch mit zwei Pads, Aktionen und Wiederholung.
func TestPollAllocs(t *testing.T) {
	s := &source{}
	pressed := []PadState{padState(1, padA, padDown, padView, padMenu), padState(2, padB, padLB)}
	idle := []PadState{padState(1), padState(2)}
	kPressed, kIdle := keyState(ebiten.KeySpace, ebiten.KeyEscape, ebiten.KeyArrowUp), keyState()
	for range 3 {
		s.poll(kPressed, pressed)
		s.poll(kIdle, idle)
	}
	n := 0
	allocs := testing.AllocsPerRun(100, func() {
		if n++; n%2 == 0 {
			s.poll(kPressed, pressed)
		} else {
			s.poll(kIdle, idle)
		}
	})
	if allocs != 0 {
		t.Errorf("Poll legt %v Mal je Aufruf an", allocs)
	}
}
