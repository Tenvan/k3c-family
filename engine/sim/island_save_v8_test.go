package sim

import (
	"bytes"
	"encoding/json"
	"strings"
	"testing"
	"testing/fstest"
)

// Spielstand Version 8 (WZ18.3, B-488/AC-03, docs/rules/welten.md § 5): Welt-ID und Insel-ID statt des Index, Migration
// alter Stände auf die Standard-Welt, unbekannte Welt oder Insel ist ein Fehler.

// nordmeer ist eine Test-Welt mit zwei Inseln, aus einem fstest.MapFS geladen (nicht aus data/worlds/).
func nordmeer(t *testing.T) WorldDef {
	t.Helper()
	raw := `{"id":"nordmeer","name":"Nordmeer","islands":[` +
		`{"id":"island1","name":"Insel 1","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1}},` +
		`{"id":"eisinsel","name":"Eisinsel","depths":[0,4],"depthScaling":{"hp":1.6,"damage":1.4,"speed":1.15}}]}`
	w, err := LoadWorld(fstest.MapFS{"nordmeer.json": {Data: []byte(raw)}}, "nordmeer")
	if err != nil {
		t.Fatal(err)
	}
	return w
}

// nordmeerIsland ist Insel 1 der Test-Welt mit n Spielern am Wechselpunkt (tiefste Stufe), Endboss besiegt.
func nordmeerIsland(t *testing.T, w WorldDef, n int) *Island {
	t.Helper()
	isl, err := CreateIslandIn(w, "nordmeer", []int{0, 4}, islandSpeed, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	for range n {
		AddIslandPlayer(isl, 1).X = isl.Stages[1].HubX
	}
	isl.EndbossDefeated = true
	return isl
}

// AC-03: Version 8 speichert `world` und `islandId`, nicht mehr den Index `island`; Speichern, Laden, Speichern ist gleich.
func TestSpielstandV8WeltUndInsel(t *testing.T) {
	w := nordmeer(t)
	next, err := NextIsland(nordmeerIsland(t, w, 2), w.Islands[1])
	if err != nil {
		t.Fatal(err)
	}
	s := next.ToSave("2026-10-19T20:00:00Z")
	first := saveJSON(t, s)
	if s.Version != 8 || s.World != "nordmeer" || s.IslandID != "eisinsel" || !bytes.Contains(first, []byte(`"world":"nordmeer","islandId":"eisinsel"`)) || bytes.Contains(first, []byte(`"island":`)) {
		t.Fatalf("Version %d, Welt %q, Insel %q: %s", s.Version, s.World, s.IslandID, first)
	}
	parsed, err := ParseIslandSave(first)
	if err != nil {
		t.Fatal(err)
	}
	got, err := FromIslandSaveIn(parsed, islandSpeed, w)
	if err != nil {
		t.Fatal(err)
	}
	if got.WorldID() != "nordmeer" || got.IslandID() != "eisinsel" || got.Number != 1 || got.Scaling != w.Islands[1].DepthScaling || !got.isLast() {
		t.Fatalf("Welt %q, Insel %q (%d), Tabelle %+v, letzte %v", got.WorldID(), got.IslandID(), got.Number, got.Scaling, got.isLast())
	}
	if second := saveJSON(t, got.ToSave("2026-10-19T20:00:00Z")); !bytes.Equal(first, second) {
		t.Errorf("Speichern, Laden, Speichern ergibt etwas anderes:\n%s\n%s", first, second)
	}
}

// AC-03: Stände bis Version 7 laden auf die Standard-Welt, Insel am alten Index (fehlt er: island1). Ein Index außerhalb
// der Standard-Welt ist ein Fehler wie bisher.
func TestSpielstandMigrationAufStandard(t *testing.T) {
	for v := 1; v < IslandSaveVersion; v++ {
		s, err := ParseIslandSave(readFixture(t, v))
		if err != nil {
			t.Fatalf("v%d: %v", v, err)
		}
		if s.Version != IslandSaveVersion || s.World != StandardWorldID || s.IslandID != "island1" {
			t.Errorf("v%d: Version %d, Welt %q, Insel %q", v, s.Version, s.World, s.IslandID)
		}
		if isl, err := FromIslandSave(s, islandSpeed); err != nil || isl.WorldID() != StandardWorldID || isl.IslandID() != "island1" {
			t.Errorf("v%d: Insel aus dem Stand: %v", v, err)
		}
	}
	raw := bytes.Replace(readFixture(t, 7), []byte(`"version": 7,`), []byte(`"version": 7, "island": 1,`), 1)
	if _, err := ParseIslandSave(raw); err == nil || !strings.Contains(err.Error(), "Insel 1") {
		t.Errorf("v7 mit Index 1: Fehler %v", err)
	}
	if id, err := legacyIslandID(nordmeer(t), 1); err != nil || id != "eisinsel" {
		t.Errorf("Index 1 in Nordmeer: %q, %v", id, err)
	}
}

// AC-03: Unbekannte Welt oder Insel lädt nicht; der Fehler nennt Welt und Insel. Ein Stand der Version 8 ohne `world`
// oder `islandId` wird abgelehnt.
func TestSpielstandWeltOderInselUnbekannt(t *testing.T) {
	cases := []struct {
		name  string
		world WorldDef
		edit  func(s *IslandSave)
		want  string
	}{
		{"Welt unbekannt", StandardWorld(), func(s *IslandSave) { s.World = "nordmeer" }, `spielstand: Welt "nordmeer" unbekannt`},
		{"Insel unbekannt", StandardWorld(), func(s *IslandSave) { s.IslandID = "eisinsel" }, `spielstand: Insel "eisinsel" gibt es in Welt "standard" nicht`},
		{"andere Welt", nordmeer(t), func(*IslandSave) {}, `spielstand: Welt "standard" unbekannt`},
	}
	for _, c := range cases {
		s, err := ParseIslandSave(readFixture(t, IslandSaveVersion))
		if err != nil {
			t.Fatal(err)
		}
		c.edit(&s)
		if _, err := FromIslandSaveIn(s, islandSpeed, c.world); err == nil || err.Error() != c.want {
			t.Errorf("%s: Fehler %v, erwartet %s", c.name, err, c.want)
		}
	}
	for _, field := range []string{"world", "islandId"} {
		var m map[string]json.RawMessage
		if err := json.Unmarshal(readFixture(t, IslandSaveVersion), &m); err != nil {
			t.Fatal(err)
		}
		delete(m, field)
		raw, _ := json.Marshal(m)
		if _, err := ParseIslandSave(raw); err == nil || !strings.Contains(err.Error(), field) {
			t.Errorf("ohne %s: Fehler %v", field, err)
		}
	}
}
