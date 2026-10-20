package studio

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"k3c/tools/ruletable/table"
)

// Abweichung der Regeltabelle (B-462/AC-03, WZ9.2): der Block „Hub-Stufe → Gebäude“ in § 3 gegen den aus dem
// Planer-Stand erzeugten Block, mit derselben Funktion wie der Task (table.Gegenprobe). Nur lesen.

const (
	rulesFile = "docs/rules/materialien-gebaeude.md"
	rulesTask = "task gen:rules"
)

// HubRulesLine ist eine abweichende Zeile des Blocks (1-basiert); fehlt sie auf einer Seite, ist der Text dort leer.
type HubRulesLine struct {
	Line int    `json:"line"`
	Ist  string `json:"ist"`
	Soll string `json:"soll"`
}

// HubRulesView ist die Abweichung: Ist (Block im Regeltext), Soll (erzeugt), abweichende Zeilen und der Task, der den
// Block neu schreibt; Error bei nicht lesbarer Datei, fehlenden Marken oder unlesbarer hub.json.
type HubRulesView struct {
	Path  string         `json:"path"`
	Task  string         `json:"task"`
	Same  bool           `json:"same"`
	Ist   string         `json:"ist"`
	Soll  string         `json:"soll"`
	Lines []HubRulesLine `json:"lines"`
	Error string         `json:"error"`
}

// HubRules vergleicht die Regeltabelle mit dem Planer-Stand hubJSON (Binding); leer = data/hub.json des Checkouts.
func (a *App) HubRules(hubJSON string) HubRulesView {
	raw := []byte(hubJSON)
	if hubJSON == "" {
		file, err := os.ReadFile(filepath.Join(a.root, filepath.FromSlash(hubFile)))
		if err != nil {
			return HubRulesView{Path: rulesFile, Task: rulesTask, Lines: []HubRulesLine{}, Error: fmt.Sprintf("%s nicht lesbar: %v", hubFile, err)}
		}
		raw = file
	}
	return hubRulesFrom(a.root, raw)
}

// hubRulesFrom liest den Regeltext unter root und vergleicht seinen Block mit dem aus hubJSON erzeugten.
func hubRulesFrom(root string, hubJSON []byte) HubRulesView {
	view := HubRulesView{Path: rulesFile, Task: rulesTask, Lines: []HubRulesLine{}}
	doc, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(rulesFile)))
	if err != nil {
		view.Error = fmt.Sprintf("%s nicht lesbar: %v", rulesFile, err)
		return view
	}
	same, ist, soll, err := table.Gegenprobe(string(doc), hubJSON)
	if err != nil {
		view.Error = "Regeltabelle nicht vergleichbar: " + err.Error()
		return view
	}
	view.Same, view.Ist, view.Soll, view.Lines = same, ist, soll, diffLines(ist, soll)
	return view
}

// diffLines stellt die Zeilen gleicher Nummer gegenüber und liefert die verschiedenen.
func diffLines(ist, soll string) []HubRulesLine {
	a, b := strings.Split(ist, "\n"), strings.Split(soll, "\n")
	out := []HubRulesLine{}
	for i := range max(len(a), len(b)) {
		var x, y string
		if i < len(a) {
			x = a[i]
		}
		if i < len(b) {
			y = b[i]
		}
		if x != y {
			out = append(out, HubRulesLine{Line: i + 1, Ist: x, Soll: y})
		}
	}
	return out
}
