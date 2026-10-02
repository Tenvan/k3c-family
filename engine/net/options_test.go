package net

import (
	"testing"

	"k3c/engine/room"
)

// `create` liest grade, goal und defeat und reicht sie an den Raum durch; die Raumliste nennt den Grad.
func TestCreateMitOptionen(t *testing.T) {
	srv, m := wsServer(t)
	c := hello(t, srv, "xbox")
	c.send(map[string]any{"t": "create", "save": "optionen", "fresh": true, "slots": []int{0},
		"grade": "hard", "goal": "gold", "defeat": "lost"})
	c.expect("joined")
	if got, want := m.Room("KRNZ").Opts, (room.Options{Grade: "hard", Goal: "gold", Defeat: "lost"}); got != want {
		t.Fatalf("Optionen: %+v", got)
	}
	if g := m.Rooms()[0].Grade; g != "hard" {
		t.Fatalf("Grad in der Raumliste: %q", g)
	}
}

// Ungültige Optionen und dev ohne Dev-Mode ergeben bad_request.
func TestCreateUngueltigeOptionen(t *testing.T) {
	srv, _ := wsServer(t)
	for _, o := range []map[string]any{{"grade": "dev"}, {"grade": "x"}, {"goal": "x"}, {"defeat": "x"}} {
		c := hello(t, srv, "xbox")
		msg := map[string]any{"t": "create", "save": "kaputt", "fresh": true, "slots": []int{0}}
		for k, v := range o {
			msg[k] = v
		}
		c.send(msg)
		c.expectError("bad_request")
	}
}
