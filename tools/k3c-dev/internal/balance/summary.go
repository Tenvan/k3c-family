package balance

import (
	"encoding/json"
	"fmt"
	"strconv"
	"strings"
)

// Bericht von `task balance` (B-157, BAL2.2): Pass/Fail je Ziel als JSON und Markdown, Vergleich mit einer Baseline.
// Die Baseline ist selbst ein Summary-JSON (testdata/balance/baseline.json), ein eigenes Format gibt es nicht.

// Entry ist ein bewertetes Ziel; Pass gilt für „ok“ und „knapp“.
type Entry struct {
	Verdict
	Pass bool `json:"pass"`
}

// Summary ist der Bericht: alle Ziele mit Urteil. Seeds ist die Zahl der gespielten Seeds (kleiner als in der Datei bei --seeds N).
type Summary struct {
	Version int     `json:"version"`
	Seeds   int     `json:"seeds"`
	Pass    bool    `json:"pass"`
	Targets []Entry `json:"targets"`
}

// Change ist ein Ziel, dessen Bewertung gegenüber der Baseline anders ist. Before/After sind Status; „neu“ und
// „entfällt“, wenn das Ziel nur auf einer Seite steht. Tipped: Pass wechselt (ok/knapp ↔ verletzt/ungültig).
type Change struct {
	Name   string `json:"name"`
	Before string `json:"before"`
	After  string `json:"after"`
	Tipped bool   `json:"tipped"`
}

// RunTargets spielt alle Szenarien der Ziele über die ersten n Seeds der Datei (0 = alle).
func (t *Targets) RunTargets(n int) (Report, error) {
	seedList := t.Seeds
	if n > 0 && n < len(seedList) {
		seedList = seedList[:n]
	}
	var rep Report
	done := map[Scenario]bool{}
	for _, g := range t.Targets {
		key := Scenario{Players: g.Players, Bot: g.Bot, Depth: g.Depth, Days: g.Days}
		if done[key] {
			continue
		}
		done[key] = true
		r, err := Run(Matrix{Seeds: seedList, Players: []int{g.Players}, Bots: []string{g.Bot}, Depths: []int{g.Depth}, Days: g.Days})
		if err != nil {
			return Report{}, err
		}
		if rep.Results == nil {
			rep.Matrix = r.Matrix
		}
		rep.Results = append(rep.Results, r.Results...)
	}
	return rep, nil
}

// Summarize bewertet einen Bericht gegen die Ziele.
func (t *Targets) Summarize(r Report) Summary {
	s := Summary{Version: 1, Seeds: len(r.Matrix.Seeds), Pass: true}
	for _, v := range t.Evaluate(r) {
		e := Entry{Verdict: v, Pass: v.Status == StatusOK || v.Status == StatusNarrow}
		s.Pass = s.Pass && e.Pass
		s.Targets = append(s.Targets, e)
	}
	return s
}

// JSON schreibt den Bericht eingerückt mit Zeilenende.
func (s Summary) JSON() ([]byte, error) {
	b, err := json.MarshalIndent(s, "", "  ")
	return append(b, '\n'), err
}

// ReadSummary liest ein Summary-JSON (Baseline).
func ReadSummary(raw []byte) (Summary, error) {
	var s Summary
	if err := json.Unmarshal(raw, &s); err != nil {
		return s, fmt.Errorf("baseline: %w", err)
	}
	return s, nil
}

// targetKey unterscheidet Ziele auch bei gleichem Namen (Szenario gehört dazu).
func (e Entry) targetKey() string {
	return fmt.Sprintf("%s|%s|%d|%s|%d|%d", e.Measure, e.Name, e.Players, e.Bot, e.Depth, e.Days)
}

