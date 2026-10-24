package planning

import (
	"maps"
	"regexp"
	"slices"
	"strings"
	"testing"

	ticketkit "k3c/tools/assetkit/ticket"
)

// headings sind die `## `-Überschriften eines Texts in Reihenfolge.
func headings(text string) []string {
	var out []string
	for _, l := range splitLines(text) {
		if strings.HasPrefix(l, "## ") {
			out = append(out, l)
		}
	}
	return out
}

// checkTicketTemplate prüft wie tests/planningDocs.ts › checkTemplate: Felder, erlaubte Werte, Überschriften, Spec.
func checkTicketTemplate(t *testing.T, root, text string) {
	t.Helper()
	tpl, _, _ := readText(root, "vorlagen/ticket.md")
	known := headFields(tpl)
	f := headFields(text)
	if !slices.Equal(slices.Sorted(maps.Keys(f)), slices.Sorted(maps.Keys(known))) {
		t.Errorf("Felder %v, Vorlage %v", slices.Sorted(maps.Keys(f)), slices.Sorted(maps.Keys(known)))
	}
	for k, v := range f {
		if err := checkValue("ticket", k, v, known); err != nil {
			t.Errorf("%s: %v", k, err)
		}
	}
	if !slices.Equal(headings(text), headings(tpl)) {
		t.Errorf("Überschriften weichen ab: %v", headings(text))
	}
	if _, err := setFields(root, "ticket", text, nil); err != nil { // Spec ↔ Freigabe, keine Platzhalter
		t.Error(err)
	}
}

// B-595/AC-02: Ein Ticket aus dem Kern (Studio-Weg) erfüllt die Regeln von tests/planning.test.ts und den
// Schreibweg von planning (Set zieht dieselbe Index-Zeile nach).
func TestKernTicketBestehtPlanungsRegeln(t *testing.T) {
	root := tempRepo(t)
	newProject(t, root, "WZG", "werkzeuge")
	got, err := ticketkit.Create(root, ticketkit.Input{Titel: "Knopf fehlt", Beschreibung: "Text", Typ: "Idee",
		Prio: "hoch", Domaene: "DEV", Projekt: "WZG", Kontext: ticketkit.Context{App: "k3c-studio", Seite: "grafik"}})
	if err != nil {
		t.Fatal(err)
	}
	rel := strings.TrimPrefix(got.Pfad, "docs/")
	text, _, err := readText(root, rel)
	if err != nil {
		t.Fatal(err)
	}
	checkTicketTemplate(t, root, text)
	tk := ParseTicket(text)
	if tk.Nr != got.ID || tk.Title != got.Titel || tk.Project == "–" || !strings.Contains(text, "## Akzeptanzkriterien\n\n- **AC-01**") {
		t.Errorf("Ticket %+v", tk)
	}
	index, _, _ := readText(root, "backlog/README.md")
	row := regexp.MustCompile(`(?m)^\| \[(B-\d{3})\]\((.+?)\) \|.*\| (\S+) \| \S+ \| .* \|$`)
	if !slices.ContainsFunc(row.FindAllStringSubmatch(index, -1), func(m []string) bool {
		return m[1] == got.ID && m[2] == strings.TrimPrefix(rel, "backlog/") && m[3] == "offen"
	}) {
		t.Fatalf("Index-Zeile fehlt:\n%s", index)
	}
	must(t)(Set(root, got.ID, map[string]string{"Prio": "niedrig"}))
	after, _, _ := readText(root, "backlog/README.md")
	if strings.Count(after, "["+got.ID+"]") != 1 || !strings.Contains(after, "| DEV | Idee | niedrig | offen |") {
		t.Errorf("Set fand die Zeile des Kerns nicht:\n%s", after)
	}
}
