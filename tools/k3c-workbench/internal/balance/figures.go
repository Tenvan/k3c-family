package balance

import (
	"math"
	"slices"
)

// Kennzahlen im Bericht von `--targets` (WZ28.4, B-596): verdichtet über alle gültigen Läufe des Berichts, je Grad
// und Nacht. Beschreibung: docs/rules/zielkorridore.md › Kennzahlen im Bericht; Korridore gibt es dafür (noch) nicht.

// Figures sind die Kennzahlen eines Berichts. Mediane über die Läufe, in denen es den Zeitpunkt gab (null: in keinem).
type Figures struct {
	Runs                    int            `json:"runs"` // gültige Läufe
	FirstWallTickMedian     *float64       `json:"firstWallTickMedian"`
	FirstWorkshopTickMedian *float64       `json:"firstWorkshopTickMedian"`
	Nights                  []NightFigures `json:"nights"`
}

// NightFigures: Mittelwerte je Lauf für Tag n samt Nacht n eines Grads (leer = normal). Survival ist der Anteil der
// Läufe in %, in denen die Burg in diesem Tag nicht fiel; IdleShare der Anteil wartender Bauern-Ticks in %.
type NightFigures struct {
	Grade        string    `json:"grade"`
	Night        int       `json:"night"`
	Runs         int       `json:"runs"`
	Survival     float64   `json:"survival"`
	DayIncome    float64   `json:"dayIncome"`
	DayExpense   float64   `json:"dayExpense"`
	NightIncome  float64   `json:"nightIncome"`
	NightExpense float64   `json:"nightExpense"`
	ResourceIn   Resources `json:"resourceIn"`
	ResourceOut  Resources `json:"resourceOut"`
	IdleShare    float64   `json:"idleShare"`
}

// Resources ist ein Rohstoff-Fluss als Mittelwert je Lauf.
type Resources struct {
	Wood    float64 `json:"wood"`
	Stone   float64 `json:"stone"`
	Copper  float64 `json:"copper"`
	Iron    float64 `json:"iron"`
	Crystal float64 `json:"crystal"`
}

// FiguresOf verdichtet die gültigen Läufe; nil ohne gültigen Lauf. Reihenfolge: Grad wie im Bericht, dann Nacht.
func FiguresOf(r Report) *Figures {
	f := &Figures{Nights: []NightFigures{}}
	var walls, shops []float64
	var idle, ticks []int // je Eintrag in f.Nights
	for _, res := range r.Results {
		if !res.Valid || res.Metrics == nil {
			continue
		}
		f.Runs++
		m := res.Metrics
		walls, shops = appendTick(walls, m.FirstWallTick), appendTick(shops, m.FirstWorkshopTick)
		for i := range min(len(m.Days), res.Days) {
			at := f.night(res.Grade, i+1)
			if at == len(idle) {
				idle, ticks = append(idle, 0), append(ticks, 0)
			}
			f.Nights[at].add(m.Days[i])
			idle[at] += m.Days[i].PeasantIdleTicks
			ticks[at] += m.Days[i].PeasantTicks
		}
	}
	if f.Runs == 0 {
		return nil
	}
	f.FirstWallTickMedian, f.FirstWorkshopTickMedian = medianOrNil(walls), medianOrNil(shops)
	for i := range f.Nights {
		f.Nights[i].finish(idle[i], ticks[i])
	}
	return f
}

// night liefert den Index der Zeile (Grad, Nacht) und legt sie bei Bedarf an.
func (f *Figures) night(grade string, n int) int {
	if i := slices.IndexFunc(f.Nights, func(x NightFigures) bool { return x.Grade == grade && x.Night == n }); i >= 0 {
		return i
	}
	f.Nights = append(f.Nights, NightFigures{Grade: grade, Night: n})
	return len(f.Nights) - 1
}

// add summiert einen Tag; finish teilt durch die Zahl der Läufe.
func (n *NightFigures) add(d DayMetrics) {
	n.Runs++
	n.Survival += b2f(!d.CastleFallen)
	n.DayIncome += float64(d.Income - d.NightIncome)
	n.DayExpense += float64(d.Expense - d.NightExpense)
	n.NightIncome += float64(d.NightIncome)
	n.NightExpense += float64(d.NightExpense)
	n.ResourceIn.add(d.StockIn.Wood, d.StockIn.Stone, d.StockIn.Copper, d.StockIn.Iron, d.StockIn.Crystal)
	n.ResourceOut.add(d.StockOut.Wood, d.StockOut.Stone, d.StockOut.Copper, d.StockOut.Iron, d.StockOut.Crystal)
}

func (n *NightFigures) finish(idle, ticks int) {
	k := float64(n.Runs)
	n.Survival = round1(100 * n.Survival / k)
	for _, p := range []*float64{&n.DayIncome, &n.DayExpense, &n.NightIncome, &n.NightExpense} {
		*p = round1(*p / k)
	}
	n.ResourceIn.div(k)
	n.ResourceOut.div(k)
	if ticks > 0 {
		n.IdleShare = round1(100 * float64(idle) / float64(ticks))
	}
}

func (r *Resources) add(wood, stone, copper, iron, crystal int) {
	r.Wood += float64(wood)
	r.Stone += float64(stone)
	r.Copper += float64(copper)
	r.Iron += float64(iron)
	r.Crystal += float64(crystal)
}

func (r *Resources) div(k float64) {
	for _, p := range []*float64{&r.Wood, &r.Stone, &r.Copper, &r.Iron, &r.Crystal} {
		*p = round1(*p / k)
	}
}

func appendTick(v []float64, tick *int) []float64 {
	if tick == nil {
		return v
	}
	return append(v, float64(*tick))
}

// round1 rundet auf eine Nachkommastelle (stabiles JSON).
func round1(x float64) float64 { return math.Round(x*10) / 10 }
