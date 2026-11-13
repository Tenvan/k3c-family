package main

import (
	"errors"
	"os/exec"
	"strings"
)

// rootModule ist der Modulpfad des Wurzel-Moduls (go.mod: module k3c).
const rootModule = "k3c"

// goPkgs liefert die Pakete des Wurzel-Moduls für task check:go: ohne die Sim-, Balance- und Lasttest-Pakete (simDirs),
// außer sim (CHECK_SIM=1). Ohne grep und tr, damit es auch im Runner der Workbench läuft (B-832).
func goPkgs(sim bool) (string, error) {
	out, err := exec.Command("go", "list", "./...").Output()
	if err != nil {
		return "", err
	}
	pkgs := filterPkgs(strings.Fields(string(out)), sim)
	if len(pkgs) == 0 {
		return "", errors.New("go list: keine Pakete")
	}
	return strings.Join(pkgs, " "), nil
}

// filterPkgs lässt die Pakete aus simDirs weg (Importpfad k3c/<ordner>), außer sim.
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
