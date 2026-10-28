package balance

import (
	"fmt"
	"slices"
	"strings"
)

// Texte der Auswertung (B-632): regelbasiert aus Ampeln, Abständen, Kennzahlen und Baseline, ohne KI.

// Hint ist ein Tuning-Hinweis zu einem Ziel.
type Hint struct {
	Target string  `json:"target"`
	Light  string  `json:"light"`
	Text   string  `json:"text"`
	Why    string  `json:"why"`
	Levers []Lever `json:"levers"`
	Plans  []LeverPlan `json:"plans"`
}

const maxBullets = 8

// label: Name des Ziels mit Grad, wenn nicht normal.
func (t TargetInsight) label() string {
	if t.Grade == "normal" {
		return t.Name
	}
	return t.Name + " (" + t.Grade + ")"
}

func (t TargetInsight) valueText() string { return t.entry.valueText() }

func (t TargetInsight) boundsText() string { return t.entry.boundsText() }

// worst: verletzte Ziele, am weitesten daneben zuerst.
func worst(in Insights) []TargetInsight {
	var out []TargetInsight
	for _, t := range in.Targets {
		if t.Light == LightRed {
			out = append(out, t)
		}
	}
	slices.SortStableFunc(out, func(a, b TargetInsight) int { return cmpFloat(*a.Distance, *b.Distance) })
	return out
}

func cmpFloat(a, b float64) int {
	switch {
	case a < b:
		return -1
	case a > b:
		return 1
	}
	return 0
}

func summary(in Insights) []string {
	c := in.Counts
	var out []string
	verdict := ""
	if in.Goal != nil {
		out = append(out, in.GoalText)
		if in.Progress != nil && in.Progress.Blocker != "" {
			out = append(out, in.Progress.Blocker)
		}
	} else if in.Pass {
		verdict = ": der Lauf besteht"
	} else {
		verdict = ": der Lauf besteht nicht"
	}
	out = append(out, fmt.Sprintf("%d von %d Zielen verletzt, %d knapp, %d im Korridor%s%s.", c[LightRed], len(in.Targets),
		c[LightYellow], c[LightGreen], invalidText(c[LightGrey]), verdict))
	if s := castleSentence(in); s != "" {
		out = append(out, s)
	}
	if w := worst(in); len(w) > 0 {
		parts := make([]string, 0, 2)
		for _, t := range w[:min(2, len(w))] {
			parts = append(parts, fmt.Sprintf("%s mit %s statt %s", t.label(), t.valueText(), t.boundsText()))
		}
		out = append(out, "Am weitesten daneben: "+strings.Join(parts, "; ")+".")
		if lv := firstLever(w); lv != nil {
			out = append(out, fmt.Sprintf("Größter Hebel: %s (%s, je Schritt um %s ändern).", lv.Label, lv.Path, stepText(lv.Step)))
		}
	}
	out = append(out, figureSentence(in))
	if in.Baseline.Present {
		out = append(out, baselineSentence(in.Baseline))
	}
	return out[:min(7, len(out))]
}

func invalidText(n int) string {
	if n == 0 {
		return ""
	}
	return fmt.Sprintf(", %d ungültig", n)
}

// castleSentence: die Pflicht-Kennzahl castleHeld über alle Grade in einem Satz.
func castleSentence(in Insights) string {
	var all, bad []TargetInsight
	for _, t := range in.Targets {
		if t.Measure == "castleHeld" {
			all = append(all, t)
			if t.Light == LightRed || t.Light == LightGrey {
				bad = append(bad, t)
			}
		}
	}
	switch {
	case len(all) == 0:
		return ""
	case len(bad) == 0:
		return fmt.Sprintf("Die Burg hält in allen %d Graden im Korridor.", len(all))
	}
	names := make([]string, 0, len(bad))
	for _, t := range bad {
		names = append(names, fmt.Sprintf("%s %s (%s)", t.Grade, t.valueText(), t.boundsText()))
	}
	return "Die Burg hält nicht im Korridor: " + strings.Join(names, ", ") + "."
}

