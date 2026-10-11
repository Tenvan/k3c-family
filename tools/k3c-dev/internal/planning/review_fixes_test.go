package planning

import (
	"strings"
	"testing"
)

// Review WZ2.8 (B-360/AC-01): Die Ablehnung nennt Feldname und Datei.
func TestFehlendesFeldNenntFeldUndDatei(t *testing.T) {
	root := tempRepo(t)
	_, err := Set(root, "B-001", map[string]string{"Prio": "hoch"})
	if err == nil || !strings.Contains(err.Error(), "Prio") || !strings.Contains(err.Error(), "B-001-alt.md") {
		t.Fatalf("erwartet Feldname und Datei, bekam %v", err)
	}
}

// Review WZ2.8 (B-363/AC-01): Prio, Umgebung und Projekt eines Tickets über Set; danach stimmen Datei und Index.
func TestTicketFlagsAendernZiehtIndexNach(t *testing.T) {
	root := tempRepo(t)
	newProject(t, root, "GRA", "grafik")
	must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "neu", Title: "Neues Ticket",
		Fields: map[string]string{"Domäne": "SRV", "Typ": "Idee", "Prio": "niedrig"}}))
	must(t)(Set(root, "B-002", map[string]string{"Prio": "hoch", "Umgebung": "live", "Projekt": "GRA"}))
	text := doc(t, root, "backlog/B-002-neu.md")
	for _, want := range []string{"- **Prio:** hoch", "- **Umgebung:** live", "- **Projekt:** GRA"} {
		if !strings.Contains(text, want) {
			t.Errorf("Datei ohne %q:\n%s", want, text)
		}
	}
	if index := doc(t, root, "backlog/README.md"); !strings.Contains(index, "| SRV | Idee | hoch | offen |") {
		t.Errorf("Index ohne Prio hoch:\n%s", index)
	}
	checkConsistent(t, root)
}
