package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/balance"
)

// sensOpts sind die Optionen von `--sensitivity` und `--curves` (task balance:sensitivity, BAL3.3).
type sensOpts struct {
	vary, curves bool
	paths        []string // leer = balance.DefaultPaths
	seeds        int      // 0 = alle Seeds aus data/balance-targets.json
	dir          string
}

func (o *sensOpts) register(fs *flag.FlagSet) {
	fs.BoolVar(&o.vary, "sensitivity", false, "Sensitivität: Werte aus data/ um −25/−10/+10/+25 % ändern (nur im Speicher), "+
		"gekippte Ziele nach reports/sensitivity-<Zeit>.json und .md; --seeds N = nur die ersten N Seeds")
	fs.Func("vary", "Pfad datei.json:a.b.c zum Variieren (setzt --sensitivity), mehrfach oder mit Komma (Standard: "+
		strings.Join(balance.DefaultPaths, ", ")+")", func(s string) error {
		o.vary, o.paths = true, append(o.paths, split(s)...)
		return nil
	})
	fs.BoolVar(&o.curves, "curves", false, "Grad-Kurven je Schwierigkeitsgrad (Nacht 1–5, Wald, 2 Spieler, saver) in den Sensitivitäts-Bericht")
}

// runSensitivity rechnet Variationen und/oder Grad-Kurven und schreibt sensitivity-<Zeit>.json und .md nach dir.
func runSensitivity(o sensOpts) error {
	ts, err := balance.DefaultTargets()
	if err != nil {
		return err
	}
	opt := balance.SensitivityOptions{Seeds: o.seeds}
	if o.vary {
		opt.Paths = o.paths
		if len(opt.Paths) == 0 {
			opt.Paths = balance.DefaultPaths
		}
	}
	if o.curves {
		opt.Nights = balance.CurveNights
	}
	start := time.Now()
	rep, err := ts.Sensitivity(opt)
	if err != nil {
		return err
	}
	b, err := rep.JSON()
	if err != nil {
		return err
	}
	if err := os.MkdirAll(o.dir, 0o755); err != nil {
		return err
	}
	stem := filepath.Join(o.dir, "sensitivity-"+time.Now().Format("20060102-150405"))
	if err := os.WriteFile(stem+".json", b, 0o644); err != nil {
		return err
	}
	if err := os.WriteFile(stem+".md", []byte(rep.Markdown()), 0o644); err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "k3c-balance: Sensitivität mit %d Seeds in %s, Bericht %s.json und %s.md\n",
		rep.Seeds, time.Since(start).Round(time.Second), stem, stem)
	return nil
}
