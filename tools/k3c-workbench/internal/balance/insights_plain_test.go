package balance

import (
	"strings"
	"testing"
)

func fp(x float64) *float64 { return &x }

// plainTarget baut ein Ziel samt Eintrag, wie BuildInsights es für den Bericht bildet.
func plainTarget(measure, name, kind string, lo, hi *float64, value float64, runs int) TargetInsight {
	e := Entry{Verdict: Verdict{Target: Target{Measure: measure, Name: name, Kind: kind, Lower: lo, Upper: hi}, Value: value}}
	if kind == KindMedian {
		e.Status = StatusOK
	}
	switch {
	case lo != nil && value < *lo, hi != nil && value > *hi:
		e.Status = StatusViolated
	default:
		e.Status = StatusOK
	}
	t := targetInsight(e, &Hints{Levers: map[string]Levers{}})
	t.Runs = runs
	return t
}

// Das Beispiel aus der Beschwerde: Burg hält 100 % bei Korridor 75–90 %.
func TestPlainAnteilZuHoch(t *testing.T) {
	g := plainTarget("castleHeld", "Burg hält Nacht 1–5", KindShare, fp(75), fp(90), 100, 5)
	want := "In 5 von 5 Läufen hielt die Burg bis zum Ende (100 %). Sollwert: 75–90 % der Läufe. Damit 10 Prozentpunkte über dem Korridor, also zu leicht."
	if got := g.plain(); got != want {
		t.Errorf("plain:\n%s\nwill:\n%s", got, want)
	}
	if got := g.gapText(); got != "10 Pkt. zu hoch" {
		t.Errorf("Abstand: %q", got)
	}
}

// Der Hinweis „0,0 % bei Korridor 90–100 %, Abstand −900 %“ wird lesbar.
func TestPlainAnteilZuNiedrig(t *testing.T) {
	g := plainTarget("castleHeld", "Burg hält Nacht 1–5", KindShare, fp(90), fp(100), 0, 5)
	got := g.plain()
	for _, w := range []string{"In 0 von 5 Läufen", "90–100 % der Läufe", "90 Prozentpunkte unter dem Korridor, also zu schwer."} {
		if !strings.Contains(got, w) {
			t.Errorf("%q fehlt in %q", w, got)
		}
	}
	if strings.Contains(got, "900") {
		t.Errorf("Abstand in %% der Korridorbreite darf nicht mehr vorkommen: %q", got)
	}
}

func TestPlainMedianUndImKorridor(t *testing.T) {
	m := plainTarget("destroyedPerWave", "Zerstörte Gebäude je Welle", KindMedian, nil, fp(1), 2, 5)
	if got := m.plain(); !strings.Contains(got, "2 Gebäude") || !strings.Contains(got, "höchstens 1.") || !strings.Contains(got, "1 Gebäude über dem Korridor, also zu schwer") {
		t.Errorf("Median: %q", got)
	}
	ok := plainTarget("castleHeld", "Burg", KindShare, fp(75), fp(90), 80, 5)
	if got := ok.plain(); !strings.Contains(got, "Im Korridor, 5 Prozentpunkte von der unteren Grenze entfernt.") {
		t.Errorf("im Korridor: %q", got)
	}
	if got := ok.gapText(); got != "5 Pkt. Reserve" {
		t.Errorf("Reserve: %q", got)
	}
}

func TestLeverRange(t *testing.T) {
	cases := map[string]struct {
		base          float64
		isInt         bool
		dir           string
		step          float64
		start, lo, hi float64
		ok            bool
	}{
		"senken, Kommazahl":  {100, false, "-", 0.1, 90, 70, 100, true},
		"erhöhen, Kommazahl": {10, false, "+", 0.2, 12, 10, 16, true},
		"ganze Zahl rundet":  {14, true, "+", 0.1, 16, 14, 19, true},
		"kleine ganze Zahl":  {2, true, "-", 0.1, 1, 1, 2, true},
		"Wert 0":             {0, false, "+", 0.1, 0, 0, 0, false},
	}
	for name, c := range cases {
		start, lo, hi, ok := leverRange(c.base, c.isInt, c.dir, c.step)
		if ok != c.ok || (ok && (start != c.start || lo != c.lo || hi != c.hi)) {
			t.Errorf("%s: %v %v %v %v", name, start, lo, hi, ok)
		}
	}
}

