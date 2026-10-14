package assets

import (
	"os"
	"path/filepath"
	"slices"
	"testing"
)

// Das echte Slot-Register des Repos: jede Zuordnung gültig, Kategorien bekannt, IDs eindeutig,
// und Speichern ändert kein Byte (k3c-dev schreibt dasselbe Format wie die Erstbefüllung).
func TestEchtesRegister(t *testing.T) {
	root, _ := filepath.Abs(filepath.Join("..", "..", "..", ".."))
	reg, err := Load(root)
	if err != nil {
		t.Fatal(err)
	}
	seen := map[string]bool{}
	for _, s := range reg.Slots {
		if seen[s.ID] {
			t.Errorf("%s doppelt", s.ID)
		}
		seen[s.ID] = true
		if !slices.Contains(Categories[s.Kind], s.Category) {
			t.Errorf("%s: Kategorie %q passt nicht zur Art %q", s.ID, s.Category, s.Kind)
		}
		for i, c := range s.Choices {
			if err := checkChoice(root, s.Kind, c); err != nil {
				t.Errorf("%s Präferenz %d: %v", s.ID, i+1, err)
			}
		}
	}
	before, _ := os.ReadFile(filepath.Join(root, RegisterPath))
	tmp := t.TempDir()
	if err := os.MkdirAll(filepath.Join(tmp, "data"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := reg.Save(tmp); err != nil {
		t.Fatal(err)
	}
	after, _ := os.ReadFile(filepath.Join(tmp, RegisterPath))
	if string(normalize(before)) != string(after) {
		t.Error("Speichern ändert data/assets.json (Format weicht ab)")
	}
}

// normalize macht CRLF (Checkout unter Windows) zu LF.
func normalize(b []byte) []byte {
	out := make([]byte, 0, len(b))
	for i, c := range b {
		if c == '\r' && i+1 < len(b) && b[i+1] == '\n' {
			continue
		}
		out = append(out, c)
	}
	return out
}
