// checkdiff prüft vor einem Commit nur, was die geänderten Dateien dieses Commits brauchen (task check:diff, B-518, B-730,
// B-742). Stufe 1: Regel-Tests, Lint, Typecheck, go vet (geänderte Pakete und ihre Abnehmer). Stufe 2, nur nach grüner
// Stufe 1: Tests der geänderten Pakete (-short) und Vitest --changed. Keine Tests der Abnehmer, keine Sim-, Balance-,
// Gen- und Race-Läufe. Regeln: docs/arbeitsweise.md › Tempo im Lauf.
//
//	checkdiff [-base <ref>] [-plan] [-only js|go]
//	checkdiff -gopkgs   (Paketliste für task check:go ohne Sim-Pakete, B-832)
//
// -base: Vergleichsbasis statt HEAD (z. B. origin/develop für den ganzen Sprint; auch Umgebung CHECK_BASE).
// -plan: nur zeigen, was laufen würde. -only: nur eine Bahn (nach einem roten Lauf den betroffenen Teil wiederholen).
package main

import (
	"flag"
	"fmt"
	"os"
	"strings"
	"time"
)

type options struct {
	base, only        string
	plan, sim, gopkgs bool
}

func parse() options {
	o := options{base: os.Getenv("CHECK_BASE"), sim: os.Getenv("CHECK_SIM") == "1"}
	flag.StringVar(&o.base, "base", o.base, "Vergleichsbasis statt HEAD")
	flag.StringVar(&o.only, "only", "", "nur eine Bahn: js oder go")
	flag.BoolVar(&o.plan, "plan", false, "nur zeigen, was laufen würde")
	flag.BoolVar(&o.gopkgs, "gopkgs", false, "nur die Pakete für task check:go ausgeben (ohne Sim-Pakete, außer CHECK_SIM=1)")
	flag.Parse()
	return o
}

func fatal(format string, args ...any) {
	fmt.Printf("❌ check:diff: "+format+"\n", args...)
	os.Exit(1)
}

// build bildet die Schritte des Laufs.
func build(o options) (string, []step, []string) {
	base, files, err := changedFiles(o.base)
	if err != nil {
		fatal("git scheitert (%v); Basis mit -base oder CHECK_BASE setzen", err)
	}
	r, err := loadRepo()
	if err != nil {
		fatal("Module nicht lesbar (%v)", err)
	}
	p := classify(r, files)
	steps := jsSteps(p, base)
	targets := allTargets(r, p, o.sim)
	steps = append(steps, goSteps(targets)...)
	var kept []step
	for _, s := range steps {
		if o.only == "" || s.Lane == o.only {
			kept = append(kept, s)
		}
	}
	return base, kept, skippedSim(p, o.sim)
}

// skippedSim nennt geänderte Pakete, deren Tests wegen der Sim-Sperre nicht laufen.
func skippedSim(p plan, sim bool) []string {
	var res []string
	if sim {
		return nil
	}
	for _, mod := range sortedKeys(p.Mods) {
		for _, d := range sortedKeys(p.Mods[mod].Dirs) {
			if dir := strings.TrimPrefix(mod+"/"+d, "./"); simDirs[dir] {
				res = append(res, dir)
			}
		}
	}
	return res
}

func main() {
	o := parse()
	if o.gopkgs {
		pkgs, err := goPkgs(o.sim)
		if err != nil {
			fatal("Paketliste für check:go: %v", err)
		}
		fmt.Println(pkgs)
		return
	}
	start := time.Now()
	base, steps, skipped := build(o)
	fmt.Printf("🚦 check:diff Basis %s · %d Schritte\n", base, len(steps))
	for _, s := range steps {
		fmt.Printf("   Stufe %d · %s\n", s.Stage, describe(s))
	}
	if len(skipped) > 0 {
		fmt.Printf("🚫 Tests gesperrt (Sim, Balance, Lasttest): %s\n", strings.Join(skipped, " "))
	}
	if o.plan {
		return
	}
	for stage := 1; stage <= 2; stage++ {
		if failed := runStage(steps, stage); len(failed) > 0 {
			fmt.Printf("❌ check:diff fehlgeschlagen in Stufe %d: %s (%.0f s)\n", stage, strings.Join(failed, ", "), time.Since(start).Seconds())
			os.Exit(1)
		}
	}
	fmt.Printf("✅ check:diff grün (%.0f s)\n", time.Since(start).Seconds())
}
