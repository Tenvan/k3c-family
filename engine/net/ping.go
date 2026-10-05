package net

import (
	"context"
	"math"
	"time"

	"k3c/engine/room"
)

// RTT je Gerät (B-281, Glossar „RTT (Ping)“): Eine Goroutine je Verbindung schickt einmal je Sekunde einen WebSocket-Ping
// (Kontrollframe, der Browser antwortet selbst, kein Protokollwechsel) und hängt RTT, Länge der Warteschlange und die seit
// dem letzten Punkt verworfenen Zustände an die Messreihe des Geräts. Nie unter der Raum-Sperre.

// pingEvery ist der Abstand der Pings und die Frist für den Pong; Test-Naht.
var pingEvery = time.Second

func (c *conn) pinger(ctx context.Context, every time.Duration) {
	t := time.NewTicker(every)
	defer t.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-t.C:
			c.probe(ctx, every)
		}
	}
}

// probe misst eine RTT (-1 ohne Pong in der Frist) und nimmt den Punkt auf.
func (c *conn) probe(ctx context.Context, timeout time.Duration) {
	start := time.Now()
	pctx, cancel := context.WithTimeout(ctx, timeout)
	err := c.ws.Ping(pctx)
	cancel()
	if ctx.Err() != nil {
		return
	}
	rtt := float32(-1)
	if err == nil {
		rtt = float32(time.Since(start)) / float32(time.Millisecond)
	}
	c.qmu.Lock()
	queue, drops := len(c.queue), c.drops
	c.drops = 0
	c.qmu.Unlock()
	c.mu.Lock()
	code := ""
	if c.room != nil {
		code = c.room.Code
	}
	c.mu.Unlock()
	c.s.cfg.Rooms.Monitor.Device(short(c.device), code, room.DevicePoint{T: time.Now().UnixMilli(), RTTMs: rtt,
		Queue: uint16(min(queue, math.MaxUint16)), Dropped: uint16(min(drops, math.MaxUint16))})
}

// short kürzt eine Geräte-ID für Log und Messreihe.
func short(id string) string {
	if len(id) > 8 {
		return id[:8]
	}
	return id
}
