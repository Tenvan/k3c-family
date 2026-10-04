// Command k3c-balance spielt eine Szenario-Matrix mit Bots und schreibt die Kennzahlen als JSON (B-099, BAL1.1).
// Start über `task balance:run -- --seeds 10 --players 2 --bots saver --days 5`.
package main

import (
	"flag"
	"fmt"
	"os"
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
	if err := fs.Parse(args); err != nil {
		return err
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
	b, err := report.JSON()
	if err != nil {
		return err
	}
	if *out == "" {
		_, err = os.Stdout.Write(b)
		return err
	}
	return os.WriteFile(*out, b, 0o644)
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
