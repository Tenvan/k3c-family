package view

import (
	"fmt"
	"math"

	"k3c/engine/sim"
)

// DeviceKind ist die Art eines Eingabegeräts.
type DeviceKind int

const (
	DeviceKeyboard DeviceKind = iota // Tastatur; Index 0 = linke Hälfte (WASD), 1 = rechte Hälfte (Pfeile)
	DeviceGamepad                    // Gamepad; Index = Reihenfolge der Verbindung (stabil, solange verbunden)
)

// DeviceID benennt ein Eingabegerät. game ordnet Geräte Spielern zu (Seat.Device), input kennt keine Spieler.
type DeviceID struct {
	Kind  DeviceKind
	Index int
}

func (d DeviceID) String() string {
	if d.Kind == DeviceGamepad {
		return fmt.Sprintf("pad%d", d.Index+1)
	}
	return fmt.Sprintf("keys%d", d.Index+1)
}

// Command ist die gehaltene Spiel-Eingabe eines Geräts in einem Update (Stand, keine Flanke).
type Command struct {
	Move   float64 // -1 (links) bis 1 (rechts), Totzone schon abgezogen
	Sprint bool
	Pay    bool // Bezahl-Taste gehalten
	Attack bool
	Skill  int // aktiver Skill-Slot 1 bis 4, 0 = keiner
}

// Sim liefert den Befehl der Simulation; Move wird auf -1 … 1 begrenzt, NaN zählt als 0, Skill außerhalb 1 … 4 als 0.
func (c Command) Sim() sim.PlayerCommand {
	m := c.Move
	if math.IsNaN(m) {
		m = 0
	}
	m = math.Max(-1, math.Min(1, m))
	skill := c.Skill
	if skill < 0 || skill > 4 {
		skill = 0
	}
	return sim.PlayerCommand{MoveX: m, Sprint: c.Sprint, Pay: c.Pay, Attack: c.Attack, Skill: skill}
}

// Action ist eine Menü-Aktion als Flanke (einmal je Druck, Wiederholung beim Halten liefert input).
type Action int

const (
	ActionUp Action = iota
	ActionDown
	ActionLeft
	ActionRight
	ActionConfirm  // A / Enter
	ActionBack     // B / Esc: in jedem Menü zurück, im laufenden Spiel unbelegt
	ActionMenu     // Menu / Esc im Spiel: Pause-Menü öffnen bzw. schließen
	ActionView     // View / Tab
	ActionPrevPage // LB / Q
	ActionNextPage // RB / E
)

// DeviceInput ist die Eingabe eines verbundenen Geräts in einem Update.
type DeviceInput struct {
	Device  DeviceID
	Command Command
	Actions []Action // Flanken dieses Updates in Druck-Reihenfolge
}

// Input ist die Eingabe aller verbundenen Geräte eines Updates, Tastatur zuerst, dann Gamepads nach Index.
type Input struct {
	Devices []DeviceInput
}

// Of liefert die Eingabe des Geräts d (leer, wenn es nicht verbunden ist).
func (in Input) Of(d DeviceID) (DeviceInput, bool) {
	for _, di := range in.Devices {
		if di.Device == d {
			return di, true
		}
	}
	return DeviceInput{Device: d}, false
}

// Has meldet, ob irgendein Gerät die Aktion a ausgelöst hat; dev ist das erste solche Gerät.
func (in Input) Has(a Action) (dev DeviceID, ok bool) {
	for _, di := range in.Devices {
		for _, x := range di.Actions {
			if x == a {
				return di.Device, true
			}
		}
	}
	return DeviceID{}, false
}

// Source liefert die Eingabe je Update (input: Tastatur und Gamepads; Aufnahme-Modus: feste Folge je Takt).
type Source interface {
	Poll() Input
}
