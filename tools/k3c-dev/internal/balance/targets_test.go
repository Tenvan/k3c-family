package balance

import (
	"os"
	"slices"
	"strconv"
	"strings"
	"testing"
)

func f(v float64) *float64 { return &v }

// BAL2/AC-01: Die eingebettete Datei lädt (100 feste Seeds); ein Ziel ohne bekannte Kennzahl ist ein Ladefehler.
func TestZielkorridoreLaden(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	if len(ts.Seeds) != 100 || len(ts.Targets) == 0 {
		t.Fatalf("%d Seeds, %d Ziele", len(ts.Seeds), len(ts.Targets))
	}
	raw, _ := os.ReadFile("../../../../data/balance-targets.json")
	for name, bad := range map[string]string{
		"Kennzahl unbekannt": strings.Replace(string(raw), `"castleHeld"`, `"gibtsNicht"`, 1),
		"Art passt nicht":    strings.Replace(string(raw), `"kind": "share"`, `"kind": "median"`, 1),
		"Bot unbekannt":      strings.Replace(string(raw), `"saver"`, `"nobody"`, 1),
	} {
		if _, err := LoadTargets([]byte(bad)); err == nil {
			t.Errorf("%s: Ladefehler erwartet", name)
		} else if name == "Kennzahl unbekannt" && !strings.Contains(err.Error(), "gibtsNicht") {
			t.Errorf("Fehler nennt die Kennzahl nicht: %v", err)
		}
	}
}

// metricsWith baut Metrics für feste Werte: gehalten oder gefallen, Tick der ersten Mauer, zerstörte Gebäude je Welle.
func metricsWith(fallen bool, wallTick int, destroyed ...int) *Metrics {
	m := &Metrics{Days: []DayMetrics{{Day: 1, DuskTick: ptr(100)}}, FirstWallTick: ptr(wallTick)}
	if fallen {
		m.CastleFallTick = ptr(500)
	}
	for i, d := range destroyed {
		m.Waves = append(m.Waves, WaveMetrics{Wave: i + 1, BuildingsDestroyed: d})
	}
	return m
}

func results(ms ...*Metrics) []Result {
	var out []Result
	for i, m := range ms {
		sc := Scenario{Seed: strconv.Itoa(i + 1), Players: 2, Bot: "saver", Depth: 0, Days: 5}
		out = append(out, Result{Scenario: sc, Valid: m != nil, Metrics: m})
	}
	return out
}

func shareTarget() Target {
	return Target{Measure: "castleHeld", Name: "x", Kind: KindShare, Players: 2, Bot: "saver", Days: 5, Lower: f(70), Upper: f(90), Rule: "r"}
}

// BAL2/AC-02: Anteil im Korridor, knapp und verletzt (Korridor 70–90 %, Rand 5 pp, 20 Seeds).
func TestBewertungAnteil(t *testing.T) {
	ts := &Targets{Seeds: seeds(20)}
	ts.Margin.SharePp = 5
	for _, c := range []struct {
		held   int // Seeds, in denen die Burg hält
		status string
		fail   int // erwartete Anzahl FailSeeds
	}{{16, StatusOK, 0}, {14, StatusNarrow, 0}, {18, StatusNarrow, 0}, {12, StatusViolated, 8}, {19, StatusViolated, 19}} {
		var ms []*Metrics
		for i := range 20 {
			ms = append(ms, metricsWith(i >= c.held, 50))
		}
		ts.Targets = []Target{shareTarget()}
		v := ts.Evaluate(Report{Results: results(ms...)})[0]
		if v.Status != c.status || v.Value != float64(c.held)*5 || len(v.FailSeeds) != c.fail {
			t.Errorf("%d von 20: %s %.0f %% Fail %v, erwartet %s", c.held, v.Status, v.Value, v.FailSeeds, c.status)
		}
	}
}

// Anteil unter der Untergrenze nennt die Seeds, in denen die Burg fiel; darüber die, in denen sie hielt.
func TestFailSeedsAnteil(t *testing.T) {
	ts := &Targets{Seeds: seeds(4), Targets: []Target{shareTarget()}}
	v := ts.Evaluate(Report{Results: results(metricsWith(false, 1), metricsWith(true, 1), metricsWith(true, 1), metricsWith(false, 1))})[0]
	if v.Status != StatusViolated || !slices.Equal(v.FailSeeds, []string{"2", "3"}) {
		t.Errorf("%s %v", v.Status, v.FailSeeds)
	}
	v = ts.Evaluate(Report{Results: results(metricsWith(false, 1), metricsWith(false, 1))})[0]
	if v.Status != StatusViolated || !slices.Equal(v.FailSeeds, []string{"1", "2"}) {
		t.Errorf("100 %% über Obergrenze: %s %v", v.Status, v.FailSeeds)
	}
}

