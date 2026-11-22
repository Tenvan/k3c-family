package usage

import (
	"encoding/json"
	"fmt"
	"math"
	"strings"
	"testing"
	"time"
)

// clock ist eine gestellte Uhr.
type clock struct{ t time.Time }

func (c *clock) now() time.Time { return c.t }

func newTracker() (*Tracker, *clock) {
	c := &clock{t: time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC)}
	return New(c.now), c
}

func (c *clock) event(tool, args string, ms float64) Event {
	return Event{At: c.t, Tool: tool, Args: args, DurationMs: ms, OK: true}
}

func TestPerzentileAusHistogramm(t *testing.T) {
	var d durAgg
	for i := 1; i <= 1000; i++ {
		d.add(float64(i))
	}
	for q, want := range map[float64]float64{0.5: 500, 0.95: 950} {
		if got := d.p(q); math.Abs(got-want)/want > 0.12 {
			t.Errorf("p%.0f = %.1f, echter Wert %.0f (über 12 %% daneben)", q*100, got, want)
		}
	}
	var one durAgg
	one.add(1234)
	if got := one.p(0.95); got > 1234 {
		t.Errorf("Perzentil %.1f über dem Maximum 1234", got)
	}
	var tiny durAgg
	tiny.add(0.3)
	if got := tiny.p(0.5); got != 0.3 {
		t.Errorf("unter 1 ms: %.2f", got)
	}
}

// outliers zählt die Ausreißer eines Tools in der Gesamtzeit.
func outliers(tr *Tracker, tool string) int {
	for _, u := range tr.Snapshot().AllTime.Tools {
		if u.Name == tool {
			return u.Outliers
		}
	}
	return -1
}

func TestAusreisserRegel(t *testing.T) {
	cases := []struct {
		name string
		fill func(tr *Tracker, c *clock)
		last Event
		want int
	}{
		{"Beispiel logs_query", func(tr *Tracker, c *clock) { repeat(tr, c.event("q", `{}`, 100), 20) },
			Event{Tool: "q", Args: `{}`, DurationMs: 1500, OK: true}, 1},
		{"Beispiel check_run unter 2× p95", func(tr *Tracker, c *clock) { repeat(tr, c.event("c", `{"target":"task:check"}`, 40_000), 20) },
			Event{Tool: "c", Args: `{"target":"task:check"}`, DurationMs: 50_000, OK: true}, 0},
		{"unter 1 s", func(tr *Tracker, c *clock) { repeat(tr, c.event("q", `{}`, 100), 20) },
			Event{Tool: "q", Args: `{}`, DurationMs: 900, OK: true}, 0},
		{"erst ab 8 Aufrufen", func(tr *Tracker, c *clock) { repeat(tr, c.event("q", `{}`, 100), 7) },
			Event{Tool: "q", Args: `{}`, DurationMs: 5000, OK: true}, 0},
		{"Vergleichswert vor dem Eintragen", func(tr *Tracker, c *clock) { repeat(tr, c.event("q", `{}`, 100), 8) },
			Event{Tool: "q", Args: `{}`, DurationMs: 5000, OK: true}, 1},
		{"Argumentgruppe vor Toolgruppe", func(tr *Tracker, c *clock) {
			repeat(tr, c.event("c", `{"target":"a"}`, 100), 200)
			repeat(tr, c.event("c", `{"target":"b"}`, 2000), 8)
		}, Event{Tool: "c", Args: `{"target":"b"}`, DurationMs: 3000, OK: true}, 0},
		{"Toolgruppe ohne Argumentgruppe", func(tr *Tracker, c *clock) { repeat(tr, c.event("c", `{"target":"a"}`, 100), 200) },
			Event{Tool: "c", Args: `{"target":"neu"}`, DurationMs: 3000, OK: true}, 1},
	}
	for _, tc := range cases {
		tr, c := newTracker()
		tc.fill(tr, c)
		before := outliers(tr, tc.last.Tool)
		tc.last.At = c.t
		tr.Record(tc.last)
		if got := outliers(tr, tc.last.Tool) - max(before, 0); got != tc.want {
			t.Errorf("%s: %d Ausreißer, erwartet %d", tc.name, got, tc.want)
		}
	}
}

func repeat(tr *Tracker, e Event, n int) {
	for range n {
		tr.Record(e)
	}
}

