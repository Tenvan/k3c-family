package balance

import (
	"fmt"
	"math"
)

// Auswertung eines Balancing-Berichts ohne KI (B-632): Ampel je Ziel und Kennzahl, Stärken, Schwächen,
// Tuning-Hinweise, Kurzauswertung und Vorschlag für den nächsten Lauf. Eingang ist ein Summary (auch ein altes ohne
// Kennzahlen) samt optionaler Baseline; Richtwerte und Hebel aus data/balance-hints.json (hints.go).

// Ampeln: grün ok, gelb knapp, rot verletzt, grau ungültig bzw. ohne Messwert.
const (
	LightGreen  = "green"
	LightYellow = "yellow"
	LightRed    = "red"
	LightGrey   = "grey"
)

// Insights ist die Auswertung; das JSON speist den HTML-Bericht (report/*.js) und die Workbench (ReportView).
type Insights struct {
	Title      string          `json:"title"`
	Run        string          `json:"run"`
	Generated  string          `json:"generated"`
	Seeds      int             `json:"seeds"`
	Pass       bool            `json:"pass"`
	Counts     map[string]int  `json:"counts"`
	Summary    []string        `json:"summary"`
	Strengths  []string        `json:"strengths"`
	Weaknesses []string        `json:"weaknesses"`
	Targets    []TargetInsight `json:"targets"`
	Figures    FigureInsights  `json:"figures"`
	Hints      []Hint          `json:"hints"`
	Next       NextRun         `json:"next"`
	Baseline   BaselineView    `json:"baseline"`
	Varianten  []string        `json:"varianten"`
}

// TargetInsight ist ein Ziel mit Ampel. Distance: Abstand zur nächsten Grenze in % der Korridorbreite, positiv im
// Korridor, negativ außerhalb; nil ohne Messwert.
type TargetInsight struct {
	Name         string   `json:"name"`
	Measure      string   `json:"measure"`
	Kind         string   `json:"kind"`
	Unit         string   `json:"unit"`
	Grade        string   `json:"grade"`
	Scenario     string   `json:"scenario"`
	Value        float64  `json:"value"`
	Lower        *float64 `json:"lower"`
	Upper        *float64 `json:"upper"`
	Status       string   `json:"status"`
	Light        string   `json:"light"`
	Distance     *float64 `json:"distance"`
	Trend        *Trend   `json:"trend"`
	FailSeeds    []string `json:"failSeeds"`
	InvalidSeeds []string `json:"invalidSeeds"`
	Rule         string   `json:"rule"`
	Levers       []Lever  `json:"levers"`
	entry        Entry
}

// Trend ist der Vergleich mit der Baseline: Better, wenn der Abstand zum Korridor größer (bzw. die Verletzung kleiner) wurde.
type Trend struct {
	Before       float64 `json:"before"`
	Delta        float64 `json:"delta"`
	Better       bool    `json:"better"`
	BeforeStatus string  `json:"beforeStatus"`
}

// BaselineView ist der Vergleich mit der Baseline für die Anzeige.
type BaselineView struct {
	Present bool     `json:"present"`
	Note    string   `json:"note"`
	Changes []Change `json:"changes"`
}

// Meta beschreibt den Lauf für Kopf und Befehle: Ordner (simtest-run-N), Zeit und Name der Baseline.
type Meta struct {
	Run       string
	Generated string
	Baseline  string
}

// BuildInsights wertet einen Bericht aus; base == nil: ohne Trend und Vergleich.
func BuildInsights(s Summary, base *Summary, h *Hints, m Meta) Insights {
	in := Insights{Title: m.Run, Run: m.Run, Generated: m.Generated, Seeds: s.Seeds, Pass: s.Pass, Varianten: s.Varianten,
		Counts: map[string]int{LightGreen: 0, LightYellow: 0, LightRed: 0, LightGrey: 0}, Targets: []TargetInsight{}}
	if in.Varianten == nil {
		in.Varianten = []string{}
	}
	before := map[string]Entry{}
	if base != nil {
		for _, e := range base.Targets {
			before[e.targetKey()] = e
		}
	}
	for _, e := range s.Targets {
		t := targetInsight(e, h)
		if b, ok := before[e.targetKey()]; ok {
			t.Trend = trend(b, e)
		}
		in.Counts[t.Light]++
		in.Targets = append(in.Targets, t)
	}
	in.Figures = figureInsights(s.Kennzahlen, h)
	in.Baseline = baselineView(s, base, m.Baseline)
	in.Strengths, in.Weaknesses = strengths(in), weaknesses(in)
	in.Hints = hints(in)
	in.Next = nextRun(in, s)
	in.Summary = summary(in)
	return in
}