func figureSentence(in Insights) string {
	var red []string
	grey := 0
	for _, l := range in.Figures.Lights {
		switch l.Light {
		case LightRed:
			red = append(red, l.label()+" "+l.valueText())
		case LightGrey:
			grey++
		}
	}
	switch {
	case len(red) > 0:
		return strings.TrimSuffix("Kennzahlen außerhalb der Richtwerte: "+strings.Join(red[:min(3, len(red))], ", "), ".") + "."
	case grey == len(in.Figures.Lights) && grey > 0:
		return "Keine Kennzahlen im Bericht: Kennzahl-Ampeln grau."
	}
	return "Alle Kennzahlen liegen im oder nahe am Richtwert."
}

func baselineSentence(b BaselineView) string {
	tipped := 0
	for _, c := range b.Changes {
		if c.Tipped {
			tipped++
		}
	}
	if len(b.Changes) == 0 {
		return "Gegenüber der Baseline hat kein Ziel den Status gewechselt."
	}
	return fmt.Sprintf("Gegenüber der Baseline wechseln %d Ziele den Status, %d kippen.", len(b.Changes), tipped)
}

func (l FigureLight) label() string {
	if l.Grade == "" || l.Grade == "normal" {
		return l.Name
	}
	return l.Name + " (" + l.Grade + ")"
}

func (l FigureLight) valueText() string {
	if l.Value == nil {
		return "–"
	}
	return strings.TrimSpace(num(*l.Value) + " " + l.Unit)
}

func firstLever(ts []TargetInsight) *Lever {
	for _, t := range ts {
		if len(t.Levers) > 0 {
			return &t.Levers[0]
		}
	}
	return nil
}

func stepText(step float64) string { return num(round1(step*100)) + " %" }

func strengths(in Insights) []string {
	var out []string
	byMeasure := map[string][]TargetInsight{}
	var order []string
	for _, t := range in.Targets {
		if _, seen := byMeasure[t.Measure]; !seen {
			order = append(order, t.Measure)
		}
		byMeasure[t.Measure] = append(byMeasure[t.Measure], t)
	}
	for _, m := range order {
		out = append(out, measureStrengths(byMeasure[m])...)
	}
	for _, l := range in.Figures.Lights {
		if l.Light == LightGreen && (l.Grade == "" || l.Grade == "normal") {
			out = append(out, fmt.Sprintf("%s %s im Richtwert.", l.label(), l.valueText()))
		}
	}
	for _, t := range in.Targets {
		if t.Trend != nil && t.Trend.Better && t.Trend.BeforeStatus == StatusViolated && t.Light != LightRed {
			out = append(out, fmt.Sprintf("%s ist gegenüber der Baseline wieder im Korridor.", t.label()))
		}
	}
	return capList(out)
}

// measureStrengths: alle Grade grün → ein Satz, sonst je grünem Ziel einer.
func measureStrengths(ts []TargetInsight) []string {
	green := slices.DeleteFunc(slices.Clone(ts), func(t TargetInsight) bool { return t.Light != LightGreen })
	if len(green) == 0 {
		return nil
	}
	if len(green) == len(ts) && len(ts) > 1 {
		return []string{fmt.Sprintf("%s: in allen %d Graden im Korridor.", ts[0].Name, len(ts))}
	}
	out := make([]string, 0, len(green))
	for _, t := range green {
		out = append(out, fmt.Sprintf("%s: %s im Korridor %s.", t.label(), t.valueText(), t.boundsText()))
	}
	return out
}

func weaknesses(in Insights) []string {
	var out []string
	for _, t := range worst(in) {
		out = append(out, fmt.Sprintf("%s: %s statt %s, %d Seeds betroffen.", t.label(), t.valueText(), t.boundsText(), len(t.FailSeeds)))
	}
	for _, t := range in.Targets {
		if t.Light == LightGrey {
			out = append(out, fmt.Sprintf("%s: ungültig, %d Seeds abgebrochen oder ohne Messwert.", t.label(), len(t.InvalidSeeds)))
		}
	}
	for _, l := range in.Figures.Lights {
		if l.Light == LightRed {
			out = append(out, fmt.Sprintf("%s %s (%s).", l.label(), l.valueText(), l.Text))
		}
	}
	for _, c := range in.Baseline.Changes {
		if c.Tipped && c.After != StatusOK && c.After != StatusNarrow {
			out = append(out, fmt.Sprintf("Gekippt gegenüber der Baseline: %s (%s → %s).", c.Name, c.Before, c.After))
		}
	}
	return capList(out)
}

func capList(s []string) []string {
	if s == nil {
		return []string{}
	}
	return s[:min(maxBullets, len(s))]
}
