package usage

import (
	"sort"
	"time"
)

const (
	// Ausreißer: länger als OutlierFactor × p95 der Vergleichsgruppe und mindestens OutlierFloorMs; eine
	// Vergleichsgruppe zählt ab BaselineCalls Aufrufen (B-062). Die Oberfläche zeigt dieselben Werte an.
	OutlierFactor  = 2.0
	OutlierFloorMs = 1000.0
	BaselineCalls  = 8
	// topN ist die Länge der Ranglisten, slowN die der Listen langsamster Aufrufe und letzter Ausreißer.
	topN  = 10
	slowN = 10
)

// SlowCall ist ein einzelner Aufruf in den Listen langsamster Aufrufe und letzter Ausreißer.
type SlowCall struct {
	At         string  `json:"at"`
	Tool       string  `json:"tool"`
	Args       string  `json:"args"`
	DurationMs float64 `json:"durationMs"`
	OK         bool    `json:"ok"`
	BaselineMs float64 `json:"baselineMs"`
}

// toolAgg sammelt alles zu einem Tool in einem Bereich.
type toolAgg struct {
	durAgg
	Errors    int                `json:"errors"`
	Outliers  int                `json:"outliers"`
	Args      map[string]int     `json:"args"`
	ArgDur    map[string]*durAgg `json:"argDur"`
	ErrorMsgs map[string]int     `json:"errorMsgs"`
}

// scope ist ein Bereich: die Sitzung oder die Gesamtzeit.
type scope struct {
	Since    time.Time           `json:"since"`
	Tools    map[string]*toolAgg `json:"tools"`
	Slowest  []SlowCall          `json:"slowest"`
	Outliers []SlowCall          `json:"outliers"` // neueste zuerst
}

func newScope(since time.Time) *scope {
	return &scope{Since: since, Tools: map[string]*toolAgg{}}
}

// baseline ist der p95 der Vergleichsgruppe: gleiches Tool mit gleichen Argumenten ab BaselineCalls Aufrufen, sonst
// das Tool ab BaselineCalls, sonst 0 (nicht bewerten).
func (s *scope) baseline(tool, args string) float64 {
	t, ok := s.Tools[tool]
	if !ok {
		return 0
	}
	if d, ok := t.ArgDur[args]; ok && d.Calls >= BaselineCalls {
		return d.p(0.95)
	}
	if t.Calls >= BaselineCalls {
		return t.p(0.95)
	}
	return 0
}

func (s *scope) add(e Event, args string, slow SlowCall, outlier bool) {
	t, ok := s.Tools[e.Tool]
	if !ok {
		t = &toolAgg{Args: map[string]int{}, ArgDur: map[string]*durAgg{}, ErrorMsgs: map[string]int{}}
		s.Tools[e.Tool] = t
	}
	t.add(e.DurationMs)
	key := bump(t.Args, args)
	if t.ArgDur[key] == nil {
		t.ArgDur[key] = &durAgg{}
	}
	t.ArgDur[key].add(e.DurationMs)
	if !e.OK {
		t.Errors++
		bump(t.ErrorMsgs, clip(e.Error))
	}
	if outlier {
		t.Outliers++
		s.Outliers = append([]SlowCall{slow}, s.Outliers...)[:min(len(s.Outliers)+1, slowN)]
	}
	s.Slowest = insertSlowest(s.Slowest, slow)
}

// insertSlowest hält die slowN langsamsten Aufrufe, längste zuerst.
func insertSlowest(list []SlowCall, c SlowCall) []SlowCall {
	i := sort.Search(len(list), func(i int) bool { return list[i].DurationMs < c.DurationMs })
	if i >= slowN {
		return list
	}
	list = append(list, SlowCall{})
	copy(list[i+1:], list[i:])
	list[i] = c
	return list[:min(len(list), slowN)]
}
