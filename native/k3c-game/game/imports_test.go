package game

import (
	"fmt"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"strings"
	"testing"
)

// Importregel (B-792, view/doc.go): Fachpakete importieren nur view, art, k3c/engine/sim und k3c/data, dazu
// Standardbibliothek und Ebitengine; nie einander, nie game oder local. view kennt nur sim. Tests der Fachpakete
// dürfen zusätzlich k3c/engine/* nutzen (Welten für Testfälle bauen). Fachpaket ist jeder Ordner des Moduls außer
// game, local, art, view und e2e; ein neues Paket fällt damit von selbst unter die Regel.

const module = "k3c/native/k3c-game"

// fachpakete sind die Fachpakete laut B-792; jedes hat einen Eintrag in Packages().
var fachpakete = []string{"actors", "audio", "fx", "hud", "input", "menu", "props", "world"}

// notFach sind die Ordner mit eigener Regel bzw. ohne Go-Paket; e2e ist der Ende-zu-Ende-Test (nur Tests, darf alles
// wie game).
var notFach = []string{"game", "local", "art", "view", "bin", "e2e"}

func isStd(path string) bool {
	first, _, _ := strings.Cut(path, "/")
	return !strings.Contains(first, ".") && first != "k3c"
}

func isEbiten(path string) bool {
	return path == "github.com/hajimehoshi/ebiten/v2" || strings.HasPrefix(path, "github.com/hajimehoshi/ebiten/v2/")
}

// allowed meldet, ob Paket pkg (Ordnername) path importieren darf; test: Import aus einer _test.go-Datei.
func allowed(pkg, path string, test bool) bool {
	if isStd(path) || isEbiten(path) || path == "k3c/engine/sim" {
		return true
	}
	if pkg == "view" {
		return false
	}
	switch path {
	case module + "/view", module + "/art", "k3c/data":
		return true
	}
	return test && strings.HasPrefix(path, "k3c/engine/")
}

// violations liefert die verbotenen Importe des Pakets pkg aus den Quellen files (Dateiname → Inhalt).
func violations(pkg string, files map[string][]byte) ([]string, error) {
	var out []string
	fset := token.NewFileSet()
	for name, src := range files {
		f, err := parser.ParseFile(fset, name, src, parser.ImportsOnly)
		if err != nil {
			return nil, err
		}
		for _, imp := range f.Imports {
			path, err := strconv.Unquote(imp.Path.Value)
			if err != nil {
				return nil, err
			}
			if !allowed(pkg, path, strings.HasSuffix(name, "_test.go")) {
				out = append(out, fmt.Sprintf("%s/%s importiert %s", pkg, name, path))
			}
		}
	}
	slices.Sort(out)
	return out, nil
}

// readPackage liest alle .go-Dateien des Ordners dir.
func readPackage(t *testing.T, dir string) map[string][]byte {
	t.Helper()
	names, err := filepath.Glob(filepath.Join(dir, "*.go"))
	if err != nil {
		t.Fatal(err)
	}
	files := map[string][]byte{}
	for _, n := range names {
		src, err := os.ReadFile(n)
		if err != nil {
			t.Fatal(err)
		}
		files[filepath.Base(n)] = src
	}
	return files
}

// checkedPackages liefert die Ordner des Moduls, für die eine Regel gilt: alle Fachpakete und view.
func checkedPackages(t *testing.T) []string {
	t.Helper()
	entries, err := os.ReadDir("..")
	if err != nil {
		t.Fatal(err)
	}
	var out []string
	for _, e := range entries {
		n := e.Name()
		if !e.IsDir() || strings.HasPrefix(n, ".") || (slices.Contains(notFach, n) && n != "view") {
			continue
		}
		out = append(out, n)
	}
	return out
}

func TestImportRule(t *testing.T) {
	pkgs := checkedPackages(t)
	for _, want := range append(slices.Clone(fachpakete), "view") {
		if !slices.Contains(pkgs, want) {
			t.Errorf("Paket %s fehlt", want)
		}
	}
	for _, pkg := range pkgs {
		bad, err := violations(pkg, readPackage(t, filepath.Join("..", pkg)))
		if err != nil {
			t.Fatal(err)
		}
		for _, b := range bad {
			t.Errorf("🚫 Importregel: %s", b)
		}
	}
}

// Der Test schlägt bei einem künstlichen Querimport an (AC-02): verbotene Importe werden gemeldet, erlaubte nicht.
func TestImportRuleCatchesCrossImport(t *testing.T) {
	src := func(pkg string, paths ...string) map[string][]byte {
		var b strings.Builder
		fmt.Fprintf(&b, "package %s\n\nimport (\n", pkg)
		for _, p := range paths {
			fmt.Fprintf(&b, "\t_ %q\n", p)
		}
		b.WriteString(")\n")
		return map[string][]byte{"x.go": []byte(b.String())}
	}
	forbidden := []string{module + "/actors", module + "/game", module + "/local", "k3c/engine/room", "k3c/native/k3c-rig/paint", "github.com/fremd/paket"}
	for _, p := range forbidden {
		bad, err := violations("world", src("world", p))
		if err != nil || len(bad) != 1 {
			t.Errorf("Querimport %s nicht erkannt: %v %v", p, bad, err)
		}
	}
	ok := []string{"image", "github.com/hajimehoshi/ebiten/v2/vector", module + "/view", module + "/art", "k3c/engine/sim", "k3c/data"}
	if bad, err := violations("world", src("world", ok...)); err != nil || len(bad) != 0 {
		t.Errorf("erlaubte Importe gemeldet: %v %v", bad, err)
	}
	if bad, _ := violations("view", src("view", module+"/art", "k3c/data")); len(bad) != 2 {
		t.Errorf("view darf nur sim kennen: %v", bad)
	}
	test := map[string][]byte{"x_test.go": src("world", "k3c/engine/level")["x.go"]}
	if bad, _ := violations("world", test); len(bad) != 0 {
		t.Errorf("Test darf engine nutzen: %v", bad)
	}
}
