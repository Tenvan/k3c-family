package balance

import (
	"math"
	"slices"
)

// Bewertung eines Ziels (BAL2.1). Statuswerte:
//   - ok: Wert im Korridor, mehr als die Randbreite von jeder gesetzten Grenze entfernt
//   - knapp: Wert im Korridor, aber höchstens die Randbreite von einer Grenze entfernt (besteht, Ampel gelb)
//   - verletzt: Wert außerhalb des Korridors
//   - ungültig: ein Lauf des Szenarios brach ab oder es gibt keinen Messwert; der Wert ist dann nicht belastbar
const (
	StatusOK       = "ok"
	StatusNarrow   = "knapp"
	StatusViolated = "verletzt"
	StatusInvalid  = "ungültig"
)

// Verdict ist das Urteil über ein Ziel. Value ist bei einem Anteil in %, bei einem Median in der Einheit der Kennzahl.
// FailSeeds nennt bei „verletzt“ die Seeds zum Nachspielen (Replay): bei einem Anteil unter der Untergrenze die Seeds
// ohne erfüllte Bedingung, über der Obergrenze die mit erfüllter, bei einem Median die Seeds mit Wert außerhalb.
type Verdict struct {
	Target
	Value        float64  `json:"value"`
	Status       string   `json:"status"`
	FailSeeds    []string `json:"failSeeds,omitempty"`
	InvalidSeeds []string `json:"invalidSeeds,omitempty"`
}

// Evaluate bewertet alle Ziele gegen die Läufe eines Berichts (nur Seeds aus der Datei, nur das Szenario des Ziels).
func (t *Targets) Evaluate(r Report) []Verdict {
	out := make([]Verdict, 0, len(t.Targets))
	for _, g := range t.Targets {
		out = append(out, g.evaluate(t, r.Results))
	}
	return out
}

func (g Target) evaluate(t *Targets, results []Result) Verdict {
	v := Verdict{Target: g}
	var vals []float64
	var owner []string // Seed zu jedem Wert
	for _, res := range results {
		if res.Players != g.Players || res.Bot != g.Bot || res.Depth != g.Depth || res.Days != g.Days || !slices.Contains(t.Seeds, res.Seed) {
			continue
		}
		if !res.Valid || res.Metrics == nil {
			v.InvalidSeeds = append(v.InvalidSeeds, res.Seed)
			continue
		}
		for _, x := range measures[g.Measure].values(res.Metrics) {
			vals, owner = append(vals, x), append(owner, res.Seed)
		}
	}
	if len(vals) == 0 || len(v.InvalidSeeds) > 0 {
		v.Status = StatusInvalid
		return v
	}
	v.Value = g.value(vals)
	v.Status = g.rate(v.Value, t)
	if v.Status == StatusViolated {
		v.FailSeeds = g.failSeeds(v.Value, vals, owner)
	}
	return v
}

// value: Anteil in % bzw. Median der Werte.
func (g Target) value(vals []float64) float64 {
	if g.Kind == KindShare {
		sum := 0.0
		for _, x := range vals {
			sum += x
		}
		return 100 * sum / float64(len(vals))
	}
	s := slices.Sorted(slices.Values(vals))
	if n := len(s); n%2 == 0 {
		return (s[n/2-1] + s[n/2]) / 2
	}
	return s[len(s)/2]
}

// bounds liefert die Grenzen, die für „knapp“ zählen: Die natürlichen Enden eines Anteils (0 % und 100 %) gehören nicht
// dazu, sonst wäre jedes Ergebnis mit 100 % „knapp“.
func (g Target) bounds() (lo, hi *float64) {
	lo, hi = g.Lower, g.Upper
	if g.Kind == KindShare {
		if lo != nil && *lo <= 0 {
			lo = nil
		}
		if hi != nil && *hi >= 100 {
			hi = nil
		}
	}
	return lo, hi
}

// margin ist die Randbreite: Prozentpunkte bei Anteilen, bei Medianen der Bruchteil der Korridorbreite (bei einer
// einzigen Grenze deren Betrag), sofern das Ziel keinen eigenen Wert hat.
func (g Target) margin(t *Targets) float64 {
	if g.Kind == KindShare {
		return pick(g.Margin, t.Margin.SharePp)
	}
	width := 0.0
	switch {
	case g.Lower != nil && g.Upper != nil:
		width = *g.Upper - *g.Lower
	case g.Lower != nil:
		width = math.Abs(*g.Lower)
	default:
		width = math.Abs(*g.Upper)
	}
	return pick(g.Margin, t.Margin.MedianFraction) * width
}

func (g Target) rate(v float64, t *Targets) string {
	if (g.Lower != nil && v < *g.Lower) || (g.Upper != nil && v > *g.Upper) {
		return StatusViolated
	}
	lo, hi := g.bounds()
	if m := g.margin(t); (lo != nil && v-*lo <= m) || (hi != nil && *hi-v <= m) {
		return StatusNarrow
	}
	return StatusOK
}

func (g Target) failSeeds(value float64, vals []float64, owner []string) []string {
	var seeds []string
	for i, x := range vals {
		var fails bool
		if g.Kind == KindShare {
			fails = (x == 0) == (g.Lower != nil && value < *g.Lower)
		} else {
			fails = (g.Lower != nil && x < *g.Lower) || (g.Upper != nil && x > *g.Upper)
		}
		if fails && !slices.Contains(seeds, owner[i]) {
			seeds = append(seeds, owner[i])
		}
	}
	return seeds
}

func pick(own *float64, def float64) float64 {
	if own != nil {
		return *own
	}
	return def
}
