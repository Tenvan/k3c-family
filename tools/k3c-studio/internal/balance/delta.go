package balance

import (
	"fmt"
	"slices"
	"strings"
)

// Delta-Bericht (B-640, BS7.2): Der Vergleich mit der Baseline eines Meilensteins, nach Werte-Gruppen geordnet. Freie
// Gruppen erscheinen immer mit Ampel und Trend, eingefrorene nur, wenn eines ihrer Ziele kippt (Warnung); Hinweise
// und Hebel nennt die Auswertung nur für freie Gruppen (BuildInsights).

// Trend einer Gruppe gegenüber der Baseline.
const (
	TrendBetter = "besser"
	TrendWorse  = "schlechter"
	TrendSame   = "gleich"
)

// GroupDelta ist eine Werte-Gruppe im Delta-Bericht.
type GroupDelta struct {
	Group   string   `json:"group"`
	Frozen  bool     `json:"frozen"`
	Light   string   `json:"light"`   // schlechteste Ampel der Ziele der Gruppe
	Trend   string   `json:"trend"`   // TrendBetter, TrendWorse oder TrendSame
	Better  int      `json:"better"`  // Ziele mit größerem Abstand zum Korridor
	Worse   int      `json:"worse"`   // Ziele mit kleinerem Abstand
	Targets int      `json:"targets"` // Ziele der Gruppe im Lauf
	Tipped  []string `json:"tipped"`  // Ziele, deren Pass gegenüber der Baseline wechselt
	Warning string   `json:"warning,omitempty"`
}

// buildDelta ordnet die Ziele einer Auswertung nach Gruppen; der Aufrufer ruft es nur mit Baseline auf.
func buildDelta(targets []TargetInsight, frozen []string) []GroupDelta {
	out := []GroupDelta{}
	for _, g := range Groups {
		d := GroupDelta{Group: g, Frozen: slices.Contains(frozen, g), Light: LightGreen, Tipped: []string{}}
		compared := 0
		for _, t := range targets {
			if t.Group != g {
				continue
			}
			d.Targets++
			d.Light = worseLight(d.Light, t.Light)
			if t.Trend == nil {
				continue
			}
			compared++
			switch {
			case t.Trend.Delta == 0: // unverändert zählt weder besser noch schlechter
			case t.Trend.Better:
				d.Better++
			default:
				d.Worse++
			}
			if passedStatus(t.Trend.BeforeStatus) != passedStatus(t.Status) {
				d.Tipped = append(d.Tipped, t.Name)
			}
		}
		if d.Targets == 0 || (d.Frozen && len(d.Tipped) == 0) {
			continue
		}
		d.Trend = trendOf(d, compared)
		if d.Frozen {
			d.Warning = fmt.Sprintf("eingefrorene Gruppe %s kippt: %s", g, strings.Join(d.Tipped, ", "))
		}
		out = append(out, d)
	}
	return out
}

func trendOf(d GroupDelta, compared int) string {
	switch {
	case compared == 0 || d.Better == d.Worse:
		return TrendSame
	case d.Better > d.Worse:
		return TrendBetter
	}
	return TrendWorse
}

func passedStatus(status string) bool { return status == StatusOK || status == StatusNarrow }

// worseLight: rot vor gelb vor grau vor grün.
func worseLight(a, b string) string {
	rank := map[string]int{LightGreen: 0, LightGrey: 1, LightYellow: 2, LightRed: 3}
	if rank[b] > rank[a] {
		return b
	}
	return a
}

// freeLevers lässt die Hebel eingefrorener Gruppen weg.
func freeLevers(levers []Lever, frozen []string) []Lever {
	out := make([]Lever, 0, len(levers))
	for _, l := range levers {
		if !slices.Contains(frozen, l.Group) {
			out = append(out, l)
		}
	}
	return out
}