func TestPlanText(t *testing.T) {
	g := plainTarget("castleHeld", "Burg hält Nacht 1–5", KindShare, fp(75), fp(90), 100, 5)
	l := Lever{Path: "x.json:a", Direction: "-", Step: 0.1, Label: "Wert senken"}
	p := g.planFor(l, 100, false, true)
	for _, w := range []string{"Aktuell 100.", "Probierbereich 70 … 100 (1 bis 3 Schritte à 10 % runter)", "Startvorschlag: 90.",
		"Wirkung: „Burg hält Nacht 1–5“ soll von 100,0 % Richtung Korridor 75–90 % sinken", "nicht berechenbar"} {
		if !strings.Contains(p.Text, w) {
			t.Errorf("%q fehlt in %q", w, p.Text)
		}
	}
	if !p.Known || p.Start != "90" || p.Min != "70" || p.Max != "100" {
		t.Errorf("Plan: %+v", p)
	}
	z := g.planFor(l, 0, false, true)
	if z.Known || !strings.Contains(z.Text, "nicht seriös abschätzbar") {
		t.Errorf("Wert 0: %+v", z)
	}
	if n := g.planFor(l, 0, false, false); n.Known || !strings.Contains(n.Text, "nicht lesbar") {
		t.Errorf("nicht lesbar: %+v", n)
	}
}

// Derselbe Wert bei mehreren Verletzungen steht einmal, mit allen Verletzungen; Gegenrichtung wird gemeldet.
func TestLeverGroupsFassenZusammen(t *testing.T) {
	a := plainTarget("castleHeld", "Ziel A", KindShare, fp(75), fp(90), 100, 5)
	b := plainTarget("endbossDefeated", "Ziel B", KindShare, fp(75), fp(90), 100, 5)
	c := plainTarget("caveBeforeDay6", "Ziel C", KindShare, fp(75), fp(90), 10, 5)
	down := Lever{Path: "x.json:a", Direction: "-", Step: 0.1, Label: "A senken"}
	up := Lever{Path: "x.json:a", Direction: "+", Step: 0.1, Label: "A erhöhen"}
	other := Lever{Path: "y.json:b", Direction: "-", Step: 0.1, Label: "B senken"}
	for i, g := range []*TargetInsight{&a, &b, &c} {
		g.Levers = []Lever{down}
		if i == 1 {
			g.Levers = []Lever{other, down}
		}
		if i == 2 {
			g.Levers = []Lever{up}
		}
		g.Plans = make([]LeverPlan, len(g.Levers))
		for j, l := range g.Levers {
			g.Plans[j] = LeverPlan{Path: l.Path, Label: l.Label, Short: "Kurz."}
		}
	}
	groups := leverGroups(Insights{Targets: []TargetInsight{a, b, c}})
	if len(groups) != 3 {
		t.Fatalf("Gruppen: %+v", groups)
	}
	first := groups[0]
	if first.Path != "x.json:a" || first.Direction != "-" || len(first.Targets) != 2 || first.Conflict == "" {
		t.Errorf("erste Gruppe: %+v", first)
	}
	if !strings.Contains(first.Text, "soll 2 Verletzung(en) verbessern: Ziel A; Ziel B") || !strings.Contains(first.Text, "Gegenrichtung") {
		t.Errorf("Text: %q", first.Text)
	}
	if groups[2].Conflict != "" && groups[2].Path == "y.json:b" {
		t.Errorf("y.json:b hat keinen Konflikt: %+v", groups[2])
	}
}

// Ein Kopierblock fasst alle Startvorschläge zusammen; Gegenrichtung und unbekannte Startwerte stehen nur als Kommentar.
func TestRecommendationsEinBlock(t *testing.T) {
	groups := []LeverGroup{
		{Path: "x.json:a", Direction: "-", Targets: []string{"Ziel A", "Ziel B"}, Plan: LeverPlan{Label: "A senken", Known: true, Current: "100", Min: "70", Max: "100", Start: "90"}},
		{Path: "x.json:a", Direction: "+", Targets: []string{"Ziel C"}, Plan: LeverPlan{Known: true, Start: "110"}},
		{Path: "y.json:b", Direction: "+", Targets: []string{"Ziel D"}, Plan: LeverPlan{Label: "B erhöhen"}},
	}
	c, ok := recommendations(groups)
	if !ok || c.Where != WhereChat {
		t.Fatalf("Befehl: %+v %v", c, ok)
	}
	for _, w := range []string{"x.json:a=90  # A senken; aktuell 100, Bereich 70 … 100; verbessert: Ziel A; Ziel B", "# x.json:a: Gegenrichtung", "# y.json:b: Startwert nicht seriös"} {
		if !strings.Contains(c.Text, w) {
			t.Errorf("%q fehlt in\n%s", w, c.Text)
		}
	}
	if strings.Contains(c.Text, "=110") {
		t.Errorf("Gegenrichtung darf keinen Wert liefern:\n%s", c.Text)
	}
	if _, ok := recommendations(nil); ok {
		t.Error("ohne Stellschrauben kein Block")
	}
}
