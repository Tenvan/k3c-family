package net

import (
	"encoding/json"
	"net/http/httptest"
	"os"
	"testing"

	"k3c/engine/room"
)

// devExamples sind die Beispiele der Nachricht dev (testdata/protocol/c2s-dev-*.json), roh gesendet.
func devExamples(t *testing.T) []string {
	t.Helper()
	var out []string
	for _, f := range []string{"gold", "material", "timescale", "pause"} {
		raw, err := os.ReadFile("../../testdata/protocol/c2s-dev-" + f + ".json")
		if err != nil || !json.Valid(raw) {
			t.Fatalf("%s: %v", f, err)
		}
		out = append(out, string(raw))
	}
	return out
}

// devServer ist wsServer mit Dev-Mode (gesetzt, bevor der Server Verbindungen annimmt).
func devServer(t *testing.T) (*httptest.Server, *room.Manager) {
	t.Helper()
	m := room.NewManager(memSaves{})
	m.Dev = true
	srv := httptest.NewServer(NewHandler(Config{Rooms: m}))
	t.Cleanup(srv.Close)
	return srv, m
}
// B-178/AC-01, AC-05: ohne Dev-Mode ergibt jedes Beispiel forbidden, die Verbindung bleibt; die Fehlernachricht hat
// die Form von s2c-error-forbidden.json.
func TestDevOhneDevModeForbidden(t *testing.T) {
	srv, _ := wsServer(t)
	x := hello(t, srv, "xbox")
	create(x, "dev", 0)
	x.entered()
	for _, ex := range devExamples(t) {
		x.send(ex)
		m := x.expect("error", "seats", "delta")
		if m["code"] != "forbidden" || m["message"] != messages["forbidden"] {
			t.Fatalf("%s: %v", ex, m)
		}
		if d := sameKeys("error", want(t, "error-forbidden"), m, true); d != "" {
			t.Fatal(d)
		}
	}
	x.send(map[string]any{"t": "leave"}) // Verbindung und Raum stehen noch
	x.expect("rooms", "seats", "delta")
}

// B-178/AC-02, AC-03, AC-05: Mit Dev-Mode liest der Server jedes Beispiel; gold und material wirken ohne Fehler (die
// Verbindung lebt, ein folgendes input wird bestätigt). dev ohne Raum ist bad_request.
func TestDevMitDevModeLiestBeispiele(t *testing.T) {
	srv, m := devServer(t)
	x := hello(t, srv, "xbox")
	ex := devExamples(t)
	x.send(ex[0])
	x.expectError("bad_request")
	create(x, "dev", 0)
	r := m.Room(x.entered()["room"].(string))
	x.send(ex[0])
	x.send(ex[1])
	x.send(map[string]any{"t": "input", "seq": 9, "p": []map[string]any{{"slot": 0}}})
	waitFor(t, func() bool { r.Tick(); return x.expect("delta", "seats")["ack"] == float64(9) })
}

// B-178/AC-04, AC-05: Im Dev-Mode hat der snap devTimescale 1; nach c2s-dev-timescale.json kommt ein delta mit
// devTimescale 4 in der Form von s2c-snapshot-delta-timescale.json. Ohne Dev-Mode fehlt das Feld in snap und delta.
func TestDevTimescaleImZustand(t *testing.T) {
	srv, m := devServer(t)
	x := hello(t, srv, "xbox")
	create(x, "dev", 0)
	j := x.expect("joined")
	x.expect("level")
	if s := x.expect("snap")["s"].(map[string]any); s["devTimescale"] != float64(1) {
		t.Fatalf("snap: devTimescale %v", s["devTimescale"])
	}
	x.send(devExamples(t)[2])
	r := m.Room(j["room"].(string))
	var got map[string]any
	waitFor(t, func() bool {
		r.Tick()
		got = x.expect("delta", "seats")
		return got["s"].(map[string]any)["devTimescale"] == float64(4)
	})
	if d := sameKeys("delta", want(t, "snapshot-delta-timescale"), got, false); d != "" { // s hat daneben andere Änderungen
		t.Fatal(d)
	}

	srv, m = wsServer(t)
	y := hello(t, srv, "xbox")
	create(y, "live", 0)
	j = y.expect("joined")
	y.expect("level")
	if _, ok := y.expect("snap")["s"].(map[string]any)["devTimescale"]; ok {
		t.Fatal("snap ohne Dev-Mode hat devTimescale")
	}
	r = m.Room(j["room"].(string))
	for range 3 {
		r.Tick()
		if _, ok := y.expect("delta", "seats")["s"].(map[string]any)["devTimescale"]; ok {
			t.Fatal("delta ohne Dev-Mode hat devTimescale")
		}
	}
}
