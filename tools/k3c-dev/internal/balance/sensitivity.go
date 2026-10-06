package balance

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"strings"
)

// Sensitivitäts-Lauf (B-158, BAL3.3): Je Pfad und Stufe die Ziele aus data/balance-targets.json neu rechnen und die
// gekippten nennen, also die, deren Bewertung (ok, knapp, verletzt, ungültig) sich gegenüber dem unveränderten Lauf
// ändert. Dazu Grad-Kurven (curves.go). Nichts wird automatisch geändert, nur berichtet.

// SensitivityOptions: Paths leer = keine Variation, Steps leer = Steps, Seeds 0 = alle Seeds der Datei, Nights 0 =
// keine Grad-Kurven (Beschluss BAL3.1: CurveNights).
type SensitivityOptions struct {
	Paths  []string
	Steps  []int
	Seeds  int
	Nights int
}

// SensitivityReport ist der Bericht; feste Feldreihenfolge, keine Wanduhr, damit gleiche Eingaben byte-gleich sind.
type SensitivityReport struct {
	Version    int          `json:"version"`
	Seeds      int          `json:"seeds"`
	Variations []Variation  `json:"variations,omitempty"`
	Curves     []GradeCurve `json:"curves,omitempty"`
}

// Variation ist ein Pfad auf einer Stufe: Wert in data/ vorher und nachher, gekippte Ziele.
type Variation struct {
	Path    string  `json:"path"`
	Percent int     `json:"percent"`
	Before  float64 `json:"before"`
	After   float64 `json:"after"`
	Flips   []Flip  `json:"flips"`
}

// Flip ist ein gekipptes Ziel mit Bewertung und Wert (Anteil in %, sonst Median) vorher und nachher.
type Flip struct {
	Name        string  `json:"name"`
	Scenario    string  `json:"scenario"`
	Kind        string  `json:"kind"`
	Before      string  `json:"before"`
	After       string  `json:"after"`
	ValueBefore float64 `json:"valueBefore"`
	ValueAfter  float64 `json:"valueAfter"`
}

// Sensitivity rechnet Variationen und Kurven. Alle Pfade werden vor dem ersten Lauf geprüft.
func (t *Targets) Sensitivity(o SensitivityOptions) (SensitivityReport, error) {
	steps := o.Steps
	if len(steps) == 0 {
		steps = Steps
	}
	type job struct {
		v   Variation
		fsy fs.FS
	}
	var jobs []job
	for _, p := range o.Paths {
		for _, pct := range steps {
			fsy, before, after, err := variedFS(p, pct)
			if err != nil {
				return SensitivityReport{}, err
			}
			jobs = append(jobs, job{Variation{Path: p, Percent: pct, Before: before, After: after, Flips: []Flip{}}, fsy})
		}
	}
	rep := SensitivityReport{Version: 1, Seeds: len(t.seedList(o.Seeds))}
	if len(jobs) > 0 {
		base, err := t.summary(o.Seeds)
		if err != nil {
			return rep, err
		}
		for _, j := range jobs {
			err := withData(j.fsy, func() error {
				cur, err := t.summary(o.Seeds)
				j.v.Flips = append(j.v.Flips, flips(base, cur)...)
				return err
			})
			if err != nil {
				return rep, err
			}
			rep.Variations = append(rep.Variations, j.v)
		}
	}
	if o.Nights > 0 {
		var err error
		if rep.Curves, err = Curves(t.seedList(o.Seeds), o.Nights); err != nil {
			return rep, err
		}
	}
	return rep, nil
}

func (t *Targets) summary(seeds int) (Summary, error) {
	r, err := t.RunTargets(seeds)
	return t.Summarize(r), err
}

// flips: Ziele mit anderer Bewertung; beide Summaries stammen aus denselben Zielen (gleiche Reihenfolge).
func flips(base, cur Summary) []Flip {
	var out []Flip
	for i, b := range base.Targets {
		c := cur.Targets[i]
		if b.Status != c.Status {
			out = append(out, Flip{Name: b.Name, Scenario: scenarioText(b.Target), Kind: b.Kind,
				Before: b.Status, After: c.Status, ValueBefore: b.Value, ValueAfter: c.Value})
		}
	}
	return out
}

func scenarioText(g Target) string {
	return fmt.Sprintf("%d Spieler, %s, Tiefe %d, %d Tage", g.Players, g.Bot, g.Depth, g.Days)
}

// JSON schreibt den Bericht eingerückt mit Zeilenende.
func (r SensitivityReport) JSON() ([]byte, error) {
	b, err := json.MarshalIndent(r, "", "  ")
	return append(b, '\n'), err
}

// Markdown: Tabelle der Variationen mit gekippten Zielen, dann je Grad die Kurve.
func (r SensitivityReport) Markdown() string {
	var b strings.Builder
	fmt.Fprintf(&b, "# Sensitivitäts-Bericht\n\n%d Seeds, Ziele nach `data/balance-targets.json`. Gekippt: Die Bewertung "+
		"ändert sich gegenüber dem Lauf mit unverändertem Wert. `data/*.json` bleiben unverändert.\n", r.Seeds)
	if len(r.Variations) > 0 {
		b.WriteString("\n## Variationen\n\n| Pfad | Stufe | Wert | Gekippte Ziele |\n|---|---|---|---|\n")
		for _, v := range r.Variations {
			fmt.Fprintf(&b, "| `%s` | %+d %% | %s → %s | %s |\n", v.Path, v.Percent, num(v.Before), num(v.After), flipsText(v.Flips))
		}
	}
	if len(r.Curves) > 0 {
		b.WriteString("\n## Grad-Kurven\n\n" + curveScenario + "; je Nacht: Anteil der Seeds, in denen die Burg hält, " +
			"Median zerstörter Gebäude der Welle, Median des Golds aller Monarchen zur Dämmerung.\n")
		for _, c := range r.Curves {
			c.markdown(&b)
		}
	}
	return b.String()
}

func flipsText(fs []Flip) string {
	if len(fs) == 0 {
		return "–"
	}
	parts := make([]string, 0, len(fs))
	for _, f := range fs {
		unit := ""
		if f.Kind == KindShare {
			unit = " %"
		}
		parts = append(parts, fmt.Sprintf("%s (%s): %s %s%s → %s %s%s", f.Name, f.Scenario,
			f.Before, dec1(f.ValueBefore), unit, f.After, dec1(f.ValueAfter), unit))
	}
	return strings.Join(parts, "; ")
}
