package resmon

import (
	"fmt"
	"sort"
	"strings"
	"time"
)

const (
	// growthWarn ist das Wachstum des Live-Heaps je Stunde, ab dem ein Warnhinweis ins Log kommt.
	growthWarn = 200 << 20
	// growthMin ist die kürzeste Spanne, über die ein Wachstum gilt (kürzer ist es Rauschen).
	growthMin = 30 * time.Minute
	// growthWindow ist die Spanne, über die der Trend gerechnet wird.
	growthWindow = 2 * time.Hour
)

type trend struct {
	perHour float64
	window  time.Duration
}

// growth schätzt das Wachstum des Live-Heaps je Stunde.
func (m *Monitor) growth() (trend, bool) {
	return trendOf(m.History(), func(s Sample) float64 { return float64(s.HeapLive) })
}

// trendOf rechnet die Steigung der Ausgleichsgeraden über die letzten growthWindow (mindestens growthMin); ein
// GC-Sägezahn verfälscht sie weniger als der Vergleich von Anfang und Ende.
func trendOf(h []Sample, val func(Sample) float64) (trend, bool) {
	if len(h) < 3 {
		return trend{}, false
	}
	last := h[len(h)-1].At
	h = h[sort.Search(len(h), func(i int) bool { return last.Sub(h[i].At) <= growthWindow }):]
	span := last.Sub(h[0].At)
	if len(h) < 3 || span < growthMin {
		return trend{}, false
	}
	var sx, sy, sxx, sxy float64
	for _, s := range h {
		x, y := s.At.Sub(h[0].At).Hours(), val(s)
		sx, sy, sxx, sxy = sx+x, sy+y, sxx+x*x, sxy+x*y
	}
	n := float64(len(h))
	den := n*sxx - sx*sx
	if den == 0 {
		return trend{}, false
	}
	return trend{perHour: (n*sxy - sx*sy) / den, window: span.Round(time.Minute)}, true
}

func formatBytes(n uint64) string {
	switch {
	case n >= 1<<30:
		return fmt.Sprintf("%.2f GiB", float64(n)/(1<<30))
	case n >= 1<<20:
		return fmt.Sprintf("%.1f MiB", float64(n)/(1<<20))
	case n >= 1<<10:
		return fmt.Sprintf("%.0f KiB", float64(n)/(1<<10))
	}
	return fmt.Sprintf("%d B", n)
}

func signedBytes(v float64) string {
	if v < 0 {
		return "-" + formatBytes(uint64(-v))
	}
	return "+" + formatBytes(uint64(v))
}

func summary(s Sample) string {
	return fmt.Sprintf("heap live %s · go gesamt %s · RSS %s · privat %s · Kinder %d (%s) · Goroutinen %d · Handles %d",
		formatBytes(s.HeapLive), formatBytes(s.Total), formatBytes(s.RSS), formatBytes(s.Private), s.Children,
		formatBytes(s.ChildrenRSS), s.Goroutines, s.Handles)
}

// Report ist der Bericht für workbench_resources: aktuelle Werte, Trend, Messwerte der Puffer, die größten Vorgänge
// nach Allokation und die abgelegten Profile.
func (m *Monitor) Report() string {
	cur, ok := m.Latest()
	if !ok {
		cur = m.Sample()
	}
	m.mu.Lock()
	since, n, top := m.started, len(m.tail()), append([]ProcInfo(nil), m.top...)
	m.mu.Unlock()
	out := []string{fmt.Sprintf("Ressourcen · Messgerät seit %s · %d Proben (alle %s)",
		since.Local().Format("02.01. 15:04:05"), n, m.cfg.Interval), summary(cur)}
	out = append(out, m.trendLines()...)
	if len(top) > 0 {
		parts := make([]string, len(top))
		for i, p := range top {
			parts[i] = fmt.Sprintf("%s(%d) %s", p.Name, p.PID, formatBytes(p.RSS))
		}
		out = append(out, "Kinder nach RSS: "+strings.Join(parts, " · "))
	}
	out = append(out, m.gaugeLines(cur)...)
	out = append(out, opLines(m.Ops(), 8)...)
	if d := m.Dumps(); len(d) > 0 {
		out = append(out, "Profile (go tool pprof): "+strings.Join(d, " · "))
	}
	return strings.Join(out, "\n")
}

// trendSpecs sind die Leitgrößen des Trends (Bericht und Ansicht).
var trendSpecs = []struct {
	name  string
	bytes bool
	val   func(Sample) float64
}{
	{"Live-Heap", true, func(s Sample) float64 { return float64(s.HeapLive) }},
	{"Privat", true, func(s Sample) float64 { return float64(s.Private) }},
	{"Kinder-RSS", true, func(s Sample) float64 { return float64(s.ChildrenRSS) }},
	{"Goroutinen", false, func(s Sample) float64 { return float64(s.Goroutines) }},
}

func (m *Monitor) trendLines() []string {
	var out []string
	for _, tr := range m.Trends() {
		if tr.Bytes {
			out = append(out, fmt.Sprintf("Trend %s: %s/h über %s", tr.Name, signedBytes(tr.PerHour), tr.Window))
		} else {
			out = append(out, fmt.Sprintf("Trend %s: %+.1f/h über %s", tr.Name, tr.PerHour, tr.Window))
		}
	}
	if len(out) == 0 {
		return []string{fmt.Sprintf("Trend: erst ab %s Messzeit", growthMin)}
	}
	return out
}

func (m *Monitor) gaugeLines(cur Sample) []string {
	m.mu.Lock()
	gs := append([]gauge(nil), m.gauges...)
	m.mu.Unlock()
	if len(gs) == 0 {
		return nil
	}
	parts := make([]string, 0, len(gs))
	for _, g := range gs {
		v := cur.Gauges[g.name]
		if g.unit == "B" {
			parts = append(parts, fmt.Sprintf("%s %s", g.name, formatBytes(uint64(max(v, 0)))))
		} else {
			parts = append(parts, fmt.Sprintf("%s %d", g.name, v))
		}
	}
	return []string{"Puffer: " + strings.Join(parts, " · ")}
}

func opLines(ops []Op, n int) []string {
	if len(ops) == 0 {
		return nil
	}
	out := []string{"Vorgänge nach Allokation (Aufrufe · Σ Allokation · Ø · max · Σ Dauer · max Dauer):"}
	for i, o := range ops {
		if i == n {
			break
		}
		out = append(out, fmt.Sprintf("  %s · %d · %s · %s · %s · %s · %s", o.Name, o.Calls, formatBytes(o.Alloc),
			formatBytes(o.Alloc/uint64(max(o.Calls, 1))), formatBytes(o.MaxAlloc), o.Total.Round(time.Millisecond),
			o.Max.Round(time.Millisecond)))
	}
	return out
}
