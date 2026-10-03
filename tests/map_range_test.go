package tests

import (
	"fmt"
	"go/ast"
	"go/importer"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
)

// Go iteriert Maps in zufälliger Reihenfolge. Hängt der Spielzustand davon ab, rechnen zwei Läufe mit gleichem Seed
// verschieden (B-138). Der Check meldet jedes `range` über eine Map in engine/sim und engine/level; harmlose Fälle
// stehen mit Begründung in mapRangeAllowed.

// mapRangeDirs sind die deterministischen Pakete (relativ zu tests/).
var mapRangeDirs = []string{"../engine/sim", "../engine/level"}

// mapRangeAllowed: Datei und Funktion → Begründung, warum die Reihenfolge das Ergebnis nicht beeinflusst.
var mapRangeAllowed = map[string]string{
	"save.go:ToSave": "Campaign.ToSave sammelt Hubs aus pendingHubs und worlds (disjunkte Tiefen, worldFor löscht den wartenden Hub) und sortiert sie danach nach der eindeutigen Tiefe.",
}

func TestKeinRangeUeberMaps(t *testing.T) {
	used := map[string]bool{}
	for _, dir := range mapRangeDirs {
		fset := token.NewFileSet()
		files := parseDir(t, fset, dir)
		for _, f := range mapRanges(t, fset, files) {
			if _, ok := mapRangeAllowed[f.key]; ok {
				used[f.key] = true
				continue
			}
			t.Errorf("%s: range über eine Map (%s) – Reihenfolge ist zufällig; Schlüssel sortieren (slices.Sorted(maps.Keys(…))) oder mit Begründung in mapRangeAllowed eintragen", f.pos, f.key)
		}
	}
	for key, why := range mapRangeAllowed {
		if strings.TrimSpace(why) == "" {
			t.Errorf("Ausnahme %s ohne Begründung", key)
		}
		if !used[key] {
			t.Errorf("Ausnahme %s trifft nichts mehr, bitte entfernen", key)
		}
	}
}

// Der Check findet einen absichtlichen Verstoß, lässt sortierte Schlüssel und Slices aber durch.
func TestRangeCheckFindetVerstoss(t *testing.T) {
	const src = `package beispiel

import (
	"maps"
	"slices"
)

type Troop struct{ X float64 }

func Bewege(troops map[string]*Troop) {
	for _, tr := range troops { // Verstoß
		tr.X++
	}
	for _, k := range slices.Sorted(maps.Keys(troops)) { // sortiert, kein Befund
		troops[k].X++
	}
	for range []int{1, 2} { // Slice, kein Befund
	}
}
`
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "beispiel.go", src, 0)
	if err != nil {
		t.Fatal(err)
	}
	found := mapRanges(t, fset, []*ast.File{f})
	if len(found) != 1 || found[0].key != "beispiel.go:Bewege" || !strings.HasSuffix(found[0].pos, ":11:2") {
		t.Fatalf("erwartet genau den Verstoß in Zeile 11, gefunden: %+v", found)
	}
}

type mapRange struct{ pos, key string }

func parseDir(t *testing.T, fset *token.FileSet, dir string) []*ast.File {
	t.Helper()
	entries, err := os.ReadDir(dir)
	if err != nil {
		t.Fatal(err)
	}
	var files []*ast.File
	for _, e := range entries {
		name := e.Name()
		if !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			continue
		}
		f, err := parser.ParseFile(fset, filepath.Join(dir, name), nil, 0)
		if err != nil {
			t.Fatal(err)
		}
		files = append(files, f)
	}
	return files
}

// mapRanges prüft die Typen des Pakets (Importe aus dem Quelltext) und liefert jedes range über eine Map.
func mapRanges(t *testing.T, fset *token.FileSet, files []*ast.File) []mapRange {
	t.Helper()
	info := &types.Info{Types: map[ast.Expr]types.TypeAndValue{}}
	conf := types.Config{Importer: importer.ForCompiler(fset, "source", nil)}
	if _, err := conf.Check("", fset, files, info); err != nil {
		t.Fatalf("Typprüfung: %v", err)
	}
	var out []mapRange
	for _, f := range files {
		for _, d := range f.Decls {
			fn, ok := d.(*ast.FuncDecl)
			if !ok || fn.Body == nil {
				continue
			}
			ast.Inspect(fn.Body, func(n ast.Node) bool {
				r, ok := n.(*ast.RangeStmt)
				if ok && isMap(info.TypeOf(r.X)) {
					p := fset.Position(r.Pos())
					out = append(out, mapRange{pos: p.String(), key: fmt.Sprintf("%s:%s", filepath.Base(p.Filename), fn.Name.Name)})
				}
				return true
			})
		}
	}
	slices.SortFunc(out, func(a, b mapRange) int { return strings.Compare(a.pos, b.pos) })
	return out
}

func isMap(typ types.Type) bool {
	if typ == nil {
		return false
	}
	_, ok := typ.Underlying().(*types.Map)
	return ok
}
