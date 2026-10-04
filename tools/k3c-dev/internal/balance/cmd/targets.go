package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"slices"
	"time"

	"k3c/tools/k3c-dev/internal/balance"
)

// targetOpts sind die Optionen des Ziel-Modus `--targets` (task balance, BAL2.2).
type targetOpts struct {
	on       bool
	seeds    int    // 0 = alle Seeds der Datei
	dir      string // Ordner für balance-<Zeit>.json und .md
	baseline string // Datei zum Vergleichen oder (write) zum Schreiben
	write    bool
	replays  string // Ordner für Replays der Seeds verletzter Ziele
}

// runTargets spielt die Ziele aus data/balance-targets.json, schreibt Bericht (JSON, Markdown) und vergleicht mit der
// Baseline. Verletzte Ziele sind kein Fehler: das Pflicht-Gate gibt es nicht (BAL2).
func runTargets(o targetOpts) error {
	ts, err := balance.DefaultTargets()
	if err != nil {
		return err
	}
	start := time.Now()
	report, err := ts.RunTargets(o.seeds)
	if err != nil {
		return err
	}
	sum := ts.Summarize(report)
	if o.write {
		return writeBaseline(o.baseline, sum)
	}
	base, err := readBaseline(o.baseline)
	if err != nil {
		return err
	}
	if err := writeReplays(onlyFailSeeds(report, sum), o.replays); err != nil {
		return err
	}
	if err := writeSummary(o.dir, sum, base); err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "k3c-balance: %d Läufe in %s\n", len(report.Results), time.Since(start).Round(time.Second))
	return nil
}

func writeBaseline(path string, sum balance.Summary) error {
	if path == "" {
		return errors.New("--write-baseline braucht --baseline DATEI")
	}
	b, err := sum.JSON()
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	return os.WriteFile(path, b, 0o644)
}

// readBaseline liefert nil, wenn keine Datei angegeben ist oder fehlt (der Bericht weist darauf hin).
func readBaseline(path string) (*balance.Summary, error) {
	if path == "" {
		return nil, nil
	}
	raw, err := os.ReadFile(path)
	if errors.Is(err, fs.ErrNotExist) {
		return nil, nil
	}
	if err != nil {
		return nil, err
	}
	s, err := balance.ReadSummary(raw)
	return &s, err
}

// writeSummary schreibt balance-<Zeit>.json und .md nach dir und nennt die Pfade.
func writeSummary(dir string, sum balance.Summary, base *balance.Summary) error {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}
	b, err := sum.JSON()
	if err != nil {
		return err
	}
	stem := filepath.Join(dir, "balance-"+time.Now().Format("20060102-150405"))
	if err := os.WriteFile(stem+".json", b, 0o644); err != nil {
		return err
	}
	if err := os.WriteFile(stem+".md", []byte(sum.Markdown(base)), 0o644); err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "k3c-balance: Bericht %s.json und %s.md\n", stem, stem)
	return nil
}

// onlyFailSeeds behält die Replays der Läufe, deren Seed bei einem verletzten Ziel genannt wird.
func onlyFailSeeds(r balance.Report, sum balance.Summary) balance.Report {
	var fail []string
	for _, e := range sum.Targets {
		fail = append(fail, e.FailSeeds...)
	}
	for i := range r.Results {
		if !slices.Contains(fail, r.Results[i].Seed) {
			r.Results[i].Replay = nil
		}
	}
	return r
}
