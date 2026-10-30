package balance

import (
	"fmt"
	"math"
	"slices"
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
	LeverGroups []LeverGroup   `json:"leverGroups"` // Stellschrauben nach Wirkung zusammengefasst (insights_levers.go)
	Next       NextRun         `json:"next"`
	Baseline   BaselineView    `json:"baseline"`
	Delta      []GroupDelta    `json:"delta"` // Vergleich je Werte-Gruppe (delta.go), leer ohne Baseline
	Varianten  []string        `json:"varianten"`
	// B-648: Urteil über das Ziel, Qualität der Korridore, Spielfortschritt (Leiter mit Pfeilen) und der Lauf, mit dem verglichen wurde.
	Goal     *GoalView    `json:"goal"`
	Quality  *QualityView `json:"quality"`
	Progress *Progress    `json:"progress"`
	Previous string       `json:"previous"`
	GoalText string       `json:"goalText"`
}

// TargetInsight ist ein Ziel mit Ampel. Distance: Abstand zur nächsten Grenze in % der Korridorbreite, positiv im
// Korridor, negativ außerhalb; nil ohne Messwert.
type TargetInsight struct {
	Name         string   `json:"name"`
	Group        string   `json:"group"`
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
	// Klartext (insights_plain.go): Runs = gespielte Läufe, Plain = Messwert, Sollwert und Lage in Sätzen, Gap = Spalte
	// „Abstand“ („10 Pkt. zu hoch“), TrendText = Vergleich mit der Baseline, Plans = Zahlen zu jedem Hebel.
	Runs      int         `json:"runs"`
	Plain     string      `json:"plain"`
	Gap       string      `json:"gap"`
	TrendText string      `json:"trendText"`
	Plans     []LeverPlan `json:"plans"`
	entry     Entry
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
	Frozen    []string // eingefrorene Werte-Gruppen: keine Hebel, Delta nur beim Kippen
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
		t.Levers = freeLevers(t.Levers, m.Frozen)
		t.Runs, t.Plain, t.Gap, t.TrendText, t.Plans = s.Seeds, t.plain(), t.gapText(), t.trendPlain(), t.plans()
		in.Counts[t.Light]++
		in.Targets = append(in.Targets, t)
	}
	in.Figures = figureInsights(s.Kennzahlen, h)
	in.Baseline = baselineView(s, base, m.Baseline)
	in.Delta = []GroupDelta{}
	if base != nil {
		in.Delta = buildDelta(in.Targets, m.Frozen)
	}
	in.Strengths, in.Weaknesses = strengths(in), weaknesses(in)
	in.Goal, in.Quality = s.Goal, s.Quality
	if s.Progress != nil {
		p := *s.Progress
		p.Ladder = slices.Clone(p.Ladder)
		in.Progress = &p
	}
	in.GoalText = goalText(in)
	in.Hints = hints(in)
	in.LeverGroups = leverGroups(in)
	in.Next = nextRun(in, s)
	in.Summary = summary(in)
	return in
}

func targetInsight(e Entry, h *Hints) TargetInsight {
	t := TargetInsight{Name: e.Name, Group: e.Group, Measure: e.Measure, Kind: e.Kind, Unit: unitOf(e.Kind), Grade: gradeName(e.Grade),
		Scenario: fmt.Sprintf("%d Spieler · %s · Tiefe %d · %d Tage", e.Players, e.Bot, e.Depth, e.Days),
		Value:    e.Value, Lower: e.Lower, Upper: e.Upper, Status: e.Status, Light: lightOf(e.Status),
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
		width = *e.Upper - *e.Lower
		if width < 1 { // ein Korridor ohne Breite (100–100) misst am Betrag der Grenze statt durch null zu teilen
			width = math.Max(math.Abs(*e.Upper), 1)
		}
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
