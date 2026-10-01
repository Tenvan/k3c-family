package net

import (
	"testing"
)

// Review SP07, Befund 5: too_many_slots geht allen anderen Prüfungen von create vor, auch einem fehlenden fresh.
func TestCreateSlotsVorFresh(t *testing.T) {
	srv, _ := wsServer(t)
	c := hello(t, srv, "a")
	c.send(map[string]any{"t": "create", "save": "x", "slots": []int{4}})
	c.expectError("too_many_slots")
	c.send(map[string]any{"t": "create", "save": "x", "slots": []int{0}})
	c.expectError("bad_request")
}

// Review SP07, Befund 5: Ein ungültiger Slot verwirft die ganze input-Nachricht, auch die gültigen Einträge.
func TestInputAllesOderNichts(t *testing.T) {
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox")
	create(x, "eingabe", 0)
	x.entered()
	x.send(map[string]any{"t": "input", "seq": 3, "p": []map[string]any{{"slot": 0, "moveX": 1}, {"slot": 2, "moveX": 1}}})
	x.expectError("bad_request")
	r := m.Room("KRNZ")
	r.Tick()
	if s := m.Room("KRNZ").Summary(); s.Tick != 1 {
		t.Fatalf("Tick %d", s.Tick)
	}
	if d := x.expect("delta", "seats"); d["ack"] != float64(0) {
		t.Fatalf("verworfene Eingabe wurde bestätigt: %v", d["ack"])
	}
}
