package planning

import (
	"reflect"
	"testing"
)

func TestRankErbtPrioUndFolgtAbhaengigkeiten(t *testing.T) {
	ses := func(nr, deps string) Session { return Session{Nr: nr, Text: "- **Abhängig von:** " + deps + "\n"} }
	d := Data{
		Tickets: []Ticket{{Nr: "B-001", Prio: "niedrig"}, {Nr: "B-002", Prio: "hoch"}, {Nr: "B-003", Prio: "mittel"}},
		Sprints: []Sprint{
			{ID: "A1", Status: "geplant", Tickets: []string{"B-001"}, Sessions: []Session{ses("A1.1", "–")}},
			{ID: "B1", Status: "geplant", Tickets: []string{"B-003"}, Sessions: []Session{ses("B1.1", "–")}},
			{ID: "C1", Status: "aktiv", Prio: "niedrig", Tickets: []string{"B-002", "B-001"}, Sessions: []Session{ses("C1.1", "A1.1, B-009")}},
			{ID: "X1", Status: "erledigt", Prio: "hoch"},
			{ID: "D1", Status: "geplant", Prio: "mittel", Tickets: []string{"B-404"}},
		},
	}
	rank(&d)
	var ids []string
	for _, s := range d.Sprints {
		ids = append(ids, s.ID)
	}
	// A (niedrig) erbt „hoch“ von C, das auf A.1 wartet; D behält die README-Prio; Erledigte zuletzt.
	if want := []string{"A1", "C1", "B1", "D1", "X1"}; !reflect.DeepEqual(ids, want) {
		t.Fatalf("Reihenfolge %v, erwartet %v", ids, want)
	}
	if c := d.Sprints[1]; c.Prio != "hoch" || !reflect.DeepEqual(c.Deps, []string{"A1"}) || !reflect.DeepEqual(c.Sessions[0].Deps, []string{"A1.1"}) {
		t.Fatalf("C: Prio %q, Deps %v, Session-Deps %v", c.Prio, c.Deps, c.Sessions[0].Deps)
	}
	if d.Sprints[3].Prio != "mittel" {
		t.Fatalf("D ohne bewertetes Ticket behält die README-Prio, hat %q", d.Sprints[3].Prio)
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
