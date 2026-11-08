package main

import (
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"k3c/engine/balance"
)

// Unterbefehl balance: dieselben Flags und Berichte wie bin/k3c-balance (tools/k3c-workbench/internal/balance/cmd),
// Fortschritt und Ergebnis als JSON-Zeilen (main.go).

func cmdBalance(args []string, v env) error {
	args, err := balanceArgs(args, v.stdin)
	if err != nil {
		return err
	}
	fs := flag.NewFlagSet("k3c-sim balance", flag.ContinueOnError)
	fs.SetOutput(io.Discard) // Fehler meldet run genau einmal; -h schreibt die Flags unten nach stderr
	var mo matrixOpts
	mo.register(fs)
	var to targetOpts
	to.register(fs)
	var so sensOpts
	so.register(fs)
	var tu tuneOpts
	tu.register(fs)
	if err := fs.Parse(args); errors.Is(err, flag.ErrHelp) {
		fs.SetOutput(v.stderr)
		fs.PrintDefaults()
		return nil
	} else if err != nil {
		return usageError{err}
	}
	if fs.NArg() > 0 {
		return usagef("unerwartete Argumente: %v", fs.Args())
	}
	to.v = v
	if done, err := early(to, so, tu); done || err != nil {
		return err
	}
	if so.vary || so.curves {
		so.dir, so.seeds = to.dir, explicitSeeds(fs, mo.seeds)
		return runSensitivity(so, v)
	}
	if to.on {
		to.replays, to.variants = mo.replayDir, so.variants
		if err := to.prepare(fs, mo.seeds, mo.seedList, mo.players, mo.days); err != nil {
			return usageError{err}
		}
		if tu.mode != "" {
			return runTune(to, tu)
		}
		return runTargets(to)
	}
	if mo.play != "" {
		return playFile(mo.play, mo.out, v)
	}
	return runMatrix(mo, v)
}

// matrixOpts sind die Flags der Szenario-Matrix (ohne --targets) und die mit --targets geteilten.
type matrixOpts struct {
	seeds                           int
	seedList, players, bots, depths string
	days, threshold                 int
	out, replayDir, play            string
}

func (o *matrixOpts) register(fs *flag.FlagSet) {
	fs.IntVar(&o.seeds, "seeds", 10, "Anzahl Seeds (feste Liste 1..N)")
	fs.StringVar(&o.seedList, "seed-list", "", "Seeds mit Komma statt 1..N, z. B. a,b,12")
	fs.StringVar(&o.players, "players", "2", "Spieleranzahlen mit Komma (1–4)")
	fs.StringVar(&o.bots, "bots", "saver", "Bot-Profile mit Komma: "+strings.Join(balance.BotNames(), ", "))
	fs.StringVar(&o.depths, "depths", "0", "Tiefen mit Komma (0 Wald, 1 Höhle, …)")
	fs.IntVar(&o.days, "days", 5, "Tage je Lauf")
	fs.IntVar(&o.threshold, "gold-threshold", 0, "Gold-Schwelle aller Monarchen zusammen (0 = nicht messen)")
	fs.StringVar(&o.out, "out", "", "Datei für das JSON (leer = JSON-Zeile result auf stdout)")
	fs.StringVar(&o.replayDir, "replay-dir", "", "Ordner für eine Replay-Datei je gültigem Lauf (leer = keine)")
	fs.StringVar(&o.play, "play", "", "Replay-Datei ohne Bot abspielen statt einer Matrix")
}

// runMatrix spielt die Szenario-Matrix (ohne --targets) und schreibt den Bericht nach --out oder als JSON-Zeile.
func runMatrix(o matrixOpts, v env) error {
	m := balance.Matrix{Seeds: seedNames(o.seeds, o.seedList), Bots: split(o.bots), Days: o.days, GoldThreshold: o.threshold}
	var err error
	if m.Players, err = ints(o.players); err != nil {
		return usageError{err}
	}
	if m.Depths, err = ints(o.depths); err != nil {
		return usageError{err}
	}
	report, err := balance.Run(m)
	if err != nil {
		return err
	}
	if err := writeReplays(report, o.replayDir); err != nil {
		return err
	}
	b, err := report.JSON()
	if err != nil {
		return err
	}
	return write(o.out, b, v)
}

