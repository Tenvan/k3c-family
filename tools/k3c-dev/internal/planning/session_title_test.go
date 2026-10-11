package planning

import "testing"

// B-402/AC-01: Sessions mit Datei tragen den Titel aus der Überschrift der Datei; Stichpunkt-Entwürfe behalten ihren.
func TestSessionTitelAusDatei(t *testing.T) {
	root := tempRepo(t)
	newSprint(t, root, "K4", nil)
	must(t)(Create(root, NewDoc{Kind: "session", ID: "K4.1", Slug: "eins", Title: "Erste Session · mit Mittelpunkt", Fields: map[string]string{"Domäne": "SRV"}}))
	sp, ok := loadSprint(docsPath(root, "sprints/geplant/K4-s"), "geplant")
	if !ok || len(sp.Sessions) != 1 {
		t.Fatalf("Sprint: %v, %+v", ok, sp.Sessions)
	}
	if got := sp.Sessions[0].Titel; got != "Erste Session · mit Mittelpunkt" {
		t.Fatalf("Titel: %q", got)
	}
}
