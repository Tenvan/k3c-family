package main

// Schritte: Aus der Auswahl werden Befehle. Stufe 1 sind die schnellen Prüfungen (Regel-Tests, Lint, Typecheck, go vet),
// Stufe 2 die Tests; Stufe 2 startet nur nach grüner Stufe 1. Je Stufe laufen die Bahnen `js` und `go` nebeneinander,
// die Schritte einer Bahn nacheinander (parallele Go-Tests mehrerer Module scheitern unter Last, B-704).

import (
	"path"
	"strings"
)

type step struct {
	Name  string
	Lane  string // js oder go
	Stage int    // 1 schnell, 2 Tests
	Dir   string // Arbeitsordner relativ zur Repo-Wurzel
	Cmd   []string
}

// simDirs: Pakete mit Simulations-, Balancing- und Lasttest-Läufen. Ihre Tests sind bis auf Weiteres gesperrt (Beschluss
// Ralf 2026-11-11); go vet und Lint laufen weiter. CHECK_SIM=1 hebt die Sperre für einen Lauf auf.
// task check:go liest sie über -gopkgs; ci.yml › GO_SIM_PKGS, WB_SIM_PKGS muss gleich sein (simlist_test.go, B-832).
var simDirs = map[string]bool{
	"engine/balance": true, "cmd/k3c-sim": true, "cmd/k3c-load": true,
	"tools/k3c-workbench/internal/balance": true, "tools/k3c-workbench/internal/simrun": true,
	"tools/k3c-workbench/internal/simexe": true, "tools/k3c-workbench/internal/botdev": true,
}

// lintMods: Module mit golangci-lint (wie task check:go, check:dev, check:starter; das Spiel-Modul seit NG1-Review).
var lintMods = map[string]bool{
	".": true, "tools/k3c-workbench": true, "tools/assetkit": true, "tools/k3c-starter": true, "native/k3c-game": true,
}

// prepTasks: Module, deren Go-Code ein gebautes Frontend einbettet (go:embed frontend/dist).
var prepTasks = map[string]string{"tools/k3c-workbench": "k3c-workbench:frontend", "tools/k3c-starter": "k3c-starter:frontend"}

// vitestExcludes: Tests, die gerenderte Papierschnitt-Grafik brauchen (gen:paper läuft hier nicht, B-742).
var vitestExcludes = []string{
	"tests/paperAssets.test.ts", "tests/buildingTargets.test.ts", "tools/paper/golden/**",
	"src/scenes/render/*Sprites.test.ts", "src/scenes/render/hubGrowth.test.ts",
}

func jsSteps(p plan, base string) []step {
	var out []step
	var scan []string
	if p.Docs {
		scan = append(scan, "tests/planning.test.ts", "tests/planningProjects.test.ts")
	}
	if p.Rules {
		scan = append(scan, "tests/projectRules.test.ts")
	}
	if len(scan) > 0 {
		out = append(out, step{"regeln", "js", 1, ".", append([]string{"bunx", "vitest", "run"}, scan...)})
	}
	if p.Client {
		changed := []string{"bunx", "vitest", "run", "--changed", base, "--passWithNoTests"}
		for _, x := range vitestExcludes {
			changed = append(changed, "--exclude", x)
		}
		out = append(out,
			step{"lint", "js", 1, ".", []string{"task", "lint"}},
			step{"typecheck", "js", 1, ".", []string{"task", "typecheck"}},
			step{"vitest", "js", 2, ".", changed})
	}
	if p.WbVitest {
		args := "VITEST_ARGS=--changed " + base + " --passWithNoTests"
		out = append(out, step{"wb-vitest", "js", 2, ".", []string{"task", "k3c-workbench:vitest", args}})
	}
	if p.StarterVitest {
		out = append(out, step{"starter-vitest", "js", 2, ".", []string{"task", "k3c-starter:vitest"}})
	}
	return out
}

// modName: kurzer Name eines Moduls für die Schritt-Namen (go, k3c-workbench, k3c-rig …).
func modName(mod string) string {
	if mod == "." {
		return "go"
	}
	return path.Base(mod)
}

// goTargets: Was ein Modul prüft. Vet = geänderte Pakete und ihre Abnehmer (nur übersetzen), Lint und Test nur die geänderten.
type goTargets struct {
	Mod              string
	Vet, Lint, Tests []string // Importpfade bzw. Ordner (Lint); "./..." = ganzes Modul
}

func relArg(dir string) string {
	if dir == "." {
		return "."
	}
	return "./" + dir
}

// targetsOf bildet die Ziele eines Moduls aus der Auswahl und den Abnehmern (Importpfade aus dependents).
func targetsOf(mod string, mp *modPlan, dependents []string, sim bool) goTargets {
	t := goTargets{Mod: mod}
	if mp == nil {
		t.Vet = dependents
		return t
	}
	for _, d := range sortedKeys(mp.Dirs) {
		t.Vet = append(t.Vet, relArg(d))
		t.Lint = append(t.Lint, relArg(d))
		if sim || !simDirs[path.Join(mod, d)] {
			t.Tests = append(t.Tests, relArg(d))
		}
	}
	t.Vet = append(t.Vet, dependents...)
	if mp.All {
		t.Vet = []string{"./..."}
	}
	if mp.LintAll {
		t.Lint = []string{"./..."}
	}
	if !lintMods[mod] {
		t.Lint = nil
	}
	return t
}

func goSteps(targets []goTargets) []step {
	var out []step
	for _, t := range targets {
		if len(t.Vet)+len(t.Lint)+len(t.Tests) == 0 {
			continue
		}
		n := modName(t.Mod)
		if task, ok := prepTasks[t.Mod]; ok {
			out = append(out, step{n + "-frontend", "go", 1, ".", []string{"task", task}})
		}
		if len(t.Vet) > 0 {
			out = append(out, step{n + "-vet", "go", 1, t.Mod, append([]string{"go", "vet"}, t.Vet...)})
		}
		if len(t.Lint) > 0 {
			out = append(out, step{n + "-lint", "go", 1, t.Mod, append([]string{"golangci-lint", "run", "--allow-serial-runners"}, t.Lint...)})
		}
		if len(t.Tests) > 0 {
			out = append(out, step{n + "-test", "go", 2, t.Mod, append([]string{"go", "test", "-short"}, t.Tests...)})
		}
	}
	return out
}

func describe(s step) string {
	dir := ""
	if s.Dir != "." {
		dir = " (in " + s.Dir + ")"
	}
	return s.Name + ": " + strings.Join(s.Cmd, " ") + dir
}
