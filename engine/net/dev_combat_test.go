package net

import "testing"

// B-603/AC-01, AC-02: Die Kampf-Beispiele (c2s-dev-boss.json, c2s-dev-event.json, c2s-dev-night.json) werden im
// Dev-Mode angenommen; bossPhase ohne Endboss, ein zweiter Boss und eine unbekannte Id sind bad_request.
func TestDevKampfUeberProtokoll(t *testing.T) {
	srv, m := devServer(t)
	x := hello(t, srv, "xbox")
	create(x, "dev", 0)
	r := m.Room(x.entered()["room"].(string))
	ex := devExamples(t)
	for _, i := range []int{5, 7, 8} {
		x.send(ex[i])
	}
	x.send(map[string]any{"t": "input", "seq": 4, "p": []map[string]any{{"slot": 0}}})
	waitFor(t, func() bool { r.Tick(); return x.expect("delta", "seats")["ack"] == float64(4) })
	for _, bad := range []any{ex[6], ex[5], map[string]any{"t": "dev", "action": "boss", "slot": 0, "boss": "drache"},
		map[string]any{"t": "dev", "action": "event", "event": "neumond"}} {
		x.send(bad)
		x.expectError("bad_request")
	}
}
