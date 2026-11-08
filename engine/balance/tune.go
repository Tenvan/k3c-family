package balance

import (
	"encoding/json"
	"fmt"
	"math"
	"slices"
	"strings"
)

// Automatisches Balancing (WZ28.6, B-596): Sweep (volles Raster) oder Suche über ein bis zwei Werte aus data/; jeder
// Punkt spielt alle Ziele mit den Werten als Varianten im Speicher. Ergebnis: alle Punkte mit Score, der beste Punkt,
// eine Begründung und ein Diff zur Übernahme von Hand. data/ wird nie geschrieben.

// Modi des Abstimmens.
const (
	TuneSweep  = "sweep"
	TuneSearch = "search"
)

// Grenzen und Vorgaben der CLI-Flags --steps und --budget.
const (
	DefaultTuneSteps  = 5
	MaxTuneSteps      = 21
	DefaultTuneBudget = 40
	MaxTuneParams     = 2
)

// TuneOptions beschreibt einen Lauf. Seeds: die ersten n Seeds der Ziele (0 = alle). Variants gelten für jeden Punkt,
// die Parameter darüber. Progress (darf nil sein) erfährt nach jeder Auswertung fertige und höchstens alle.
type TuneOptions struct {
	Mode     string
	Params   []string // datei.json:a.b.c=min..max
	Steps    int
	Budget   int
	Seeds    int
	Variants []string
	Progress func(done, total int)
}

// TunePoint ist ein ausgewerteter Punkt. Score: Summe der normierten Abstände außerhalb der Korridore (0 = alle
// getroffen); Failed nennt die Ziele ohne Pass.
type TunePoint struct {
	Values []float64 `json:"values"`
	Score  float64   `json:"score"`
	Pass   bool      `json:"pass"`
	Failed []string  `json:"failed"`
}

// TuneBest ist der beste Punkt (kleinster Score, bei Gleichstand der erste).
type TuneBest struct {
	Values []float64 `json:"values"`
	Score  float64   `json:"score"`
	Pass   bool      `json:"pass"`
}

// Tuning ist der Inhalt von reports/…/tuning-<Zeit>.json.
type Tuning struct {
	Version int         `json:"version"`
	Mode    string      `json:"mode"`
	Params  []TuneParam `json:"params"`
	Points  []TunePoint `json:"points"`
	Best    TuneBest    `json:"best"`
	Reason  string      `json:"reason"`
	Diff    string      `json:"diff"`
}

// JSON schreibt das Ergebnis eingerückt mit Zeilenende.
func (t Tuning) JSON() ([]byte, error) {
	b, err := json.MarshalIndent(t, "", "  ")
	return append(b, '\n'), err
}

// tuner wertet Punkte aus, merkt sie (keine doppelte Auswertung) und hält Budget und Abbruch bei Score 0.
type tuner struct {
	t      *Targets
	o      TuneOptions
	params []TuneParam
	points []TunePoint
	sums   []Summary // je Punkt
	total  int
}

// Tune stimmt die Parameter gegen die Ziele ab. Ungültige Optionen oder Parameter sind Fehler, verfehlte Ziele nicht.
func (t *Targets) Tune(o TuneOptions) (Tuning, error) {
	tu, err := newTuner(t, o)
	if err != nil {
		return Tuning{}, err
	}
	if o.Mode == TuneSweep {
		err = tu.sweep()
	} else {
		err = tu.search()
	}
	if err != nil {
		return Tuning{}, err
	}
	return tu.result()
}

func newTuner(t *Targets, o TuneOptions) (*tuner, error) {
	if o.Steps == 0 {
		o.Steps = DefaultTuneSteps
	}
	if o.Budget == 0 {
		o.Budget = DefaultTuneBudget
	}
	switch {
	case o.Mode != TuneSweep && o.Mode != TuneSearch:
		return nil, fmt.Errorf("tune: modus %q, erwartet %s oder %s", o.Mode, TuneSweep, TuneSearch)
	case len(o.Params) < 1 || len(o.Params) > MaxTuneParams:
		return nil, fmt.Errorf("tune: 1 bis %d Parameter, waren %d", MaxTuneParams, len(o.Params))
	case o.Steps < 2 || o.Steps > MaxTuneSteps:
		return nil, fmt.Errorf("tune: steps 2 bis %d, war %d", MaxTuneSteps, o.Steps)
	case o.Budget < 1:
		return nil, fmt.Errorf("tune: budget ab 1, war %d", o.Budget)
	}
	tu := &tuner{t: t, o: o, total: o.Budget}
	for _, s := range o.Params {
		p, err := ParseTuneParam(s)
		if err != nil {
			return nil, err
		}
		tu.params = append(tu.params, p)
	}
	if o.Mode == TuneSweep {
		tu.total = min(o.Budget, int(math.Pow(float64(o.Steps), float64(len(tu.params)))))
	}
	return tu, nil
}

