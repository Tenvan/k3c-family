package balance

import (
	"os"
	"slices"
	"strings"
	"testing"
)

// fixture: echter Bericht simtest-run-7 (13 Ziele, 4 verletzt, 2 knapp, mit Kennzahlen).
func fixture(t *testing.T) Summary {
	t.Helper()
	raw, err := os.ReadFile("testdata/insights-run7.json")
	if err != nil {
		t.Fatal(err)
	}
	s, err := ReadSummary(raw)
	if err != nil {
		t.Fatal(err)
	}
	return s
}

func testHints(t *testing.T) *Hints {
	t.Helper()
	h, err := DefaultHints()
	if err != nil {
		t.Fatal(err)
	}
	return h
}

func find(in Insights, measure, grade string) TargetInsight {
	for _, t := range in.Targets {
		if t.Measure == measure && t.Grade == grade {
			return t
		}
	}
	return TargetInsight{}
}

func run7(t *testing.T) Insights {
	t.Helper()
	return BuildInsights(fixture(t), nil, testHints(t), Meta{Run: "simtest-run-7"})
}

// B-632/AC-01: Ampel je Ziel mit Abstand in % der Korridorbreite und Hebeln in die nötige Richtung.
func TestInsightsAmpeln(t *testing.T) {
	in := run7(t)
	if c := in.Counts; c[LightGreen] != 7 || c[LightYellow] != 2 || c[LightRed] != 4 || c[LightGrey] != 0 {
		t.Errorf("Ampeln: %v", c)
	}
	hub := find(in, "hubLevel2BeforeDay5", "normal")
	if hub.Light != LightRed || hub.Distance == nil || *hub.Distance != -240 || len(hub.Levers) == 0 || hub.Levers[0].Direction != "-" {
		t.Errorf("Hub-Stufe 2: %+v", hub)
	}
	if castle := find(in, "castleHeld", "normal"); castle.Light != LightYellow || *castle.Distance != 33.3 {
		t.Errorf("Burg: %+v", castle)
	}
}

// AC-01: Kurzauswertung (3 bis 6 Sätze), Stärken, Schwächen und Tuning-Hinweise, verletzte Ziele zuerst.
func TestInsightsTexte(t *testing.T) {
	in := run7(t)
	if n := len(in.Summary); n < 3 || n > 6 || !strings.Contains(in.Summary[0], "4 von 13 Zielen verletzt") {
		t.Errorf("Kurzauswertung: %q", in.Summary)
	}
	if len(in.Strengths) == 0 || len(in.Weaknesses) == 0 || !strings.HasPrefix(in.Weaknesses[0], "Hub-Stufe 2") {
		t.Errorf("Stärken %q, Schwächen %q", in.Strengths, in.Weaknesses)
	}
	if len(in.Hints) != 6 || in.Hints[0].Light != LightRed || !strings.Contains(in.Hints[0].Text, "zu niedrig") {
		t.Errorf("Hinweise: %+v", in.Hints)
	}
}

// AC-01: Ampel je Kennzahl und Vorschlag für den nächsten Lauf mit Ziel je Befehl.
func TestInsightsKennzahlenUndNaechsterLauf(t *testing.T) {
	in := run7(t)
	if len(in.Figures.Lights) == 0 || in.Figures.Lights[0].Key != "firstWallTick" || in.Figures.Lights[0].Light != LightGreen {
		t.Errorf("Kennzahl-Ampeln: %+v", in.Figures.Lights)
	}
	if !slices.Equal(in.Next.Seeds, []string{"1", "2", "3"}) || len(in.Next.Commands) < 4 {
		t.Errorf("nächster Lauf: %+v", in.Next)
	}
	for _, c := range in.Next.Commands {
		if c.Where != WhereChat && c.Where != WhereShell && c.Where != WhereWorkbench {
			t.Errorf("Befehl ohne Ziel: %+v", c)
		}
	}
}

// AC-01: Trend zur Baseline und Vergleich; ohne Baseline kein Trend.
func TestInsightsTrend(t *testing.T) {
	cur := fixture(t)
	base := fixture(t)
	for i := range base.Targets {
		if base.Targets[i].Measure == "goldAtDawnPerPlayer" {
			base.Targets[i].Value, base.Targets[i].Status, base.Targets[i].Pass = 22, StatusOK, true
		}
	}
	in := BuildInsights(cur, &base, testHints(t), Meta{Baseline: "b.json"})
	gold := find(in, "goldAtDawnPerPlayer", "normal")
	if gold.Trend == nil || gold.Trend.Delta != -5 || gold.Trend.Better || gold.Trend.BeforeStatus != StatusOK {
		t.Errorf("Trend: %+v", gold.Trend)
	}
	if !in.Baseline.Present || len(in.Baseline.Changes) != 1 || !in.Baseline.Changes[0].Tipped {
		t.Errorf("Vergleich: %+v", in.Baseline)
	}
	if in := BuildInsights(cur, nil, testHints(t), Meta{}); in.Targets[0].Trend != nil || in.Baseline.Present {
		t.Error("ohne Baseline darf es keinen Trend geben")
	}
}

