package balance

import (
	"fmt"
	"strings"
)

// ShipFigures sind die Abschnitte im Bericht: Median des Tags über die Läufe, in denen der Abschnitt bezahlbar war
// (Reached = Anteil der Läufe in %), und die Ampel gegen den Korridor „Schiff fertig“ (zielkorridore.md § 6): zwischen
// 2 Tagen vor und 1 Tag nach dem Sieg über den Endboss.
type ShipFigures struct {
	Stages     []ShipStageFigures `json:"stages"`
	EndbossDay *float64           `json:"endbossDay"` // Median des Endboss-Tags, null: kein Lauf besiegte ihn
	Light      string             `json:"light"`
	Text       string             `json:"text"`
}

// ShipStageFigures: ein Abschnitt.
type ShipStageFigures struct {
	ID        string   `json:"id"`
	Name      string   `json:"name"`
	Reached   float64  `json:"reached"`
	DayMedian *float64 `json:"dayMedian"`
}

const (
	shipBefore = 2 // Tage vor dem Endboss-Sieg, ab denen „Schiff fertig“ grün ist
	shipAfter  = 1 // Tage danach
)

// shipFiguresOf verdichtet die Abschnitte der Läufe; nil, wenn kein gültiger Lauf Schiff-Werte hat.
func shipFiguresOf(r Report) *ShipFigures {
	stages, err := LoadShip()
	if err != nil || len(stages) == 0 {
		return nil
	}
	var runs []*Metrics
	for _, res := range r.Results {
		if res.Valid && res.Metrics != nil && len(res.Metrics.Ship) == len(stages) {
			runs = append(runs, res.Metrics)
		}
	}
	if len(runs) == 0 {
		return nil
	}
	f := &ShipFigures{}
	for i, st := range stages {
		var days []float64
		for _, m := range runs {
			if d := m.Ship[i].AffordableDay; d != nil {
				days = append(days, float64(*d))
			}
		}
		f.Stages = append(f.Stages, ShipStageFigures{ID: st.ID, Name: st.Name,
			Reached: round1(100 * float64(len(days)) / float64(len(runs))), DayMedian: medianOrNil(days)})
	}
	var end []float64
	for _, m := range runs {
		if m.EndbossDay != nil {
			end = append(end, float64(*m.EndbossDay))
		}
	}
	f.EndbossDay = medianOrNil(end)
	f.Light, f.Text = shipLight(f)
	return f
}

// shipLight bewertet den letzten Abschnitt (Schiff komplett bezahlbar) gegen den Korridor um den Endboss-Tag.
func shipLight(f *ShipFigures) (light, text string) {
	last := f.Stages[len(f.Stages)-1]
	var parts []string
	for _, s := range f.Stages {
		parts = append(parts, fmt.Sprintf("%s bezahlbar %s", s.Name, dayLabel(s.DayMedian)))
	}
	text = "Schiff (Bedarf): " + strings.Join(parts, ", ")
	if f.EndbossDay == nil {
		return LightGrey, text + " (Ziel offen: Endboss nicht besiegt)"
	}
	lo, hi := *f.EndbossDay-shipBefore, *f.EndbossDay+shipAfter
	text += fmt.Sprintf(" (Ziel T%s–T%s)", dayNum(lo), dayNum(hi))
	if last.DayMedian == nil {
		return LightRed, text + "; Hinweis: Schiff in keinem Lauf bezahlbar, Einnahmen oder Schiffskosten in data/ship.json prüfen"
	}
	d := *last.DayMedian
	switch {
	case d >= lo && d <= hi:
		return LightGreen, text
	case d < lo && lo-d <= 2, d > hi && d-hi <= 2:
		return LightYellow, text + shipHint(d < lo)
	}
	return LightRed, text + shipHint(d < lo)
}

func shipHint(early bool) string {
	if early {
		return "; Hinweis: Schiff zu früh bezahlbar, Schiffskosten in data/ship.json erhöhen"
	}
	return "; Hinweis: Schiff zu spät bezahlbar, Schiffskosten in data/ship.json senken oder Einnahmen erhöhen"
}

func dayLabel(d *float64) string {
	if d == nil {
		return "nie"
	}
	return "T" + dayNum(*d)
}

func dayNum(d float64) string { return strings.TrimSuffix(fmt.Sprintf("%.1f", d), ".0") }

// markdown schreibt die Zeile „Schiff (Bedarf): …“ mit Ampel.
func (f *ShipFigures) markdown(b *strings.Builder) {
	fmt.Fprintf(b, "\n%s %s\n", lightIcon[f.Light], f.Text)
}
