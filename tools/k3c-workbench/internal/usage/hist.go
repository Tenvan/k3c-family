package usage

import (
	"encoding/json"
	"math"
	"sort"
)

const (
	// histBase ist der Faktor zwischen zwei Bucket-Grenzen; das geometrische Mittel eines Buckets liegt höchstens
	// um den Faktor √1,25 (rund 12 %) neben jedem Wert darin.
	histBase = 1.25
	// PercentileErrorPct ist die Genauigkeit der Perzentile in Prozent (√histBase − 1, gerundet); die Oberfläche
	// nennt sie in der Erklärzeile der Statistik.
	PercentileErrorPct = 12
	// maxValues begrenzt verschiedene Argumente bzw. Fehlermeldungen je Tool; der Rest zählt unter overflowKey.
	maxValues   = 50
	overflowKey = "(weitere)"
	// valueRunes kürzt Argumente und Fehlermeldungen.
	valueRunes = 120
)

// durAgg sammelt Laufzeiten: Anzahl, Summe, Maximum und Histogramm. Die JSON-Namen gelten auch für die Datei (M2.2).
type durAgg struct {
	Calls int         `json:"calls"`
	SumMs float64     `json:"sumMs"`
	MaxMs float64     `json:"maxMs"`
	Hist  map[int]int `json:"hist"`
}

func (d *durAgg) add(ms float64) {
	if d.Hist == nil {
		d.Hist = map[int]int{}
	}
	d.Calls++
	d.SumMs += ms
	d.MaxMs = math.Max(d.MaxMs, ms)
	d.Hist[histIndex(ms)]++
}

func (d *durAgg) merge(o *durAgg) {
	if d.Hist == nil {
		d.Hist = map[int]int{}
	}
	d.Calls += o.Calls
	d.SumMs += o.SumMs
	d.MaxMs = math.Max(d.MaxMs, o.MaxMs)
	for k, v := range o.Hist {
		d.Hist[k] += v
	}
}

func (d *durAgg) avg() float64 {
	if d.Calls == 0 {
		return 0
	}
	return d.SumMs / float64(d.Calls)
}

// p liefert das Perzentil q (0..1) aus dem Histogramm, nie über dem echten Maximum.
func (d *durAgg) p(q float64) float64 {
	if d.Calls == 0 {
		return 0
	}
	keys := make([]int, 0, len(d.Hist))
	for k := range d.Hist {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	rank := int(math.Ceil(q * float64(d.Calls)))
	seen := 0
	for _, k := range keys {
		if seen += d.Hist[k]; seen >= rank {
			return math.Min(histValue(k), d.MaxMs)
		}
	}
	return d.MaxMs
}

// histIndex ist der Bucket einer Dauer: (1,25^(i-1), 1,25^i]; alles bis 1 ms liegt in Bucket 0.
func histIndex(ms float64) int {
	if ms <= 1 {
		return 0
	}
	return int(math.Ceil(math.Log(ms) / math.Log(histBase)))
}

// histValue ist der Schätzwert eines Buckets: das geometrische Mittel seiner Grenzen.
func histValue(i int) float64 {
	if i <= 0 {
		return 0.5
	}
	return math.Pow(histBase, float64(i)-0.5)
}

// normArgs macht gleiche Argumente gleich: Schlüssel sortiert, kompakt, gekürzt. Kein JSON bleibt Text.
func normArgs(raw string) string {
	if raw == "" || raw == "null" {
		return "{}"
	}
	var v any
	if err := json.Unmarshal([]byte(raw), &v); err == nil {
		if b, err := json.Marshal(v); err == nil {
			raw = string(b)
		}
	}
	return clip(raw)
}

func clip(s string) string {
	r := []rune(s)
	if len(r) <= valueRunes {
		return s
	}
	return string(r[:valueRunes-1]) + "…"
}

// bump zählt einen Wert und liefert den Schlüssel, unter dem er zählt: ab maxValues verschiedenen Werten overflowKey.
func bump(m map[string]int, value string) string {
	if _, ok := m[value]; !ok && len(m) >= maxValues {
		value = overflowKey
	}
	m[value]++
	return value
}
