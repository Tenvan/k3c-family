// Package resmon misst den Ressourcenverbrauch von k3c-workbench über Stunden: Speicher (Go-Heap, Prozess, Kindprozesse),
// Goroutinen, Handles und die Größe der eigenen Puffer. Ein Messgerät je Prozess nimmt in festem Takt Proben, hält einen
// begrenzten Verlauf, schreibt in großen Abständen eine Zeile ins Log und legt beim Überschreiten einer Schwelle ein
// Heap-Profil ab (`go tool pprof`). Je Vorgang (Tool-Aufruf, Oberflächen-Aufruf, Wächter) merkt es Dauer und
// Allokation, damit ein Dauerläufer auffällt, bevor der Speicher voll ist.
package resmon

import (
	"context"
	"log/slog"
	"runtime/metrics"
	"sort"
	"sync"
	"time"
)

const (
	defaultInterval  = 30 * time.Second
	defaultHistory   = 2880 // 24 h bei 30 s
	defaultLogEvery  = 10   // alle 5 min eine Zeile im Log
	defaultKeepDumps = 5
	// DefaultHeapDump ist der Live-Heap, ab dem das erste Profil entsteht; danach je 50 % mehr.
	DefaultHeapDump = 1 << 30
	maxOps          = 256 // verschiedene Vorgangsnamen, danach zählt overflowOp
	overflowOp      = "(weitere)"
)

// Config beschreibt ein Messgerät; Nullwerte nehmen die Vorgaben.
type Config struct {
	Interval  time.Duration
	History   int
	LogEvery  int    // jede n-te Probe als Zeile im Log
	Dir       string // Ordner der Profile (heap-*.pprof, goroutines-*.txt); leer: keine Profile
	HeapDump  uint64 // Live-Heap in Bytes für das erste Heap-Profil
	KeepDumps int
	Log       *slog.Logger
	Now       func() time.Time
}

// Sample ist eine Probe.
type Sample struct {
	At          time.Time
	HeapLive    uint64 // Live-Heap nach dem letzten GC
	Total       uint64 // vom Go-Laufzeitsystem gehaltener Speicher
	Private     uint64 // privater Speicher des Prozesses (Windows: Commit)
	RSS         uint64 // Arbeitssatz des Prozesses
	Children    int
	ChildrenRSS uint64
	Goroutines  int
	Handles     int
	GCCycles    uint64
	Gauges      map[string]int64
}

type gauge struct {
	name string
	unit string // "B" für Bytes, sonst Zähler
	fn   func() int64
}

// Monitor ist das Messgerät. Alle Methoden sind nebenläufig sicher.
type Monitor struct {
	cfg Config

	mu       sync.Mutex
	gauges   []gauge
	history  []Sample
	ops      map[string]*opStat
	started  time.Time
	nextHeap uint64
	nextGor  int
	n        int
	top      []ProcInfo
}

// New legt ein Messgerät an; gestartet wird es mit Run.
func New(cfg Config) *Monitor {
	if cfg.Interval <= 0 {
		cfg.Interval = defaultInterval
	}
	if cfg.History <= 0 {
		cfg.History = defaultHistory
	}
	if cfg.LogEvery <= 0 {
		cfg.LogEvery = defaultLogEvery
	}
	if cfg.HeapDump == 0 {
		cfg.HeapDump = DefaultHeapDump
	}
	if cfg.KeepDumps <= 0 {
		cfg.KeepDumps = defaultKeepDumps
	}
	if cfg.Now == nil {
		cfg.Now = time.Now
	}
	if cfg.Log == nil {
		cfg.Log = slog.New(slog.DiscardHandler)
	}
	return &Monitor{cfg: cfg, ops: map[string]*opStat{}, started: cfg.Now(), nextHeap: cfg.HeapDump, nextGor: 2000}
}

// Gauge meldet einen Messwert an, der bei jeder Probe gelesen wird (Größe eines Puffers, Zahl der Läufe). unit "B"
// formatiert den Wert als Bytes. fn muss schnell sein und darf nicht blockieren.
func (m *Monitor) Gauge(name, unit string, fn func() int64) {
	m.mu.Lock()
	m.gauges = append(m.gauges, gauge{name: name, unit: unit, fn: fn})
	m.mu.Unlock()
}

