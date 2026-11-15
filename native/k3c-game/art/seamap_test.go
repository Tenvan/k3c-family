package art

import (
	"slices"
	"testing"
)

// B-964/R1, R5: Die Zuordnung der Seekarte in data/paper.json ist vollständig; eine Welt ohne Eintrag nutzt die
// Miniatur von standard.
func TestSeaMapAusDaten(t *testing.T) {
	m := DataSeaMap()
	if m == nil {
		t.Fatal("data/paper.json › seekarte nicht lesbar")
	}
	if got, std := m.Island("gibt-es-nicht"), m.Islands[StandardWorld]; got != std || m.Island("") != std {
		t.Errorf("Welt ohne Eintrag: %q, erwartet %q", got, std)
	}
	if m.Island("sengoku") == m.Island(StandardWorld) {
		t.Error("sengoku hat keine eigene Miniatur")
	}
	a := m.Assets()
	if !slices.IsSorted(a) || len(slices.Compact(slices.Clone(a))) != len(a) || len(a) != 6+len(m.Toppers)+len(m.Islands) {
		t.Errorf("Assets %v", a)
	}
}

// B-964/R2: Der Aufsatz hängt nur am Hash, ohne Allokation; jeder Aufsatz kommt vor.
func TestSeaMapAufsatzFest(t *testing.T) {
	m := &SeaMap{Toppers: []string{"a", "b", "c"}}
	seen := map[string]bool{}
	for seed := range uint32(9) {
		if m.Topper(seed) != m.Toppers[seed%3] {
			t.Errorf("seed %d: %q", seed, m.Topper(seed))
		}
		seen[m.Topper(seed)] = true
	}
	if len(seen) != 3 {
		t.Errorf("Aufsätze %v", seen)
	}
	var nilMap *SeaMap
	if nilMap.Topper(1) != "" || nilMap.Island("x") != "" || nilMap.Assets() != nil {
		t.Error("nil-Zuordnung liefert Assets")
	}
	if n := testing.AllocsPerRun(50, func() { _, _ = m.Topper(7), m.Island("x") }); n != 0 {
		t.Errorf("%v Allokationen", n)
	}
}

// Unvollständige Zuordnung ist ein Fehler (das HUD zeichnet dann Formen).
func TestSeaMapUnvollstaendig(t *testing.T) {
	full := `{"seekarte":{"inseln":{"standard":"o/i"},"meer":{"hinten":"o/h","mitte":"o/m","vorn":"o/v"},"schiff":"o/s",` +
		`"aufsaetze":["o/t"],"zeichen":{"abgeschlossen":"o/f","gesperrt":"o/n"}}}`
	if _, err := ParseSeaMap([]byte(full)); err != nil {
		t.Fatal(err)
	}
	for name, raw := range map[string]string{
		"fehlt":         `{}`,
		"ohne standard": `{"seekarte":{"inseln":{"nil":"o/i"},"meer":{"hinten":"o/h","mitte":"o/m","vorn":"o/v"},"schiff":"o/s","aufsaetze":["o/t"],"zeichen":{"abgeschlossen":"o/f","gesperrt":"o/n"}}}`,
		"ohne aufsatz":  `{"seekarte":{"inseln":{"standard":"o/i"},"meer":{"hinten":"o/h","mitte":"o/m","vorn":"o/v"},"schiff":"o/s","aufsaetze":[],"zeichen":{"abgeschlossen":"o/f","gesperrt":"o/n"}}}`,
		"ohne schiff":   `{"seekarte":{"inseln":{"standard":"o/i"},"meer":{"hinten":"o/h","mitte":"o/m","vorn":"o/v"},"aufsaetze":["o/t"],"zeichen":{"abgeschlossen":"o/f","gesperrt":"o/n"}}}`,
		"kaputt":        `{"seekarte":`,
	} {
		if _, err := ParseSeaMap([]byte(raw)); err == nil {
			t.Errorf("%s: kein Fehler", name)
		}
	}
}
