// Package input liest Tastatur und Gamepads und liefert view.Input (Fach-Sprint der Eingabe, NG5). Bis dahin
// Platzhalter: zwei Tastatur-Hälften und alle Gamepads mit Standard-Belegung, je ein Gerät.
//
//	keys1 (links):  A/D laufen, Shift links Sprint, S zahlen, Leertaste Schlag; Menü: W/A/S/D, Leertaste, Esc, Pos1
//	keys2 (rechts): ←/→ laufen, Shift rechts Sprint, ↓ zahlen, Enter Schlag; Menü: Pfeile, Enter, Rücktaste
//	Gamepad:        Stick/D-Pad laufen, RT Sprint, A zahlen, X Schlag; Menü: D-Pad, A, B zurück, Menu, View, LB/RB,
//	                View + Menu gemeinsam = ActionHome
//
// B ist im Spiel unbelegt (nur ActionBack, die Schleife wertet sie im Spiel nicht aus).
package input

import (
	"slices"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// padDeadZone ist die Totzone des linken Sticks.
const padDeadZone = 0.25

type keyAction struct {
	key    ebiten.Key
	action view.Action
}

// keyLayout ist die Belegung einer Tastatur-Hälfte.
type keyLayout struct {
	left, right, sprint, pay, attack ebiten.Key
	actions                          []keyAction
}

// halves sind die beiden Tastatur-Hälften (DeviceID Index 0 und 1).
var halves = [2]keyLayout{
	{ebiten.KeyA, ebiten.KeyD, ebiten.KeyShiftLeft, ebiten.KeyS, ebiten.KeySpace, []keyAction{
		{ebiten.KeyW, view.ActionUp}, {ebiten.KeyS, view.ActionDown}, {ebiten.KeyA, view.ActionLeft},
		{ebiten.KeyD, view.ActionRight}, {ebiten.KeySpace, view.ActionConfirm}, {ebiten.KeyEscape, view.ActionBack},
		{ebiten.KeyEscape, view.ActionMenu}, {ebiten.KeyTab, view.ActionView}, {ebiten.KeyQ, view.ActionPrevPage},
		{ebiten.KeyE, view.ActionNextPage}, {ebiten.KeyHome, view.ActionHome},
	}},
	{ebiten.KeyArrowLeft, ebiten.KeyArrowRight, ebiten.KeyShiftRight, ebiten.KeyArrowDown, ebiten.KeyEnter, []keyAction{
		{ebiten.KeyArrowUp, view.ActionUp}, {ebiten.KeyArrowDown, view.ActionDown}, {ebiten.KeyArrowLeft, view.ActionLeft},
		{ebiten.KeyArrowRight, view.ActionRight}, {ebiten.KeyEnter, view.ActionConfirm}, {ebiten.KeyBackspace, view.ActionBack},
	}},
}

type padAction struct {
	button ebiten.StandardGamepadButton
	action view.Action
}

// padActions ist die Menü-Belegung der Gamepads (Standard-Layout, Xbox-Namen).
var padActions = []padAction{
	{ebiten.StandardGamepadButtonLeftTop, view.ActionUp},
	{ebiten.StandardGamepadButtonLeftBottom, view.ActionDown},
	{ebiten.StandardGamepadButtonLeftLeft, view.ActionLeft},
	{ebiten.StandardGamepadButtonLeftRight, view.ActionRight},
	{ebiten.StandardGamepadButtonRightBottom, view.ActionConfirm}, // A
	{ebiten.StandardGamepadButtonRightRight, view.ActionBack},     // B
	{ebiten.StandardGamepadButtonCenterRight, view.ActionMenu},    // Menu
	{ebiten.StandardGamepadButtonCenterLeft, view.ActionView},     // View
	{ebiten.StandardGamepadButtonFrontTopLeft, view.ActionPrevPage},
	{ebiten.StandardGamepadButtonFrontTopRight, view.ActionNextPage},
}

// source hält die Puffer der Eingabe; Poll legt nach dem ersten Bild nichts mehr an.
type source struct {
	buf  []view.DeviceInput
	n    int
	pads []ebiten.GamepadID
}

// New liefert die Eingabe-Quelle; lib ist einheitlich dabei, Eingabe zeichnet nichts und braucht sie nicht.
func New(_ *art.Library) view.Source { return &source{} }

// Poll liefert beide Tastatur-Hälften und die Gamepads nach Index. Das Ergebnis gilt bis zum nächsten Poll.
func (s *source) Poll() view.Input {
	s.n = 0
	for i := range halves {
		di := s.slot(view.DeviceID{Kind: view.DeviceKeyboard, Index: i})
		di.Command = CommandOf(i, ebiten.IsKeyPressed)
		for _, ka := range halves[i].actions {
			if inpututil.IsKeyJustPressed(ka.key) {
				di.Actions = append(di.Actions, ka.action)
			}
		}
	}
	s.pads = ebiten.AppendGamepadIDs(s.pads[:0])
	slices.Sort(s.pads)
	for _, id := range s.pads {
		if ebiten.IsStandardGamepadLayoutAvailable(id) {
			s.pollPad(id)
		}
	}
	return view.Input{Devices: s.buf[:s.n]}
}

// pollPad liest das Gamepad id; sein Index ist die Gamepad-ID von Ebitengine (stabil, solange es verbunden ist).
func (s *source) pollPad(id ebiten.GamepadID) {
	di := s.slot(view.DeviceID{Kind: view.DeviceGamepad, Index: int(id)})
	pressed := func(b ebiten.StandardGamepadButton) bool { return ebiten.IsStandardGamepadButtonPressed(id, b) }
	di.Command = PadCommand(ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisLeftStickHorizontal), pressed)
	for _, pa := range padActions {
		if inpututil.IsStandardGamepadButtonJustPressed(id, pa.button) {
			di.Actions = append(di.Actions, pa.action)
		}
	}
	just := func(b ebiten.StandardGamepadButton) bool { return inpututil.IsStandardGamepadButtonJustPressed(id, b) }
	bView, bMenu := ebiten.StandardGamepadButtonCenterLeft, ebiten.StandardGamepadButtonCenterRight
	if HomeChord(pressed(bView), pressed(bMenu), just(bView), just(bMenu)) {
		di.Actions = append(di.Actions, view.ActionHome)
	}
}