// Run nimmt in festem Takt Proben, bis ctx endet. Die erste Probe kommt sofort.
func (m *Monitor) Run(ctx context.Context) {
	m.Sample()
	t := time.NewTicker(m.cfg.Interval)
	defer t.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-t.C:
			m.Sample()
		}
	}
}

// Sample nimmt eine Probe, hängt sie an den Verlauf und prüft die Schwellen.
func (m *Monitor) Sample() Sample {
	s := m.read()
	m.mu.Lock()
	m.history = append(m.history, s)
	if len(m.history) >= 2*m.cfg.History { // erst bei doppelter Länge kürzen: ein Kopieren je History Proben, nicht je Probe
		m.history = append(m.history[:0], m.history[len(m.history)-m.cfg.History:]...)
	}
	m.n++
	logNow := m.n == 1 || m.n%m.cfg.LogEvery == 0
	m.mu.Unlock()
	if logNow {
		m.cfg.Log.Info("💓 ressourcen: "+summary(s), "ns", "res", "heap_live", s.HeapLive, "total", s.Total,
			"rss", s.RSS, "private", s.Private, "children_rss", s.ChildrenRSS, "goroutines", s.Goroutines,
			"handles", s.Handles, "gc", s.GCCycles)
	}
	m.checkThresholds(s)
	return s
}

// read liest Laufzeit, Prozess und Messwerte. Die Messwerte laufen außerhalb der Sperre: sie nehmen eigene Sperren.
func (m *Monitor) read() Sample {
	s := Sample{At: m.cfg.Now()}
	vals := readMetrics("/gc/heap/live:bytes", "/memory/classes/total:bytes", "/sched/goroutines:goroutines",
		"/gc/cycles/total:gc-cycles")
	s.HeapLive, s.Total, s.Goroutines, s.GCCycles = vals[0], vals[1], int(vals[2]), vals[3]
	p := readProcess()
	s.RSS, s.Private, s.Handles = p.rss, p.private, p.handles
	s.Children, s.ChildrenRSS = len(p.children), 0
	for _, c := range p.children {
		s.ChildrenRSS += c.RSS
	}
	m.mu.Lock()
	gs := slicesClone(m.gauges)
	m.top = topProcs(p.children, 5)
	m.mu.Unlock()
	s.Gauges = make(map[string]int64, len(gs))
	for _, g := range gs {
		s.Gauges[g.name] = g.fn()
	}
	return s
}

func slicesClone(in []gauge) []gauge { return append([]gauge(nil), in...) }

func topProcs(in []ProcInfo, n int) []ProcInfo {
	out := append([]ProcInfo(nil), in...)
	sort.Slice(out, func(i, j int) bool { return out[i].RSS > out[j].RSS })
	if len(out) > n {
		out = out[:n]
	}
	return out
}

// readMetrics liest uint64-Messwerte aus runtime/metrics; ein unbekannter Name ergibt 0.
func readMetrics(names ...string) []uint64 {
	samples := make([]metrics.Sample, len(names))
	for i, n := range names {
		samples[i].Name = n
	}
	metrics.Read(samples)
	out := make([]uint64, len(names))
	for i, s := range samples {
		if s.Value.Kind() == metrics.KindUint64 {
			out[i] = s.Value.Uint64()
		}
	}
	return out
}

// History liefert den Verlauf (höchstens Config.History Proben), älteste zuerst.
func (m *Monitor) History() []Sample {
	m.mu.Lock()
	defer m.mu.Unlock()
	return append([]Sample(nil), m.tail()...)
}

// tail ist der gültige Teil des Verlaufs; der Aufrufer hält m.mu.
func (m *Monitor) tail() []Sample {
	return m.history[max(0, len(m.history)-m.cfg.History):]
}

// Latest liefert die letzte Probe; ohne Probe false.
func (m *Monitor) Latest() (Sample, bool) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if len(m.history) == 0 {
		return Sample{}, false
	}
	return m.history[len(m.history)-1], true
}
