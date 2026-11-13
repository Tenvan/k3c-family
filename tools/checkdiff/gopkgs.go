package main

import (
	"errors"
	"os/exec"
	"strings"
)

// rootModule ist der Modulpfad des Wurzel-Moduls (go.mod: module k3c); wbModule der der Workbench (tools/k3c-workbench).
const (
	rootModule = "k3c"
	wbModule   = "k3c/tools/k3c-workbench"
	wbDir      = "tools/k3c-workbench"
)

// goPkgs liefert die Pakete des Wurzel-Moduls für task check:go (dir "."), bzw. des Workbench-Moduls für
// task k3c-workbench:test (dir wbDir): ohne die Sim-, Balance- und Lasttest-Pakete (simDirs), außer sim (CHECK_SIM=1).
// Ohne grep und tr, damit es auch im Runner der Workbench läuft (B-832, B-857).
func goPkgs(dir string, sim bool) (string, error) {
	cmd := exec.Command("go", "list", "-e", "./...") // -e: auch ohne gebautes Frontend (go:embed), Variablen laufen vor den deps
	cmd.Dir = dir
	out, err := cmd.Output()
	if err != nil {
		return "", err
	}
	pkgs := filterPkgs(strings.Fields(string(out)), sim)
	if len(pkgs) == 0 {
		return "", errors.New("go list: keine Pakete")
	}
	return strings.Join(pkgs, " "), nil
}

// filterPkgs lässt die Pakete aus simDirs weg, außer sim. Importpfade `k3c/<ordner>` (Wurzel-Modul) und
// `k3c/tools/k3c-workbench/<ordner>` entsprechen den simDirs-Schlüsseln `<ordner>` bzw. `tools/k3c-workbench/<ordner>`.
func filterPkgs(all []string, sim bool) []string {
	var res []string
	for _, p := range all {
		if !sim && simDirs[strings.TrimPrefix(p, rootModule+"/")] {
			continue
		}
		res = append(res, p)
	}
	return res
}
