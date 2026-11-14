package fx

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"testing"
)

// Textregel (textRule.test.ts): Spieltexte stehen nur in texts.go; kein anderes Literal mit Umlaut oder zwei Wörtern.
func TestKeineTexteAusserhalbDerTextdatei(t *testing.T) {
	german := regexp.MustCompile(`[äöüßÄÖÜ]|[A-Za-z]{2,}\s+[A-Za-z]{2,}`)
	files, _ := filepath.Glob("*.go")
	for _, name := range files {
		if strings.HasSuffix(name, "_test.go") || name == "texts.go" {
			continue
		}
		src, err := os.ReadFile(name)
		if err != nil {
			t.Fatal(err)
		}
		f, err := parser.ParseFile(token.NewFileSet(), name, src, 0)
		if err != nil {
			t.Fatal(err)
		}
		ast.Inspect(f, func(n ast.Node) bool {
			if lit, ok := n.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				if s, _ := strconv.Unquote(lit.Value); german.MatchString(s) && !strings.Contains(s, "💥") {
					t.Errorf("%s: Text-Literal %q gehört nach texts.go", name, s)
				}
			}
			return true
		})
	}
}
