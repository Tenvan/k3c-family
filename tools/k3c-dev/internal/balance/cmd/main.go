// Command k3c-balance spielt eine Szenario-Matrix mit Bots und schreibt die Kennzahlen als JSON (B-099, BAL1.1).
// Start über `task balance:run -- --seeds 10 --players 2 --bots saver --days 5`. Mit `--replay-dir DIR` schreibt er je
// gültigem Lauf eine Replay-Datei, `--play DATEI` spielt eine Datei ohne Bot ab (BAL1.2, B-159).
package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"k3c/tools/k3c-dev/internal/balance"
)

func main() {
	if err := run(os.Args[1:]); err != nil {
		fmt.Fprintln(os.Stderr, "k3c-balance:", err)
		os.Exit(1)
	}
}

func run(args []string) error {
	fs := flag.NewFlagSet("k3c-balance", flag.ContinueOnError)
	seeds := fs.Int("seeds", 10, "Anzahl Seeds (feste Liste 1..N)")
	seedList := fs.String("seed-list", "", "Seeds mit Komma statt 1..N, z. B. a,b,12")
	players := fs.String("players", "2", "Spieleranzahlen mit Komma (1–4)")
	botList := fs.String("bots", "saver", "Bot-Profile mit Komma: "+strings.Join(balance.BotNames(), ", "))
	depths := fs.String("depths", "0", "Tiefen mit Komma (0 Wald, 1 Höhle, …)")
	days := fs.Int("days", 5, "Tage je Lauf")
	threshold := fs.Int("gold-threshold", 0, "Gold-Schwelle aller Monarchen zusammen (0 = nicht messen)")
	out := fs.String("out", "", "Datei für das JSON (leer = Standardausgabe)")
	replayDir := fs.String("replay-dir", "", "Ordner für eine Replay-Datei je gültigem Lauf (leer = keine)")
	play := fs.String("play", "", "Replay-Datei ohne Bot abspielen statt einer Matrix")
	var to targetOpts
	fs.BoolVar(&to.on, "targets", false, "Ziele aus data/balance-targets.json bewerten, Bericht als JSON und Markdown; --seeds N = nur die ersten N, sonst alle")
	fs.StringVar(&to.dir, "report-dir", "reports", "Ordner für balance-<Zeit>.json und .md (nur mit --targets)")
	fs.StringVar(&to.baseline, "baseline", "", "Baseline-Datei zum Vergleichen (nur mit --targets)")
	fs.BoolVar(&to.write, "write-baseline", false, "Bericht nach --baseline schreiben statt vergleichen")
	if err := fs.Parse(args); err != nil {
		return err
	}
	if to.on {
		to.replays = *replayDir
		fs.Visit(func(f *flag.Flag) {
			if f.Name == "seeds" {
				to.seeds = *seeds
			}
		})
		return runTargets(to)
	}
	if *play != "" {
		return playFile(*play, *out)
	}
	m := balance.Matrix{Seeds: seedNames(*seeds, *seedList), Bots: split(*botList), Days: *days, GoldThreshold: *threshold}
	var err error
	if m.Players, err = ints(*players); err != nil {
		return err
	}
	if m.Depths, err = ints(*depths); err != nil {
		return err
	}
	report, err := balance.Run(m)
	if err != nil {
		return err
	}
	if err := writeReplays(report, *replayDir); err != nil {
		return err
	}
	b, err := report.JSON()
	if err != nil {
		return err
	}
	return write(*out, b)
}

// write schreibt nach out oder auf die Standardausgabe.
func write(out string, b []byte) error {
	if out == "" {
		_, err := os.Stdout.Write(b)
		return err
	}
	return os.WriteFile(out, b, 0o644)
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
func playFile(path, out string) error {
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
		fmt.Fprintln(os.Stderr, "k3c-balance: Warnung:", w)
	}
	b, err := json.MarshalIndent(pb, "", "  ")
	if err != nil {
		return err
	}
	return write(out, append(b, '\n'))
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
