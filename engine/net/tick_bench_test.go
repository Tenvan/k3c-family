package net

import (
	"io"
	"log/slog"
	"strconv"
	"testing"

	"k3c/engine/room"
)

// BenchmarkTickDevices4 misst Room.Tick mit 4 Geräten auf einer Stufe, inklusive der Arbeit der Verbindungen unter
// der Raum-Sperre (B-276/AC-04). Die Verbindungen haben kein WebSocket; je Gerät leert eine Goroutine die
// Warteschlange, ohne zu kodieren (Delta und JSON laufen in der Schreib-Goroutine, außerhalb der Sperre).
func BenchmarkTickDevices4(b *testing.B) {
	m := room.NewManager(memSaves{})
	NewHandler(Config{Rooms: m}) // setzt m.Snapshot
	s := &server{log: slog.New(slog.NewTextHandler(io.Discard, nil))}
	var r *room.Room
	for i := range 4 {
		c := benchConn(s, "bench"+strconv.Itoa(i))
		var err error
		if i == 0 {
			r, err = m.Create(c.device, c, "bench", true, 0, []int{0}, room.Options{})
		} else {
			_, err = m.Join(c.device, c, r.Code, []int{0})
		}
		if err != nil {
			b.Fatal(err)
		}
	}
	for b.Loop() {
		r.Tick()
	}
}

// benchConn ist eine Verbindung ohne WebSocket, deren Warteschlange eine Goroutine leert.
func benchConn(s *server, device string) *conn {
	c := &conn{s: s, device: device, wake: make(chan struct{}, 1), cancel: func() {}}
	go func() {
		for range c.wake {
			for _, ok := c.pop(); ok; _, ok = c.pop() {
			}
		}
	}()
	return c
}
