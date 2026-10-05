package net

import (
	"encoding/json"
	"fmt"
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// zweiStufenSave ist ein Spielstand mit zwei Spielern: Spieler 0 in Stufe 0, Spieler 1 in Stufe 1.
func zweiStufenSave(t *testing.T) []byte {
	t.Helper()
	isl, err := sim.CreateIsland("stufen", []int{0, 1, 2}, 1)
	if err != nil {
		t.Fatal(err)
	}
	sim.AddIslandPlayer(isl, 0)
	sim.AddIslandPlayer(isl, 1)
	b, err := json.Marshal(isl.ToSave("2026-10-05T12:00:00.000Z"))
	if err != nil {
		t.Fatal(err)
	}
	return b
}

// (d) B-176/AC-02: Über WebSocket bekommt ein Gerät mit zwei Spielern in Stufe 0 und 1 level und snap beider Stufen
// (aufsteigend), danach je Tick ein delta je Stufe. Die Ströme haben getrennte prev: Ein delta enthält nur Änderungen
// seiner Stufe (depth unterscheidet sich zwischen den Stufen und fehlt deshalb in beiden).
func TestStufenZweiStroemeUeberWebSocket(t *testing.T) {
	srv, m := wsServer(t)
	m.Store.(memSaves)["stufen"] = zweiStufenSave(t)
	x := hello(t, srv, "xbox")
	x.send(map[string]any{"t": "create", "save": "stufen", "fresh": false, "slots": []int{0, 1}})
	if j := x.expect("joined"); fmt.Sprint(j["you"]) != "[map[depth:0 monarch:0 slot:0 stage:0] map[depth:1 monarch:1 slot:1 stage:1]]" {
		t.Fatalf("joined: %v", j["you"])
	}
	for _, want := range []string{"level 0", "snap 0", "level 1", "snap 1"} {
		if m := x.next(); fmt.Sprintf("%s %v", m["t"], m["stage"]) != want {
			t.Fatalf("erwartet %s, bekam %v %v", want, m["t"], m["stage"])
		}
	}
	x.expect("seats")
	x.send(map[string]any{"t": "input", "seq": 1, "p": []map[string]any{{"slot": 0, "moveX": 1}}})
	r := m.Room("KRNZ")
	for tick := 0; tick < 5; tick++ {
		r.Tick()
		for stage := range 2 {
			d := x.expect("delta", "seats")
			s := d["s"].(map[string]any)
			if d["stage"] != float64(stage) || s["depth"] != nil {
				t.Fatalf("Tick %d: erwartet delta der Stufe %d ohne depth, bekam %v", tick, stage, d)
			}
		}
	}
}

// Eine blockierte Schreib-Goroutine mit zwei Stufen: Ein neuer Zustand ersetzt nur den wartenden derselben Stufe und
// übernimmt nur dessen Ereignisse; je Stufe bleibt die Reihenfolge level → snap → delta, die Warteschlange wächst nicht.
func TestStufenBlockierteVerbindungZweiStroeme(t *testing.T) {
	cancelled := false
	c := queueConn("x", &cancelled)
	state := func(tick, stage int) map[string]any {
		return map[string]any{"x": tick, "events": []any{fmt.Sprintf("s%d", stage)}}
	}
	c.Level(0, 0, level.Layout{})
	c.State(1, 0, state(1, 0))
	c.Level(1, 1, level.Layout{})
	for tick := 1; tick <= 10; tick++ {
		if tick > 1 {
			c.State(tick, 0, state(tick, 0))
		}
		c.State(tick, 1, state(tick, 1))
	}
	if n := len(c.queue); cancelled || n > 5 {
		t.Fatalf("getrennt %v, %d Nachrichten in der Warteschlange", cancelled, n)
	}
	got, events := map[float64][]string{}, map[float64]int{}
	for _, m := range drain(t, c) {
		st := m["stage"].(float64)
		got[st] = append(got[st], fmt.Sprintf("%s %v", m["t"], m["tick"]))
		s, _ := m["s"].(map[string]any)
		evs, _ := s["events"].([]any)
		events[st] += len(evs)
		for _, ev := range evs {
			if ev != fmt.Sprintf("s%v", st) {
				t.Fatalf("Ereignis %v im Strom der Stufe %v", ev, st)
			}
		}
	}
	if fmt.Sprint(got[0]) != "[level <nil> snap 1 delta 10]" || fmt.Sprint(got[1]) != "[level <nil> snap 10]" {
		t.Fatalf("Stufe 0 %v, Stufe 1 %v", got[0], got[1])
	}
	if events[0] != 10 || events[1] != 10 {
		t.Fatalf("Ereignisse je Stufe: %v, erwartet je 10", events)
	}
}