func targetInsight(e Entry, h *Hints) TargetInsight {
	t := TargetInsight{Name: e.Name, Measure: e.Measure, Kind: e.Kind, Unit: unitOf(e.Kind), Grade: gradeName(e.Grade),
		Scenario: fmt.Sprintf("%d Spieler · %s · Tiefe %d · %d Tage", e.Players, e.Bot, e.Depth, e.Days),
		Value: e.Value, Lower: e.Lower, Upper: e.Upper, Status: e.Status, Light: lightOf(e.Status),
		FailSeeds: e.FailSeeds, InvalidSeeds: e.InvalidSeeds, Rule: e.Rule, Levers: []Lever{}, entry: e}
	if e.Status != StatusInvalid {
		d := distance(e)
		t.Distance = &d
		if e.Status != StatusOK {
			t.Levers = h.leversFor(e)
		}
	}
	return t
}

func lightOf(status string) string {
	switch status {
	case StatusOK:
		return LightGreen
	case StatusNarrow:
		return LightYellow
	case StatusViolated:
		return LightRed
	}
	return LightGrey
}

func unitOf(kind string) string {
	if kind == KindShare {
		return "%"
	}
	return ""
}

func gradeName(g string) string {
	if g == "" {
		return "normal"
	}
	return g
}

// distance: Abstand zur nächsten Grenze in % der Korridorbreite (eine Grenze: deren Betrag, mindestens 1).
// Außerhalb negativ, gemessen an der verletzten Grenze; im Korridor an der nächsten nicht natürlichen Grenze (bounds).
func distance(e Entry) float64 {
	width := 1.0
	switch {
	case e.Lower != nil && e.Upper != nil:
		width = math.Max(*e.Upper-*e.Lower, 1) // ein Korridor ohne Breite (100–100) gäbe sonst Abstände um 10^13 %
	case e.Lower != nil:
		width = math.Max(math.Abs(*e.Lower), 1)
	case e.Upper != nil:
		width = math.Max(math.Abs(*e.Upper), 1)
	}
	v := e.Value
	switch {
	case e.Lower != nil && v < *e.Lower:
		return round1(100 * (v - *e.Lower) / width)
	case e.Upper != nil && v > *e.Upper:
		return round1(100 * (*e.Upper - v) / width)
	}
	lo, hi := e.bounds()
	best := math.Inf(1)
	for _, b := range []*float64{lo, hi} {
		if b != nil {
			best = math.Min(best, math.Abs(v-*b))
		}
	}
	if math.IsInf(best, 1) {
		return 100
	}
	return round1(100 * best / width)
}

func trend(b, e Entry) *Trend {
	if b.Status == StatusInvalid || e.Status == StatusInvalid {
		return nil
	}
	return &Trend{Before: b.Value, Delta: round1(e.Value - b.Value), Better: distance(e) >= distance(b), BeforeStatus: b.Status}
}

func baselineView(s Summary, base *Summary, name string) BaselineView {
	if base == nil {
		return BaselineView{Note: "Keine Baseline: kein Trend.", Changes: []Change{}}
	}
	v := BaselineView{Present: true, Note: "Baseline: " + name, Changes: Compare(*base, s)}
	if v.Changes == nil {
		v.Changes = []Change{}
	}
	if base.Seeds != s.Seeds {
		v.Note += fmt.Sprintf(" (%d Seeds, dieser Lauf %d)", base.Seeds, s.Seeds)
	}
	return v
}
