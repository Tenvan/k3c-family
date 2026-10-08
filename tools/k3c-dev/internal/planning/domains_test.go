package planning

import (
	"strings"
	"testing"
)

func newSession(t *testing.T, root, id, domain string) {
	t.Helper()
	must(t)(Create(root, NewDoc{Kind: "session", ID: id, Slug: "s", Title: "Session " + id, Fields: map[string]string{"Domäne": domain}}))
}

// TestSprintDomaeneFolgtSessions: Domänen in Reihenfolge des ersten Auftretens in Feld, Überschrift und Fahrplan (AC-04).
func TestSprintDomaeneFolgtSessions(t *testing.T) {
	root := tempRepo(t)
	newSprint(t, root, "X5", nil)
	newSession(t, root, "X5.1", "SIM")
	newSession(t, root, "X5.2", "SRV")
	newSession(t, root, "X5.3", "SIM")
	readme := doc(t, root, "sprints/geplant/X5-s/README.md")
	if !strings.HasPrefix(readme, "# X5 · SIM, SRV · Sprint X5\n") || !strings.Contains(readme, "- **Domäne:** SIM, SRV\n") {
		t.Fatalf("README:\n%s", readme)
	}
	if !strings.Contains(doc(t, root, "sprints/README.md"), "| X5 | SIM, SRV | Sprint X5 |") {
		t.Fatalf("Fahrplan:\n%s", doc(t, root, "sprints/README.md"))
	}
	must(t)(Set(root, "X5.1", map[string]string{"Domäne": "CLI"}))
	if readme := doc(t, root, "sprints/geplant/X5-s/README.md"); !strings.HasPrefix(readme, "# X5 · CLI, SRV, SIM · ") ||
		!strings.Contains(readme, "- **Domäne:** CLI, SRV, SIM\n") {
		t.Fatalf("nach Änderung:\n%s", readme)
	}
	must(t)(Delete(root, "X5.1"))
	if readme := doc(t, root, "sprints/geplant/X5-s/README.md"); !strings.HasPrefix(readme, "# X5 · SRV, SIM · ") {
		t.Fatalf("nach Löschen:\n%s", readme)
	}
	d := load(t, root)
	if s := d.Sprints[0]; s.Domain != "SRV, SIM" || s.Sessions[0].Domain != "SRV" || s.Sessions[1].Domain != "SIM" {
		t.Fatalf("geladen: %q %+v", s.Domain, s.Sessions)
	}
	checkConsistent(t, root)
}

// TestSessionDomaeneUndVerworfen: Domäne Pflicht und geprüft, Sprint-Domäne nicht direkt setzbar, verworfen erlaubt (AC-04, AC-05).
func TestSessionDomaeneUndVerworfen(t *testing.T) {
	root := tempRepo(t)
	newSprint(t, root, "X6", nil)
	newSession(t, root, "X6.1", "SRV")
	before := snapshot(t, root)
	bad := map[string]func() error{
		"ohne Domäne": func() error {
			_, err := Create(root, NewDoc{Kind: "session", ID: "X6.2", Slug: "a", Title: "x"})
			return err
		},
		"Domäne XYZ":    func() error { _, err := Set(root, "X6.1", map[string]string{"Domäne": "XYZ"}); return err },
		"Sprint-Domäne": func() error { _, err := Set(root, "X6", map[string]string{"Domäne": "SIM"}); return err },
	}
	for name, call := range bad {
		if err := call(); err == nil {
			t.Errorf("%s: kein Fehler", name)
		}
		if snapshot(t, root) != before {
			t.Errorf("%s: Dateien geändert", name)
		}
	}
	must(t)(Set(root, "X6.1", map[string]string{"Status": "verworfen"}))
	if !strings.Contains(doc(t, root, "sprints/geplant/X6-s/README.md"), "| X6.1 | `X6.1-s.md` | Umsetzung | autonom | verworfen |") {
		t.Fatal("Session-Tabelle nicht nachgezogen")
	}
	if out := must(t)(List(root, Filter{Sprint: "X6"})); !strings.Contains(out, "Sessions 1/1") || !strings.Contains(out, "X6.1 SRV ") {
		t.Fatalf("Liste:\n%s", out)
	}
}

// TestProjektSprintVorPrio: Ein Sprint mit Projekt (Rang 1) steht vor einem ohne Projekt mit Prio hoch; innerhalb des
// Projekts zählt der Platz in der Sprint-Tabelle (AC-04).
func TestProjektSprintVorPrio(t *testing.T) {
	root := tempRepo(t)
	must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "h", Title: "Hoch", Fields: map[string]string{"Domäne": "SRV", "Typ": "Idee", "Prio": "hoch"}}))
	newProject(t, root, "GRA", "grafik")
	newSprint(t, root, "X7", map[string]string{"Tickets": "B-002"})
	newSprint(t, root, "X8", map[string]string{"Projekt": "GRA", "Tickets": "B-001"})
	newSprint(t, root, "X9", map[string]string{"Projekt": "GRA"})
	must(t)(Set(root, "GRA", map[string]string{"Sprints": "X9, X8"}))
	out := must(t)(List(root, Filter{Kind: "sprint"}))
	if i9, i8, i7 := strings.Index(out, "X9 "), strings.Index(out, "X8 "), strings.Index(out, "X7 "); i9 > i8 || i8 > i7 {
		t.Fatalf("Reihenfolge:\n%s", out)
	}
	d := load(t, root)
	if d.Sprints[0].ID != "X9" || d.Sprints[2].ID != "X7" || d.Sprints[2].Prio != "hoch" {
		t.Fatalf("Load: %s %s %s (%s)", d.Sprints[0].ID, d.Sprints[1].ID, d.Sprints[2].ID, d.Sprints[2].Prio)
	}
}

func load(t *testing.T, root string) Data {
	t.Helper()
	d, err := Load(root)
	if err != nil {
		t.Fatal(err)
	}
	return d
}
