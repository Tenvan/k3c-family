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
	if g := m.Rooms()[0].Grade; g != "normal" {
		t.Fatalf("Grad in der Raumliste: %q", g)
	}
}
