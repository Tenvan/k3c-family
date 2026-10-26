package main

import (
	"errors"
	"flag"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"slices"
	"time"

	"k3c/tools/k3c-workbench/internal/balance"
)

// targetOpts sind die Optionen des Ziel-Modus `--targets` (task balance, BAL2.2).
type targetOpts struct {
	on       bool
	seeds    int      // 0 = Seed-Modus der Datei (dev: nur der erste, sonst alle)
	seedList []string // --seed-list: genau diese Seeds statt der Datei (B-634)
	dir      string // Ordner für balance-<Zeit>.json und .md
	baseline string // Datei zum Vergleichen oder (write) zum Schreiben
	write    bool
	replays  string // Ordner für Replays der Seeds verletzter Ziele
	variants []string
	scope    balance.Scope
	depth    int
	bot      string
	grade    string
	world    string
	progress bool
	insights string // vorhandener balance-*.json: Auswertung (.md, .html) neu schreiben (B-632)
}

func (o *targetOpts) register(fs *flag.FlagSet) {
	fs.BoolVar(&o.on, "targets", false, "Ziele aus data/balance-targets.json bewerten, Bericht als JSON und Markdown; --seeds N = nur die ersten N, sonst alle")
	fs.StringVar(&o.dir, "report-dir", "reports", "Ordner für balance-<Zeit>.json und .md (--targets) bzw. sensitivity-<Zeit>.* (--sensitivity, --curves)")
	fs.StringVar(&o.baseline, "baseline", "", "Baseline-Datei zum Vergleichen (nur mit --targets)")
	fs.BoolVar(&o.write, "write-baseline", false, "Bericht nach --baseline schreiben statt vergleichen")
	fs.IntVar(&o.depth, "depth", 0, "nur mit --targets: Tiefe aller Zielläufe statt der des Ziels")
	fs.StringVar(&o.bot, "bot", "", "nur mit --targets: Bot aller Zielläufe statt des Ziel-Bots")
	fs.StringVar(&o.grade, "grade", "", "nur mit --targets: Schwierigkeitsgrad aller Zielläufe (leer = normal)")
	fs.StringVar(&o.world, "world", "", "nur mit --targets: Welt aller Zielläufe, z. B. standard; die Läufe spielen die ganze erste Insel (BS11)")
	fs.BoolVar(&o.progress, "progress", false, "nur mit --targets: nach jedem Lauf PROGRESS <fertig>/<gesamt> auf stdout")
	fs.StringVar(&o.insights, "insights", "", "vorhandenen Bericht balance-<Zeit>.json auswerten: .md und .html daneben neu schreiben (Baseline: "+balance.BaselinePath+")")
}

// prepare setzt Seeds und Szenario der Zielläufe aus den Flags.
func (o *targetOpts) prepare(fs *flag.FlagSet, n int, list, players string, days int) error {
	if err := o.seedsFrom(fs, n, list); err != nil {
		return err
	}
	return o.scopeFrom(fs, players, days)
}

// seedsFrom setzt die Seeds der Zielläufe: --seeds N (die ersten N) oder --seed-list (genau diese), sonst der
// Seed-Modus der Datei (B-634).
func (o *targetOpts) seedsFrom(fs *flag.FlagSet, n int, list string) error {
	o.seeds = explicitSeeds(fs, n)
	if list == "" {
		return nil
	}
	if o.seeds > 0 {
		return fmt.Errorf("--seeds und --seed-list schließen sich aus")
	}
	o.seedList = split(list)
	return nil
}

// scopeFrom setzt das Szenario der Zielläufe aus den gesetzten Flags --depth, --players, --days, --bot, --grade
// (--players und --days gelten ohne --targets weiter für die Matrix).
func (o *targetOpts) scopeFrom(fs *flag.FlagSet, players string, days int) error {
	o.scope = balance.Scope{Bot: o.bot, Grade: o.grade, World: o.world}
	if isSet(fs, "depth") {
		o.scope.Depth = &o.depth
	}
	if isSet(fs, "days") {
		o.scope.Days = &days
	}
	if isSet(fs, "players") {
		n, err := ints(players)
		if err != nil || len(n) != 1 {
			return fmt.Errorf("--players mit --targets braucht genau eine Zahl, war %q", players)
		}
		o.scope.Players = &n[0]
	}
	return nil
}

// runTargets spielt die Ziele aus data/balance-targets.json, schreibt Bericht (JSON, Markdown) und vergleicht mit der
// Baseline. Verletzte Ziele sind kein Fehler: das Pflicht-Gate gibt es nicht (BAL2).
func runTargets(o targetOpts) error {
	ts, err := o.targets()
	if err != nil {
		return err
	}
	start := time.Now()
	var report balance.Report
	err = balance.WithVariants(o.variants, func() (err error) {
		report, err = ts.RunTargetsProgress(o.seeds, o.onProgress())
		return err
	})
	if err != nil {
		return err
	}
	sum := ts.Summarize(report)
	sum.Varianten, sum.Kennzahlen = o.variants, balance.FiguresOf(report)
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
	if err := writeSummary(o.dir, sum, base, o.baseline); err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "k3c-balance: %d Läufe in %s\n", len(report.Results), time.Since(start).Round(time.Second))
	return nil
}

// onProgress schreibt mit --progress nach jedem Lauf eine Zeile für die Laufsteuerung (simrun).
func (o *targetOpts) onProgress() func(done, total int) {
	if !o.progress {
		return nil
	}
	return func(done, total int) { fmt.Printf("PROGRESS %d/%d\n", done, total) }
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

// writeSummary schreibt balance-<Zeit>.json, .md und .html (Auswertung B-632) nach dir und nennt die Pfade.
func writeSummary(dir string, sum balance.Summary, base *balance.Summary, baseName string) error {
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
	if err := balance.WriteReports(stem, sum, base, baseName, "."); err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "k3c-balance: Bericht %s.json, %s.md und %s.html\n", stem, stem, stem)
	return nil
}

// runInsights wertet einen vorhandenen Bericht neu aus (--insights); Grafiken aus public/ des aktuellen Ordners.
func runInsights(path string) error {
	page, err := balance.Reevaluate(path, ".")
	if err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "k3c-balance: Auswertung %s\n", page)
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

// targets lädt die Ziele mit dem Szenario aus den Flags und, mit --seed-list, genau diesen Seeds.
func (o targetOpts) targets() (*balance.Targets, error) {
	ts, err := balance.DefaultTargets()
	if err != nil {
		return nil, err
	}
	if ts, err = ts.Restrict(o.scope); err != nil {
		return nil, err
	}
	if len(o.seedList) > 0 {
		return ts.WithSeeds(o.seedList)
	}
	return ts, nil
}
