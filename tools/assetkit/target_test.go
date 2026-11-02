package assets

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// B-685/AC-03: Zielmaß setzen, entfernen und Schreibregel; nur Grafik-Slots, ungültige Werte ohne Schreibzugriff.
func TestSetTargetIf(t *testing.T) {
	root := repo(t)
	stand, _ := Stamp(root)
	slot, next, err := SetTargetIf(root, "farm", &Units{W: 6, H: 4.5}, stand)
	if err != nil || slot.Target == nil || *slot.Target != (Units{W: 6, H: 4.5}) || next == stand {
		t.Fatalf("setzen: %+v %v", slot, err)
	}
	reg, err := Load(root)
	if err != nil || *reg.Find("farm").Target != (Units{W: 6, H: 4.5}) {
		t.Fatalf("gespeichert: %+v %v", reg, err)
	}
	if _, _, err := SetTargetIf(root, "farm", &Units{W: 1, H: 1}, stand); !errors.Is(err, ErrStale) {
		t.Errorf("veralteter Stand: %v", err)
	}
	before, _ := os.ReadFile(filepath.Join(root, RegisterPath))
	for name, c := range map[string]struct {
		id string
		t  *Units
	}{"negativ": {"farm", &Units{W: -1, H: 2}}, "zu groß": {"farm", &Units{W: 2, H: 101}}, "Sound": {"sfx:coin", &Units{W: 1, H: 1}}, "unbekannt": {"x", nil}} {
		if _, _, err := SetTargetIf(root, c.id, c.t, next); err == nil {
			t.Errorf("%s: kein Fehler", name)
		}
	}
	if after, _ := os.ReadFile(filepath.Join(root, RegisterPath)); string(after) != string(before) {
		t.Error("abgelehntes Zielmaß hat geschrieben")
	}
	slot, _, err = SetTargetIf(root, "farm", &Units{W: 0, H: 3}, next)
	if err != nil || slot.Target != nil {
		t.Errorf("Seite 0 entfernt das Zielmaß: %+v %v", slot, err)
	}
}

// B-685: Ein ungültiges Zielmaß im Register ist ein Ladefehler mit Slot.
func TestLoadPrueftZiel(t *testing.T) {
	root := repo(t)
	writeFile(t, filepath.Join(root, RegisterPath), `{"_comment": "", "slots": [{"id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "", "ziel": {"w": -2, "h": 3}, "wahl": []}]}`)
	if _, err := Load(root); err == nil || !strings.Contains(err.Error(), "farm") {
		t.Errorf("Fehler %v, erwartet mit Slot farm", err)
	}
}
