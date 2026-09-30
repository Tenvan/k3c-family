// Package tests prüft die Verschachtelungstiefe von Go-Code so, wie Oxlint `max-depth` sie für TypeScript zählt.
// Kein Linter aus golangci-lint zählt `for range` und `else if` dabei gleich (B-057).
package tests

import (
	"errors"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"io/fs"
	"path/filepath"
	"strings"
	"testing"
)

// maxDepth ist die harte Grenze „Verschachtelung“ aus docs/arbeitsweise.md › Komplexitäts-Budget.
const maxDepth = 4

// goDirs sind die Ordner mit Go-Code, relativ zu diesem Paket (tests/); fehlende werden übersprungen.
var goDirs = []string{"../data", "../engine", "../cmd", "../tools", "."}

func TestGoNestingDepth(t *testing.T) {
	fset := token.NewFileSet()
	for _, path := range goFiles(t) {
		file, err := parser.ParseFile(fset, path, nil, 0)
		if err != nil {
			t.Errorf("%s lässt sich nicht parsen: %v", path, err)
			continue
		}
		c := checker{fset: fset}
		c.walk("Paketebene", file, 0)
		for _, msg := range c.found {
			t.Error(msg)
		}
	}
}

func goFiles(t *testing.T) []string {
	var files []string
	for _, dir := range goDirs {
		err := filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
			if err == nil && !d.IsDir() && strings.HasSuffix(path, ".go") {
				files = append(files, path)
			}
			return err
		})
		if err != nil && !errors.Is(err, fs.ErrNotExist) {
			t.Fatal(err)
		}
	}
	return files
}

// checker zählt die Tiefe wie ESLint/Oxlint `max-depth`: `if`, `for`, `for range`, `switch`, Typ-`switch` und
// `select` erhöhen sie um 1, ein `else if` nicht; ein Funktionsliteral beginnt wieder bei 0.
type checker struct {
	fset  *token.FileSet
	found []string
}

// walk durchläuft die Kinder von root auf Tiefe level (root selbst zählt nicht).
func (c *checker) walk(name string, root ast.Node, level int) {
	ast.Inspect(root, func(n ast.Node) bool {
		if n == root || n == nil {
			return true
		}
		switch n := n.(type) {
		case *ast.FuncDecl:
			if n.Body != nil {
				c.walk(n.Name.Name, n.Body, 0)
			}
		case *ast.FuncLit:
			c.walk("Funktionsliteral in "+name, n.Body, 0)
		case *ast.IfStmt:
			c.ifStmt(name, n, level+1)
		case *ast.ForStmt, *ast.RangeStmt, *ast.SwitchStmt, *ast.TypeSwitchStmt, *ast.SelectStmt:
			if !c.tooDeep(name, n, level+1) {
				c.walk(name, n, level+1)
			}
		default:
			return true
		}
		return false
	})
}

// ifStmt prüft ein `if` auf Tiefe level; ein `else if` bleibt auf derselben Tiefe.
func (c *checker) ifStmt(name string, n *ast.IfStmt, level int) {
	if c.tooDeep(name, n, level) {
		return
	}
	for _, part := range []ast.Node{n.Init, n.Cond, n.Body} {
		if part != nil {
			c.walk(name, part, level)
		}
	}
	switch e := n.Else.(type) {
	case *ast.IfStmt:
		c.ifStmt(name, e, level)
	case *ast.BlockStmt:
		c.walk(name, e, level)
	}
}

// tooDeep meldet n, wenn level über der Grenze liegt; darunter wird dann nicht weiter gesucht.
func (c *checker) tooDeep(name string, n ast.Node, level int) bool {
	if level <= maxDepth {
		return false
	}
	pos := c.fset.Position(n.Pos())
	c.found = append(c.found, fmt.Sprintf("%s:%d: Funktion %s hat Tiefe %d (> %d)", pos.Filename, pos.Line, name, level, maxDepth))
	return true
}
