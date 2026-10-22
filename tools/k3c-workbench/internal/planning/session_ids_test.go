package planning

import (
	"strings"
	"testing"
)

// B-372/AC-04: Session-IDs mit Buchstaben wie K3.2a lassen sich anlegen, lesen und ändern.
func TestSessionIDMitBuchstabe(t *testing.T) {
	root := tempRepo(t)
	newSprint(t, root, "K3", nil)
	out := must(t)(Create(root, NewDoc{Kind: "session", ID: "K3.2a", Slug: "teil", Title: "Teil a", Fields: map[string]string{"Domäne": "SRV"}}))
	if !strings.Contains(out, "K3.2a angelegt") {
		t.Fatalf("Antwort: %q", out)
	}
	if readme := doc(t, root, "sprints/geplant/K3-s/README.md"); !strings.Contains(readme, "| K3.2a |") {
		t.Fatalf("Session-Tabelle:\n%s", readme)
	}
	if got := must(t)(Get(root, "K3.2a")); !strings.Contains(got, "K3.2a · Teil a") {
		t.Fatalf("Get: %q", got)
	}
	must(t)(Set(root, "K3.2a", map[string]string{"Agent": "Mensch"}))
	if sp := ParseSprint(doc(t, root, "sprints/geplant/K3-s/README.md"), "K3-s", "geplant"); len(sp.Sessions) != 1 || sp.Sessions[0].Nr != "K3.2a" {
		t.Fatalf("Sessions: %+v", sp.Sessions)
	}
}
