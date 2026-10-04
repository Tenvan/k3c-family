package sim

import (
	"bytes"
	"testing"
)

// Spielstand Version 4 (S2.2, B-147): Tag und Phase beim Speichern; Version 3 ohne die Felder lädt weiter.

// nightIsland ist eine Insel mit zwei Spielern in den Stufen 0 und 1, bis in die Nacht gerechnet (schneller Zyklus).
func nightIsland(t *testing.T) *Island {
	t.Helper()
	isl := mustIsland(t, "familie", []int{0, 1})
	a, b := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 1)
	a.Gold, b.Gold = 23, 8
	isl.Stock.Stone = 12
	for i := 0; isl.Stages[0].Cycle.Phase != "night"; i++ {
		if i > 30*60 {
			t.Fatalf("keine Nacht nach 60 s, Phase %q", isl.Stages[0].Cycle.Phase)
		}
		StepIsland(isl, nil, 1.0/30)
	}
	return isl
}

// (a/c) ToSave schreibt Tag, Phase und Tiefen; Speichern, Laden, Speichern ist gleich.
func TestIslandSaveV4Rundlauf(t *testing.T) {
	isl := nightIsland(t)
	s := isl.ToSave("2026-10-04T22:00:00Z")
	if s.Version != 4 || s.Phase != "night" || s.Day != isl.Stages[0].Cycle.Day || s.Day < 1 {
		t.Fatalf("Version %d, Tag %d, Phase %q", s.Version, s.Day, s.Phase)
	}
	if len(s.Players) != 2 || s.Players[0].Depth != 0 || s.Players[1].Depth != 1 {
		t.Fatalf("Spieler: %+v", s.Players)
	}
	first := saveJSON(t, s)
	loaded := loadIsland(t, first)
	if second := saveJSON(t, loaded.ToSave("2026-10-04T22:00:00Z")); !bytes.Equal(first, second) {
		t.Fatalf("Speichern, Laden, Speichern ergibt etwas anderes:\n%s\n%s", first, second)
	}
}

// (c) Das Fixture v4 lädt mit Tag und Phase; das Fixture v3 (ohne die Felder) lädt mit leeren Feldern als Version 4.
func TestIslandSaveV4FixtureUndV3(t *testing.T) {
	s, err := ParseIslandSave(readFixture(t, 4))
	if err != nil || s.Version != IslandSaveVersion || s.Phase != "night" || s.Day < 1 || s.SavedAt == "" {
		t.Fatalf("v4: %v, Version %d, Tag %d, Phase %q, savedAt %q", err, s.Version, s.Day, s.Phase, s.SavedAt)
	}
	old, err := ParseIslandSave(readFixture(t, 3))
	if err != nil || old.Version != IslandSaveVersion || old.Day != 0 || old.Phase != "" {
		t.Fatalf("v3: %v, Version %d, Tag %d, Phase %q", err, old.Version, old.Day, old.Phase)
	}
	if bytes.Contains(saveJSON(t, old), []byte(`"phase"`)) {
		t.Fatal("ein Stand ohne Phase schreibt kein leeres Feld")
	}
	if _, err := FromIslandSave(old, islandSpeed); err != nil {
		t.Fatalf("v3: Insel aus dem Stand: %v", err)
	}
}
