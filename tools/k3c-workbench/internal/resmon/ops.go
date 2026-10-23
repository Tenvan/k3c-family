package resmon

import (
	"sort"
	"time"
)

// opStat sammelt Dauer und Allokation eines Vorgangs (Tool, Oberflächen-Aufruf, Wächter).
type opStat struct {
	Calls    int64
	Total    time.Duration
	Max      time.Duration
	Alloc    uint64 // allokierte Bytes insgesamt (prozessweit gemessen, bei Parallelität ungenau nach oben)
	MaxAlloc uint64
}

// Op ist ein Vorgang mit seinen Summen.
type Op struct {
	Name string
	opStat
}

// allocBytes sind die seit Prozessstart allokierten Heap-Bytes (kumulativ).
func allocBytes() uint64 { return readMetrics("/gc/heap/allocs:bytes")[0] }

// Track misst einen Vorgang: `defer m.Track("tool:check_run")()`. Allokation wird prozessweit gelesen; laufen
// Vorgänge parallel, gehört ein Teil davon den anderen. Über viele Aufrufe zeigt die Summe trotzdem den Verursacher.
// Ein Messgerät nil misst nichts.
func (m *Monitor) Track(name string) func() {
	if m == nil {
		return func() {}
	}
	start, a0 := m.cfg.Now(), allocBytes()
	return func() {
		d, a := m.cfg.Now().Sub(start), allocBytes()-a0
		m.mu.Lock()
		defer m.mu.Unlock()
		o, ok := m.ops[name]
		if !ok {
			if len(m.ops) >= maxOps {
				if o = m.ops[overflowOp]; o == nil {
					o = &opStat{}
					m.ops[overflowOp] = o
				}
			} else {
				o = &opStat{}
				m.ops[name] = o
			}
		}
		o.Calls++
		o.Total += d
		o.Alloc += a
		o.Max, o.MaxAlloc = max(o.Max, d), max(o.MaxAlloc, a)
	}
}

// Ops liefert die Vorgänge, absteigend nach allokierten Bytes.
func (m *Monitor) Ops() []Op {
	m.mu.Lock()
	out := make([]Op, 0, len(m.ops))
	for n, o := range m.ops {
		out = append(out, Op{Name: n, opStat: *o})
	}
	m.mu.Unlock()
	sort.Slice(out, func(i, j int) bool { return out[i].Alloc > out[j].Alloc })
	return out
}