// early prüft die Flags und erledigt --insights (vorhandenen Bericht auswerten, B-632); done: nichts weiter tun.
func early(to targetOpts, so sensOpts, tu tuneOpts) (done bool, err error) {
	if err := checkFlags(to, so, tu); err != nil {
		return true, usageError{err}
	}
	if to.insights != "" {
		return true, runInsights(to.insights, to.v)
	}
	return false, nil
}

// checkFlags lehnt Flags ab, die --targets bzw. --tune brauchen.
func checkFlags(to targetOpts, so sensOpts, tu tuneOpts) error {
	switch {
	case len(so.variants) > 0 && !to.on:
		return fmt.Errorf("--vary datei.json:pfad=wert braucht --targets")
	case to.world != "" && !to.on:
		return fmt.Errorf("--world braucht --targets")
	case to.helps != "" && !to.on:
		return fmt.Errorf("--helps braucht --targets")
	case (tu.mode != "" || len(tu.params) > 0) && !to.on:
		return fmt.Errorf("--tune und --param brauchen --targets")
	case len(tu.params) > 0 && tu.mode == "":
		return fmt.Errorf("--param braucht --tune sweep|search")
	case to.write && to.baseline == "":
		return fmt.Errorf("--write-baseline braucht --baseline DATEI")
	}
	return nil
}

// explicitSeeds: n, wenn --seeds gesetzt ist, sonst 0 (= alle Seeds aus data/balance-targets.json).
func explicitSeeds(fs *flag.FlagSet, n int) int {
	if isSet(fs, "seeds") {
		return n
	}
	return 0
}

// isSet: Das Flag steht in den Argumenten.
func isSet(fs *flag.FlagSet, name string) bool {
	set := false
	fs.Visit(func(f *flag.Flag) {
		set = set || f.Name == name
	})
	return set
}

// write schreibt nach out (Bytes wie bin/k3c-balance) oder, ohne Datei, als JSON-Zeile {"event":"result","report":…}.
func write(out string, b []byte, v env) error {
	if out != "" {
		return os.WriteFile(out, b, 0o644)
	}
	if !json.Valid(b) {
		return fmt.Errorf("bericht ist kein JSON")
	}
	v.out.emit(map[string]any{"event": "result", "report": json.RawMessage(b)})
	return nil
}

// writeReplays schreibt je gültigem Lauf <seed>-p<spieler>-<bot>-d<tiefe>.replay.json nach dir.
func writeReplays(r balance.Report, dir string) error {
	if dir == "" {
		return nil
	}
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}
	for _, res := range r.Results {
		if res.Replay == nil {
			continue
		}
		b, err := res.Replay.JSON()
		if err != nil {
			return err
		}
		name := fmt.Sprintf("%s-p%d-%s-d%d.replay.json", res.Seed, res.Players, res.Bot, res.Depth)
		if err := os.WriteFile(filepath.Join(dir, name), b, 0o644); err != nil {
			return err
		}
	}
	return nil
}

// playFile spielt eine Replay-Datei ab und schreibt Ticks, Endzustand-Hash, Burgfall-Tick und Warnungen als JSON.
func playFile(path, out string, v env) error {
	raw, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	r, err := balance.ReadReplay(raw)
	if err != nil {
		return fmt.Errorf("%s: %w", path, err)
	}
	pb, err := balance.Play(r)
	if err != nil {
		return err
	}
	for _, w := range pb.Warnings {
		v.logf("🚫 k3c-sim: Warnung: %s", w)
	}
	b, err := json.MarshalIndent(pb, "", "  ")
	if err != nil {
		return err
	}
	return write(out, append(b, '\n'), v)
}

func seedNames(n int, list string) []string {
	if list != "" {
		return split(list)
	}
	seeds := make([]string, n)
	for i := range seeds {
		seeds[i] = strconv.Itoa(i + 1)
	}
	return seeds
}

func split(s string) []string {
	var out []string
	for _, part := range strings.Split(s, ",") {
		if part = strings.TrimSpace(part); part != "" {
			out = append(out, part)
		}
	}
	return out
}

func ints(s string) ([]int, error) {
	var out []int
	for _, part := range split(s) {
		n, err := strconv.Atoi(part)
		if err != nil {
			return nil, fmt.Errorf("keine Zahl: %q", part)
		}
		out = append(out, n)
	}
	return out, nil
}

