package planning

import (
	"strings"
	"testing"
)

func TestSetTitel(t *testing.T) {
	t.Parallel()
	root := tempRepo(t)
	must(t)(Set(root, "B-001", map[string]string{"Titel": "Neuer Ticket-Titel"}))
	idx, _, err := readText(root, "backlog/README.md")
	if err != nil || !strings.Contains(idx, "Neuer Ticket-Titel") || strings.Contains(idx, "Altes Ticket") {
		t.Errorf("Index: %v\n%s", err, idx)
	}
	must(t)(Create(root, NewDoc{Kind: "sprint", ID: "X1", Slug: "test", Title: "Test-Sprint", Fields: map[string]string{"Domäne": "SRV"}}))
	must(t)(Create(root, NewDoc{Kind: "session", ID: "X1.1", Slug: "eins", Title: "Erste", Fields: map[string]string{"Domäne": "SRV"}}))
	if _, err := Set(root, "X1.1", map[string]string{"Titel": "x"}); err == nil || !strings.Contains(err.Error(), "Sprints und Tickets") {
		t.Errorf("Titel bei Session: %v", err)
	}
	must(t)(Set(root, "X1", map[string]string{"Titel": "Neuer Sprint-Titel"}))
	text := doc(t, root, "sprints/geplant/X1-test/README.md")
	if !strings.HasPrefix(text, "# X1 · ") || !strings.Contains(strings.SplitN(text, "\n", 2)[0], "Neuer Sprint-Titel") {
		t.Errorf("Sprint-H1: %v\n%.80s", err, text)
	}
	road := doc(t, root, "sprints/README.md")
	if !strings.Contains(road, "Neuer Sprint-Titel") {
		t.Errorf("Fahrplan:\n%s", road)
	}
}
