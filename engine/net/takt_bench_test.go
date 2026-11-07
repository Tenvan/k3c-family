package net

import (
	"context"
	"io"
	"log/slog"
	"os"
	"slices"
	"strconv"
	"sync"
	"testing"
	"time"

	"k3c/engine/room"
)

// BenchmarkTaktAbstand4 misst den echten 30-Hz-Takt (Manager.Run) mit 4 Geräten eine Dauer lang: Abstand zwischen zwei
// Zustandslieferungen an Gerät 0 (p50/p99/max in ms). Messung RS1 (B-725): `-benchtime 1x`,
// Dauer über die Umgebung K3C_TAKT_SECONDS (Standard 20). Läuft in Echtzeit, daher nur auf Anforderung.
func BenchmarkTaktAbstand4(b *testing.B) {
	secs := 20
	if v, err := strconv.Atoi(os.Getenv("K3C_TAKT_SECONDS")); err == nil && v > 0 {
		secs = v
	}
	m := room.NewManager(memSaves{})
	NewHandler(Config{Rooms: m})
	s := &server{log: slog.New(slog.NewTextHandler(io.Discard, nil))}
	var (
		mu    sync.Mutex
		stamp []time.Time
		r     *room.Room
	)
	for i := range 4 {
		c := &conn{s: s, device: "takt" + strconv.Itoa(i), wake: make(chan struct{}, 1), cancel: func() {}}
		first := i == 0
		go func() {
			for range c.wake {
				for _, ok := c.pop(); ok; _, ok = c.pop() {
					if first {
						mu.Lock()
						stamp = append(stamp, time.Now())
						mu.Unlock()
					}
				}
			}
		}()
		var err error
		if i == 0 {
			r, err = m.Create(c.device, c, "takt", true, 0, []int{0}, room.Options{})
		} else {
			_, err = m.Join(c.device, c, r.Code, []int{0})
		}
		if err != nil {
			b.Fatal(err)
		}
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(secs)*time.Second)
	defer cancel()
	m.Run(ctx)
	mu.Lock()
	defer mu.Unlock()
	var gaps []float64
	for i := 1; i < len(stamp); i++ {
		gaps = append(gaps, float64(stamp[i].Sub(stamp[i-1]))/float64(time.Millisecond))
	}
	if len(gaps) < 10 {
		b.Fatalf("zu wenige Lieferungen: %d", len(gaps))
	}
	slices.Sort(gaps)
	q := func(p int) float64 { return gaps[min(len(gaps)-1, len(gaps)*p/100)] }
	b.ReportMetric(q(50), "gapP50-ms")
	b.ReportMetric(q(99), "gapP99-ms")
	b.ReportMetric(gaps[len(gaps)-1], "gapMax-ms")
	b.ReportMetric(float64(len(gaps)+1), "lieferungen")
}
