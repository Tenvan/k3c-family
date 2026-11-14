package input

// Eingabe-Quelle: liest den Rohzustand aus Ebitengine (Poll) und wertet ihn rein aus (poll). Geräte kommen und gehen:
// Ein Pad hat seinen tracker, solange seine ebiten.GamepadID verbunden ist; eine neue ID ist ein neues Gerät.

import (
	"cmp"
	"log/slog"
	"slices"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// padSlot ist der Zustand eines verbundenen Gamepads.
type padSlot struct {
	id ebiten.GamepadID
	t  tracker
}

// source hält Rohzustand, tracker und Puffer; Poll legt nach dem ersten Aufruf nichts mehr an.
type source struct {
	keys   KeyState
	ids    []ebiten.GamepadID
	raw    []PadState
	halves [2]tracker
	pads   []padSlot
	warned []ebiten.GamepadID // Pads ohne Standard-Belegung, schon gemeldet
	buf    []view.DeviceInput
	n      int
}

// New liefert die Eingabe-Quelle; lib ist einheitlich dabei, Eingabe zeichnet nichts und braucht sie nicht.
func New(_ *art.Library) view.Source { return &source{} }

// Poll liest Tastatur und Gamepads aus Ebitengine. Das Ergebnis gilt bis zum nächsten Poll.
func (s *source) Poll() view.Input {
	for _, k := range usedKeys {
		s.keys[k] = ebiten.IsKeyPressed(k)
	}
	s.ids = ebiten.AppendGamepadIDs(s.ids[:0])
	s.raw = s.raw[:0]
	for _, id := range s.ids {
		s.raw = append(s.raw, PadState{})
		readPad(id, &s.raw[len(s.raw)-1])
	}
	return s.poll(&s.keys, s.raw)
}

// readPad liest den Rohzustand des Gamepads id nach p.
func readPad(id ebiten.GamepadID, p *PadState) {
	*p = PadState{ID: id, Standard: ebiten.IsStandardGamepadLayoutAvailable(id)}
	if !p.Standard {
		p.Name = ebiten.GamepadName(id)
		return
	}
	for b := range p.Buttons {
		p.Buttons[b] = ebiten.IsStandardGamepadButtonPressed(id, ebiten.StandardGamepadButton(b))
	}
	p.X = ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisLeftStickHorizontal)
	p.Y = ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisLeftStickVertical)
}

// poll wertet den Rohzustand eines Updates aus: beide Tastatur-Hälften, dann die Gamepads mit Standard-Belegung nach
// ID (pads wird dafür sortiert). Pads ohne Standard-Belegung meldet es einmal im Log und lässt sie weg.
func (s *source) poll(k *KeyState, pads []PadState) view.Input {
	s.n = 0
	for i := range halves {
		di := s.slot(view.DeviceID{Kind: view.DeviceKeyboard, Index: i})
		di.Command = keyCommand(i, k)
		held := keyHeld(i, k)
		di.Actions = s.halves[i].step(&held, false, di.Actions)
	}
	slices.SortFunc(pads, func(a, b PadState) int { return cmp.Compare(a.ID, b.ID) })
	s.forget(pads)
	for i := range pads {
		p := &pads[i]
		if !p.Standard {
			s.warn(p)
			continue
		}
		t := s.tracker(p.ID)
		di := s.slot(view.DeviceID{Kind: view.DeviceGamepad, Index: int(p.ID)})
		di.Command = t.padCommand(p)
		held := t.padHeld(p)
		di.Actions = t.step(&held, true, di.Actions)
	}
	return view.Input{Devices: s.buf[:s.n]}
}

// forget entfernt die tracker und Meldungen getrennter Pads (Reihenfolge bleibt, Speicher wird wiederverwendet).
func (s *source) forget(pads []PadState) {
	gone := func(id ebiten.GamepadID) bool {
		return !slices.ContainsFunc(pads, func(p PadState) bool { return p.ID == id })
	}
	s.pads = slices.DeleteFunc(s.pads, func(ps padSlot) bool { return gone(ps.id) })
	s.warned = slices.DeleteFunc(s.warned, gone)
}

// tracker liefert den tracker des Pads id; ein neues Pad bekommt einen frischen (erstes Update ohne Aktionen).
func (s *source) tracker(id ebiten.GamepadID) *tracker {
	for i := range s.pads {
		if s.pads[i].id == id {
			return &s.pads[i].t
		}
	}
	s.pads = append(s.pads, padSlot{id: id})
	return &s.pads[len(s.pads)-1].t
}

// warn meldet ein Pad ohne Standard-Belegung einmal, solange es verbunden ist.
func (s *source) warn(p *PadState) {
	if slices.Contains(s.warned, p.ID) {
		return
	}
	s.warned = append(s.warned, p.ID)
	slog.Warn("🚫 Gamepad ohne Standard-Belegung ignoriert", "pad", int(p.ID), "name", p.Name)
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
