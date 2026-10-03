package net

import (
	"encoding/json"
	"net/http/httptest"
	"os"
	"testing"

	"k3c/engine/room"
)

// devExamples sind die drei Beispiele der Nachricht dev (testdata/protocol/c2s-dev-*.json), roh gesendet.
func devExamples(t *testing.T) []string {
	t.Helper()
	var out []string
	for _, f := range []string{"gold", "material", "timescale"} {
		raw, err := os.ReadFile("../../testdata/protocol/c2s-dev-" + f + ".json")
		if err != nil || !json.Valid(raw) {
			t.Fatalf("%s: %v", f, err)
		}
		out = append(out, string(raw))
	}
	return out
}

// devServer ist wsServer mit Dev-Mode (gesetzt, bevor der Server Verbindungen annimmt).
func devServer(t *testing.T) *httptest.Server {
	t.Helper()
	m := room.NewManager(memSaves{})
	m.Dev = true
	srv := httptest.NewServer(NewHandler(Config{Rooms: m}))
	t.Cleanup(srv.Close)
	return srv
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

// B-178/AC-05: Mit Dev-Mode liest der Server jedes Beispiel; die Aktionen fehlen noch (DBG1.2, DBG1.3) → bad_request.
// dev ohne Raum ist bad_request.
func TestDevMitDevModeLiestBeispiele(t *testing.T) {
	srv := devServer(t)
	x := hello(t, srv, "xbox")
	x.send(devExamples(t)[0])
	x.expectError("bad_request")
	create(x, "dev", 0)
	x.entered()
	for _, ex := range devExamples(t) {
		x.send(ex)
		if m := x.expect("error", "seats", "delta"); m["code"] != "bad_request" {
			t.Fatalf("%s: %v", ex, m)
		}
	}
}
