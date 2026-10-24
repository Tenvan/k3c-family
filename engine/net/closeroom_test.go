package net

import (
	"errors"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/room"
	"k3c/engine/store"
)

// BD9.1, B-604/AC-02: Jeder Eintrag von `rooms` nennt die Welt des Raums.
func TestRaumlisteNenntWelt(t *testing.T) {
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox")
	create(x, "familie", 0)
	x.entered()
	c := dial(t, srv)
	c.send(map[string]any{"t": "hello", "v": ProtocolVersion, "device": "handy"})
	c.expect("welcome")
	list := c.expect("rooms")["rooms"].([]any)
	if len(list) != 1 || list[0].(map[string]any)["world"] != "standard" {
		t.Fatalf("rooms: %v", list)
	}
	if got := m.Rooms(); len(got) != 1 || got[0].World != "standard" {
		t.Fatalf("Rooms: %+v", got)
	}
}

// BD9.1, B-581/AC-03: POST /api/room/close speichert den Raum, das verbundene Gerät bekommt room_closed und die leere
// Raumliste, der Raum ist weg und der Stand danach löschbar. Unbekannter Raum → 404, ungültiger Name → 400.
func TestRaumSchliessenUeberHTTP(t *testing.T) {
	saves := &store.Saves{Dir: filepath.Join(t.TempDir(), "saves")}
	m := room.NewManager(saves)
	srv := httptest.NewServer(NewHandler(Config{Saves: saves, Rooms: m}))
	t.Cleanup(srv.Close)
	x := hello(t, srv, "xbox")
	create(x, "offen", 0)
	x.entered()

	res, body := do(t, http.MethodPost, srv.URL+"/api/room/close?slot=offen", "")
	if res.StatusCode != http.StatusOK || !strings.Contains(body, `"ok":true`) {
		t.Fatalf("schließen: %d %s", res.StatusCode, body)
	}
	x.expectError("room_closed")
	if l := x.expect("rooms", "seats")["rooms"].([]any); len(l) != 0 {
		t.Fatalf("Raumliste nach Schließen: %v", l)
	}
	if _, err := saves.Load("offen"); err != nil {
		t.Fatalf("Stand nicht gespeichert: %v", err)
	}
	if len(m.Rooms()) != 0 {
		t.Fatalf("Raum noch offen: %+v", m.Rooms())
	}
	if res, body := do(t, http.MethodDelete, srv.URL+"/api/save?slot=offen", ""); res.StatusCode != http.StatusOK {
		t.Fatalf("löschen nach Schließen: %d %s", res.StatusCode, body)
	}
	for _, tc := range []struct {
		method, query string
		code          int
	}{
		{http.MethodPost, "?slot=offen", http.StatusNotFound},
		{http.MethodPost, "?slot=..%2Fx", http.StatusBadRequest},
		{http.MethodPost, "", http.StatusBadRequest},
		{http.MethodGet, "?slot=offen", http.StatusMethodNotAllowed},
	} {
		if res, body := do(t, tc.method, srv.URL+"/api/room/close"+tc.query, ""); res.StatusCode != tc.code {
			t.Fatalf("%s %s: %d %s", tc.method, tc.query, res.StatusCode, body)
		}
	}
	if err := m.CloseRoom("nix"); !errors.Is(err, room.ErrRoomNotFound) {
		t.Fatalf("unbekannter Raum: %v", err)
	}
}
