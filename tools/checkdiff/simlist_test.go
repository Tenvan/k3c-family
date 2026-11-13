package main

import (
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"sort"
	"strings"
	"testing"
)

const wbPrefix = "tools/k3c-workbench/"

// simListe liest die Paketliste aus einer Zeile `<name>: '<präfix>(a|b|c)$'` (ci.yml).
func simListe(t *testing.T, datei, name, praefix string) []string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join("..", "..", datei))
	if err != nil {
		t.Fatalf("%s: %v", datei, err)
	}
	re := regexp.MustCompile(`(?m)^\s*` + regexp.QuoteMeta(name) + `:\s*'` + regexp.QuoteMeta(praefix) + `\(([^)]*)\)\$'`)
	m := re.FindSubmatch(b)
	if m == nil {
		t.Fatalf("%s: keine Zeile %s: '%s(…)$'", datei, name, praefix)
	}
	l := strings.Split(string(m[1]), "|")
	sort.Strings(l)
	return l
}

// erwartet teilt simDirs in Wurzel-Modul und Workbench-Modul (Workbench relativ zu internal/).
func erwartet() (root, wb []string) {
	for d := range simDirs {
		if rest, ok := strings.CutPrefix(d, wbPrefix+"internal/"); ok {
			wb = append(wb, rest)
		} else {
			root = append(root, d)
		}
	}
	sort.Strings(root)
	sort.Strings(wb)
	return root, wb
}

// TestSimListeGleich: ci.yml nennt dieselben Sim-Pakete wie simDirs; task check:go nimmt sie aus simDirs selbst
// (checkdiff -gopkgs), B-832/AC-02.
func TestSimListeGleich(t *testing.T) {
	root, wb := erwartet()
	gleich := func(was string, ist, soll []string) {
		if strings.Join(ist, "|") != strings.Join(soll, "|") {
			t.Errorf("%s: %v, simDirs: %v", was, ist, soll)
		}
	}
	gleich("ci.yml GO_SIM_PKGS", simListe(t, ".github/workflows/ci.yml", "GO_SIM_PKGS", "/"), root)
	gleich("ci.yml WB_SIM_PKGS", simListe(t, ".github/workflows/ci.yml", "WB_SIM_PKGS", "/internal/"), wb)
	b, err := os.ReadFile(filepath.Join("..", "..", "Taskfile.yml"))
	if err != nil || !strings.Contains(string(b), "go run ./tools/checkdiff -gopkgs") {
		t.Errorf("Taskfile.yml: check:go holt die Pakete nicht über checkdiff -gopkgs (%v)", err)
	}
}

// TestFilterPkgs: check:go lässt die Sim-Pakete weg, CHECK_SIM=1 nimmt sie mit (B-832/AC-01).
func TestFilterPkgs(t *testing.T) {
	all := []string{"k3c/cmd/k3c-load", "k3c/cmd/k3c-server", "k3c/cmd/k3c-sim", "k3c/engine/balance", "k3c/engine/balance/x", "k3c/engine/sim"}
	if got := filterPkgs(all, false); !slices.Equal(got, []string{"k3c/cmd/k3c-server", "k3c/engine/balance/x", "k3c/engine/sim"}) {
		t.Errorf("ohne Sim: %v", got)
	}
	if got := filterPkgs(all, true); !slices.Equal(got, all) {
		t.Errorf("CHECK_SIM=1: %v", got)
	}
}
