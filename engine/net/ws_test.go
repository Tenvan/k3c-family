package net

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http/httptest"
	"os"
	"slices"
	"strings"
	"testing"
	"time"

	"github.com/coder/websocket"

	"k3c/engine/room"
	"k3c/engine/store"
)

type memSaves map[string][]byte

func (s memSaves) Load(name string) ([]byte, error) {
	if d, ok := s[name]; ok {
		return d, nil
	}
	return nil, store.ErrNotFound
}

func (s memSaves) Store(name string, data []byte) (string, error) { s[name] = data; return "", nil }

// wsServer startet einen Server mit /ws; Codes der Räume der Reihe nach.
func wsServer(t *testing.T) (*httptest.Server, *room.Manager) {
	t.Helper()
	m := room.NewManager(memSaves{})
	codes := []string{"KRNZ", "BWTQ", "MXPL", "HJKA", "ZZZZ"}
	m.NewCode = func() string { c := codes[0]; codes = codes[1:]; return c }
	srv := httptest.NewServer(NewHandler(Config{Rooms: m}))
	t.Cleanup(srv.Close)
	return srv, m
}

type client struct {
	t  *testing.T
	ws *websocket.Conn
}

func dial(t *testing.T, srv *httptest.Server) *client {
	t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	ws, _, err := websocket.Dial(ctx, "ws"+strings.TrimPrefix(srv.URL, "http")+"/ws", nil)
	if err != nil {
		t.Fatal(err)
	}
	ws.SetReadLimit(1 << 20)
	t.Cleanup(func() { _ = ws.CloseNow() })
	return &client{t, ws}
}

func (c *client) send(v any) {
	c.t.Helper()
	b, _ := json.Marshal(v)
	if s, ok := v.(string); ok {
		b = []byte(s)
	}
	if err := c.ws.Write(context.Background(), websocket.MessageText, b); err != nil {
		c.t.Fatal(err)
	}
}

// next liest die nächste Nachricht (Zeitlimit 2 s, der Test hängt nie).
func (c *client) next() map[string]any {
	c.t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	_, data, err := c.ws.Read(ctx)
	if err != nil {
		c.t.Fatalf("keine Nachricht: %v", err)
	}
	var m map[string]any
	if err := json.Unmarshal(data, &m); err != nil {
		c.t.Fatal(err)
	}
	return m
}

// expect liest bis zur Nachricht vom Typ t und liefert sie; dazwischen nur erlaubte Typen (z. B. seats).
func (c *client) expect(typ string, skip ...string) map[string]any {
	c.t.Helper()
	for {
		m := c.next()
		if m["t"] == typ {
			return m
		}
		if !slices.Contains(skip, m["t"].(string)) {
			c.t.Fatalf("erwartet %s, bekam %v", typ, m)
		}
	}
}

func (c *client) expectError(code string) {
	c.t.Helper()
	if m := c.expect("error", "seats", "rooms"); m["code"] != code || m["message"] == "" {
		c.t.Fatalf("erwartet Fehler %s, bekam %v", code, m)
	}
}

// hello schickt den Handschlag und liest welcome und rooms.
func hello(t *testing.T, srv *httptest.Server, device string) *client {
	t.Helper()
	c := dial(t, srv)
	c.send(map[string]any{"t": "hello", "v": 2, "device": device})
	if m := c.expect("welcome"); m["v"] != float64(2) || m["tickHz"] != float64(30) {
		t.Fatalf("welcome: %v", m)
	}
	c.expect("rooms")
	return c
}

func create(c *client, name string, slots ...int) {
	c.send(map[string]any{"t": "create", "save": name, "fresh": true, "depth": 0, "slots": slots})
}

func joinRoom(c *client, code string, slots ...int) {
	c.send(map[string]any{"t": "join", "room": code, "slots": slots})
}

// entered liest joined, level und snap (AC-06) und liefert joined.
func (c *client) entered() map[string]any {
	c.t.Helper()
	j := c.expect("joined")
	if l := c.expect("level"); l["depth"] != float64(0) || l["layout"] == nil {
		c.t.Fatalf("level: %v", l)
	}
	if s := c.expect("snap"); s["s"].(map[string]any)["depth"] != float64(0) {
		c.t.Fatalf("snap: %v", s)
	}
	return j
}

