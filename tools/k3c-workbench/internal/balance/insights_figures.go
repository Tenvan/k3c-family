package balance

import (
	"fmt"
	"math"
	"slices"
)

// Ampeln der Kennzahlen (B-632) aus Figures und den Richtwerten in data/balance-hints.json. Je Grad: normal immer,
// die anderen Grade nur bei allGrades und nur, wenn die Ampel nicht grün ist (sonst wird die Leiste unübersichtlich).

// FigureInsights sind Kennzahlen mit Ampel, die Nächte für die Diagramme und die grünen Bänder je Diagramm.
type FigureInsights struct {
	Note   string          `json:"note"`
	Bands  map[string]Band `json:"bands"`
	Lights []FigureLight   `json:"lights"`
	Nights []NightFigures  `json:"nights"`
}

// FigureLight ist eine Kennzahl mit Ampel; Value nil: kein Messwert (graue Ampel).
type FigureLight struct {
	Key   string   `json:"key"`
	Name  string   `json:"name"`
	Unit  string   `json:"unit"`
	Grade string   `json:"grade,omitempty"`
	Value *float64 `json:"value"`
	Light string   `json:"light"`
	Text  string   `json:"text"`
}

// gradeFigures berechnen Kennzahlen aus den Nächten eines Grads; false: kein Wert.
var gradeFigures = map[string]func([]NightFigures) (float64, bool){
	"idleShare":    meanOf(func(n NightFigures) float64 { return n.IdleShare }, 1),
	"goldBalance":  meanOf(func(n NightFigures) float64 { return goldOf(n) }, 2),
	"woodBalance":  meanOf(func(n NightFigures) float64 { return n.ResourceIn.Wood - n.ResourceOut.Wood }, 1),
	"stoneBalance": meanOf(func(n NightFigures) float64 { return n.ResourceIn.Stone - n.ResourceOut.Stone }, 1),
	"survivalDrop": survivalDrop,
}

func figureInsights(f *Figures, h *Hints) FigureInsights {
	out := FigureInsights{Bands: map[string]Band{}, Lights: []FigureLight{}, Nights: []NightFigures{}}
	for _, k := range []string{"idleShare", "goldBalance"} {
		if fh, ok := h.Figures[k]; ok {
			out.Bands[k] = fh.Green
		}
	}
	if f == nil {
		out.Note = "Bericht ohne Kennzahlen (älter als WZ28.4): Kennzahl-Ampeln grau."
		for _, k := range figureKeys {
			if fh, ok := h.Figures[k]; ok {
				out.Lights = append(out.Lights, FigureLight{Key: k, Name: fh.Name, Unit: fh.Unit, Light: LightGrey, Text: "kein Messwert"})
			}
		}
		return out
	}
	out.Note = fmt.Sprintf("Kennzahlen aus %d gültigen Läufen; Richtwerte sind Startwerte (data/balance-hints.json).", f.Runs)
	out.Nights = f.Nights
	out.Lights = append(out.Lights, tickLight(h, "firstWallTick", f.FirstWallTickMedian), tickLight(h, "firstWorkshopTick", f.FirstWorkshopTickMedian))
	out.Lights = slices.DeleteFunc(out.Lights, func(l FigureLight) bool { return l.Key == "" })
	for _, g := range gradesOf(f.Nights) {
		out.Lights = append(out.Lights, gradeLights(h, g, nightsOf(f.Nights, g))...)
	}
	return out
}

func tickLight(h *Hints, key string, v *float64) FigureLight {
	fh, ok := h.Figures[key]
	if !ok {
		return FigureLight{}
	}
	l := FigureLight{Key: key, Name: fh.Name, Unit: fh.Unit, Value: v, Light: LightGrey, Text: "in keinem Lauf erreicht"}
	if v != nil {
		l.Light, l.Text = fh.Light(*v), "Median; "+fh.bandText()
	}
	return l
}

func gradeLights(h *Hints, grade string, nights []NightFigures) []FigureLight {
	var out []FigureLight
	for _, key := range figureKeys {
		fh, ok := h.Figures[key]
		calc := gradeFigures[key]
		if !ok || calc == nil || (grade != "" && !fh.AllGrades) {
			continue
		}
		v, has := calc(nights)
		l := FigureLight{Key: key, Name: fh.Name, Unit: fh.Unit, Grade: gradeName(grade), Light: LightGrey, Text: "kein Messwert"}
		if has {
			v = round1(v)
			l.Value, l.Light, l.Text = &v, fh.Light(v), fh.bandText()
		}
		if grade == "" || l.Light != LightGreen {
			out = append(out, l)
		}
	}
	return out
}

// bandText: „Richtwert grün -20 bis 60, gelb -60 bis 120“.
func (f FigureHint) bandText() string {
	return "Richtwert grün " + f.Green.text() + ", gelb " + f.Yellow.text()
}

func (b Band) text() string {
	switch {
	case b[0] == nil && b[1] == nil:
		return "beliebig"
	case b[0] == nil:
		return "≤ " + num(*b[1])
	case b[1] == nil:
		return "≥ " + num(*b[0])
	}
	return num(*b[0]) + " bis " + num(*b[1])
}

// meanOf: nach Läufen gewichtetes Mittel ab Nacht from.
func meanOf(pick func(NightFigures) float64, from int) func([]NightFigures) (float64, bool) {
	return func(nights []NightFigures) (float64, bool) {
		sum, runs := 0.0, 0
		for _, n := range nights {
			if n.Night >= from && n.Runs > 0 {
				sum += pick(n) * float64(n.Runs)
				runs += n.Runs
			}
		}
		if runs == 0 {
			return 0, false
		}
		return sum / float64(runs), true
	}
}

// survivalDrop: größter Abfall des Überlebens von einer Nacht zur nächsten in Prozentpunkten (vor Nacht 1: 100 %).
func survivalDrop(nights []NightFigures) (float64, bool) {
	if len(nights) == 0 {
		return 0, false
	}
	prev, drop := 100.0, 0.0
	for _, n := range nights {
		drop = math.Max(drop, prev-n.Survival)
		prev = n.Survival
	}
	return drop, true
}

func goldOf(n NightFigures) float64 {
	return n.DayIncome - n.DayExpense + n.NightIncome - n.NightExpense
}

func gradesOf(nights []NightFigures) []string {
	var out []string
	for _, n := range nights {
		if !slices.Contains(out, n.Grade) {
			out = append(out, n.Grade)
		}
	}
	return out
}

// nightsOf: Nächte eines Grads, nach Nacht sortiert.
func nightsOf(nights []NightFigures, grade string) []NightFigures {
	var out []NightFigures
	for _, n := range nights {
		if n.Grade == grade {
			out = append(out, n)
		}
	}
	slices.SortFunc(out, func(a, b NightFigures) int { return a.Night - b.Night })
	return out
}