// BAL2/AC-02: Median im Korridor, knapp und verletzt (nur Obergrenze 1, Rand 10 % von |1| = 0,1).
func TestBewertungMedian(t *testing.T) {
	g := Target{Measure: "destroyedPerWave", Name: "y", Kind: KindMedian, Players: 2, Bot: "saver", Days: 5, Upper: f(1), Rule: "r"}
	ts := &Targets{Seeds: seeds(3), Targets: []Target{g}}
	ts.Margin.MedianFraction = 0.1
	for _, c := range []struct {
		destroyed [][]int // je Seed die Zerstörungen je Welle
		value     float64
		status    string
		fail      []string
	}{
		{[][]int{{0, 0}, {0, 1}, {0, 0}}, 0, StatusOK, nil},
		{[][]int{{1, 1}, {1, 1}, {1, 1}}, 1, StatusNarrow, nil},
		{[][]int{{2, 3}, {0, 4}, {1, 1}}, 1.5, StatusViolated, []string{"1", "2"}},
	} {
		var ms []*Metrics
		for _, d := range c.destroyed {
			ms = append(ms, metricsWith(false, 1, d...))
		}
		v := ts.Evaluate(Report{Results: results(ms...)})[0]
		if v.Status != c.status || v.Value != c.value || !slices.Equal(v.FailSeeds, c.fail) {
			t.Errorf("%v: %s %v %v, erwartet %s %v %v", c.destroyed, v.Status, v.Value, v.FailSeeds, c.status, c.value, c.fail)
		}
	}
}

// Ein abgebrochener Lauf macht das Urteil ungültig und nennt den Seed.
func TestBewertungUngueltig(t *testing.T) {
	ts := &Targets{Seeds: seeds(2), Targets: []Target{shareTarget()}}
	v := ts.Evaluate(Report{Results: results(metricsWith(false, 1), nil)})[0]
	if v.Status != StatusInvalid || !slices.Equal(v.InvalidSeeds, []string{"2"}) {
		t.Errorf("%s %v", v.Status, v.InvalidSeeds)
	}
}

// Die Mauer-Kennzahl zählt nur Mauern bis Dämmerungsbeginn von Tag 1.
func TestMauerVorDaemmerung(t *testing.T) {
	m := measures["firstWallBeforeDusk1"].values
	if m(metricsWith(false, 100))[0] != 1 || m(metricsWith(false, 101))[0] != 0 || m(&Metrics{})[0] != 0 {
		t.Error("Grenze bei Dämmerungsbeginn falsch")
	}
}

func seeds(n int) []string {
	var s []string
	for i := 1; i <= n; i++ {
		s = append(s, strconv.Itoa(i))
	}
	return s
}

// Datei und Tabelle in docs/rules/zielkorridore.md nennen dieselben Grenzen (Zeile = Name als Anfang der Kennzahl,
// Szenario genau „Standard“).
func TestGrenzenWieTabelle(t *testing.T) {
	raw, err := os.ReadFile("../../../../docs/rules/zielkorridore.md")
	if err != nil {
		t.Fatal(err)
	}
	var rows [][]string
	for _, line := range strings.Split(string(raw), "\n") {
		if line = strings.TrimSpace(line); strings.HasPrefix(line, "|") {
			rows = append(rows, strings.Split(strings.Trim(line, "|"), "|"))
		}
	}
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	for _, g := range ts.Targets {
		var hit [][]string
		for _, r := range rows {
			if len(r) >= 4 && strings.HasPrefix(strings.TrimSpace(r[0]), g.Name) && strings.TrimSpace(r[1]) == "Standard" {
				hit = append(hit, r)
			}
		}
		if len(hit) != 1 {
			t.Errorf("%q: %d Zeilen in der Tabelle, erwartet 1", g.Name, len(hit))
			continue
		}
		for i, got := range []*float64{g.Lower, g.Upper} {
			if want := tableBound(hit[0][2+i]); !equalBound(got, want) {
				t.Errorf("%q: Grenze %d in der Datei %v, in der Tabelle %v", g.Name, i+1, boundText(got), boundText(want))
			}
		}
	}
}

// tableBound liest „75 %“, „20 Gold“, „1“ oder „–“ (keine Grenze).
func tableBound(cell string) *float64 {
	n, err := strconv.ParseFloat(strings.Fields(strings.ReplaceAll(strings.TrimSpace(cell), ",", "."))[0], 64)
	if err != nil {
		return nil
	}
	return &n
}

func equalBound(a, b *float64) bool {
	return (a == nil && b == nil) || (a != nil && b != nil && *a == *b)
}

func boundText(p *float64) any {
	if p == nil {
		return "–"
	}
	return *p
}
