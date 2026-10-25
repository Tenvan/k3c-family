package net

import (
	"testing"
)

// B-190/AC-01: Nach einem Zustand mit verworfenen Ereignissen und einem ohne hat der aus snap und delta
// zusammengesetzte Zustand kein eventsDropped mehr (= 0).
func TestEventsDroppedFaelltAufNull(t *testing.T) {
	cancelled := false
	c := queueConn("x", &cancelled)
	c.State(1, 0, map[string]any{"x": float64(1), "eventsDropped": float64(5)})
	msgs := drain(t, c)
	client := msgs[0]["s"].(map[string]any)
	if msgs[0]["t"] != "snap" || client["eventsDropped"] != float64(5) {
		t.Fatalf("snap: %v", msgs[0])
	}
	c.State(2, 0, map[string]any{"x": float64(2)})
	msgs = drain(t, c)
	d := msgs[0]["s"].(map[string]any)
	if _, unset := d["unset"]; msgs[0]["t"] != "delta" || unset {
		t.Fatalf("delta: %v", msgs[0])
	}
	if client = apply(client, d); client["eventsDropped"] != nil || client["x"] != float64(2) {
		t.Fatalf("Client nach dem Delta: %v", client)
	}
}

// Ersetzt ein Zustand wartende, zählt eventsDropped deren verworfene Ereignisse und die über maxEvents mit.
func TestEventsDroppedZaehltErsetzteZustaende(t *testing.T) {
	cancelled := false
	c := queueConn("x", &cancelled)
	full := make([]any, maxEvents)
	for i := range full {
		full[i] = i
	}
	c.State(1, 0, map[string]any{"events": full, "eventsDropped": float64(3)})
	c.State(2, 0, map[string]any{"events": []any{"neu"}, "eventsDropped": float64(2)})
	msgs := drain(t, c)
	s := msgs[0]["s"].(map[string]any)
	if len(msgs) != 1 || s["eventsDropped"] != float64(3+1+2) || len(s["events"].([]any)) != maxEvents {
		t.Fatalf("erwartet ein snap mit eventsDropped 6 und %d Ereignissen: %v", maxEvents, s["eventsDropped"])
	}
}
