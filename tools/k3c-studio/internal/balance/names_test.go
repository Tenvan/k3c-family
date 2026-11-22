package balance

import (
	"go/ast"
	"go/parser"
	"go/token"
	"path/filepath"
	"slices"
	"strconv"
	"testing"
)

// RS2.2 (B-726): Die Namen der Workbench folgen engine/balance, ohne es zu importieren: Der Test liest den Quelltext
// (Schlüssel der Map bots, minPlayers, Konstanten der Hilfen) und vergleicht.

const engineBalance = "../../../../engine/balance"

func parseEngine(t *testing.T, file string) *ast.File {
	t.Helper()
	f, err := parser.ParseFile(token.NewFileSet(), filepath.Join(engineBalance, file), nil, 0)
	if err != nil {
		t.Fatal(err)
	}
	return f
}

// mapLiteral liefert die Schlüssel und (falls Zahl) Werte der Paket-Variable name.
func mapLiteral(t *testing.T, f *ast.File, name string) map[string]string {
	t.Helper()
	out := map[string]string{}
	ast.Inspect(f, func(n ast.Node) bool {
		vs, ok := n.(*ast.ValueSpec)
		if !ok || len(vs.Names) != 1 || vs.Names[0].Name != name || len(vs.Values) != 1 {
			return true
		}
		lit, _ := vs.Values[0].(*ast.CompositeLit)
		for _, e := range lit.Elts {
			kv := e.(*ast.KeyValueExpr)
			key, _ := strconv.Unquote(kv.Key.(*ast.BasicLit).Value)
			val := ""
			if b, ok := kv.Value.(*ast.BasicLit); ok {
				val = b.Value
			}
			out[key] = val
		}
		return false
	})
	if len(out) == 0 {
		t.Fatalf("%s nicht gefunden", name)
	}
	return out
}

func TestBotNamenWieEngine(t *testing.T) {
	f := parseEngine(t, "bots.go")
	bots := mapLiteral(t, f, "bots")
	names := make([]string, 0, len(bots))
	for n := range bots {
		names = append(names, n)
	}
	slices.Sort(names)
	if !slices.Equal(names, BotNames()) {
		t.Errorf("Profile engine/balance %v, Workbench %v", names, BotNames())
	}
	for n, v := range mapLiteral(t, f, "minPlayers") {
		if strconv.Itoa(minPlayers[n]) != v {
			t.Errorf("minPlayers[%s]: engine %s, Workbench %d", n, v, minPlayers[n])
		}
	}
	if len(mapLiteral(t, f, "minPlayers")) != len(minPlayers) {
		t.Errorf("minPlayers: andere Anzahl als engine/balance")
	}
}

func TestHilfenWieEngine(t *testing.T) {
	want := map[string]string{"HelpsAids": HelpsAids, "HelpsFull": HelpsFull, "HelpsBoss": HelpsBoss, "MixSep": MixSep}
	got := map[string]string{}
	for _, file := range []string{"helps.go", "bots.go"} {
		for _, d := range parseEngine(t, file).Decls {
			g, ok := d.(*ast.GenDecl)
			if !ok || g.Tok != token.CONST {
				continue
			}
			for _, s := range g.Specs {
				vs := s.(*ast.ValueSpec)
				if lit, ok := vs.Values[0].(*ast.BasicLit); ok && want[vs.Names[0].Name] != "" {
					got[vs.Names[0].Name], _ = strconv.Unquote(lit.Value)
				}
			}
		}
	}
	for k, v := range want {
		if got[k] != v {
			t.Errorf("%s: engine %q, Workbench %q", k, got[k], v)
		}
	}
}

func TestKnownBotUndWelten(t *testing.T) {
	for name, ok := range map[string]bool{"saver": true, "coop2": true, "saver+passive": true, "coop2+saver": false, "": false, "raten": false} {
		if KnownBot(name) != ok {
			t.Errorf("KnownBot(%q) = %v", name, !ok)
		}
	}
	if w, err := loadWorld("standard"); err != nil || len(w.Islands) == 0 || len(w.Islands[0].Depths) == 0 {
		t.Errorf("standard: %+v %v", w, err)
	}
	if _, err := loadWorld("../targets"); err == nil {
		t.Error("Pfad außerhalb von data/worlds angenommen")
	}
}
