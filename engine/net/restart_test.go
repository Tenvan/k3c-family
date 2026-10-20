package net

import (
	"net/http/httptest"
	"testing"

	"k3c/engine/room"
)

// Der Hinweis „Server startet neu“ geht an jedes verbundene Gerät, in der Raumliste wie im Raum; Raum und Verbindung
// bleiben, erst das Herunterfahren schickt room_closed (B-507/AC-04).
func TestHinweisNeustartAnAlleGeraete(t *testing.T) {
	m := room.NewManager(memSaves{})
	m.NewCode = func() string { return "KRNZ" }
	notices := &Notices{}
	srv := httptest.NewServer(NewHandler(Config{Rooms: m, Notices: notices}))
	t.Cleanup(srv.Close)
	inRoom := hello(t, srv, "xbox")
	create(inRoom, "familie", 0)
	inRoom.entered()
	lobby := hello(t, srv, "handy")
	notices.Restarting()
	for _, c := range []*client{inRoom, lobby} {
		m := c.expect("error", "seats", "rooms", "delta")
		if m["code"] != "restarting" || m["message"] != "Server startet neu" {
			t.Fatalf("Hinweis: %v", m)
		}
	}
	if len(m.Rooms()) != 1 {
		t.Fatal("Hinweis schließt den Raum")
	}
	m.Close()
	inRoom.expectError("room_closed")
}

// Ohne Handler (nil oder nie an NewHandler gegeben) ist Restarting ohne Wirkung.
func TestHinweisOhneHandler(t *testing.T) {
	var n *Notices
	n.Restarting()
	(&Notices{}).Restarting()
}
