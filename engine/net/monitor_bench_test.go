package net

import (
	"context"
	"io"
	"log/slog"
	"strconv"
	"testing"
	"time"

	"k3c/engine/room"
)

// BenchmarkTickMonitor misst Room.Tick wie BenchmarkTickDevices4; bei „mit“ läuft daneben der Sammler 1000-mal so oft wie
// im Betrieb (jede Millisekunde Sample und je Gerät ein Punkt der Messreihe). B-281/AC-03: „mit“ höchstens 1 % bzw. 20 µs
// langsamer als „ohne“, Ergebnis in B-281 › Notizen. Der Zähler in safeTick (ein Inkrement) liegt außerhalb von Room.Tick.
func BenchmarkTickMonitor(b *testing.B) {
	for _, name := range []string{"ohne", "mit"} {
		b.Run(name, func(b *testing.B) {
			m, r := benchRoom(b)
			if name == "mit" {
				ctx, cancel := context.WithCancel(context.Background())
				defer cancel()
				go hammer(ctx, m)
			}
			for b.Loop() {
				r.Tick()
			}
		})
	}
}

// benchRoom ist ein Raum mit 4 Geräten ohne WebSocket wie in BenchmarkTickDevices4.
func benchRoom(b *testing.B) (*room.Manager, *room.Room) {
	m := room.NewManager(memSaves{})
	NewHandler(Config{Rooms: m})
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
	return m, r
}

// hammer nimmt jede Millisekunde Punkte auf, bis ctx endet.
func hammer(ctx context.Context, m *room.Manager) {
	t := time.NewTicker(time.Millisecond)
	defer t.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case now := <-t.C:
			m.Sample()
			for i := range 4 {
				m.Monitor.Device("bench"+strconv.Itoa(i), "BENCH", room.DevicePoint{T: now.UnixMilli(), RTTMs: 1})
			}
		}
	}
}