// done: Budget erschöpft oder ein Punkt trifft alle Korridore.
func (tu *tuner) done() bool {
	return len(tu.points) >= tu.o.Budget || slices.ContainsFunc(tu.points, func(p TunePoint) bool { return p.Score == 0 })
}

// eval wertet einen Punkt aus (gerundet wie in der Datei) und liefert seinen Score; ein bekannter Punkt kostet nichts.
func (tu *tuner) eval(values []float64) (float64, error) {
	vals := make([]float64, len(values))
	for i, p := range tu.params {
		vals[i] = p.norm(math.Min(p.Max, math.Max(p.Min, values[i])))
	}
	if i := slices.IndexFunc(tu.points, func(p TunePoint) bool { return slices.Equal(p.Values, vals) }); i >= 0 {
		return tu.points[i].Score, nil
	}
	if len(tu.points) >= tu.o.Budget {
		return math.Inf(1), nil
	}
	specs := slices.Clone(tu.o.Variants)
	for i, p := range tu.params {
		specs = append(specs, p.variant(vals[i]))
	}
	var rep Report
	err := WithVariants(specs, func() (err error) {
		rep, err = tu.t.RunTargets(tu.o.Seeds)
		return err
	})
	if err != nil {
		return 0, err
	}
	sum := tu.t.Summarize(rep)
	pt := TunePoint{Values: vals, Score: score(sum), Pass: sum.Pass, Failed: []string{}}
	for _, e := range sum.Targets {
		if !e.Pass {
			pt.Failed = append(pt.Failed, e.Name)
		}
	}
	tu.points, tu.sums = append(tu.points, pt), append(tu.sums, sum)
	if tu.o.Progress != nil {
		tu.o.Progress(len(tu.points), tu.total)
	}
	return pt.Score, nil
}

// score: Summe der normierten Abstände außerhalb der Korridore. Anteile durch 100 Prozentpunkte, Mediane durch die
// Korridorbreite (bei einer Grenze ihr Betrag, mindestens 1); ein ungültiges Ziel zählt 1. Gerundet auf 4 Stellen.
func score(s Summary) float64 {
	total := 0.0
	for _, e := range s.Targets {
		switch {
		case e.Status == StatusInvalid:
			total++
		case e.Lower != nil && e.Value < *e.Lower:
			total += (*e.Lower - e.Value) / e.scale()
		case e.Upper != nil && e.Value > *e.Upper:
			total += (e.Value - *e.Upper) / e.scale()
		}
	}
	return math.Round(total*1e4) / 1e4
}

func (e Entry) scale() float64 {
	if e.Kind == KindShare {
		return 100
	}
	if e.Lower != nil && e.Upper != nil && *e.Upper > *e.Lower {
		return *e.Upper - *e.Lower
	}
	b := e.Lower
	if b == nil {
		b = e.Upper
	}
	return math.Max(1, math.Abs(*b))
}

// grid: steps Werte gleichmäßig von lo bis hi.
func grid(lo, hi float64, steps int) []float64 {
	out := make([]float64, steps)
	for i := range out {
		out[i] = lo + (hi-lo)*float64(i)/float64(steps-1)
	}
	return out
}

// sweep wertet das volle Raster aus (höchstens Budget Punkte).
func (tu *tuner) sweep() error {
	return tu.evalGrid(tu.axes(nil, 0))
}

// axes: Rasterwerte je Parameter; mit center das Raster center ± cell (3 Werte je Achse), sonst min..max in steps.
func (tu *tuner) axes(center []float64, cell float64) [][]float64 {
	out := make([][]float64, len(tu.params))
	for i, p := range tu.params {
		if center == nil {
			out[i] = grid(p.Min, p.Max, tu.o.Steps)
			continue
		}
		d := cell * (p.Max - p.Min)
		out[i] = []float64{center[i] - d, center[i], center[i] + d}
	}
	return out
}