// Ablauf aus docs/protocol.md › Beispiel: 2 Controller an der Xbox + 1 Handy, über echte WebSockets.
func TestWebSocketXboxUndHandy(t *testing.T) {
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox")
	create(x, "familie", 0)
	if j := x.entered(); j["room"] != "KRNZ" || j["name"] != "familie" || fmt.Sprint(j["you"]) != "[map[monarch:0 slot:0]]" {
		t.Fatalf("joined: %v", j)
	}
	x.expect("seats")
	x.send(map[string]any{"t": "addSlot", "slot": 1})
	if s := x.expect("seats"); fmt.Sprint(s["monarchs"]) != "[taken taken]" {
		t.Fatalf("seats nach addSlot: %v", s)
	}
	h := hello(t, srv, "handy")
	joinRoom(h, "KRNZ", 0)
	if j := h.entered(); fmt.Sprint(j["you"]) != "[map[monarch:2 slot:0]]" {
		t.Fatalf("Handy: %v", j)
	}
	x.expect("seats")
	x.send(map[string]any{"t": "input", "seq": 1, "p": []map[string]any{{"slot": 0, "moveX": 1}, {"slot": 1, "moveX": -1}}})
	// Die Eingabe verrechnet die Lese-Schleife des Servers; sobald sie da ist, bestätigt ack sie.
	for i := 0; ; i++ {
		m.Room("KRNZ").Tick()
		if x.expect("snap")["ack"] == float64(1) {
			break
		}
		if i > 100 {
			t.Fatal("ack bleibt 0")
		}
		time.Sleep(5 * time.Millisecond)
	}
	h.expect("snap", "seats")
}

func waitFor(t *testing.T, ok func() bool) {
	t.Helper()
	for range 200 {
		if ok() {
			return
		}
		time.Sleep(10 * time.Millisecond)
	}
	t.Fatal("Bedingung nicht erreicht")
}

func TestHandschlagFalsch(t *testing.T) {
	srv, _ := wsServer(t)
	for _, first := range []string{`kein json`, `{"t":"join","room":"KRNZ","slots":[0]}`, `{"t":"hello","v":1,"device":"x"}`,
		`{"t":"hello","v":2,"device":""}`, `{"t":"hello","v":2,"device":"` + strings.Repeat("x", 65) + `"}`} {
		c := dial(t, srv)
		c.send(first)
		if m := c.next(); m["code"] != "version" {
			t.Fatalf("%s: %v", first, m)
		}
		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		if _, _, err := c.ws.Read(ctx); err == nil {
			t.Fatalf("%s: Verbindung nicht geschlossen", first)
		}
		cancel()
	}
}

func TestFehlerCodes(t *testing.T) {
	srv, m := wsServer(t)
	a := hello(t, srv, "a")
	a.send(`{nicht json`)
	a.expectError("bad_request")
	a.send(map[string]any{"t": "input", "seq": 1, "p": []map[string]any{{"slot": 0}}})
	a.expectError("bad_request")
	a.send(map[string]any{"t": "gibtsnicht"})
	a.expectError("bad_request")
	joinRoom(a, "QQQQ", 0)
	a.expectError("room_not_found")
	create(a, "voll", 4)
	a.expectError("too_many_slots")
	m.Store.(memSaves)["alt"] = []byte(`{}`)
	create(a, "alt", 0)
	a.expectError("save_exists")
	a.send(map[string]any{"t": "create", "save": "fehlt", "fresh": false, "slots": []int{0}})
	a.expectError("save_not_found")
	create(a, "voll", 0, 1, 2, 3)
	a.entered()
	create(a, "noch-einer", 0)
	a.expectError("bad_request") // create im Raum
	b := hello(t, srv, "b")
	joinRoom(b, "KRNZ", 0)
	b.expectError("room_full")
	for i := range 3 {
		create(hello(t, srv, fmt.Sprint("r", i)), fmt.Sprint("raum-", i), 0)
	}
	waitFor(t, func() bool { return len(m.Rooms()) == 4 })
	create(b, "raum-5", 0)
	b.expectError("too_many_rooms")
}

