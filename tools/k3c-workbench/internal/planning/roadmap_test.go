package planning

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

const roadmapProjektMD = "# Fahrplan\n\n## Aktiv\n\n| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |\n|---|---|---|---|---|---|\n\n" +
	"## Geplant (nach Projekt-Rang)\n\n| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Reife | Ordner |\n|---|---|---|---|---|---|---|\n\n" +
	"## Erledigt\n\n| Sprint | Thema | Ordner |\n|---|---|---|\n"

// roadmapOrder liefert die Sprint-IDs in der Reihenfolge der Tabelle unter heading.
func roadmapOrder(t *testing.T, root, heading string) []string {
	t.Helper()
	lines := splitLines(doc(t, root, "sprints/README.md"))
	tbl, ok := findTable(lines, func(l string) bool { return strings.HasPrefix(l, heading) })
	if !ok {
		t.Fatalf("Tabelle %q fehlt", heading)
	}
	var ids []string
	for _, l := range lines[tbl.first:tbl.end] {
		ids = append(ids, cells(l)[0])
	}
	return ids
}

func roadmapRepo(t *testing.T) string {
	t.Helper()
	root := tempRepo(t)
	if err := os.WriteFile(filepath.Join(root, "docs", "sprints", "README.md"), []byte(roadmapProjektMD), 0o644); err != nil {
		t.Fatal(err)
	}
	newProject(t, root, "GRA", "grafik")
	newProject(t, root, "SND", "sound")
	return root
}

// B-366/AC-01: neue Zeilen tragen das Projekt und stehen nach Projekt-Rang und Sprint-Reihenfolge des Projekts.
func TestFahrplanZeileNachProjektRang(t *testing.T) {
	root := roadmapRepo(t)
	newSprint(t, root, "SND1", map[string]string{"Projekt": "SND"})
	newSprint(t, root, "GRA1", map[string]string{"Projekt": "GRA"})
	newSprint(t, root, "GRA2", map[string]string{"Projekt": "GRA"})
	if got := strings.Join(roadmapOrder(t, root, "## Geplant"), " "); got != "GRA1 GRA2 SND1" {
		t.Fatalf("Reihenfolge: %s\n%s", got, doc(t, root, "sprints/README.md"))
	}
	if !strings.Contains(doc(t, root, "sprints/README.md"), "| GRA2 | SRV | GRA | Sprint GRA2 |") {
		t.Fatalf("Projekt-Spalte fehlt:\n%s", doc(t, root, "sprints/README.md"))
	}
}

// B-366/AC-02: Aktivieren verschiebt die Zeile nach „Aktiv“ an die Rang-Stelle.
func TestFahrplanAktivNachRang(t *testing.T) {
	root := roadmapRepo(t)
	newSprint(t, root, "SND1", map[string]string{"Projekt": "SND"})
	newSprint(t, root, "GRA1", map[string]string{"Projekt": "GRA"})
	activate := map[string]string{"Reife": "bereit", "Spec": "freigegeben", "Freigabe": "2026-10-11, Test"}
	must(t)(Set(root, "SND1", activate))
	must(t)(Set(root, "SND1", map[string]string{"Status": "aktiv"}))
	must(t)(Set(root, "GRA1", activate))
	must(t)(Set(root, "GRA1", map[string]string{"Status": "aktiv"}))
	if got := strings.Join(roadmapOrder(t, root, "## Aktiv"), " "); got != "GRA1 SND1" {
		t.Fatalf("Aktiv: %s\n%s", got, doc(t, root, "sprints/README.md"))
	}
}

// B-366: Sprint ohne Projekt landet am Tabellenende, die Antwort nennt den Grund.
func TestFahrplanOhneProjektAmEnde(t *testing.T) {
	root := roadmapRepo(t)
	newSprint(t, root, "GRA1", map[string]string{"Projekt": "GRA"})
	out := must(t)(Create(root, NewDoc{Kind: "sprint", ID: "X1", Slug: "x", Title: "X", Fields: map[string]string{"Domäne": "SRV"}}))
	if got := strings.Join(roadmapOrder(t, root, "## Geplant"), " "); got != "GRA1 X1" || !strings.Contains(out, "kein bekanntes Projekt") {
		t.Fatalf("Reihenfolge %q, Antwort %q", got, out)
	}
}

// B-403/N4: ein Sprint ohne Projekt behält seinen Platz, wenn sich nur der Status ändert (Projekt bleibt „–“).
func TestFahrplanOhneProjektBehaeltPlatz(t *testing.T) {
	root := roadmapRepo(t)
	newSprint(t, root, "X1", nil)
	newSprint(t, root, "X2", nil)
	// Kopffeld von Hand leer machen (Create lehnt leere Werte ab), die Zeile bleibt bei „–“.
	readme := filepath.Join(root, "docs", "sprints", "geplant", "X1-s", "README.md")
	text, err := os.ReadFile(readme)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(readme, []byte(strings.Replace(string(text), "- **Projekt:** –", "- **Projekt:**", 1)), 0o644); err != nil {
		t.Fatal(err)
	}
	must(t)(Set(root, "X1", map[string]string{"Reife": "bereit"}))
	if got := strings.Join(roadmapOrder(t, root, "## Geplant"), " "); got != "X1 X2" {
		t.Fatalf("Reihenfolge nach plan_set: %s\n%s", got, doc(t, root, "sprints/README.md"))
	}
}

// roadmapProject liefert die Projekt-Zelle der Fahrplan-Zeile des Sprints (Spalte 3 in Aktiv und Geplant), "" ohne Zeile.
func roadmapProject(t *testing.T, root, id string) string {
	t.Helper()
	for _, l := range splitLines(doc(t, root, "sprints/README.md")) {
		if c := cells(l); strings.HasPrefix(l, "| "+id+" |") && len(c) > 2 {
			return c[2]
		}
	}
	return ""
}

// B-404: die Fahrplan-Zeile trägt das Projekt aus dem Kopffeld, auch nach Reife, Sessions, Freigabe und Aktivieren.
func TestFahrplanAktivierterSprintTraegtProjekt(t *testing.T) {
	root := roadmapRepo(t)
	must(t)(Create(root, NewDoc{Kind: "sprint", ID: "GRA1", Slug: "s", Title: "Sprint GRA1",
		Fields: map[string]string{"Domäne": "DEV", "Projekt": "GRA", "Reife": "bereit", "Tickets": "B-001"}}))
	check := func(step string) {
		t.Helper()
		if got := roadmapProject(t, root, "GRA1"); got != "GRA" {
			t.Errorf("nach %s: Projekt-Zelle %q statt GRA\n%s", step, got, doc(t, root, "sprints/README.md"))
		}
	}
	check("Anlegen")
	must(t)(Set(root, "GRA1", map[string]string{"Reife": "Entwurf"}))
	check("Reife Entwurf")
	must(t)(Create(root, NewDoc{Kind: "session", ID: "GRA1.1", Slug: "u", Title: "Umsetzung",
		Fields: map[string]string{"Domäne": "DEV", "Typ": "Umsetzung", "Agent": "autonom", "Umgebung": "offline"}}))
	check("Session anlegen")
	must(t)(Set(root, "GRA1", map[string]string{"Spec": "freigegeben", "Freigabe": "2026-10-11, Test"}))
	check("Spec freigegeben")
	must(t)(Set(root, "GRA1", map[string]string{"Reife": "bereit"}))
	check("Reife bereit")
	must(t)(Set(root, "GRA1", map[string]string{"Status": "aktiv", "Start-Commit": "abc1234"}))
	check("Aktivieren")
}
