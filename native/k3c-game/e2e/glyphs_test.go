package e2e

// Zeichenabdeckung der Schrift (NG18.1): Jedes Zeichen in den Textliteralen von hud und menu muss in
// public/schrift/k3c.ttf vorkommen. Fehlende Zeichen werden alle genannt (Ersatz entscheidet der HUD-Sprint).

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"strings"
	"testing"

	"golang.org/x/image/font/sfnt"
)

const fontFile = "../../../public/schrift/k3c.ttf"

// knownGaps: Zeichen, die Go-Regular nicht hat (leer seit NG13: Balken ■ □, Pfeile ► ◄ statt ▮ ▯ ▶ ◀).
// Jedes weitere fehlende Zeichen macht den Test rot.
const knownGaps = ""

// literalRunes sammelt die Zeichen aller String-Literale der Nicht-Test-Dateien in dir.

// logEmoji: Emojis der Log-Liste aus CLAUDE.md unterhalb von U+1F000; sie stehen nur in Log-Meldungen, nie im Bild.
var logEmoji = map[rune]bool{'✅': true, '❌': true, '⏳': true, '⏩': true, '⚠': true}

func literalRunes(t *testing.T, dir string, into map[rune]string) {
	t.Helper()
	files, err := filepath.Glob(filepath.Join(dir, "*.go"))
	if err != nil {
		t.Fatal(err)
	}
	fset := token.NewFileSet()
	for _, name := range files {
		if strings.HasSuffix(name, "_test.go") {
			continue
		}
		file, err := parser.ParseFile(fset, name, nil, 0)
		if err != nil {
			t.Fatal(err)
		}
		ast.Inspect(file, func(n ast.Node) bool {
			lit, ok := n.(*ast.BasicLit)
			if !ok || lit.Kind != token.STRING {
				return true
			}
			s, err := strconv.Unquote(lit.Value)
			if err != nil {
				return true
			}
			for _, r := range s {
				if r >= ' ' && r != 0x7f && r < 0x1F000 && !logEmoji[r] { // Emojis stehen nur in Log-Meldungen
					into[r] = filepath.Base(name)
				}
			}
			return true
		})
	}
}

func TestFontCoversTexts(t *testing.T) {
	data, err := os.ReadFile(fontFile)
	if err != nil {
		t.Skipf("Schrift fehlt: %v", err)
	}
	f, err := sfnt.Parse(data)
	if err != nil {
		t.Fatal(err)
	}
	runes := map[rune]string{}
	for _, r := range "–→←■□►◄·äöüÄÖÜß" {
		runes[r] = "Pflichtliste"
	}
	literalRunes(t, "../hud", runes)
	literalRunes(t, "../menu", runes)
	var buf sfnt.Buffer
	var missing []string
	for r, from := range runes {
		if idx, err := f.GlyphIndex(&buf, r); err != nil || idx == 0 {
			if strings.ContainsRune(knownGaps, r) {
				t.Logf("bekannte Lücke %q (%s)", r, from)
				continue
			}
			missing = append(missing, strconv.QuoteRune(r)+" ("+from+")")
		}
	}
	slices.Sort(missing)
	if len(missing) > 0 {
		t.Errorf("%d Zeichen fehlen in der Schrift: %s", len(missing), strings.Join(missing, ", "))
	}
}