// evalGrid wertet alle Kombinationen der Achsen aus, bis das Budget erschöpft ist (Score 0 bricht nur bei search ab).
func (tu *tuner) evalGrid(axes [][]float64) error {
	combos := [][]float64{{}}
	for _, axis := range axes {
		var next [][]float64
		for _, c := range combos {
			for _, v := range axis {
				next = append(next, append(slices.Clone(c), v))
			}
		}
		combos = next
	}
	for _, c := range combos {
		if len(tu.points) >= tu.o.Budget || (tu.o.Mode == TuneSearch && tu.done()) {
			return nil
		}
		if _, err := tu.eval(c); err != nil {
			return err
		}
	}
	return nil
}

// search: ein Parameter Ternärsuche auf dem Score, zwei Parameter Raster und zwei Verfeinerungsrunden um den besten
// Punkt (halbe Zellbreite je Runde). Abbruch bei Score 0 oder Budget.
func (tu *tuner) search() error {
	if len(tu.params) == 1 {
		return tu.ternary()
	}
	if err := tu.evalGrid(tu.axes(nil, 0)); err != nil {
		return err
	}
	cell := 1 / float64(tu.o.Steps-1)
	for range 2 {
		cell /= 2
		if err := tu.evalGrid(tu.axes(tu.best().Values, cell)); err != nil {
			return err
		}
	}
	return nil
}

// ternary sucht das Minimum des Scores über [min, max]: erst beide Enden, dann je Runde die Drittelpunkte.
func (tu *tuner) ternary() error {
	p := tu.params[0]
	lo, hi := p.Min, p.Max
	for _, v := range []float64{lo, hi} {
		if _, err := tu.eval([]float64{v}); err != nil {
			return err
		}
	}
	for !tu.done() && hi-lo > (p.Max-p.Min)*1e-3 && !(p.Int && hi-lo < 1) {
		m1, m2 := lo+(hi-lo)/3, hi-(hi-lo)/3
		s1, err := tu.eval([]float64{m1})
		if err != nil {
			return err
		}
		s2, err := tu.eval([]float64{m2})
		if err != nil {
			return err
		}
		if s1 <= s2 {
			hi = m2
		} else {
			lo = m1
		}
	}
	return nil
}

func (tu *tuner) best() TunePoint {
	best := 0
	for i, p := range tu.points {
		if p.Score < tu.points[best].Score {
			best = i
		}
	}
	return tu.points[best]
}

func (tu *tuner) result() (Tuning, error) {
	out := Tuning{Version: 1, Mode: tu.o.Mode, Params: tu.params, Points: tu.points}
	if len(tu.points) == 0 {
		return out, fmt.Errorf("tune: kein Punkt ausgewertet")
	}
	b := tu.best()
	out.Best = TuneBest{Values: b.Values, Score: b.Score, Pass: b.Pass}
	out.Reason = tu.reason(b)
	diff, err := proposalDiff(tu.params, b.Values)
	out.Diff = diff
	return out, err
}

// reason erklärt den besten Punkt in 1–3 Sätzen: Werte, getroffene und verfehlte Korridore mit Abstand.
func (tu *tuner) reason(b TunePoint) string {
	sum := tu.sums[slices.IndexFunc(tu.points, func(p TunePoint) bool { return slices.Equal(p.Values, b.Values) })]
	var set []string
	for i, p := range tu.params {
		set = append(set, fmt.Sprintf("%s:%s = %s (bisher %s)", p.File, p.Path, p.format(b.Values[i]), p.format(p.Base)))
	}
	head := fmt.Sprintf("Bester von %d Punkten (%s): %s.", len(tu.points), tu.o.Mode, strings.Join(set, ", "))
	var missed []string
	for _, e := range sum.Targets {
		if !e.Pass {
			missed = append(missed, fmt.Sprintf("%s %s statt %s", e.Name, e.valueText(), e.boundsText()))
		}
	}
	hit := len(sum.Targets) - len(missed)
	if len(missed) == 0 {
		return fmt.Sprintf("%s Alle %d Korridore getroffen (Score 0).", head, hit)
	}
	return fmt.Sprintf("%s %d von %d Korridoren getroffen, Score %s. Verfehlt: %s.", head, hit, len(sum.Targets),
		num(b.Score), strings.Join(missed, "; "))
}
