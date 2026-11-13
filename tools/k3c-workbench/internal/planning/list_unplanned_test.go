package planning

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// setSprintRaw schreibt das Sprint-Feld eines Tickets direkt (leer oder eine ID), wie es ältere Tickets haben.
func setSprintRaw(t *testing.T, root, nr, sprint string) {
	t.Helper()
	files, _ := filepath.Glob(filepath.Join(root, "docs", "backlog", nr+"-*.md"))
	if len(files) != 1 {
		t.Fatalf("Ticket %s: %v", nr, files)
	}
	data, err := os.ReadFile(files[0])
	if err != nil {
		t.Fatal(err)
	}
	var lines []string
	for _, l := range strings.Split(string(data), "\n") {
		if strings.HasPrefix(l, "- **Sprint:**") {
			l = strings.TrimRight("- **Sprint:** "+sprint, " ")
		}
		lines = append(lines, l)
	}
	if err := os.WriteFile(files[0], []byte(strings.Join(lines, "\n")), 0o644); err != nil {
		t.Fatal(err)
	}
}

// plan_list mit Sprint „–“ oder „ungeplant“ liefert wie der Schnellfilter Ungeplant der Workbench alle Tickets ohne
// Sprint, auch mit leerem Feld, und keine Sprints.
func TestListUngeplant(t *testing.T) {
	t.Parallel()
	root := tempRepo(t)
	for range 3 {
		newTicket(t, root)
	}
	setSprintRaw(t, root, "B-002", "X1")
	setSprintRaw(t, root, "B-003", "")
	newSprint(t, root, "X1", map[string]string{"Tickets": "B-002"})
	for _, want := range []string{"–", "ungeplant", "Ungeplant"} {
		out := must(t)(List(root, Filter{Sprint: want}))
		if !strings.Contains(out, "B-001 ") || !strings.Contains(out, "B-003 ") || strings.Contains(out, "B-002 ") || strings.Contains(out, "X1 ") {
			t.Errorf("Filter %q:\n%s", want, out)
		}
	}
	if out := must(t)(List(root, Filter{Kind: "ticket", Sprint: "X1"})); !strings.Contains(out, "B-002 ") || strings.Contains(out, "B-001 ") {
		t.Errorf("Filter X1:\n%s", out)
	}
}

// Kind ticket beginnt mit der Kennzahl-Zeile; „ungeplant“ zählt unabhängig vom Status und vom Filter.
func TestListKennzahlen(t *testing.T) {
	t.Parallel()
	root := tempRepo(t)
	for range 3 {
		newTicket(t, root)
	}
	setSprintRaw(t, root, "B-002", "X1")
	newSprint(t, root, "X1", map[string]string{"Tickets": "B-002"})
	want := "Tickets: 1 von 4 · 4 offen · 0 eingeplant · 3 ungeplant" + "\n" // tempRepo bringt ein Ticket mit
	out := must(t)(List(root, Filter{Kind: "ticket", Sprint: "X1"}))
	if !strings.HasPrefix(out, want) {
		t.Errorf("Kennzahl-Zeile:\n%s", out)
	}
	if first := strings.SplitN(must(t)(List(root, Filter{})), "\n", 2)[0]; strings.HasPrefix(first, "Tickets:") {
		t.Errorf("Sprint-Zeile zuerst erwartet: %s", first)
	}
}
