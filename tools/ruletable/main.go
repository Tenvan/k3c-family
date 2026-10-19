// Command regeltabelle schreibt den Block „Hub-Stufe → Gebäude“ aus data/hub.json in docs/rules/materialien-gebaeude.md
// (§ 3, zwischen den Marken). Aufruf nur über den Task `rules:update`; gleiche Daten ergeben byte-gleichen Text (WZ15.2, B-462).
package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"

	"k3c/tools/ruletable/table"
)

func main() {
	root := flag.String("root", ".", "Projektwurzel")
	flag.Parse()
	changed, err := run(*root)
	if err != nil {
		fmt.Fprintln(os.Stderr, "❌ regeltabelle:", err)
		os.Exit(1)
	}
	if changed {
		fmt.Println("✅ regeltabelle: Block aktualisiert")
		return
	}
	fmt.Println("✅ regeltabelle: Block ist aktuell")
}

// run ersetzt den Block und schreibt die Datei nur bei einer Änderung (atomar über eine temporäre Datei).
func run(root string) (bool, error) {
	hubJSON, err := os.ReadFile(filepath.Join(root, "data", "hub.json"))
	if err != nil {
		return false, err
	}
	path := filepath.Join(root, "docs", "rules", "materialien-gebaeude.md")
	doc, err := os.ReadFile(path)
	if err != nil {
		return false, err
	}
	block, err := table.Block(hubJSON)
	if err != nil {
		return false, err
	}
	next, err := table.Ersetzen(string(doc), block)
	if err != nil {
		return false, fmt.Errorf("%s: %w", path, err)
	}
	if next == string(doc) {
		return false, nil
	}
	f, err := os.CreateTemp(filepath.Dir(path), "*.tmp")
	if err != nil {
		return false, err
	}
	tmp := f.Name()
	defer func() { _ = os.Remove(tmp) }() // bleibt nur bei einem Fehler übrig; nach dem Rename gibt es sie nicht mehr
	if _, err := f.WriteString(next); err != nil {
		_ = f.Close()
		return false, err
	}
	if err := f.Close(); err != nil {
		return false, err
	}
	return true, os.Rename(tmp, path)
}
