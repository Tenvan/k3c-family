package balance

import (
	"cmp"
	"encoding/json"
	"fmt"
	"slices"
	"strconv"
	"strings"

	"k3c/data"
)

// Grad-Kurven (B-158, BAL3.3, Beschluss BAL3.1): je Schwierigkeitsgrad aus data/difficulty.json eine Tabelle über die
// Nächte 1 bis CurveNights im Standardszenario (Wald, 2 Spieler, saver).

// CurveNights: Nächte je Kurve (Beschluss BAL3.1).
const CurveNights = 5

const curveScenario = "Wald, 2 Spieler, `saver`"

// GradeCurve ist die Kurve eines Grads; Invalid zählt abgebrochene Läufe (sie fehlen in allen Werten).
type GradeCurve struct {
	Grade   string       `json:"grade"`
	Invalid int          `json:"invalid"`
	Nights  []NightPoint `json:"nights"`
}

// NightPoint: CastleHeld = Anteil der gültigen Läufe in %, deren Burg die Welle der Nacht hält; Destroyed = Median der
// zerstörten Gebäude dieser Welle; GoldAtDusk = Median des Golds aller Monarchen zur Dämmerung des Tags. null = kein Messwert.
type NightPoint struct {
	Night      int      `json:"night"`
	CastleHeld float64  `json:"castleHeld"`
	Destroyed  *float64 `json:"destroyed"`
	GoldAtDusk *float64 `json:"goldAtDusk"`
}

// Grades liefert die Grade aus data/difficulty.json, nach Wellengröße sortiert (dev, easy, normal, hard, ultra).
func Grades() ([]string, error) {
	var d struct {
		Grades map[string]struct{ WaveSize float64 }
	}
	raw, err := data.Files.ReadFile("difficulty.json")
	if err == nil {
		err = json.Unmarshal(raw, &d)
	}
	if err != nil {
		return nil, fmt.Errorf("data/difficulty.json: %w", err)
	}
	names := make([]string, 0, len(d.Grades))
	for n := range d.Grades {
		names = append(names, n)
	}
	slices.SortFunc(names, func(a, b string) int {
		return cmp.Or(cmp.Compare(d.Grades[a].WaveSize, d.Grades[b].WaveSize), strings.Compare(a, b))
	})
	return names, nil
}

// Curves spielt das Standardszenario je Grad über die Seeds und nights Tage.
func Curves(seeds []string, nights int) ([]GradeCurve, error) {
	grades, err := Grades()
	if err != nil {
		return nil, err
	}
	out := make([]GradeCurve, 0, len(grades))
	for _, g := range grades {
		r, err := Run(Matrix{Seeds: seeds, Players: []int{2}, Bots: []string{"saver"}, Depths: []int{0}, Days: nights, Grade: g})
		if err != nil {
			return nil, err
		}
		out = append(out, curve(g, r, nights))
	}
	return out, nil
}

func curve(grade string, r Report, nights int) GradeCurve {
	c := GradeCurve{Grade: grade, Nights: make([]NightPoint, 0, nights)}
	valid := 0
	for _, res := range r.Results {
		if res.Valid && res.Metrics != nil {
			valid++
		} else {
			c.Invalid++
		}
	}
	for n := 1; n <= nights; n++ {
		var held float64
		var destroyed, gold []float64
		for _, res := range r.Results {
			if !res.Valid || res.Metrics == nil {
				continue
			}
			for _, w := range res.Metrics.Waves {
				if w.Wave == n {
					held += b2f(w.Survived)
					destroyed = append(destroyed, float64(w.BuildingsDestroyed))
				}
			}
			if d := res.Metrics.Days; len(d) >= n && d[n-1].GoldAtDusk != nil {
				gold = append(gold, float64(*d[n-1].GoldAtDusk))
			}
		}
		p := NightPoint{Night: n, Destroyed: medianOrNil(destroyed), GoldAtDusk: medianOrNil(gold)}
		if valid > 0 {
			p.CastleHeld = 100 * held / float64(valid)
		}
		c.Nights = append(c.Nights, p)
	}
	return c
}

func medianOrNil(vals []float64) *float64 {
	if len(vals) == 0 {
		return nil
	}
	m := median(vals)
	return &m
}

func (c GradeCurve) markdown(b *strings.Builder) {
	fmt.Fprintf(b, "\n### %s\n\n", c.Grade)
	if c.Invalid > 0 {
		fmt.Fprintf(b, "%d Läufe abgebrochen (nicht mitgezählt).\n\n", c.Invalid)
	}
	b.WriteString("| Nacht | Burg hält | Zerstörte Gebäude (Median) | Gold zur Dämmerung (Median) |\n|---|---|---|---|\n")
	for _, p := range c.Nights {
		fmt.Fprintf(b, "| %d | %s %% | %s | %s |\n", p.Night, dec1(p.CastleHeld), optDec1(p.Destroyed), optDec1(p.GoldAtDusk))
	}
}

func dec1(x float64) string {
	return strings.ReplaceAll(strconv.FormatFloat(x, 'f', 1, 64), ".", ",")
}

func optDec1(x *float64) string {
	if x == nil {
		return "–"
	}
	return dec1(*x)
}
