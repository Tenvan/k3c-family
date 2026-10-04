// Package balance ist der Balancing-Tester (B-099, BAL1): Er spielt eine Szenario-Matrix headless mit Bots und
// liefert Kennzahlen je Lauf als JSON. Die Simulation (engine/sim) wird nur benutzt; gleiche Matrix und gleiche
// Daten ergeben byte-gleiche Ausgabe (keine Wanduhr, keine Map-Iteration in der Ausgabe).
package balance

import (
	"encoding/json"
	"fmt"

	"k3c/engine/sim"
	"k3c/tools/k3c-dev/internal/enginetools"
)

// maxTicksPerDay begrenzt einen Lauf: eine Stunde Spielzeit je Tag (ein Tag dauert heute 16 min, data/biomes).
const maxTicksPerDay = 3600 * enginetools.TickHz

// Matrix beschreibt alle Läufe: jedes Paar aus Seed × Spieleranzahl × Bot × Tiefe läuft `Days` Tage.
type Matrix struct {
	Seeds   []string `json:"seeds"`
	Players []int    `json:"players"`
	Bots    []string `json:"bots"`
	Depths  []int    `json:"depths"` // Tiefe = Biom (0 Wald, 1 Höhle, …)
	Days    int      `json:"days"`
	// GoldThreshold: Schwelle für Metrics.GoldThresholdTick (Gold aller Monarchen zusammen); 0 = nicht messen.
	GoldThreshold int `json:"goldThreshold"`
}

// Scenario ist ein einzelner Lauf.
type Scenario struct {
	Seed    string `json:"seed"`
	Players int    `json:"players"`
	Bot     string `json:"bot"`
	Depth   int    `json:"depth"`
	Days    int    `json:"days"`
}

// Result ist ein Lauf mit Kennzahlen; ein ungültiger Lauf hat Error statt Metrics.
type Result struct {
	Scenario
	Valid   bool     `json:"valid"`
	Error   string   `json:"error,omitempty"`
	Ticks   int      `json:"ticks"`
	Metrics *Metrics `json:"metrics"`
}

// Report ist die Ausgabe: die Matrix und alle Läufe in der Reihenfolge der Matrix.
type Report struct {
	Matrix  Matrix   `json:"matrix"`
	Results []Result `json:"results"`
}

// Scenarios prüft die Matrix und zählt die Läufe auf (Seeds außen, Tiefe innen).
func (m Matrix) Scenarios() ([]Scenario, error) {
	if len(m.Seeds) == 0 || len(m.Players) == 0 || len(m.Bots) == 0 || len(m.Depths) == 0 {
		return nil, fmt.Errorf("matrix: Seeds, Spieler, Bots und Tiefen brauchen je mindestens einen Wert")
	}
	if m.Days < 1 {
		return nil, fmt.Errorf("matrix: days muss mindestens 1 sein, war %d", m.Days)
	}
	for _, n := range m.Players {
		if n < 1 || n > 4 {
			return nil, fmt.Errorf("matrix: Spieleranzahl muss 1 bis 4 sein, war %d", n)
		}
	}
	for _, b := range m.Bots {
		if bots[b] == nil {
			return nil, fmt.Errorf("matrix: unbekannter Bot %q (bekannt: %v)", b, BotNames())
		}
	}
	var out []Scenario
	for _, seed := range m.Seeds {
		for _, n := range m.Players {
			for _, b := range m.Bots {
				for _, d := range m.Depths {
					out = append(out, Scenario{Seed: seed, Players: n, Bot: b, Depth: d, Days: m.Days})
				}
			}
		}
	}
	return out, nil
}

// Run spielt alle Läufe der Matrix nacheinander. Ein abgebrochener Lauf erscheint als ungültig, der Rest läuft weiter.
func Run(m Matrix) (Report, error) {
	scenarios, err := m.Scenarios()
	if err != nil {
		return Report{}, err
	}
	r := Report{Matrix: m, Results: make([]Result, 0, len(scenarios))}
	for _, sc := range scenarios {
		r.Results = append(r.Results, runOne(sc, m.GoldThreshold))
	}
	return r, nil
}

// JSON schreibt den Bericht eingerückt mit Zeilenende; Feldreihenfolge aus den Structs.
func (r Report) JSON() ([]byte, error) {
	b, err := json.MarshalIndent(r, "", "  ")
	return append(b, '\n'), err
}

// runOne rechnet einen Lauf bis zum Morgen nach dem letzten Tag (Cycle.Day > Days).
func runOne(sc Scenario, threshold int) (res Result) {
	res = Result{Scenario: sc}
	defer func() {
		if p := recover(); p != nil {
			res.Valid, res.Metrics, res.Error = false, nil, fmt.Sprintf("abgebrochen: %v", p)
		}
	}()
	isl, err := sim.CreateIsland(sc.Seed, []int{sc.Depth}, 1)
	if err != nil {
		res.Error = err.Error()
		return res
	}
	for range sc.Players {
		sim.AddIslandPlayer(isl, 0)
	}
	bot, c := bots[sc.Bot], newCollector(isl, threshold)
	cmds := make([]sim.PlayerCommand, sc.Players)
	for tick := 1; tick <= sc.Days*maxTicksPerDay; tick++ {
		for _, w := range isl.Stages {
			for _, p := range w.Players {
				cmds[p.Index] = bot(w, p)
			}
		}
		sim.StepIsland(isl, cmds, 1.0/enginetools.TickHz)
		c.observe(tick)
		res.Ticks = tick
		if isl.Stages[0].Cycle.Day > sc.Days {
			res.Valid, res.Metrics = true, c.result()
			return res
		}
	}
	res.Error = fmt.Sprintf("Tag %d nach %d Ticks nicht beendet", sc.Days, res.Ticks)
	return res
}
