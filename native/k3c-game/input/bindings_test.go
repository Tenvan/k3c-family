package input

import (
	"slices"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// keyState liefert eine Tastatur mit den gehaltenen Tasten ks.
func keyState(ks ...ebiten.Key) *KeyState {
	var k KeyState
	for _, x := range ks {
		k[x] = true
	}
	return &k
}

// padState liefert ein Pad mit Standard-Belegung, den gehaltenen Tasten bs und dem Stick in Ruhe.
func padState(id ebiten.GamepadID, bs ...ebiten.StandardGamepadButton) PadState {
	p := PadState{ID: id, Standard: true}
	for _, b := range bs {
		p.Buttons[b] = true
	}
	return p
}

// AC-01: Jede Zeile der Belegung (Befehl, gehalten) ergibt den erwarteten Befehl, je Tastatur-Hälfte.
func TestKeyCommandTable(t *testing.T) {
	cases := []struct {
		name string
		half int
		keys []ebiten.Key
		want view.Command
	}{
		{"keys1 nichts", 0, nil, view.Command{}},
		{"keys1 links", 0, keys(ebiten.KeyA), view.Command{Move: -1}},
		{"keys1 rechts", 0, keys(ebiten.KeyD), view.Command{Move: 1}},
		{"keys1 beide Richtungen", 0, keys(ebiten.KeyA, ebiten.KeyD), view.Command{}},
		{"keys1 Galopp", 0, keys(ebiten.KeyD, ebiten.KeyShiftLeft), view.Command{Move: 1, Sprint: true}},
		{"keys1 zahlen", 0, keys(ebiten.KeySpace), view.Command{Pay: true}},
		{"keys1 Schlag", 0, keys(ebiten.KeyE), view.Command{Attack: true}},
		{"keys1 Skill 1", 0, keys(ebiten.KeyQ), view.Command{Skill: 1}},
		{"keys1 Skill 2", 0, keys(ebiten.KeyR), view.Command{Skill: 2}},
		{"keys1 Skill 3", 0, keys(ebiten.KeyT), view.Command{Skill: 3}},
		{"keys1 Skill 4 (Z)", 0, keys(ebiten.KeyZ), view.Command{Skill: 4}},
		{"keys1 Skill 4 (Z auf QWERTZ)", 0, keys(ebiten.KeyY), view.Command{Skill: 4}},
		{"keys1 niedrigster Skill gewinnt", 0, keys(ebiten.KeyT, ebiten.KeyR), view.Command{Skill: 2}},
		{"keys1 ignoriert rechte Hälfte", 0, keys(ebiten.KeyArrowLeft, ebiten.KeyEnter, ebiten.KeyNumpad0), view.Command{}},
		{"keys2 links", 1, keys(ebiten.KeyArrowLeft), view.Command{Move: -1}},
		{"keys2 rechts", 1, keys(ebiten.KeyArrowRight), view.Command{Move: 1}},
		{"keys2 Galopp Strg", 1, keys(ebiten.KeyControlRight), view.Command{Sprint: true}},
		{"keys2 Galopp Shift", 1, keys(ebiten.KeyShiftRight), view.Command{Sprint: true}},
		{"keys2 zahlen Enter", 1, keys(ebiten.KeyEnter), view.Command{Pay: true}},
		{"keys2 zahlen Num-Enter", 1, keys(ebiten.KeyNumpadEnter), view.Command{Pay: true}},
		{"keys2 Schlag", 1, keys(ebiten.KeyNumpad0), view.Command{Attack: true}},
		{"keys2 Skill 1", 1, keys(ebiten.KeyNumpad1), view.Command{Skill: 1}},
		{"keys2 Skill 4", 1, keys(ebiten.KeyNumpad4), view.Command{Skill: 4}},
		{"keys2 ignoriert linke Hälfte", 1, keys(ebiten.KeyD, ebiten.KeySpace, ebiten.KeyE, ebiten.KeyShiftLeft), view.Command{}},
	}
	for _, c := range cases {
		if got := keyCommand(c.half, keyState(c.keys...)); got != c.want {
			t.Errorf("%s: %+v, erwartet %+v", c.name, got, c.want)
		}
	}
}

// AC-01: Jede Zeile der Belegung (Befehl, gehalten) am Gamepad; B ist im Spiel unbelegt.
func TestPadCommandTable(t *testing.T) {
	cases := []struct {
		name string
		x    float64
		bs   []ebiten.StandardGamepadButton
		want view.Command
	}{
		{"nichts", 0, nil, view.Command{}},
		{"Stick in der Totzone", 0.2, nil, view.Command{}},
		{"Stick ganz rechts", 1, nil, view.Command{Move: 1}},
		{"Stick ganz links", -1, nil, view.Command{Move: -1}},
		{"Steuerkreuz links", 0, []ebiten.StandardGamepadButton{padLeft}, view.Command{Move: -1}},
		{"Steuerkreuz rechts", 0, []ebiten.StandardGamepadButton{padRight}, view.Command{Move: 1}},
		{"Steuerkreuz geht vor Stick", 0.9, []ebiten.StandardGamepadButton{padLeft}, view.Command{Move: -1}},
		{"Galopp RT", 1, []ebiten.StandardGamepadButton{padRT}, view.Command{Move: 1, Sprint: true}},
		{"zahlen A", 0, []ebiten.StandardGamepadButton{padA}, view.Command{Pay: true}},
		{"Schlag X", 0, []ebiten.StandardGamepadButton{padX}, view.Command{Attack: true}},
		{"Skill 1 LB", 0, []ebiten.StandardGamepadButton{padLB}, view.Command{Skill: 1}},
		{"Skill 2 RB", 0, []ebiten.StandardGamepadButton{padRB}, view.Command{Skill: 2}},
		{"Skill 3 LT", 0, []ebiten.StandardGamepadButton{padLT}, view.Command{Skill: 3}},
		{"Skill 4 Steuerkreuz hoch", 0, []ebiten.StandardGamepadButton{padUp}, view.Command{Skill: 4}},
		{"niedrigster Skill gewinnt", 0, []ebiten.StandardGamepadButton{padUp, padRB}, view.Command{Skill: 2}},
		{"B unbelegt", 0, []ebiten.StandardGamepadButton{padB}, view.Command{}},
		{"View, Menu, Y unbelegt", 0, []ebiten.StandardGamepadButton{padView, padMenu, ebiten.StandardGamepadButtonRightTop}, view.Command{}},
	}
	for _, c := range cases {
		var tr tracker
		p := padState(0, c.bs...)
		p.X = c.x
		if got := tr.padCommand(&p); got != c.want {
			t.Errorf("%s: %+v, erwartet %+v", c.name, got, c.want)
		}
	}
}

// pressOnce liefert die Aktionen, wenn ein frisches Gerät nach einem leeren Update held drückt.
func pressOnce(held [actionCount]bool, pad bool) []view.Action {
	var tr tracker
	var none [actionCount]bool
	tr.step(&none, pad, nil)
	return tr.step(&held, pad, nil)
}

// AC-01: Jede Menü-Taste beider Tastatur-Hälften löst beim Drücken genau ihre Aktion(en) aus.
func TestKeyActionTable(t *testing.T) {
	cases := []struct {
		half int
		key  ebiten.Key
		want []view.Action
	}{
		{0, ebiten.KeyW, []view.Action{view.ActionUp}},
		{0, ebiten.KeyS, []view.Action{view.ActionDown}},
		{0, ebiten.KeyA, []view.Action{view.ActionLeft}},
		{0, ebiten.KeyD, []view.Action{view.ActionRight}},
		{0, ebiten.KeySpace, []view.Action{view.ActionConfirm}},
		{0, ebiten.KeyEscape, []view.Action{view.ActionBack, view.ActionMenu}},
		{0, ebiten.KeyTab, []view.Action{view.ActionView}},
		{0, ebiten.KeyQ, []view.Action{view.ActionPrevPage}},
		{0, ebiten.KeyE, []view.Action{view.ActionNextPage}},
		{0, ebiten.KeyHome, []view.Action{view.ActionHome}},
		{0, ebiten.KeyK, []view.Action{view.ActionSkillMenu}},
		{0, ebiten.KeyF, []view.Action{view.ActionFullscreen}},
		{0, ebiten.KeyNumpad5, nil},
		{0, ebiten.KeyArrowUp, nil},
		{1, ebiten.KeyArrowUp, []view.Action{view.ActionUp}},
		{1, ebiten.KeyArrowDown, []view.Action{view.ActionDown}},
		{1, ebiten.KeyArrowLeft, []view.Action{view.ActionLeft}},
		{1, ebiten.KeyArrowRight, []view.Action{view.ActionRight}},
		{1, ebiten.KeyEnter, []view.Action{view.ActionConfirm}},
		{1, ebiten.KeyNumpadEnter, []view.Action{view.ActionConfirm}},
		{1, ebiten.KeyBackspace, []view.Action{view.ActionBack, view.ActionMenu}},
		{1, ebiten.KeyPageUp, []view.Action{view.ActionPrevPage}},
		{1, ebiten.KeyPageDown, []view.Action{view.ActionNextPage}},
		{1, ebiten.KeyNumpad5, []view.Action{view.ActionSkillMenu}},
		{1, ebiten.KeyK, nil},
		{1, ebiten.KeyF, nil},
		{1, ebiten.KeyW, nil},
	}
	for _, c := range cases {
		if got := pressOnce(keyHeld(c.half, keyState(c.key)), false); !slices.Equal(got, c.want) {
			t.Errorf("keys%d %v: %v, erwartet %v", c.half+1, c.key, got, c.want)
		}
	}
}

// AC-01: Jede Menü-Taste des Gamepads löst beim Drücken ihre Aktion aus; Menu und View erst beim Loslassen.
func TestPadActionTable(t *testing.T) {
	cases := []struct {
		b    ebiten.StandardGamepadButton
		want []view.Action
	}{
		{padUp, []view.Action{view.ActionUp}},
		{padDown, []view.Action{view.ActionDown, view.ActionSkillMenu}}, // Menü nimmt ↓, das Skill-Menü (Spiel) das andere
		{padLeft, []view.Action{view.ActionLeft}},
		{padRight, []view.Action{view.ActionRight}},
		{padA, []view.Action{view.ActionConfirm}},
		{padB, []view.Action{view.ActionBack}},
		{padLB, []view.Action{view.ActionPrevPage}},
		{padRB, []view.Action{view.ActionNextPage}},
		{padMenu, nil},
		{padView, nil},
		{padX, nil},
		{padRT, nil},
		{ebiten.StandardGamepadButtonRightStick, []view.Action{view.ActionFullscreen}},
	}
	for _, c := range cases {
		var tr tracker
		idle, p := padState(0), padState(0, c.b)
		h0 := tr.padHeld(&idle)
		tr.step(&h0, true, nil)
		h1 := tr.padHeld(&p)
		if got := tr.step(&h1, true, nil); !slices.Equal(got, c.want) {
			t.Errorf("Taste %v: %v, erwartet %v", c.b, got, c.want)
		}
	}
}

// Der Stick nach unten ist nur Menü-Richtung: Das Skill-Menü öffnet nur das Steuerkreuz (wie im Browser).
func TestStickDownIsNoSkillMenu(t *testing.T) {
	var tr tracker
	idle, down := padState(0), padState(0)
	down.Y = 1
	h0 := tr.padHeld(&idle)
	tr.step(&h0, true, nil)
	h1 := tr.padHeld(&down)
	if got := tr.step(&h1, true, nil); !slices.Equal(got, []view.Action{view.ActionDown}) {
		t.Errorf("Stick ↓: %v, erwartet nur ActionDown", got)
	}
}

// Jede Taste der Belegung wird bei Ebitengine abgefragt; keine Taste liegt doppelt in den Befehlen einer Hälfte.
func TestUsedKeysAndHalves(t *testing.T) {
	for i := range halves {
		l := &halves[i]
		var seen []ebiten.Key
		groups := append([][]ebiten.Key{l.left, l.right, l.sprint, l.pay, l.attack}, l.skills[:]...)
		for _, g := range groups {
			for _, k := range g {
				if slices.Contains(seen, k) {
					t.Errorf("keys%d: Taste %v doppelt im Befehl", i+1, k)
				}
				seen = append(seen, k)
				if !slices.Contains(usedKeys, k) {
					t.Errorf("keys%d: Taste %v wird nicht abgefragt", i+1, k)
				}
			}
		}
	}
}
