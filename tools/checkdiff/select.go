package main

// Auswahl: Welche Prüfungen lösen die geänderten Dateien eines Commits aus? Reine Logik ohne Git und Dateisystem, damit sie
// als Tabelle testbar bleibt (select_test.go). Regeln: docs/arbeitsweise.md › Tempo im Lauf.

import (
	"path"
	"sort"
	"strings"
)

// modPlan ist die Auswahl für ein Go-Modul (Ordner relativ zur Repo-Wurzel, "." = Wurzel-Modul).
type modPlan struct {
	All     bool            // go.mod oder go.sum geändert: alle Pakete übersetzen (vet), keine Tests
	LintAll bool            // Lint-Konfiguration geändert: ganzes Modul linten
	Dirs    map[string]bool // geänderte Paket-Ordner, relativ zum Modul ("." = Modul-Wurzel)
	Gone    map[string]bool // gelöschte oder verschobene Paket-Ordner: nur ihre Abnehmer werden übersetzt
}

// plan ist die Auswahl für einen Lauf.
type plan struct {
	Docs          bool // Planungs-Tests (docs/, *.md)
	Rules         bool // Regel-Test des Repos (alles außer Doku)
	Client        bool // Oxlint, tsc, Vitest --changed
	WbVitest      bool
	StarterVitest bool
	Mods          map[string]*modPlan
}

// repo beantwortet, was die Auswahl über den Checkout wissen muss.
type repo struct {
	modules []string              // Modul-Ordner, "." zuerst
	hasGo   func(dir string) bool // enthält der Ordner (repo-relativ) Go-Dateien?
}

// goldenOwners: Pakete des Wurzel-Moduls, deren Tests die Dateien unter testdata/<name> an der Repo-Wurzel lesen.
var goldenOwners = map[string][]string{
	"golden":       {"engine/sim", "engine/level"},
	"hub":          {"engine/sim", "engine/level"},
	"planned":      {"engine/level"},
	"protocol":     {"engine/net"},
	"saves":        {"engine/store", "engine/sim"},
	"torches.json": {"engine/sim"},
}

// clientRoots: Ordner, deren Dateien Lint, Typecheck und Vitest des Clients brauchen.
var clientRoots = []string{"src/", "tests/", "public/", "data/", "tools/paper/"}

func hasPrefix(f string, prefixes ...string) bool {
	for _, p := range prefixes {
		if strings.HasPrefix(f, p) {
			return true
		}
	}
	return false
}

func isDoc(f string) bool {
	return hasPrefix(f, "docs/", "reports/") || strings.HasSuffix(f, ".md")
}

// isClient: Ordner des Clients oder eine Konfigurations- bzw. Seiten-Datei an der Repo-Wurzel.
func isClient(f string) bool {
	if hasPrefix(f, clientRoots...) {
		return true
	}
	if strings.Contains(f, "/") {
		return false
	}
	switch path.Ext(f) {
	case ".html", ".ts", ".js", ".mjs", ".json", ".lock", ".lockb":
		return true
	}
	return false
}

func (p *plan) mod(m string) *modPlan {
	if p.Mods[m] == nil {
		p.Mods[m] = &modPlan{Dirs: map[string]bool{}, Gone: map[string]bool{}}
	}
	return p.Mods[m]
}

// frontend ordnet Dateien der Wails-Oberflächen zu; true = Datei ist damit erledigt.
func (p *plan) frontend(f string) bool {
	switch {
	case hasPrefix(f, "tools/webkit/"):
		p.WbVitest, p.StarterVitest = true, true
	case hasPrefix(f, "tools/k3c-workbench/frontend/"):
		p.WbVitest = true
	case hasPrefix(f, "tools/k3c-starter/frontend/"):
		p.StarterVitest = true
	default:
		return false
	}
	return true
}

// moduleOf liefert das Modul mit dem längsten passenden Ordner und den Pfad der Datei darin.
func (r repo) moduleOf(f string) (mod, rel string) {
	mod, rel = ".", f
	for _, m := range r.modules {
		if m != "." && strings.HasPrefix(f, m+"/") && len(m) > len(mod) {
			mod, rel = m, strings.TrimPrefix(f, m+"/")
		}
	}
	return mod, rel
}

// webExts: Quelltexte, die nie in ein Go-Paket eingebettet sind (Export-Skripte, Oberflächen).
var webExts = map[string]bool{".ts": true, ".tsx": true, ".js": true, ".mjs": true, ".css": true, ".html": true}

// pkgDir liefert den Paket-Ordner (relativ zum Modul), zu dem die Datei gehört: Go-Dateien ihr Ordner; Daten-Dateien
// (go:embed, testdata) der nächste Ordner darüber mit Go-Dateien. ok=false: kein Paket (bei Go-Dateien: Paket gelöscht).
func (r repo) pkgDir(mod, rel string) (dir string, ok bool) {
	dir = path.Dir(rel)
	if strings.HasSuffix(rel, ".go") || webExts[path.Ext(rel)] {
		return dir, !webExts[path.Ext(rel)] && r.hasGo(path.Join(mod, dir))
	}
	for ; ; dir = path.Dir(dir) {
		if r.hasGo(path.Join(mod, dir)) {
			return dir, true
		}
		if dir == "." {
			return dir, false
		}
	}
}

func (p *plan) goFile(r repo, f string) {
	mod, rel := r.moduleOf(f)
	base := path.Base(rel)
	switch {
	case base == "go.mod" || base == "go.sum":
		p.mod(mod).All = true
	case strings.HasPrefix(base, ".golangci"):
		p.mod(mod).LintAll = true
	default:
		dir, ok := r.pkgDir(mod, rel)
		switch {
		case ok:
			p.mod(mod).Dirs[dir] = true
		case strings.HasSuffix(rel, ".go"):
			p.mod(mod).Gone[dir] = true
		}
	}
}

// classify bildet aus den geänderten Dateien (Pfade mit /, relativ zur Repo-Wurzel) die Auswahl.
func classify(r repo, files []string) plan {
	p := plan{Mods: map[string]*modPlan{}}
	for _, f := range files {
		if isDoc(f) {
			p.Docs = true
			continue
		}
		p.Rules = true
		if path.Base(f) == "Taskfile.yml" || hasPrefix(f, ".github/") || p.frontend(f) {
			continue
		}
		if name, ok := strings.CutPrefix(f, "testdata/"); ok {
			for _, d := range goldenOwners[strings.SplitN(name, "/", 2)[0]] {
				p.mod(".").Dirs[d] = true
			}
			continue
		}
		p.Client = p.Client || isClient(f)
		p.goFile(r, f)
	}
	return p
}

func sortedKeys[V any](m map[string]V) []string {
	keys := make([]string, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	return keys
}
