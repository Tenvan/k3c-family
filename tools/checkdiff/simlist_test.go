package main

import (
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"testing"
)

const wbPrefix = "tools/k3c-workbench/"

// simListe liest die Paketliste aus einer Zeile `<name>: '<präfix>(a|b|c)$'` (Taskfile.yml, ci.yml).
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

// TestSimListeGleich: Taskfile.yml (check:go), ci.yml und simDirs nennen dieselben Sim-Pakete (B-832/AC-02).
func TestSimListeGleich(t *testing.T) {
	root, wb := erwartet()
	gleich := func(was string, ist, soll []string) {
		if strings.Join(ist, "|") != strings.Join(soll, "|") {
			t.Errorf("%s: %v, simDirs: %v", was, ist, soll)
		}
	}
	gleich("Taskfile.yml GO_SIM_PKGS", simListe(t, "Taskfile.yml", "GO_SIM_PKGS", "/"), root)
	gleich("ci.yml GO_SIM_PKGS", simListe(t, ".github/workflows/ci.yml", "GO_SIM_PKGS", "/"), root)
	gleich("ci.yml WB_SIM_PKGS", simListe(t, ".github/workflows/ci.yml", "WB_SIM_PKGS", "/internal/"), wb)
}
