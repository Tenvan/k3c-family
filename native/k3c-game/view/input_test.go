package view

import (
	"math"
	"testing"

	"k3c/engine/sim"
)

func TestCommandSimClamps(t *testing.T) {
	cases := []struct {
		in   Command
		want sim.PlayerCommand
	}{
		{Command{Move: 0.5, Pay: true, Skill: 2}, sim.PlayerCommand{MoveX: 0.5, Pay: true, Skill: 2}},
		{Command{Move: 3, Sprint: true}, sim.PlayerCommand{MoveX: 1, Sprint: true}},
		{Command{Move: -7, Attack: true, Skill: 5}, sim.PlayerCommand{MoveX: -1, Attack: true}},
		{Command{Move: math.NaN(), Skill: -1}, sim.PlayerCommand{}},
	}
	for _, c := range cases {
		if got := c.in.Sim(); got != c.want {
			t.Errorf("%+v.Sim() = %+v, erwartet %+v", c.in, got, c.want)
		}
	}
}

func TestInputLookup(t *testing.T) {
	kb := DeviceID{DeviceKeyboard, 0}
	pad := DeviceID{DeviceGamepad, 0}
	in := Input{Devices: []DeviceInput{
		{Device: kb, Command: Command{Move: 1}},
		{Device: pad, Actions: []Action{ActionDown, ActionConfirm}},
	}}
	if di, ok := in.Of(kb); !ok || di.Command.Move != 1 {
		t.Error("Tastatur nicht gefunden")
	}
	if di, ok := in.Of(DeviceID{DeviceGamepad, 3}); ok || di.Device.Index != 3 {
		t.Error("nicht verbundenes Pad gefunden")
	}
	if d, ok := in.Has(ActionConfirm); !ok || d != pad {
		t.Errorf("Confirm von %v", d)
	}
	if _, ok := in.Has(ActionBack); ok {
		t.Error("Back ohne Druck")
	}
	if kb.String() != "keys1" || pad.String() != "pad1" {
		t.Error("Gerätenamen")
	}
}
