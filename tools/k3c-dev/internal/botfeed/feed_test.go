package botfeed

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/coder/websocket"

	"k3c/engine/sim"
)

func TestFeedNurLoopback(t *testing.T) {
	h := NewHub()
	h.Open("run-1/0")
	for _, tc := range []struct {
		remote, path string
		want         int
	}{
		{"192.168.1.5:4000", "/bot/run-1/0", http.StatusForbidden},
		{"[fe80::1]:4000", "/bot/run-1/0", http.StatusForbidden},
		{"127.0.0.1:4000", "/bot/run-9/0", http.StatusNotFound},
	} {
		req := httptest.NewRequest(http.MethodGet, tc.path, nil)
		req.RemoteAddr = tc.remote
		rec := httptest.NewRecorder()
		h.ServeHTTP(rec, req)
		if rec.Code != tc.want {
			t.Errorf("%s %s: %d, erwartet %d", tc.remote, tc.path, rec.Code, tc.want)
		}
	}
	if !Loopback("[::1]:80") || Loopback("10.0.0.1:80") || Loopback("kaputt") {
		t.Error("Loopback falsch")
	}
}

func TestFeedSchicktJeSlot(t *testing.T) {
	h := NewHub()
	f := h.Open("run-1/0")
	srv := httptest.NewServer(h)
	defer srv.Close()
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	header := http.Header{"Origin": {"http://127.0.0.1:5183"}} // Seite auf dem Vite-Port
	c, _, err := websocket.Dial(ctx, "ws"+strings.TrimPrefix(srv.URL, "http")+"/bot/run-1/0", &websocket.DialOptions{HTTPHeader: header})
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = c.CloseNow() }()
	for now, _ := f.Connected(); !now; now, _ = f.Connected() {
		time.Sleep(10 * time.Millisecond)
	}
	f.Send(ctx, []Cmd{CmdOf(0, sim.PlayerCommand{MoveX: -1}), CmdOf(1, sim.PlayerCommand{Pay: true, Skill: 2})})
	var got []Cmd
	for range 2 {
		_, data, err := c.Read(ctx)
		if err != nil {
			t.Fatal(err)
		}
		var m Cmd
		if err := json.Unmarshal(data, &m); err != nil {
			t.Fatal(err)
		}
		got = append(got, m)
	}
	if got[0] != (Cmd{Slot: 0, MoveX: -1}) || got[1] != (Cmd{Slot: 1, Pay: true, Skill: 2}) || f.Sent() != 2 {
		t.Errorf("Nachrichten %+v, gesendet %d", got, f.Sent())
	}
	h.Close("run-1/0")
	if _, _, err := c.Read(ctx); err == nil {
		t.Error("Verbindung nach Close offen")
	}
}
