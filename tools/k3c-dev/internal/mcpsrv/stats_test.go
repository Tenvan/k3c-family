package mcpsrv

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

func testStats() *stats {
	c := &clock{t: time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC), step: 100 * time.Millisecond}
	st := newStats(c.now)
	st.register("a", "Tool A")
	st.register("b", "Tool B")
	return st
}

func TestZaehlerJeTool(t *testing.T) {
	st := testStats()
	st.end(st.begin("a", `{}`), outcome{ok: true})
	st.end(st.begin("a", `{}`), outcome{err: "kaputt"})
	st.end(st.begin("fremd", `{}`), outcome{ok: true})
	snap := st.snapshot()
	if snap.TotalCalls != 3 || snap.Errors != 1 {
		t.Errorf("Summen: %+v", snap)
	}
	a, b := snap.Tools[0], snap.Tools[1]
	if a.Calls != 2 || a.Errors != 1 || a.AvgMs != 100 || a.LastCall == "" || b.Calls != 0 || len(snap.Tools) != 2 {
		t.Errorf("Tools: %+v", snap.Tools)
	}
}

func TestParallelUndFolge(t *testing.T) {
	st := testStats()
	first := st.begin("a", "")
	second := st.begin("b", "")
	if st.snapshot().PeakInFlight != 2 {
		t.Errorf("parallel max: %+v", st.snapshot())
	}
	running := st.calls()
	if len(running) != 2 || !running[0].Running || running[0].Tool != "b" {
		t.Errorf("laufende Aufrufe: %+v", running)
	}
	st.end(first, outcome{ok: true})
	st.end(second, outcome{ok: true})
	calls := st.calls()
	b, a := calls[0], calls[1]
	// Folge: a startet (1), b startet (2), a endet (3), b endet (4).
	if a.StartSeq != 1 || b.StartSeq != 2 || a.EndSeq != 3 || b.EndSeq != 4 || st.snapshot().InFlight != 0 {
		t.Errorf("Folge: a=%+v b=%+v", a, b)
	}
}

func TestRingHaelt200NeuesteZuerst(t *testing.T) {
	st := testStats()
	for range callRingSize + 5 {
		st.end(st.begin("a", ""), outcome{ok: true})
	}
	calls := st.calls()
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
	st := testStats()
	var started, ended []Call
	st.onStart = func(c Call) { started = append(started, c) }
	st.onCall = func(c Call) { ended = append(ended, c) }
	st.end(st.begin("a", strings.Repeat("x", 500)), outcome{ok: true, summary: "fertig"})
	if len(started) != 1 || len(ended) != 1 || ended[0].Summary != "fertig" || !started[0].Running {
		t.Errorf("Rückrufe: %+v %+v", started, ended)
	}
	if n := len([]rune(ended[0].Args)); n != textRunes {
		t.Errorf("Argumente nicht gekürzt: %d Zeichen", n)
	}
	st.end(99, outcome{ok: true}) // unbekannte ID: nichts passiert
	if st.snapshot().TotalCalls != 1 {
		t.Error("unbekannte ID gezählt")
	}
}

func TestFormatUptime(t *testing.T) {
	cases := map[time.Duration]string{
		30 * time.Second: "< 1 min", 13 * time.Minute: "13 min",
		133 * time.Minute: "2 h 13 min", 76 * time.Hour: "3 T 4 h",
	}
	for d, want := range cases {
		if got := formatUptime(d); got != want {
			t.Errorf("formatUptime(%v) = %q, erwartet %q", d, got, want)
		}
	}
}
