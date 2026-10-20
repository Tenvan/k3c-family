package rules

import (
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
)

// repoRoot ist die Wurzel des Hauptmoduls k3c, relativ zu diesem Paket (engine/internal/rules).
const repoRoot = "../../.."

// serverDirs sind die Ordner, die nichts aus tools/ importieren dürfen (B-507/AC-01): die Engine und der Spielserver.
var serverDirs = []string{"engine", "cmd/k3c-server"}

// TestServerImportsNothingFromTools prüft die Importe von engine/ und cmd/k3c-server samt aller Pakete des Hauptmoduls,
// die sie (auch über Umwege) einbinden: kein Import beginnt mit k3c/tools. Das Studio darf die Engine nutzen, nie umgekehrt.
func TestServerImportsNothingFromTools(t *testing.T) {
	for _, imp := range moduleImports(t, startDirs(t)) {
		if imp.path == "k3c/tools" || strings.HasPrefix(imp.path, "k3c/tools/") {
			t.Errorf("%s importiert %s (engine/ und cmd/k3c-server importieren nichts aus tools/, B-507)", imp.file, imp.path)
		}
	}
}

// startDirs sammelt alle Ordner unter serverDirs (relativ zur Wurzel, mit Schrägstrichen).
func startDirs(t *testing.T) []string {
	var dirs []string
	for _, base := range serverDirs {
		err := filepath.WalkDir(filepath.Join(repoRoot, base), func(path string, d os.DirEntry, err error) error {
			if err == nil && d.IsDir() {
				rel, _ := filepath.Rel(repoRoot, path)
				dirs = append(dirs, filepath.ToSlash(rel))
			}
			return err
		})
		if err != nil {
			t.Fatal(err)
		}
	}
	return dirs
}

type goImport struct{ file, path string }

// moduleImports liest die Importe der Startordner (mit Tests) und folgt Importen des Hauptmoduls (k3c/…) in deren
// Ordner (ohne Tests), bis keine neuen mehr dazukommen.
func moduleImports(t *testing.T, start []string) []goImport {
	isStart := map[string]bool{}
	seen := map[string]bool{}
	queue := append([]string(nil), start...)
	for _, dir := range start {
		isStart[dir], seen[dir] = true, true
	}
	var all []goImport
	for len(queue) > 0 {
		dir := queue[0]
		queue = queue[1:]
		for _, imp := range dirImports(t, dir, isStart[dir]) {
			all = append(all, imp)
			if rel, ok := strings.CutPrefix(imp.path, "k3c/"); ok && !seen[rel] {
				seen[rel] = true
				queue = append(queue, rel)
			}
		}
	}
	if len(all) == 0 {
		t.Fatal("keine Importe gefunden – serverDirs prüfen")
	}
	return all
}

// dirImports liest die Importe der Go-Dateien eines Ordners; Testdateien nur, wenn withTests gilt.
func dirImports(t *testing.T, dir string, withTests bool) []goImport {
	entries, err := os.ReadDir(filepath.Join(repoRoot, filepath.FromSlash(dir)))
	if err != nil {
		t.Errorf("Ordner %s fehlt: %v", dir, err)
		return nil
	}
	fset := token.NewFileSet()
	var out []goImport
	for _, e := range entries {
		name := e.Name()
		if e.IsDir() || !strings.HasSuffix(name, ".go") || (!withTests && strings.HasSuffix(name, "_test.go")) {
			continue
		}
		file := dir + "/" + name
		f, err := parser.ParseFile(fset, filepath.Join(repoRoot, filepath.FromSlash(file)), nil, parser.ImportsOnly)
		if err != nil {
			t.Errorf("%s lässt sich nicht parsen: %v", file, err)
			continue
		}
		for _, spec := range f.Imports {
			path, _ := strconv.Unquote(spec.Path.Value)
			out = append(out, goImport{file: file, path: path})
		}
	}
	return out
}
