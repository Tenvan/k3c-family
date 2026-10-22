package planning

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

// B-436 (WZ12.1): Platzhalter-Zeilen ohne Datei in der Session-Tabelle sind keine bestehenden Sessions.

var reSessionsSection = regexp.MustCompile(`(?s)## Sessions\r?\n.*?(\r?\n## |\z)`)

// withSessionRows ersetzt die Session-Tabelle der README von sprints/geplant/<dir> durch die Zeilen.
func withSessionRows(t *testing.T, root, dir string, rows ...string) {
	t.Helper()
	p := filepath.Join(root, "docs", "sprints", "geplant", dir, "README.md")
	b, err := os.ReadFile(p)
	if err != nil {
		t.Fatal(err)
	}
	table := "## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n" + strings.Join(rows, "\n") + "\n\n"
	text := reSessionsSection.ReplaceAllStringFunc(string(b), func(m string) string {
		if strings.HasSuffix(m, "## ") {
			return table + "## "
		}
		return table
	})
	if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

func TestPlatzhalterZeileWirdErsetzt(t *testing.T) {
	root := tempRepo(t)
	newSprint(t, root, "GEB1", nil)
	withSessionRows(t, root, "GEB1-s",
		"| GEB1.1 | – (Entwurf) | Umsetzung | autonom | offen |",
		"| GEB1.2 | – | Umsetzung | autonom | offen |")
	must(t)(Create(root, NewDoc{Kind: "session", ID: "GEB1.1", Slug: "erste", Title: "Erste", Fields: map[string]string{"Domäne": "SRV"}}))
	readme := doc(t, root, "sprints/geplant/GEB1-s/README.md")
	if !strings.Contains(readme, "| GEB1.1 | `GEB1.1-erste.md` |") {
		t.Fatalf("Zeile GEB1.1 nicht ersetzt:\n%s", readme)
	}
	if strings.Contains(readme, "– (Entwurf)") || strings.Count(readme, "| GEB1.1 |") != 1 {
		t.Fatalf("Platzhalter GEB1.1 blieb stehen:\n%s", readme)
	}
	if !strings.Contains(readme, "| GEB1.2 | – |") {
		t.Fatalf("Platzhalter GEB1.2 muss stehen bleiben:\n%s", readme)
	}
	// Auch eine Änderung danach (plan_set) scheitert nicht an der verbleibenden Platzhalter-Zeile.
	must(t)(Set(root, "GEB1.1", map[string]string{"Agent": "Mensch"}))
}

func TestPlatzhalterAndereNummerBleibtStehen(t *testing.T) {
	root := tempRepo(t)
	newSprint(t, root, "GEB1", nil)
	withSessionRows(t, root, "GEB1-s", "| GEB1.2 | – (Entwurf) | Umsetzung | autonom | offen |")
	must(t)(Create(root, NewDoc{Kind: "session", ID: "GEB1.1", Slug: "erste", Title: "Erste", Fields: map[string]string{"Domäne": "SRV"}}))
	readme := doc(t, root, "sprints/geplant/GEB1-s/README.md")
	if !strings.Contains(readme, "| GEB1.2 | – (Entwurf) |") || !strings.Contains(readme, "| GEB1.1 | `GEB1.1-erste.md` |") {
		t.Fatalf("beide Zeilen erwartet:\n%s", readme)
	}
}

func TestZeileMitFehlenderDateiBleibtFehler(t *testing.T) {
	root := tempRepo(t)
	newSprint(t, root, "GEB1", nil)
	withSessionRows(t, root, "GEB1-s", "| GEB1.2 | `GEB1.2-weg.md` | Umsetzung | autonom | offen |")
	_, err := Create(root, NewDoc{Kind: "session", ID: "GEB1.1", Slug: "erste", Title: "Erste", Fields: map[string]string{"Domäne": "SRV"}})
	if err == nil {
		t.Fatal("fehlende Datei einer echten Zeile muss ein Fehler bleiben")
	}
}
