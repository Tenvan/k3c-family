package net

import (
	"encoding/json"
	"os"
	"testing"

	"k3c/engine/room"
)

// W12.2 (B-470, Sprint W12 AC-03): Die Phase dawn ist ein neuer Wert von cycle.phase ohne neue Protokollversion. Das
// delta beim Beginn des Morgengrauens hat die Form von s2c-snapshot-delta-dawn.json: cycle mit phase dawn und dem
// neuen Tag, dazu das Ereignis dawn mit day und stage.
func TestDeltaMorgengrauenWieBeispiel(t *testing.T) {
	raw, err := os.ReadFile("../../testdata/protocol/s2c-snapshot-delta-dawn.json")
	if err != nil {
		t.Fatal(err)
	}
	var example map[string]any
	if err := json.Unmarshal(raw, &example); err != nil {
		t.Fatal(err)
	}
	exS := example["s"].(map[string]any)
	srv, m := devServer(t)
	x := hello(t, srv, "xbox")
	create(x, "dawn", 0)
	r := m.Room(x.expect("joined")["room"].(string))
	x.expect("level")
	x.expect("snap")
	if err := r.DevPage(room.DevAction{Action: "phase", Phase: "dawn"}); err != nil {
		t.Fatal(err)
	}
	var s map[string]any
	waitFor(t, func() bool {
		r.Tick()
		s = x.expect("delta", "seats")["s"].(map[string]any)
		c, _ := s["cycle"].(map[string]any)
		return c != nil && c["phase"] == "dawn"
	})
	if d := sameKeys("delta.s.cycle", exS["cycle"], s["cycle"], true); d != "" {
		t.Error(d)
	}
	events, _ := s["events"].([]any)
	for _, e := range events {
		if ev := e.(map[string]any); ev["type"] == "dawn" {
			if d := sameKeys("dawn", exS["events"].([]any)[0], ev, true); d != "" {
				t.Error(d)
			}
			if ev["day"] != s["cycle"].(map[string]any)["day"] {
				t.Errorf("dawn Tag %v, Zyklus %v", ev["day"], s["cycle"])
			}
			return
		}
	}
	t.Fatalf("kein Ereignis dawn: %v", events)
}