func TestRanglistenUndDeckel(t *testing.T) {
	tr, c := newTracker()
	repeat(tr, c.event("q", `{"b":1,"a":2}`, 10), 8) // Schlüssel werden sortiert
	fail := c.event("q", `{}`, 5)
	fail.OK, fail.Error = false, "Ziel unbekannt"
	repeat(tr, fail, 2)
	for i := range maxValues + 5 { // 48 passen noch, 7 zählen unter (weitere)
		tr.Record(c.event("q", fmt.Sprintf(`{"n":%d}`, i), float64(i+1)))
	}
	s := tr.Snapshot().Session
	q := s.Tools[0]
	if q.Args[0].Value != `{"a":2,"b":1}` || q.Args[1].Value != overflowKey || q.Args[1].Count != 7 || len(q.Args) != topN {
		t.Errorf("Argumente: %+v", q.Args[:2])
	}
	if len(q.TopErrors) != 1 || q.TopErrors[0].Count != 2 || s.TopErrors[0].Tool != "q" || s.Errors != 2 {
		t.Errorf("Fehler: %+v / %+v", q.TopErrors, s.TopErrors)
	}
	if len(s.Slowest) != slowN || s.Slowest[0].DurationMs != 55 || s.Slowest[1].DurationMs != 54 {
		t.Errorf("Langsamste: %+v", s.Slowest[:2])
	}
}

func TestLetzteAusreisserNeuesteZuerst(t *testing.T) {
	tr2, c2 := newTracker()
	repeat(tr2, c2.event("q", `{}`, 100), 300) // groß genug, dass 12 langsame den p95 nicht heben
	for i := range slowN + 2 {
		tr2.Record(c2.event("q", `{}`, float64(5000+i)))
	}
	if got := tr2.Snapshot().AllTime.RecentOutliers; len(got) != slowN || got[0].DurationMs != 5011 {
		t.Errorf("letzte Ausreißer: %d, erster %.0f", len(got), got[0].DurationMs)
	}
}

func TestZeitreiheSiebenTage(t *testing.T) {
	tr, c := newTracker()
	tr.Record(c.event("q", `{}`, 10))
	c.t = c.t.Add(30 * time.Second)
	tr.Record(c.event("q", `{}`, 30))
	c.t = c.t.Add(2 * time.Minute)
	tr.Record(c.event("c", `{}`, 5))
	m := tr.Snapshot().Minutes
	if len(m) != 2 || m[0].Calls["q"] != 2 || m[0].Ms["q"] != 40 || m[0].MaxMs["q"] != 30 || m[1].Calls["c"] != 1 {
		t.Fatalf("Minuten: %+v", m)
	}
	if m[0].TS != time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC).UnixMilli() {
		t.Errorf("Zeitstempel %d", m[0].TS)
	}
	c.t = c.t.Add(SeriesSpan - time.Minute)
	if m = tr.Snapshot().Minutes; len(m) != 1 || m[0].Calls["c"] != 1 {
		t.Errorf("nach 7 Tagen: %+v", m)
	}
}

func TestSchnappschussAlsJSON(t *testing.T) {
	tr, _ := newTracker()
	b, err := json.Marshal(tr.Snapshot())
	if err != nil {
		t.Fatal(err)
	}
	text := string(b)
	for _, key := range []string{`"session":`, `"allTime":`, `"minutes":[]`, `"tools":[]`, `"topErrors":[]`,
		`"recentOutliers":[]`, `"slowest":[]`, `"p95Ms":0`} {
		if !strings.Contains(text, key) {
			t.Errorf("fehlt %s in %s", key, text)
		}
	}
	if strings.Contains(text, "null") {
		t.Errorf("null in %s", text)
	}
}

func TestMinutenwendeInBeliebigerReihenfolge(t *testing.T) {
	tr, c := newTracker()
	later := c.event("q", `{}`, 5)
	later.At = c.t.Add(time.Minute)
	tr.Record(later)
	tr.Record(c.event("q", `{}`, 5)) // früher, kommt aber später an
	tr.Record(later)
	m := tr.Snapshot().Minutes
	if len(m) != 2 || m[0].TS >= m[1].TS || m[0].Calls["q"] != 1 || m[1].Calls["q"] != 2 {
		t.Errorf("Minuten: %+v", m)
	}
}
