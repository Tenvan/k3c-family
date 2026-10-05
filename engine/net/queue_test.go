package net

import (
	"encoding/json"
	"io"
	"log/slog"
	"reflect"
	"testing"

	"k3c/engine/room"
)

// queueConn ist eine Verbindung ohne WebSocket und ohne Schreib-Goroutine: Sie steht, bis der Test drain aufruft.
func queueConn(device string, cancelled *bool) *conn {
	s := &server{log: slog.New(slog.NewTextHandler(io.Discard, nil))}
	return &conn{s: s, device: device, wake: make(chan struct{}, 1), cancel: func() { *cancelled = true }}
}

// drain kodiert wie die Schreib-Goroutine (encode), nur ohne WebSocket, und liefert die Nachrichten.
func drain(t *testing.T, c *conn) []map[string]any {
	t.Helper()
	var msgs []map[string]any
	for o, ok := c.pop(); ok; o, ok = c.pop() {
		var m map[string]any
		if err := json.Unmarshal(c.encode(o), &m); err != nil {
			t.Fatal(err)
		}
		msgs = append(msgs, m)
	}
	return msgs
}

// N1/AC-02 (B-276/AC-02): Eine Verbindung, deren Schreib-Goroutine steht, wird nicht getrennt; danach bekommt sie ein
// Delta zum zuletzt gesendeten Zustand, und der Client-Zustand nach dem Anwenden ist der des Servers.
func TestBlockierteVerbindungVerwirftAlteZustaende(t *testing.T) {
	m := room.NewManager(memSaves{})
	NewHandler(Config{Rooms: m}) // setzt m.Snapshot
	cancelled := false
	c := queueConn("langsam", &cancelled)
	r, err := m.Create(c.device, c, "langsam", true, 0, []int{0}, room.Options{})
	if err != nil {
		t.Fatal(err)
	}
	msgs := drain(t, c)
	if len(msgs) < 3 || msgs[1]["t"] != "level" || msgs[2]["t"] != "snap" {
		t.Fatalf("Beitreten: %v", msgs)
	}
	client := msgs[2]["s"].(map[string]any)
	for range 300 { // 10 s ohne Senden
		r.Tick()
	}
	c.qmu.Lock()
	queued, last := len(c.queue), c.queue[len(c.queue)-1]
	c.qmu.Unlock()
	if cancelled || queued > 2 || !r.Has(c.device) {
		t.Fatalf("getrennt %v, %d Nachrichten in der Warteschlange", cancelled, queued)
	}
	msgs = drain(t, c)
	d := msgs[len(msgs)-1]
	if d["t"] != "delta" || d["tick"] != float64(last.tick) {
		t.Fatalf("erwartet delta zu Tick %d: %v", last.tick, d["t"])
	}
	client = apply(client, d["s"].(map[string]any))
	want := wire(t, last.state)
	delete(client, "events")
	delete(want, "events")
	if !reflect.DeepEqual(client, want) {
		t.Fatal("Client-Zustand nach dem Delta weicht vom Server ab")
	}
}

// Ein verworfener Zustand gibt seine Ereignisse an den neuesten weiter; andere Nachrichten werden nie ersetzt.
func TestVerworfenerZustandBehaeltEreignisse(t *testing.T) {
	cancelled := false
	c := queueConn("x", &cancelled)
	c.push(out{state: map[string]any{"events": []any{"a"}}, tick: 1})
	c.push(out{state: map[string]any{"events": []any{}}, tick: 2})
	c.push(out{state: map[string]any{"events": []any{"b"}}, tick: 3})
	c.push(out{data: []byte(`{"t":"seats"}`)})
	c.push(out{state: map[string]any{"events": []any{}}, tick: 4})
	if len(c.queue) != 3 || c.queue[0].tick != 3 || !reflect.DeepEqual(c.queue[0].events, []any{"a", "b"}) || c.queue[2].events != nil {
		t.Fatalf("Warteschlange: %+v", c.queue)
	}
}

// Volle Warteschlange aus anderen Nachrichten (nicht Zuständen) trennt die Verbindung weiterhin.
func TestVolleWarteschlangeIstAbbruch(t *testing.T) {
	cancelled := false
	c := queueConn("x", &cancelled)
	for range sendBuffer + 1 {
		c.enqueue(seatsMsg{T: "seats"})
	}
	if !cancelled {
		t.Fatal("nicht getrennt")
	}
}

// ack ist das seq beim Aufbau des Zustands (unter der Raum-Sperre), nicht beim Senden: Eine später verrechnete Eingabe
// bestätigt erst ein späterer Zustand; ersetzt ein Zustand einen wartenden, gilt sein eigenes ack.
func TestAckIstSeqDesZustands(t *testing.T) {
	cancelled := false
	c := queueConn("x", &cancelled)
	c.seq.Store(3)
	c.State(1, 0, map[string]any{})
	c.seq.Store(4)
	c.State(2, 0, map[string]any{}) // ersetzt Tick 1
	c.seq.Store(5)                  // nach dem Zustand verrechnet
	msgs := drain(t, c)
	if len(msgs) != 1 || msgs[0]["tick"] != float64(2) || msgs[0]["ack"] != float64(4) {
		t.Fatalf("erwartet Tick 2 mit ack 4: %v", msgs)
	}
}
