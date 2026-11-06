package assetsui

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

// root ist die Repo-Wurzel; die Fixtures liegen unter tools/paper/fixtures/ui/ (PV1.4).
const root = "../../../.."

// fixture liest die Beispielantwort einer Methode als generischen JSON-Wert.
func fixture(t *testing.T, method string) any {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(root, "tools/paper/fixtures/ui", method+".json"))
	if err != nil {
		t.Fatal(err)
	}
	var v any
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatal(err)
	}
	return v
}

// roundtrip macht aus einem typisierten Ergebnis wieder generisches JSON (so sieht es die Oberfläche).
func roundtrip(t *testing.T, v any) any {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	var out any
	if err := json.Unmarshal(raw, &out); err != nil {
		t.Fatal(err)
	}
	return out
}

// TestAssetsUIStubs: jede RPC-Methode der Asset-Oberfläche liefert ihre Fixture-Antwort (AC-07).
func TestAssetsUIStubs(t *testing.T) {
	s := Stubs{Root: root}
	calls := map[string]func() (any, error){
		"AssetsList":      func() (any, error) { return s.AssetsList() },
		"AssetsGet":       func() (any, error) { return s.AssetsGet("monarch") },
		"AssetsStyle":     func() (any, error) { return s.AssetsStyle() },
		"AssetsPrompts":   func() (any, error) { return s.AssetsPrompts() },
		"AssetsReview":    func() (any, error) { return s.AssetsReview("HEAD~1") },
		"AssetsFrame":     func() (any, error) { return s.AssetsFrame("monarch", "reiten", 0, "", "") },
		"AssetsPixelDiff": func() (any, error) { return s.AssetsPixelDiff("monarch", "reiten", 0, "HEAD~1") },
	}
	for name, call := range calls {
		t.Run(name, func(t *testing.T) {
			got, err := call()
			if err != nil {
				t.Fatal(err)
			}
			if want := fixture(t, name); !reflect.DeepEqual(roundtrip(t, got), want) {
				t.Errorf("%s weicht von der Fixture ab", name)
			}
		})
	}
}

// TestAssetsUIUnbekanntesAsset: Get, Frame und PixelDiff kennen nur die Fixture monarch; sonst Vertragsfehler.
func TestAssetsUIUnbekanntesAsset(t *testing.T) {
	s := Stubs{Root: root}
	want := "Asset „drache“ fehlt"
	_, e1 := s.AssetsGet("drache")
	_, e2 := s.AssetsFrame("drache", "idle", 0, "", "")
	_, e3 := s.AssetsPixelDiff("drache", "idle", 0, "HEAD")
	for i, err := range []error{e1, e2, e3} {
		if err == nil || err.Error() != want {
			t.Errorf("Aufruf %d: %v", i, err)
		}
	}
}

// TestAssetsUIBasis: ungültige Basis wird mit den erlaubten Werten abgelehnt; gültige Formen gehen durch.
func TestAssetsUIBasis(t *testing.T) {
	s := Stubs{Root: root}
	for _, ok := range []string{"", "HEAD", "HEAD~1", "origin/develop", "32eb7a7", "32EB7A7", strings.Repeat("a", 40)} {
		if _, err := s.AssetsReview(ok); err != nil {
			t.Errorf("Basis %q abgelehnt: %v", ok, err)
		}
	}
	// PixelDiff vergleicht den Arbeitsstand mit einer Basis: leer ist dort keine Basis.
	if _, err := s.AssetsPixelDiff("monarch", "reiten", 0, ""); err == nil {
		t.Error("PixelDiff ohne Basis angenommen")
	}
	for _, bad := range []string{"main", "HEAD~2", "abc", "; rm -rf"} {
		_, err := s.AssetsReview(bad)
		if err == nil || !strings.Contains(err.Error(), "HEAD, HEAD~1, Commit-SHA oder origin/develop") {
			t.Errorf("Basis %q: %v", bad, err)
		}
		if _, err := s.AssetsPixelDiff("monarch", "reiten", 0, bad); err == nil {
			t.Errorf("PixelDiff mit Basis %q angenommen", bad)
		}
	}
}
