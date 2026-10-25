package net

import (
	"fmt"
	"testing"
)

// TR4.2, B-352/AC-01: Ein Beobachter tritt mit dem Beispiel c2s-join-observe.json ohne Platz bei, bekommt Level und
// Zustände wie ein Spieler, die Zahl der Monarchen bleibt gleich, und seine Eingaben werden abgelehnt.
func TestBeobachterOhnePlatz(t *testing.T) {
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox")
	create(x, "familie", 0)
	x.entered()
	o := hello(t, srv, "beobachter")
	o.send(pauseExample(t, "join-observe"))
	if j := o.entered(); j["room"] != "KRNZ" || fmt.Sprint(j["you"]) != "[]" {
		t.Fatalf("joined: %v", j)
	}
	if s := o.expect("seats"); fmt.Sprint(s["you"]) != "[]" || fmt.Sprint(s["monarchs"]) != "[taken]" {
		t.Fatalf("seats: %v", s)
	}
	if info := m.Rooms()[0]; info.Taken != 1 || info.Free != 3 {
		t.Fatalf("Plätze nach Beobachter: %+v", info)
	}
	for _, msg := range []map[string]any{
		{"t": "input", "seq": 1, "p": []map[string]any{{"slot": 0, "moveX": 1}}},
		{"t": "addSlot", "slot": 0},
		{"t": "pause"},
	} {
		o.send(msg)
		o.expectError("bad_request")
	}
	r := m.Room("KRNZ")
	r.Tick()
	if d := o.expect("delta", "seats"); d["stage"] != float64(0) {
		t.Fatalf("delta: %v", d)
	}
	if info := m.Rooms()[0]; info.Taken != 1 {
		t.Fatalf("Plätze nach Eingaben des Beobachters: %+v", info)
	}
}

// TR4.2, B-352/AC-02: sim_test legt den Raum als Beobachter an (c2s-create-observe.json); der Client tritt danach mit
// 4 Slots bei. Der Testraum bleibt, solange der Beobachter da ist, und wird erst nach seinem leave aufgeräumt.
func TestBeobachterLegtRaumAnVierSpieler(t *testing.T) {
	srv, m := wsServer(t)
	o := hello(t, srv, "simtest")
	o.send(pauseExample(t, "create-observe"))
	if j := o.entered(); j["room"] != "KRNZ" || fmt.Sprint(j["you"]) != "[]" {
		t.Fatalf("joined: %v", j)
	}
	c := hello(t, srv, "client")
	joinRoom(c, "KRNZ", 0, 1, 2, 3)
	c.entered()
	if info := m.Rooms()[0]; info.Taken != 4 || !info.Running {
		t.Fatalf("Plätze: %+v", info)
	}
	m.Room("KRNZ").Tick()
	d := o.expect("delta", "seats")
	ps, _ := d["s"].(map[string]any)["players"].(map[string]any)
	if set, _ := ps["set"].([]any); len(set) != 4 { // der Beobachter sieht alle 4 Monarchen des Clients
		t.Fatalf("delta: Spieler %v", ps)
	}
	c.send(map[string]any{"t": "leave"})
	c.expect("rooms", "seats", "delta")
	m.Sweep()
	if len(m.Rooms()) != 1 {
		t.Fatal("Testraum mit Beobachter aufgeräumt")
	}
	o.send(map[string]any{"t": "leave"})
	o.expect("rooms", "seats", "delta")
	waitFor(t, func() bool { m.Sweep(); return len(m.Rooms()) == 0 })
}
