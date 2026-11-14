package planning

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// refreshRepo: Fahrplan mit veralteten Zeilen (NG15 aktiv, LG1 geplant, beide nur in erledigt/), KB6 mit Domänen als
// Thema und Freitext unter „Offen am Gerät“; aktiver Sprint A1 mit Mensch-Sessions an PC und Xbox (B-842).
func refreshRepo(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	w := func(rel, text string) {
		p := filepath.Join(root, "docs", filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	w("backlog/README.md", "# Backlog\n")
	w("sprints/geplant/.keep", "")
	for _, s := range [][2]string{{"NG15-ton", "CLI"}, {"LG1-logs", "SRV"}, {"KB6-kampf", "SIM, REG"}} {
		id, _, _ := strings.Cut(s[0], "-")
		w("sprints/erledigt/"+s[0]+"/README.md", "# "+id+" · "+s[1]+" · Titel "+id+"\n\n- **Status:** erledigt\n")
	}
	w("sprints/aktiv/A1-a/README.md", "# A1 · DEV · Sprint A1\n\n- **Status:** aktiv\n\n## Sessions\n\n"+
		"| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n"+
		"| A1.1 | `A1.1-x.md` | Abnahme | Mensch | offen |\n| A1.2 | `A1.2-x.md` | Abnahme | Mensch | fertig |\n"+
		"| A1.3 | `A1.3-x.md` | Umsetzung | autonom | offen |\n| A1.4 | `A1.4-x.md` | Abnahme | Mensch | offen |\n"+
		"| A1.5 | `A1.5-x.md` | Abnahme | Mensch | offen |\n")
	for nr, ctx := range map[string]string{"A1.1": "Gerät: PC", "A1.2": "Gerät: PC", "A1.3": "", "A1.4": "Gerät: Xbox", "A1.5": ""} {
		w("sprints/aktiv/A1-a/"+nr+"-x.md", "# "+nr+" · Titel "+nr+"\n\n- **Status:** offen\n\n## Kontext\n\n"+ctx+"\n")
	}
	w("sprints/README.md", "# Fahrplan\n\n## Aktiv\n\n| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |\n"+
		"|---|---|---|---|---|---|\n| A1 | DEV | – | Sprint A1 | – | `aktiv/A1-a/` |\n"+
		"| NG15 | CLI | NAT | Ton | – | `aktiv/NG15-ton/` |\n\n"+
		"## Offen am Gerät\n\nAbnahmen sammelt das Projekt ABN im Sprint HW1.\n\n"+
		"## Geplant (nach Projekt-Rang)\n\n| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Reife | Ordner |\n"+
		"|---|---|---|---|---|---|---|\n| LG1 | SRV | LST | Logs | – | Entwurf | `geplant/LG1-logs/` |\n\n"+
		"## Erledigt\n\n| Sprint | Thema | Ordner |\n|---|---|---|\n| KB6 | SIM, REG | `erledigt/KB6-kampf/` |\n"+
		"| LG1 | Logs | `erledigt/LG1-logs/` |\n| NG15 | Ton | `erledigt/NG15-ton/` |\n")
	return root
}

// TestRefreshRoadmap: Nach einem schreibenden Aufruf (changeSet.apply) steht jede offene Mensch-Session genau einmal
// unter ihrem Gerät, veraltete Zeilen sind weg, KB6 trägt seinen Titel.
func TestRefreshRoadmap(t *testing.T) {
	t.Parallel()
	root := refreshRepo(t)
	if err := newChangeSet(root).apply(); err != nil {
		t.Fatal(err)
	}
	text, _, err := readText(root, roadmapRel)
	if err != nil {
		t.Fatal(err)
	}
	checkText(t, text, []string{
		"### PC\n\n- A1.1 · Titel A1.1 (Sprint A1, offen, Agent Mensch)\n\n### Xbox/TV\n\n- A1.4 · Titel A1.4",
		"### unbekannt\n\n- A1.5 · Titel A1.5",
		"| KB6 | Titel KB6 | `erledigt/KB6-kampf/` |",
		"| LG1 | Logs | `erledigt/LG1-logs/` |",
		"| A1 | DEV | – | Sprint A1 | – | `aktiv/A1-a/` |",
	}, []string{"A1.2 ·", "A1.3 ·", "HW1", "`aktiv/NG15-ton/`", "`geplant/LG1-logs/`", "SIM, REG"})
	for _, nr := range []string{"A1.1 ·", "A1.4 ·", "A1.5 ·"} {
		if n := strings.Count(text, nr); n != 1 {
			t.Errorf("%s steht %d-mal im Fahrplan", nr, n)
		}
	}
	// Ein zweiter Lauf ändert nichts mehr.
	if err := refreshRoadmap(root); err != nil {
		t.Fatal(err)
	}
	again, _, _ := readText(root, roadmapRel)
	if again != text {
		t.Errorf("zweiter Lauf ändert den Fahrplan:\n%s", again)
	}
}

// TestTakeRoadmapRowWrongColumns: Eine Zeile mit falscher Spaltenzahl liefert keine alten Werte (KB6: sonst wird
// die zweite Zelle zum Thema).
func TestTakeRoadmapRowWrongColumns(t *testing.T) {
	t.Parallel()
	lines := splitLines("## Erledigt\n\n| Sprint | Thema | Ordner |\n|---|---|---|\n" +
		"| KB6 | SIM, REG | KMP | x | – | – | `erledigt/KB6-kampf/` |\n")
	_, old, at := takeRoadmapRow(lines, "KB6", "sprints/erledigt/KB6-kampf")
	if at < 0 || len(old) != 0 {
		t.Fatalf("at=%d old=%v", at, old)
	}
}