// AC-06 und Fehlerfälle: Bericht ohne Kennzahlen → graue Kennzahl-Ampeln mit Hinweis; ungültiges Ziel → grau.
func TestInsightsAlterBericht(t *testing.T) {
	s := fixture(t)
	s.Kennzahlen = nil
	s.Targets[0].Status, s.Targets[0].InvalidSeeds = StatusInvalid, []string{"4", "9"}
	in := BuildInsights(s, nil, testHints(t), Meta{})
	if !strings.Contains(in.Figures.Note, "ohne Kennzahlen") || len(in.Figures.Nights) != 0 {
		t.Errorf("Hinweis: %q", in.Figures.Note)
	}
	for _, l := range in.Figures.Lights {
		if l.Light != LightGrey || l.Value != nil {
			t.Errorf("Kennzahl nicht grau: %+v", l)
		}
	}
	if in.Targets[0].Light != LightGrey || in.Targets[0].Distance != nil || in.Counts[LightGrey] != 1 {
		t.Errorf("ungültiges Ziel: %+v", in.Targets[0])
	}
	if !slices.ContainsFunc(in.Hints, func(h Hint) bool { return h.Light == LightGrey && strings.Contains(h.Text, "4, 9") }) {
		t.Errorf("Hinweis zu ungültigen Seeds fehlt: %+v", in.Hints)
	}
}

// AC-01: Ein bestandener Lauf ohne Gelb schlägt vor, die Baseline neu zu schreiben, und hat keine roten Hinweise.
func TestInsightsAllesGruen(t *testing.T) {
	s := fixture(t)
	s.Pass = true
	s.Targets = slices.DeleteFunc(s.Targets, func(e Entry) bool { return e.Status != StatusOK })
	in := BuildInsights(s, nil, testHints(t), Meta{})
	if len(in.Hints) != 0 || len(in.Next.Seeds) != 0 || !strings.Contains(strings.Join(in.Next.Steps, " "), "Baseline neu schreiben") {
		t.Errorf("alles grün: %+v %+v", in.Hints, in.Next)
	}
}

// AC-03: Der Markdown-Bericht hat oben Kurzauswertung, Ampeln, Stärken/Schwächen, Hinweise, nächsten Lauf und Kennzahlen je Nacht.
func TestMarkdownMitAuswertung(t *testing.T) {
	md := fixture(t).Markdown(nil)
	for _, want := range []string{"## Kurzauswertung", "## Ampeln", "🔴 verletzt | Hub-Stufe 2", "## Stärken", "## Schwächen",
		"## Tuning-Hinweise", "`hub.json:levels.1.steps.0.cost.stone`", "## Nächster Lauf", "(in den Chat an Claude)",
		"## Kennzahlen", "| Grad | Nacht | Läufe |", "| normal | 1 | 40 | 90 |", "## Ziele im Detail"} {
		if !strings.Contains(md, want) {
			t.Errorf("%q fehlt", want)
		}
	}
	if strings.Index(md, "## Kurzauswertung") > strings.Index(md, "| Ziel | Szenario |") {
		t.Error("Auswertung muss vor der Tabelle je Ziel stehen")
	}
}

// AC-01: Der Bereich fürs Abstimmen geht vom Wert in data/ höchstens drei Schritte in Richtung des Hebels.
func TestHebelBereich(t *testing.T) {
	p, err := ParseTuneParam("buildings.json:wall.levels.0.hp=0..1")
	if err != nil {
		t.Fatal(err)
	}
	got, ok := Lever{Path: "buildings.json:wall.levels.0.hp", Direction: "+", Step: 0.2, Label: "x"}.tuneParam()
	want := "buildings.json:wall.levels.0.hp=" + p.format(p.Base) + ".." + p.format(p.Base*1.6)
	if !ok || got != want {
		t.Errorf("Bereich %q, erwartet %q", got, want)
	}
	if _, ok := (Lever{Path: "buildings.json:fehlt.hp", Direction: "-", Step: 0.1}).tuneParam(); ok {
		t.Error("unbekannter Pfad darf keinen Bereich liefern")
	}
}
