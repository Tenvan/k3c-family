// Package usage wertet die MCP-Aufrufe von k3c-dev aus (B-062): je Tool Laufzeiten, Perzentile aus einem Histogramm,
// Ausreißer, häufigste Argumente und Fehler, einmal für die Sitzung und einmal für die Gesamtzeit, dazu eine
// Minuten-Zeitreihe der letzten sieben Tage für die Live-Monitore. Das Paket kennt den Server nicht; die Middleware
// gibt jeden beendeten Aufruf mit den rohen Argumenten hinein.
package usage

import (
	"maps"
	"sync"
	"time"
)

const (
	// SeriesSpan ist das Fenster der Minuten-Zeitreihe.
	SeriesSpan = 7 * 24 * time.Hour
	timeLayout = "2006-01-02T15:04:05.000Z07:00"
)

// Event ist ein beendeter Tool-Aufruf.
type Event struct {
	At         time.Time
	Tool       string
	Args       string // roh, wie vom Client gesendet
	DurationMs float64
	OK         bool
	Error      string
}

// minute ist ein Eintrag der Zeitreihe; T zählt Minuten seit 1970.
type minute struct {
	T        int64              `json:"t"`
	Calls    map[string]int     `json:"calls"`
	Errors   int                `json:"errors"`
	Ms       map[string]float64 `json:"ms"`
	MaxMs    map[string]float64 `json:"maxMs"`
	Outliers map[string]int     `json:"outliers"`
}

// Bucket ist eine Minute der Zeitreihe im Schnappschuss; TS ist der Minutenanfang in ms seit 1970.
type Bucket struct {
	TS       int64              `json:"ts"`
	Calls    map[string]int     `json:"calls"`
	Errors   int                `json:"errors"`
	Ms       map[string]float64 `json:"ms"`
	MaxMs    map[string]float64 `json:"maxMs"`
	Outliers map[string]int     `json:"outliers"`
}

// Tracker hält beide Bereiche und die Zeitreihe; alle Methoden sind nebenläufig sicher.
type Tracker struct {
	mu      sync.Mutex
	now     func() time.Time
	session *scope
	allTime *scope
	minutes []minute // aufsteigend nach T, ohne leere Minuten
	persist *persist // nil = nur im Speicher (New)
}

// New legt einen leeren Tracker nur im Speicher an; now ist die Uhr (Tests stellen sie). Mit Datei: Open.
func New(now func() time.Time) *Tracker {
	start := now()
	return &Tracker{now: now, session: newScope(start), allTime: newScope(start)}
}

// Record zählt einen beendeten Aufruf. Der Vergleichswert für den Ausreißer entsteht vor dem Eintragen und stammt
// aus der Gesamtzeit, die mehr Aufrufe kennt als die Sitzung.
func (t *Tracker) Record(e Event) {
	args := normArgs(e.Args)
	t.mu.Lock()
	defer t.mu.Unlock()
	base := t.allTime.baseline(e.Tool, args)
	outlier := base > 0 && e.DurationMs > OutlierFactor*base && e.DurationMs >= OutlierFloorMs
	slow := SlowCall{At: e.At.Format(timeLayout), Tool: e.Tool, Args: args, DurationMs: e.DurationMs, OK: e.OK, BaselineMs: base}
	t.session.add(e, args, slow, outlier)
	t.allTime.add(e, args, slow, outlier)
	t.addMinute(e, outlier)
	t.scheduleSave()
}

func (t *Tracker) addMinute(e Event, outlier bool) {
	m := e.At.Unix() / 60
	if n := len(t.minutes); n == 0 || t.minutes[n-1].T != m {
		t.minutes = append(t.minutes, minute{T: m, Calls: map[string]int{}, Ms: map[string]float64{},
			MaxMs: map[string]float64{}, Outliers: map[string]int{}})
	}
	b := &t.minutes[len(t.minutes)-1]
	b.Calls[e.Tool]++
	b.Ms[e.Tool] += e.DurationMs
	b.MaxMs[e.Tool] = max(b.MaxMs[e.Tool], e.DurationMs)
	if !e.OK {
		b.Errors++
	}
	if outlier {
		b.Outliers[e.Tool]++
	}
	t.prune()
}

// prune verwirft Minuten, die älter als SeriesSpan sind.
func (t *Tracker) prune() {
	oldest := t.now().Add(-SeriesSpan).Unix() / 60
	i := 0
	for i < len(t.minutes) && t.minutes[i].T < oldest {
		i++
	}
	t.minutes = t.minutes[i:]
}

// Snapshot liefert beide Bereiche und die Zeitreihe; alle Listen sind leer statt nil.
func (t *Tracker) Snapshot() Usage {
	t.mu.Lock()
	defer t.mu.Unlock()
	t.prune()
	out := Usage{Session: t.session.snapshot(), AllTime: t.allTime.snapshot(), Minutes: make([]Bucket, 0, len(t.minutes))}
	for _, m := range t.minutes {
		// Kopien: Record ändert die Maps der letzten Minute weiter, während der Aufrufer liest.
		out.Minutes = append(out.Minutes, Bucket{TS: m.T * 60_000, Calls: maps.Clone(m.Calls), Errors: m.Errors,
			Ms: maps.Clone(m.Ms), MaxMs: maps.Clone(m.MaxMs), Outliers: maps.Clone(m.Outliers)})
	}
	return out
}