// Compare nennt die Ziele, deren Status sich gegenüber der Baseline geändert hat, in der Reihenfolge von cur
// (entfallene am Ende). Wertänderungen ohne Statuswechsel sind keine Änderung.
func Compare(base, cur Summary) []Change {
	before := map[string]Entry{}
	for _, e := range base.Targets {
		before[e.targetKey()] = e
	}
	var out []Change
	for _, e := range cur.Targets {
		b, ok := before[e.targetKey()]
		delete(before, e.targetKey())
		switch {
		case !ok:
			out = append(out, Change{Name: e.Name, Before: "neu", After: e.Status, Tipped: !e.Pass})
		case b.Status != e.Status:
			out = append(out, Change{Name: e.Name, Before: b.Status, After: e.Status, Tipped: b.Pass != e.Pass})
		}
	}
	for _, b := range base.Targets {
		if _, left := before[b.targetKey()]; left {
			out = append(out, Change{Name: b.Name, Before: b.Status, After: "entfällt", Tipped: b.Pass})
		}
	}
	return out
}

// Markdown schreibt die Tabelle je Ziel und den Vergleich. base == nil: Hinweis „keine Baseline“.
func (s Summary) Markdown(base *Summary) string {
	var b strings.Builder
	verdict := "Pass"
	if !s.Pass {
		verdict = "Fail"
	}
	fmt.Fprintf(&b, "# Balancing-Bericht\n\nGesamt: **%s**, %d Seeds, Ziele nach `data/balance-targets.json`.\n\n", verdict, s.Seeds)
	b.WriteString("| Ziel | Szenario | Wert | Grenzen | Bewertung | Seeds zum Nachspielen |\n|---|---|---|---|---|---|\n")
	for _, e := range s.Targets {
		fmt.Fprintf(&b, "| %s | %d Spieler, %s, Tiefe %d, %d Tage | %s | %s | %s | %s |\n",
			e.Name, e.Players, e.Bot, e.Depth, e.Days, e.valueText(), e.boundsText(), e.statusText(), e.seedsText())
	}
	b.WriteString("\n## Vergleich mit der Baseline\n\n")
	if base == nil {
		b.WriteString("Keine Baseline vorhanden (`testdata/balance/baseline.json`); `task balance:baseline` legt sie an.\n")
		return b.String()
	}
	if base.Seeds != s.Seeds {
		fmt.Fprintf(&b, "Hinweis: Die Baseline hat %d Seeds, dieser Lauf %d.\n\n", base.Seeds, s.Seeds)
	}
	changes := Compare(*base, s)
	if len(changes) == 0 {
		b.WriteString("Keine Bewertung hat sich geändert.\n")
	}
	for _, c := range changes {
		mark := ""
		if c.Tipped {
			mark = " **gekippt**"
		}
		fmt.Fprintf(&b, "- %s: %s → %s%s\n", c.Name, c.Before, c.After, mark)
	}
	return b.String()
}

func num(x float64) string {
	return strings.ReplaceAll(strconv.FormatFloat(x, 'f', -1, 64), ".", ",")
}

func (e Entry) unit() string {
	if e.Kind == KindShare {
		return " %"
	}
	return ""
}

func (e Entry) valueText() string {
	if e.Status == StatusInvalid {
		return "–"
	}
	return strings.ReplaceAll(strconv.FormatFloat(e.Value, 'f', 1, 64), ".", ",") + e.unit()
}

func (e Entry) boundsText() string {
	switch {
	case e.Lower != nil && e.Upper != nil:
		return num(*e.Lower) + "–" + num(*e.Upper) + e.unit()
	case e.Lower != nil:
		return "≥ " + num(*e.Lower) + e.unit()
	}
	return "≤ " + num(*e.Upper) + e.unit()
}

func (e Entry) statusText() string {
	if e.Pass {
		return "Pass (" + e.Status + ")"
	}
	return "Fail (" + e.Status + ")"
}

func (e Entry) seedsText() string {
	var parts []string
	if len(e.FailSeeds) > 0 {
		parts = append(parts, strings.Join(e.FailSeeds, ", "))
	}
	if len(e.InvalidSeeds) > 0 {
		parts = append(parts, "ungültig: "+strings.Join(e.InvalidSeeds, ", "))
	}
	if len(parts) == 0 {
		return "–"
	}
	return strings.Join(parts, "; ")
}
