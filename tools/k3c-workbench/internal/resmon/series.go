package resmon

import (
	"sort"
	"time"
)

// GaugeInfo beschreibt einen Messwert der Puffer (Reihenfolge wie in Sample.Gauges der Ansicht).
type GaugeInfo struct {
	Name string
	Unit string // "B" für Bytes, sonst Zähler
}

// TrendInfo ist die Steigung einer Messgröße je Stunde über Window.
type TrendInfo struct {
	Name    string
	Bytes   bool
	PerHour float64
	Window  time.Duration
}

// Interval ist der Abstand der Proben.
func (m *Monitor) Interval() time.Duration { return m.cfg.Interval }

// Started ist der Beginn der Messung.
func (m *Monitor) Started() time.Time { return m.started }

// Gauges nennt die angemeldeten Messwerte in Anmelde-Reihenfolge.
func (m *Monitor) Gauges() []GaugeInfo {
	m.mu.Lock()
	defer m.mu.Unlock()
	out := make([]GaugeInfo, len(m.gauges))
	for i, g := range m.gauges {
		out[i] = GaugeInfo{Name: g.name, Unit: g.unit}
	}
	return out
}

// Top sind die Kindprozesse mit dem größten Arbeitssatz der letzten Probe.
func (m *Monitor) Top() []ProcInfo {
	m.mu.Lock()
	defer m.mu.Unlock()
	return append([]ProcInfo(nil), m.top...)
}

// Trends liefert den Trend der vier Leitgrößen; eine Größe ohne genug Messzeit fehlt.
func (m *Monitor) Trends() []TrendInfo {
	h := m.History()
	var out []TrendInfo
	for _, t := range trendSpecs {
		if tr, ok := trendOf(h, t.val); ok {
			out = append(out, TrendInfo{Name: t.name, Bytes: t.bytes, PerHour: tr.perHour, Window: tr.window})
		}
	}
	return out
}

// Series liefert die Proben der letzten window, auf höchstens maxPoints verdichtet: je Zeitfenster der größte Wert
// jeder Größe (ein Ausschlag bleibt sichtbar), der Zeitpunkt ist der der letzten Probe im Fenster. Das Ergebnis ist
// eine Kopie; der Aufrufer darf sie behalten, ohne den Verlauf des Messgeräts zu halten.
func (m *Monitor) Series(window time.Duration, maxPoints int) []Sample {
	h := m.History()
	if len(h) == 0 || maxPoints <= 0 {
		return nil
	}
	from := h[len(h)-1].At.Add(-window)
	h = h[sort.Search(len(h), func(i int) bool { return !h[i].At.Before(from) }):]
	step := (len(h) + maxPoints - 1) / maxPoints
	if step <= 1 {
		return h
	}
	out := make([]Sample, 0, len(h)/step+1)
	for i := 0; i < len(h); i += step {
		out = append(out, peak(h[i:min(i+step, len(h))]))
	}
	return out
}

// peak fasst Proben zu einer zusammen: Maximum jeder Größe, Zeit der letzten.
func peak(bucket []Sample) Sample {
	out := Sample{At: bucket[len(bucket)-1].At, Gauges: map[string]int64{}}
	for _, s := range bucket {
		out.HeapLive, out.Total = max(out.HeapLive, s.HeapLive), max(out.Total, s.Total)
		out.Private, out.RSS = max(out.Private, s.Private), max(out.RSS, s.RSS)
		out.ChildrenRSS, out.GCCycles = max(out.ChildrenRSS, s.ChildrenRSS), max(out.GCCycles, s.GCCycles)
		out.Children, out.Goroutines, out.Handles = max(out.Children, s.Children), max(out.Goroutines, s.Goroutines), max(out.Handles, s.Handles)
		for k, v := range s.Gauges {
			out.Gauges[k] = max(out.Gauges[k], v)
		}
	}
	return out
}
