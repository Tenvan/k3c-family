package resmon

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// clock ist eine Uhr, die der Test vorstellt.
type clock struct{ t time.Time }

func (c *clock) now() time.Time { return c.t }

func TestSampleLiestGaugesUndHaeltVerlaufBegrenzt(t *testing.T) {
	c := &clock{t: time.Unix(1_000_000, 0)}
	m := New(Config{History: 3, Now: c.now})
	v := int64(7)
	m.Gauge("puffer", "B", func() int64 { return v })
	for i := 0; i < 5; i++ {
		c.t = c.t.Add(time.Minute)
		m.Sample()
	}
	if got := len(m.History()); got != 3 {
		t.Fatalf("Verlauf %d, will 3", got)
	}
	s, ok := m.Latest()
	if !ok || s.Gauges["puffer"] != 7 || s.Goroutines < 1 || s.Total == 0 {
		t.Fatalf("Probe unvollständig: %+v", s)
	}
}

func TestTrendErkenntWachstumErstNachMindestspanne(t *testing.T) {
	start := time.Unix(0, 0)
	mk := func(minutes int, live uint64) Sample {
		return Sample{At: start.Add(time.Duration(minutes) * time.Minute), HeapLive: live}
	}
	live := func(s Sample) float64 { return float64(s.HeapLive) }
	if _, ok := trendOf([]Sample{mk(0, 1), mk(5, 2), mk(10, 3)}, live); ok {
		t.Fatal("10 min sind zu kurz für einen Trend")
	}
	var h []Sample
	for i := 0; i <= 60; i += 5 {
		h = append(h, mk(i, uint64(100<<20+i*(10<<20)/5))) // +10 MiB je 5 min = +120 MiB/h
	}
	tr, ok := trendOf(h, live)
	if !ok {
		t.Fatal("kein Trend über 60 min")
	}
	if perHour := tr.perHour / (1 << 20); perHour < 119 || perHour > 121 {
		t.Fatalf("Wachstum %.1f MiB/h, will 120", perHour)
	}
}

func TestTrackSummiertUndBegrenztNamen(t *testing.T) {
	m := New(Config{})
	for i := 0; i < maxOps+10; i++ {
		m.Track("op" + string(rune('A'+i%26)) + strings.Repeat("x", i))()
	}
	m.Track("opA")()
	ops := m.Ops()
	if len(ops) > maxOps+1 {
		t.Fatalf("%d Vorgänge, Grenze %d", len(ops), maxOps)
	}
	var overflow bool
	for _, o := range ops {
		overflow = overflow || o.Name == overflowOp
	}
	if !overflow {
		t.Fatal("kein Sammelname nach der Grenze")
	}
	var nilMon *Monitor
	nilMon.Track("x")() // darf nicht abstürzen
}

func TestHeapProfilWennSchwelleUeberschrittenUndAltesFaelltWeg(t *testing.T) {
	dir := t.TempDir()
	c := &clock{t: time.Unix(1_000_000, 0)}
	m := New(Config{Dir: dir, HeapDump: 1, KeepDumps: 2, Now: c.now})
	for i := 0; i < 4; i++ {
		c.t = c.t.Add(time.Second)
		m.writeHeap(Sample{At: c.t})
	}
	files, _ := filepath.Glob(filepath.Join(dir, "heap-*.pprof"))
	if len(files) != 2 {
		t.Fatalf("%d Profile, will 2", len(files))
	}
	if fi, _ := os.Stat(files[0]); fi.Size() == 0 {
		t.Fatal("leeres Profil")
	}
	if got := len(m.Dumps()); got != 2 {
		t.Fatalf("Dumps %d", got)
	}
}

func TestReportNenntWerteUndPuffer(t *testing.T) {
	m := New(Config{})
	m.Gauge("konsole", "B", func() int64 { return 2048 })
	m.Gauge("läufe", "", func() int64 { return 3 })
	m.Track("tool:check_run")()
	r := m.Report()
	for _, want := range []string{"heap live", "konsole 2 KiB", "läufe 3", "tool:check_run", "Trend"} {
		if !strings.Contains(r, want) {
			t.Errorf("Bericht ohne %q:\n%s", want, r)
		}
	}
}
