package net

import (
	"context"
	"testing"
	"time"

	"k3c/engine/room"
)

// readAll liest im Hintergrund, damit der Client Pings beantwortet (coder/websocket verarbeitet Kontrollframes in Read).
func (c *client) readAll() {
	go func() {
		for {
			if _, _, err := c.ws.Read(context.Background()); err != nil {
				return
			}
		}
	}()
}

// B-281/AC-04: Zwei Geräte in einem Raum haben getrennte RTT- und Warteschlangen-Reihen.
func TestZweiGeraeteGetrennteReihen(t *testing.T) {
	old := pingEvery
	pingEvery = 20 * time.Millisecond
	t.Cleanup(func() { pingEvery = old })
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox-0001")
	create(x, "familie", 0)
	x.entered()
	h := hello(t, srv, "handy-002")
	joinRoom(h, "KRNZ", 0)
	h.entered()
	x.readAll()
	h.readAll()
	deadline := time.Now().Add(3 * time.Second)
	for time.Now().Before(deadline) {
		d := m.Monitor.Since(0, time.Now()).Devices
		xs, hs := d["xbox-000"], d["handy-00"]
		if len(d) == 2 && pinged(xs.Points) && pinged(hs.Points) && xs.Room == "KRNZ" && hs.Room == "KRNZ" {
			return
		}
		time.Sleep(20 * time.Millisecond)
	}
	t.Fatalf("Reihen: %+v", m.Monitor.Since(0, time.Now()).Devices)
}

// pinged: mindestens ein Punkt mit Pong.
func pinged(points []room.DevicePoint) bool {
	for _, p := range points {
		if p.RTTMs >= 0 {
			return true
		}
	}
	return false
}
