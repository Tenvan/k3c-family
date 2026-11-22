package mcpkit

import (
	"strings"
	"testing"
	"time"
)

// clock ist eine gestellte Uhr, die bei jedem Ablesen um step weiterläuft.
type clock struct {
	t    time.Time
	step time.Duration
}

func (c *clock) now() time.Time {
	c.t = c.t.Add(c.step)
	return c.t
}

func testStats() *Stats {
	c := &clock{t: time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC), step: 100 * time.Millisecond}
	st := NewStats(c.now, nil, nil, nil)
	st.Register("a", "Tool A")
	st.Register("b", "Tool B")
	return st
}

func TestZaehlerJeTool(t *testing.T) {
	t.Parallel()
	st := testStats()
	st.End(st.Begin("a", `{}`), Outcome{OK: true})
	st.End(st.Begin("a", `{}`), Outcome{Err: "kaputt"})
	st.End(st.Begin("fremd", `{}`), Outcome{OK: true})
	snap := st.Snapshot()
	if snap.TotalCalls != 3 || snap.Errors != 1 {
		t.Errorf("Summen: %+v", snap)
	}
	a, b := snap.Tools[0], snap.Tools[1]
	if a.Calls != 2 || a.Errors != 1 || a.AvgMs != 100 || a.LastCall == "" || b.Calls != 0 || len(snap.Tools) != 2 {
		t.Errorf("Tools: %+v", snap.Tools)
	}
}

// Der Zeitpunkt des letzten Fehlers je Tool: leer ohne Fehler, sonst der des jüngsten fehlgeschlagenen Aufrufs.
func TestLetzterFehlerJeTool(t *testing.T) {
	t.Parallel()
	st := testStats()
	st.End(st.Begin("a", `{}`), Outcome{Err: "erster"})
	st.End(st.Begin("a", `{}`), Outcome{OK: true})
	st.End(st.Begin("b", `{}`), Outcome{OK: true})
	first := st.Snapshot().Tools[0].LastError
	st.End(st.Begin("a", `{}`), Outcome{Err: "zweiter"})
	snap := st.Snapshot()
	if first == "" || snap.Tools[0].LastError <= first {
		t.Errorf("letzter Fehler von a: %q nach %q", snap.Tools[0].LastError, first)
	}
	if snap.Tools[1].LastError != "" {
		t.Errorf("b ohne Fehler hat LastError %q", snap.Tools[1].LastError)
	}
}

func TestParallelUndFolge(t *testing.T) {
	t.Parallel()
	st := testStats()
	first := st.Begin("a", "")
	second := st.Begin("b", "")
	if st.Snapshot().PeakInFlight != 2 {
		t.Errorf("parallel max: %+v", st.Snapshot())
	}
	running := st.Calls()
	if len(running) != 2 || !running[0].Running || running[0].Tool != "b" {
		t.Errorf("laufende Aufrufe: %+v", running)
	}
	st.End(first, Outcome{OK: true})
	st.End(second, Outcome{OK: true})
	calls := st.Calls()
	b, a := calls[0], calls[1]
	// Folge: a startet (1), b startet (2), a endet (3), b endet (4).
	if a.StartSeq != 1 || b.StartSeq != 2 || a.EndSeq != 3 || b.EndSeq != 4 || st.Snapshot().InFlight != 0 {
		t.Errorf("Folge: a=%+v b=%+v", a, b)
	}
}

func TestRingHaelt200NeuesteZuerst(t *testing.T) {
	t.Parallel()
	st := testStats()
	for range callRingSize + 5 {
		st.End(st.Begin("a", ""), Outcome{OK: true})
	}
	calls := st.Calls()
	if len(calls) != callRingSize || calls[0].ID != callRingSize+5 || calls[len(calls)-1].ID != 6 {
		t.Errorf("Ring: %d Einträge, erster %d, letzter %d", len(calls), calls[0].ID, calls[len(calls)-1].ID)
	}
	seen := map[string]bool{}
	for _, c := range calls {
		if seen[c.Ref] || len(c.Ref) != 7 {
			t.Fatalf("ref %q doppelt oder falsch lang", c.Ref)
		}
		seen[c.Ref] = true
	}
}

func TestRueckrufeUndKuerzen(t *testing.T) {
	t.Parallel()
	st := testStats()
	var started, ended []Call
	st.onStart = func(c Call) { started = append(started, c) }
	st.onCall = func(c Call) { ended = append(ended, c) }
	st.End(st.Begin("a", strings.Repeat("x", 500)), Outcome{OK: true, Summary: "fertig"})
	if len(started) != 1 || len(ended) != 1 || ended[0].Summary != "fertig" || !started[0].Running {
		t.Errorf("Rückrufe: %+v %+v", started, ended)
	}
	if n := len([]rune(ended[0].Args)); n != TextRunes {
		t.Errorf("Argumente nicht gekürzt: %d Zeichen", n)
	}
	st.End(99, Outcome{OK: true}) // unbekannte ID: nichts passiert
	if st.Snapshot().TotalCalls != 1 {
		t.Error("unbekannte ID gezählt")
	}
}