// HomeChord meldet „View + Menu gemeinsam“ (ActionHome): beide gehalten und mindestens eine in diesem Update gedrückt,
// damit Halten nicht wiederholt auslöst.
func HomeChord(heldView, heldMenu, justView, justMenu bool) bool {
	return heldView && heldMenu && (justView || justMenu)
}

// slot liefert den nächsten Eintrag des Puffers für Gerät d, geleert; die Aktionen behalten ihren Speicher.
func (s *source) slot(d view.DeviceID) *view.DeviceInput {
	if s.n == len(s.buf) {
		s.buf = append(s.buf, view.DeviceInput{})
	}
	di := &s.buf[s.n]
	s.n++
	di.Device, di.Command, di.Actions = d, view.Command{}, di.Actions[:0]
	return di
}

// CommandOf liefert den Befehl der Tastatur-Hälfte half (0 links, 1 rechts) aus den gehaltenen Tasten.
func CommandOf(half int, pressed func(ebiten.Key) bool) view.Command {
	l := halves[half]
	var c view.Command
	if pressed(l.left) {
		c.Move--
	}
	if pressed(l.right) {
		c.Move++
	}
	c.Sprint, c.Pay, c.Attack = pressed(l.sprint), pressed(l.pay), pressed(l.attack)
	return c
}

// PadCommand liefert den Befehl eines Gamepads aus dem linken Stick (stickX, -1 … 1) und den gehaltenen Tasten; das
// D-Pad geht vor dem Stick.
func PadCommand(stickX float64, pressed func(ebiten.StandardGamepadButton) bool) view.Command {
	c := view.Command{
		Move:   deadZone(stickX),
		Sprint: pressed(ebiten.StandardGamepadButtonFrontBottomRight), // RT
		Pay:    pressed(ebiten.StandardGamepadButtonRightBottom),      // A
		Attack: pressed(ebiten.StandardGamepadButtonRightLeft),        // X
	}
	left, right := pressed(ebiten.StandardGamepadButtonLeftLeft), pressed(ebiten.StandardGamepadButtonLeftRight)
	switch {
	case left && !right:
		c.Move = -1
	case right && !left:
		c.Move = 1
	}
	return c
}

// deadZone zieht die Totzone ab und skaliert den Rest wieder auf 0 … 1.
func deadZone(x float64) float64 {
	switch {
	case x > padDeadZone:
		return min(1, (x-padDeadZone)/(1-padDeadZone))
	case x < -padDeadZone:
		return max(-1, (x+padDeadZone)/(1-padDeadZone))
	}
	return 0
}
