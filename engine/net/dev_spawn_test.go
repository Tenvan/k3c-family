package net

import (
	"encoding/json"
	"os"
	"testing"
)

// B-766: Die Dev-Aktion spawn (c2s-dev-spawn.json) wird im Dev-Mode angenommen und setzt Gegner und Truppen in die Stufe
// des Spielers; unbekannte Sorte oder Art, zu viele und ein fehlender slot sind bad_request.
func TestDevSpawnUeberProtokoll(t *testing.T) {
	srv, m := devServer(t)
	x := hello(t, srv, "xbox")
	create(x, "dev-spawn", 0)
	r := m.Room(x.entered()["room"].(string))
	raw, err := os.ReadFile("../../testdata/protocol/c2s-dev-spawn.json")
	if err != nil || !json.Valid(raw) {
		t.Fatalf("Beispiel: %v", err)
	}
	x.send(string(raw))
	x.send(map[string]any{"t": "dev", "action": "spawn", "slot": 0, "kind": "troop", "unit": "archer", "amount": 5})
	x.send(map[string]any{"t": "input", "seq": 4, "p": []map[string]any{{"slot": 0}}})
	waitFor(t, func() bool { r.Tick(); return x.expect("delta", "seats")["ack"] == float64(4) })
	for _, bad := range []any{
		map[string]any{"t": "dev", "action": "spawn", "slot": 0, "kind": "enemy", "unit": "drache", "amount": 1},
		map[string]any{"t": "dev", "action": "spawn", "slot": 0, "kind": "boss", "unit": "wolf", "amount": 1},
		map[string]any{"t": "dev", "action": "spawn", "slot": 0, "kind": "enemy", "unit": "wolf", "amount": 0},
		map[string]any{"t": "dev", "action": "spawn", "slot": 0, "kind": "enemy", "unit": "wolf", "amount": 1000},
		map[string]any{"t": "dev", "action": "spawn", "kind": "enemy", "unit": "wolf", "amount": 1},
	} {
		x.send(bad)
		x.expectError("bad_request")
	}
}

// Ohne Dev-Mode ist spawn verboten.
func TestDevSpawnOhneDevModeForbidden(t *testing.T) {
	srv, _ := wsServer(t)
	x := hello(t, srv, "xbox")
	create(x, "spawn-live", 0)
	x.entered()
	x.send(map[string]any{"t": "dev", "action": "spawn", "slot": 0, "kind": "enemy", "unit": "wolf", "amount": 1})
	x.expectError("forbidden")
}
