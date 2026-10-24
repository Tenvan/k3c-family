package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"k3c/tools/k3c-workbench/internal/balance"
)

// tuneOpts sind die Optionen des automatischen Balancings (WZ28.6): --tune sweep|search mit --targets.
type tuneOpts struct {
	mode   string
	params paramList
	steps  int
	budget int
}

// paramList sammelt --param (mehrfach).
type paramList []string

func (p *paramList) String() string     { return strings.Join(*p, ",") }
func (p *paramList) Set(v string) error { *p = append(*p, v); return nil }

func (o *tuneOpts) register(fs *flag.FlagSet) {
	fs.StringVar(&o.mode, "tune", "", "nur mit --targets: Werte abstimmen, sweep (volles Raster) oder search (Suche); Ergebnis tuning-<Zeit>.json")
	fs.Var(&o.params, "param", "mit --tune: datei.json:pfad=min..max (1 oder 2 mal)")
	fs.IntVar(&o.steps, "steps", balance.DefaultTuneSteps, fmt.Sprintf("mit --tune: Rasterpunkte je Parameter (2 bis %d)", balance.MaxTuneSteps))
	fs.IntVar(&o.budget, "budget", balance.DefaultTuneBudget, "mit --tune: höchstens so viele Auswertungen")
}

// runTune stimmt die Parameter gegen die Ziele ab und schreibt tuning-<Zeit>.json nach --report-dir. data/ bleibt
// unverändert; verfehlte Ziele sind kein Fehler.
func runTune(o targetOpts, tu tuneOpts) error {
	ts, err := balance.DefaultTargets()
	if err != nil {
		return err
	}
	if ts, err = ts.Restrict(o.scope); err != nil {
		return err
	}
	start := time.Now()
	res, err := ts.Tune(balance.TuneOptions{Mode: tu.mode, Params: tu.params, Steps: tu.steps, Budget: tu.budget,
		Seeds: o.seeds, Variants: o.variants, Progress: o.onProgress()})
	if err != nil {
		return err
	}
	if err := os.MkdirAll(o.dir, 0o755); err != nil {
		return err
	}
	b, err := res.JSON()
	if err != nil {
		return err
	}
	path := filepath.Join(o.dir, "tuning-"+time.Now().Format("20060102-150405")+".json")
	if err := os.WriteFile(path, b, 0o644); err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "k3c-balance: %d Punkte in %s, Ergebnis %s\n%s\n", len(res.Points),
		time.Since(start).Round(time.Second), path, res.Reason)
	return nil
}
