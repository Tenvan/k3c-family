package balance

import (
	"fmt"
	"slices"
	"strings"
)

// Kennzahlen je Tiefe der Insel im Bericht (BS11, B-642): verdichtet über die gültigen Läufe mit World.

// IslandFigures: eine Zeile je Tiefe, je Boss und der Endboss.
type IslandFigures struct {
	Runs    int            `json:"runs"` // gültige Insel-Läufe
	Depths  []DepthFigures `json:"depths"`
	Bosses  []BossFigures  `json:"bosses"`
	Endboss BossFigures    `json:"endboss"`
}

// DepthFigures: Anteil der Läufe, die die Tiefe erreichten (Arrived in %), Median des Ankunftstags und Anteil der
// Läufe, in denen die Burg der Stufe fiel (CastleFell in %).
type DepthFigures struct {
	Depth            int      `json:"depth"`
	Arrived          float64  `json:"arrived"`
	ArrivalDayMedian *float64 `json:"arrivalDayMedian"`
	CastleFell       float64  `json:"castleFell"`
}

// BossFigures: Anteil der Läufe, die den Boss besiegten (Defeated in %), und Median des Tags (null: nie besiegt).
type BossFigures struct {
	Boss      string   `json:"boss"`
	Defeated  float64  `json:"defeated"`
	DayMedian *float64 `json:"dayMedian"`
}

// islandFiguresOf verdichtet die Insel-Läufe; nil ohne einen gültigen Lauf mit Stufen.
func islandFiguresOf(r Report) *IslandFigures {
	var runs []*Metrics
	for _, res := range r.Results {
		if res.Valid && res.Metrics != nil && len(res.Metrics.Stages) > 0 {
			runs = append(runs, res.Metrics)
		}
	}
	if len(runs) == 0 {
		return nil
	}
	f := &IslandFigures{Runs: len(runs), Depths: []DepthFigures{}, Bosses: []BossFigures{}}
	n := float64(len(runs))
	for i := range runs[0].Stages {
		var days []float64
		fell := 0
		for _, m := range runs {
			if i >= len(m.Stages) {
				continue
			}
			if d := m.Stages[i].ArrivalDay; d != nil {
				days = append(days, float64(*d))
			}
			fell += b2i(m.Stages[i].CastleFallTick != nil)
		}
		f.Depths = append(f.Depths, DepthFigures{Depth: runs[0].Stages[i].Depth, Arrived: round1(100 * float64(len(days)) / n),
			ArrivalDayMedian: medianOrNil(days), CastleFell: round1(100 * float64(fell) / n)})
	}
	f.Bosses = bossFigures(runs)
	var end []float64
	for _, m := range runs {
		if m.EndbossDay != nil {
			end = append(end, float64(*m.EndbossDay))
		}
	}
	f.Endboss = BossFigures{Boss: "endboss", Defeated: round1(100 * float64(len(end)) / n), DayMedian: medianOrNil(end)}
	return f
}

// bossFigures: ein Eintrag je Boss-ID in der Reihenfolge des ersten Auftretens (über alle Läufe).
func bossFigures(runs []*Metrics) []BossFigures {
	var ids []string
	for _, m := range runs {
		for _, b := range m.Bosses {
			if !slices.Contains(ids, b.Boss) {
				ids = append(ids, b.Boss)
			}
		}
	}
	out := []BossFigures{}
	for _, id := range ids {
		var days []float64
		for _, m := range runs {
			if i := slices.IndexFunc(m.Bosses, func(b BossMetrics) bool { return b.Boss == id }); i >= 0 {
				days = append(days, float64(m.Bosses[i].Day))
			}
		}
		out = append(out, BossFigures{Boss: id, Defeated: round1(100 * float64(len(days)) / float64(len(runs))), DayMedian: medianOrNil(days)})
	}
	return out
}

func dayText(d *float64) string {
	if d == nil {
		return "–"
	}
	return num(*d)
}

// markdown schreibt die Zeilen je Tiefe und je Boss.
func (f *IslandFigures) markdown(b *strings.Builder) {
	b.WriteString("\n| Tiefe | Erreicht % | Ankunftstag (Median) | Burg gefallen % |\n|---|---|---|---|\n")
	for _, d := range f.Depths {
		fmt.Fprintf(b, "| %d | %s | %s | %s |\n", d.Depth, num(d.Arrived), dayText(d.ArrivalDayMedian), num(d.CastleFell))
	}
	b.WriteString("\n| Boss | Besiegt % | Tag (Median) |\n|---|---|---|\n")
	for _, x := range append(slices.Clone(f.Bosses), f.Endboss) {
		fmt.Fprintf(b, "| %s | %s | %s |\n", x.Boss, num(x.Defeated), dayText(x.DayMedian))
	}
}
