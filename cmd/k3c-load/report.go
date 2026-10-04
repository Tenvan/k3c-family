package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// Bewertungen gegen das Ziel (B-175, Ziel aus B-042): erreicht = größter p99 unter dem Ziel; knapp = Mittel unter dem Ziel,
// aber ein Wert darüber; verfehlt = Mittel nicht unter dem Ziel; ohneDaten = keine Probe mit Tick-Dauer.
const (
	erreicht  = "erreicht"
	knapp     = "knapp"
	verfehlt  = "verfehlt"
	ohneDaten = "keine Daten"
)

// sample ist eine Probe aus /api/status für einen Raum.
type sample struct {
	AtS      float64  `json:"atS"` // Sekunden seit Start der Messung
	Phase    string   `json:"phase"`
	Tick     int      `json:"tick"`
	TickLast float64  `json:"tickMsLast"`
	TickP99  float64  `json:"tickMsP99"`
	CPU      *float64 `json:"cpu,omitempty"` // Prozess des Servers, Prozent einer CPU; fehlt, wo der Server keine Quelle hat
}

// row ist eine Zeile der Tabelle: ein Raum in einer Phase.
type row struct {
	Room, Phase             string
	Samples                 int
	P99Min, P99Mean, P99Max float64
	CPUMean, CPUPeak        *float64
	Verdict                 string
}

// roomReport ist die Zeitreihe und die Zeilen eines Raums.
type roomReport struct {
	Room    string   `json:"room"`
	Code    string   `json:"code"`
	Samples []sample `json:"samples"`
	Rows    []row    `json:"rows"`
}

type report struct {
	Start    time.Time    `json:"start"`
	URL      string       `json:"url"`
	Seed     string       `json:"seed"`
	Players  int          `json:"players"`
	TargetMs float64      `json:"targetP99Ms"`
	Verdict  string       `json:"verdict"`
	Rooms    []roomReport `json:"rooms"`
}

// evaluate bewertet p99-Werte (ms) gegen das Ziel.
func evaluate(p99 []float64, target float64) string {
	if len(p99) == 0 {
		return ohneDaten
	}
	var sum, hi float64
	for _, v := range p99 {
		sum += v
		hi = max(hi, v)
	}
	switch {
	case hi < target:
		return erreicht
	case sum/float64(len(p99)) < target:
		return knapp
	}
	return verfehlt
}

// exitCode: 0 erreicht oder knapp, 1 verfehlt, 2 keine Daten.
func exitCode(verdict string) int {
	switch verdict {
	case verfehlt:
		return 1
	case ohneDaten:
		return 2
	}
	return 0
}

// worst ist die schlechteste Bewertung der Zeilen.
func worst(verdicts ...string) string {
	rank := map[string]int{erreicht: 0, knapp: 1, verfehlt: 2, ohneDaten: 3}
	out := erreicht
	for _, v := range verdicts {
		if rank[v] > rank[out] {
			out = v
		}
	}
	if len(verdicts) == 0 {
		return ohneDaten
	}
	return out
}

// rowsOf gruppiert die Proben eines Raums nach Phase (in der Reihenfolge des ersten Auftretens).
func rowsOf(name string, samples []sample, target float64) []row {
	var order []string
	by := map[string][]sample{}
	for _, s := range samples {
		if s.TickP99 <= 0 {
			continue // Raum hat noch nicht getickt
		}
		if _, ok := by[s.Phase]; !ok {
			order = append(order, s.Phase)
		}
		by[s.Phase] = append(by[s.Phase], s)
	}
	var rows []row
	for _, ph := range order {
		r := row{Room: name, Phase: ph, Samples: len(by[ph]), P99Min: by[ph][0].TickP99}
		var p99s []float64
		var cpuSum float64
		var cpuN int
		for _, s := range by[ph] {
			p99s = append(p99s, s.TickP99)
			r.P99Min, r.P99Max, r.P99Mean = min(r.P99Min, s.TickP99), max(r.P99Max, s.TickP99), r.P99Mean+s.TickP99
			if s.CPU != nil {
				cpuSum, cpuN = cpuSum+*s.CPU, cpuN+1
				peak := max(*s.CPU, ptrOr(r.CPUPeak, 0))
				r.CPUPeak = &peak
			}
		}
		r.P99Mean /= float64(len(p99s))
		if cpuN > 0 {
			m := cpuSum / float64(cpuN)
			r.CPUMean = &m
		}
		r.Verdict = evaluate(p99s, target)
		rows = append(rows, r)
	}
	return rows
}

func ptrOr(p *float64, def float64) float64 {
	if p == nil {
		return def
	}
	return *p
}

// buildReport fasst die Läufe zum Bericht zusammen; Verdict ist die schlechteste Zeile.
func buildReport(c config, start time.Time, runs []*roomRun) report {
	rep := report{Start: start, URL: c.url, Seed: c.seed, Players: c.players, TargetMs: c.target}
	var verdicts []string
	for _, r := range runs {
		rr := roomReport{Room: r.Name, Code: r.Code, Samples: r.Samples, Rows: rowsOf(r.Name, r.Samples, c.target)}
		for _, w := range rr.Rows {
			verdicts = append(verdicts, w.Verdict)
		}
		rep.Rooms = append(rep.Rooms, rr)
	}
	rep.Verdict = worst(verdicts...)
	return rep
}

func (r report) markdown() string {
	var b strings.Builder
	fmt.Fprintf(&b, "# Lasttest %s\n\n", r.Start.Format("2006-01-02 15:04:05"))
	fmt.Fprintf(&b, "Server %s, %d Räume × %d Bots, Seed `%s`, Ziel: p99 < %g ms.\n\n", r.URL, len(r.Rooms), r.Players, r.Seed, r.TargetMs)
	b.WriteString("| Raum | Phase | Proben | p99 min (ms) | p99 Mittel (ms) | p99 max (ms) | CPU Mittel (%) | CPU Spitze (%) | Bewertung |\n")
	b.WriteString("|---|---|---|---|---|---|---|---|---|\n")
	for _, rr := range r.Rooms {
		for _, w := range rr.Rows {
			fmt.Fprintf(&b, "| %s | %s | %d | %.2f | %.2f | %.2f | %s | %s | %s |\n", w.Room, w.Phase, w.Samples,
				w.P99Min, w.P99Mean, w.P99Max, fmtPtr(w.CPUMean), fmtPtr(w.CPUPeak), w.Verdict)
		}
	}
	fmt.Fprintf(&b, "\n**Gesamt: %s**\n", r.Verdict)
	return b.String()
}

func fmtPtr(p *float64) string {
	if p == nil {
		return "–"
	}
	return fmt.Sprintf("%.1f", *p)
}

// write schreibt <base>.json und <base>.md und liefert die Pfade.
func (r report) write(base string) ([]string, error) {
	if err := os.MkdirAll(filepath.Dir(base), 0o755); err != nil {
		return nil, err
	}
	data, err := json.MarshalIndent(r, "", "  ")
	if err != nil {
		return nil, err
	}
	files := map[string][]byte{base + ".json": data, base + ".md": []byte(r.markdown())}
	paths := []string{base + ".json", base + ".md"}
	for _, p := range paths {
		if err := os.WriteFile(p, files[p], 0o644); err != nil {
			return nil, err
		}
	}
	return paths, nil
}
