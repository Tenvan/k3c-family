package net

import (
	"encoding/json"
	"os"
	"testing"
)

// pauseExample ist testdata/protocol/c2s-<name>.json, roh gesendet.
func pauseExample(t *testing.T, name string) string {
	t.Helper()
	raw, err := os.ReadFile("../../testdata/protocol/c2s-" + name + ".json")
	if err != nil || !json.Valid(raw) {
		t.Fatalf("%s: %v", name, err)
	}
	return string(raw)
}

// BD3.1, B-214/AC-03: pause und resume ohne Raum sind bad_request; im Raum nimmt der Server beide Beispiele an, auch
// doppelt (zweites pause, resume ohne Pause), ohne Fehler: ein folgendes input wird bestätigt.
func TestPauseResumeBeispiele(t *testing.T) {
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox")
	pause, resume := pauseExample(t, "pause"), pauseExample(t, "resume")
	x.send(pause)
	x.expectError("bad_request")
	x.send(resume)
	x.expectError("bad_request")
	create(x, "pause", 0)
	r := m.Room(x.entered()["room"].(string))
	for _, ex := range []string{pause, pause, resume, resume} {
		x.send(ex)
	}
	x.send(map[string]any{"t": "input", "seq": 4, "p": []map[string]any{{"slot": 0}}})
	waitFor(t, func() bool { r.Tick(); return x.expect("delta", "seats")["ack"] == float64(4) }) // ein error bricht ab
}
