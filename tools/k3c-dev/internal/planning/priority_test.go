package planning

import (
	"reflect"
	"testing"
)

// TestRankErbtPlatzUndFolgtAbhaengigkeiten: Sprints eines Projekts nach Rang und Tabellenplatz, die ohne Projekt
// dahinter in Fahrplan-Reihenfolge; eine Voraussetzung erbt den Platz ihres Abnehmers.
func TestRankErbtPlatzUndFolgtAbhaengigkeiten(t *testing.T) {
	ses := func(nr, deps string) Session { return Session{Nr: nr, Text: "- **Abhängig von:** " + deps + "\n"} }
	d := Data{
		Projects: []Project{{ID: "GRA", Rang: "1", Sprints: []string{"C1", "B1"}}},
		Sprints: []Sprint{
			{ID: "A1", Status: "geplant", Sessions: []Session{ses("A1.1", "–")}},
			{ID: "B1", Status: "geplant", Sessions: []Session{ses("B1.1", "–")}},
			{ID: "C1", Status: "aktiv", Sessions: []Session{ses("C1.1", "A1.1, B-009")}},
			{ID: "X1", Status: "erledigt"},
			{ID: "D1", Status: "geplant"},
		},
	}
	rank(&d)
	var ids []string
	for _, s := range d.Sprints {
		ids = append(ids, s.ID)
	}
	// A (ohne Projekt) erbt den Platz von C, das auf A.1 wartet; D ohne Projekt zuletzt; Erledigte am Ende.
	if want := []string{"A1", "C1", "B1", "D1", "X1"}; !reflect.DeepEqual(ids, want) {
		t.Fatalf("Reihenfolge %v, erwartet %v", ids, want)
	}
	if c := d.Sprints[1]; !reflect.DeepEqual(c.Deps, []string{"A1"}) || !reflect.DeepEqual(c.Sessions[0].Deps, []string{"A1.1"}) {
		t.Fatalf("C: Deps %v, Session-Deps %v", c.Deps, c.Sessions[0].Deps)
	}
}

func TestOrderBrichtZyklus(t *testing.T) {
	deps := map[string][]string{"a": {"b"}, "b": {"a"}, "c": nil}
	got := Order([]string{"a", "b", "c"}, func(s string) string { return s }, func(s string) []string { return deps[s] },
		func(x, y string) bool { return false })
	if len(got) != 3 || got[2] == "" {
		t.Fatalf("alle Einträge erwartet, kam %v", got)
	}
}
