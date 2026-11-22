package balance

import (
	"encoding/json"
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
