package main

import (
	"time"

	"k3c/tools/k3c-workbench/internal/resmon"
)

// Seite Monitoring: Messwerte des Messgeräts (internal/resmon) für die Diagramme. Die Seite fragt im Takt der Proben
// neu, solange sie sichtbar ist; es gibt keinen Verlauf im Frontend, der wachsen könnte.

const (
	// resMaxPoints begrenzt die Punkte je Diagramm (der Server verdichtet mit dem Maximum je Fenster).
	resMaxPoints = 240
	// resMinWindow und resMaxWindow begrenzen den gewählten Zeitraum in Minuten.
	resMinWindow = 5
	resMaxWindow = 24 * 60
	resTopOps    = 10
)

// ResPoint ist eine (verdichtete) Probe; Gauges folgen der Reihenfolge von ResourceView.Gauges.
type ResPoint struct {
	T           int64   `json:"t"` // Unix-Millisekunden
	HeapLive    int64   `json:"heapLive"`
	Total       int64   `json:"total"`
	Private     int64   `json:"private"`
	RSS         int64   `json:"rss"`
	ChildrenRSS int64   `json:"childrenRss"`
	Children    int     `json:"children"`
	Goroutines  int     `json:"goroutines"`
	Handles     int     `json:"handles"`
	GaugeValues []int64 `json:"gaugeValues"`
	GCCycles    int64   `json:"gcCycles"`
}

// ResGauge nennt einen Puffer-Messwert.
type ResGauge struct {
	Name  string `json:"name"`
	Bytes bool   `json:"bytes"`
}

// ResOp ist ein Vorgang (Tool-Aufruf, Oberflächen-Aufruf) mit seinen Summen.
type ResOp struct {
	Name     string  `json:"name"`
	Calls    int64   `json:"calls"`
	TotalMs  float64 `json:"totalMs"`
	MaxMs    float64 `json:"maxMs"`
	Alloc    int64   `json:"alloc"`
	MaxAlloc int64   `json:"maxAlloc"`
}

// ResTrend ist die Steigung einer Leitgröße je Stunde.
type ResTrend struct {
	Name      string  `json:"name"`
	Bytes     bool    `json:"bytes"`
	PerHour   float64 `json:"perHour"`
	WindowMin int     `json:"windowMin"`
}

// ResProc ist ein Kindprozess mit seinem Arbeitssatz.
type ResProc struct {
	Name string `json:"name"`
	PID  int    `json:"pid"`
	RSS  int64  `json:"rss"`
}

// ResourceView ist alles, was die Seite Monitoring zeichnet.
type ResourceView struct {
	Enabled   bool       `json:"enabled"`
	Started   int64      `json:"started"`
	IntervalS int        `json:"intervalS"`
	WindowMin int        `json:"windowMin"`
	Points    []ResPoint `json:"points"`
	Gauges    []ResGauge `json:"gauges"`
	Ops       []ResOp    `json:"ops"`
	Trends    []ResTrend `json:"trends"`
	Children  []ResProc  `json:"children"`
	Profiles  []string   `json:"profiles"`
}

// Resources liefert die Messwerte der letzten windowMin Minuten (Binding); ohne Messgerät ist Enabled falsch.
func (a *App) Resources(windowMin int) ResourceView {
	a.wait()
	if a.res == nil {
		return ResourceView{}
	}
	windowMin = min(max(windowMin, resMinWindow), resMaxWindow)
	gauges := a.res.Gauges()
	v := ResourceView{Enabled: true, Started: a.res.Started().UnixMilli(), IntervalS: int(a.res.Interval() / time.Second),
		WindowMin: windowMin, Gauges: make([]ResGauge, len(gauges)), Profiles: a.res.Dumps(),
		Points: []ResPoint{}, Ops: []ResOp{}, Trends: []ResTrend{}, Children: []ResProc{}}
	if v.Profiles == nil {
		v.Profiles = []string{}
	}
	for i, g := range gauges {
		v.Gauges[i] = ResGauge{Name: g.Name, Bytes: g.Unit == "B"}
	}
	for _, s := range a.res.Series(time.Duration(windowMin)*time.Minute, resMaxPoints) {
		v.Points = append(v.Points, resPoint(s, gauges))
	}
	for i, o := range a.res.Ops() {
		if i == resTopOps {
			break
		}
		v.Ops = append(v.Ops, ResOp{Name: o.Name, Calls: o.Calls, TotalMs: float64(o.Total) / float64(time.Millisecond),
			MaxMs: float64(o.Max) / float64(time.Millisecond), Alloc: int64(o.Alloc), MaxAlloc: int64(o.MaxAlloc)})
	}
	for _, t := range a.res.Trends() {
		v.Trends = append(v.Trends, ResTrend{Name: t.Name, Bytes: t.Bytes, PerHour: t.PerHour, WindowMin: int(t.Window / time.Minute)})
	}
	for _, p := range a.res.Top() {
		v.Children = append(v.Children, ResProc{Name: p.Name, PID: int(p.PID), RSS: int64(p.RSS)})
	}
	return v
}

func resPoint(s resmon.Sample, gauges []resmon.GaugeInfo) ResPoint {
	p := ResPoint{T: s.At.UnixMilli(), HeapLive: int64(s.HeapLive), Total: int64(s.Total), Private: int64(s.Private),
		RSS: int64(s.RSS), ChildrenRSS: int64(s.ChildrenRSS), Children: s.Children, Goroutines: s.Goroutines,
		Handles: s.Handles, GCCycles: int64(s.GCCycles), GaugeValues: make([]int64, len(gauges))}
	for i, g := range gauges {
		p.GaugeValues[i] = s.Gauges[g.Name]
	}
	return p
}