func TestReplacedUndRoomClosed(t *testing.T) {
	srv, m := wsServer(t)
	old := hello(t, srv, "tab")
	create(old, "tabs", 0)
	old.entered()
	neu := hello(t, srv, "tab")
	joinRoom(neu, "KRNZ", 0)
	neu.entered()
	old.expectError("replaced")
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	if _, _, err := old.ws.Read(ctx); err == nil {
		t.Fatal("ersetzte Verbindung nicht geschlossen")
	}
	m.Close()
	neu.expectError("room_closed")
	neu.expect("rooms", "seats", "snap")
}

func TestVollerSendepufferIstAbbruch(t *testing.T) {
	srv, m := wsServer(t)
	x := hello(t, srv, "langsam")
	create(x, "puffer", 0)
	x.entered() // danach liest der Client nicht mehr
	r := m.Room("KRNZ")
	waitFor(t, func() bool {
		for range 50 {
			r.Tick()
		}
		return !m.Rooms()[0].Running
	})
	if info := m.Rooms()[0]; info.Taken != 1 || info.Free != 3 {
		t.Fatalf("Monarch nicht wartend: %+v", info)
	}
}

// Jede Nachricht des Servers hat die Schlüssel ihres Beispiels in testdata/protocol/ (bei snap: oberste Ebene von s).
func TestFormWieBeispiele(t *testing.T) {
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox")
	got := map[string]map[string]any{}
	x.send(map[string]any{"t": "hello", "v": 2, "device": "xbox"})
	got["error"] = x.expect("error")
	create(x, "familie", 0)
	got["joined"], got["level"], got["snap"], got["seats"] = x.expect("joined"), x.expect("level"), x.expect("snap"), x.expect("seats")
	c := dial(t, srv)
	c.send(map[string]any{"t": "hello", "v": 2, "device": "handy"})
	got["welcome"], got["rooms"] = c.expect("welcome"), c.expect("rooms")
	_ = m
	for typ, file := range map[string]string{"welcome": "welcome", "rooms": "rooms", "joined": "joined", "level": "level",
		"snap": "snapshot-full", "seats": "seats", "error": "error"} {
		raw, err := os.ReadFile("../../testdata/protocol/s2c-" + file + ".json")
		if err != nil {
			t.Fatal(err)
		}
		var want map[string]any
		if err := json.Unmarshal(raw, &want); err != nil {
			t.Fatal(err)
		}
		if d := sameKeys(typ, want, got[typ], typ != "snap"); d != "" {
			t.Errorf("%s: %s", typ, d)
		}
	}
	if d := sameKeys("snap.s", want(t, "snapshot-full")["s"], got["snap"]["s"], false); d != "" {
		t.Errorf("%s", d)
	}
}

func want(t *testing.T, file string) map[string]any {
	t.Helper()
	raw, _ := os.ReadFile("../../testdata/protocol/s2c-" + file + ".json")
	var m map[string]any
	_ = json.Unmarshal(raw, &m)
	return m
}

// sameKeys vergleicht die Schlüssel zweier Objekte, rekursiv in Objekten und ersten Listeneinträgen.
func sameKeys(at string, want, got any, deep bool) string {
	w, ok1 := want.(map[string]any)
	g, ok2 := got.(map[string]any)
	if !ok1 || !ok2 {
		return ""
	}
	for k := range w {
		if _, ok := g[k]; !ok {
			return at + "." + k + " fehlt"
		}
	}
	for k := range g {
		if _, ok := w[k]; !ok {
			return at + "." + k + " ist zu viel"
		}
	}
	if !deep {
		return ""
	}
	for k := range w {
		wv, gv := w[k], g[k]
		if wl, ok := wv.([]any); ok && len(wl) > 0 {
			if gl, ok := gv.([]any); ok && len(gl) > 0 {
				wv, gv = wl[0], gl[0]
			}
		}
		if d := sameKeys(at+"."+k, wv, gv, true); d != "" {
			return d
		}
	}
	return ""
}
