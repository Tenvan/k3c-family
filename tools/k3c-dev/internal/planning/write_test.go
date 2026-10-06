package planning

import (
	"crypto/sha256"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

const indexMD = "# Backlog\n\n## Offen\n\n| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n|---|---|---|---|---|---|---|\n" +
	"| [B-001](B-001-alt.md) | SRV | Idee | mittel | offen | – | Altes Ticket |\n\n## Archiv\n\nText\n\n" +
	"| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n|---|---|---|---|---|---|---|\n"

const roadmapMD = "# Fahrplan\n\n## Aktiv\n\n| Sprint | Domäne | Thema | Am Ende sichtbar | Ordner |\n|---|---|---|---|---|\n\n" +
	"## Offen am Gerät\n\n| Session | Gerät | Kriterium | Ordner |\n|---|---|---|---|\n| X1.3 | Xbox | AC-04 | `erledigt/X1-test/` |\n\n" +
	"## Geplant (in dieser Reihenfolge)\n\n| Sprint | Domäne | Thema | Am Ende sichtbar | Reife | Ordner |\n|---|---|---|---|---|---|\n\n" +
	"**Einschiebbar** (…):\n\n| Sprint | Domäne | Thema | Reife | Ordner |\n|---|---|---|---|---|\n\n" +
	"## Erledigt\n\n| Sprint | Thema | Ordner |\n|---|---|---|\n"

// tempRepo legt docs/ mit den echten Vorlagen, einem Ticket, Index und Fahrplan an.
func tempRepo(t *testing.T) string {
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
	for _, k := range []string{"ticket", "sprint", "session"} {
		b, err := os.ReadFile(filepath.Join("..", "..", "..", "..", "docs", "vorlagen", k+".md"))
		if err != nil {
			t.Fatal(err)
		}
		w("vorlagen/"+k+".md", string(b))
	}
	w("backlog/README.md", indexMD)
	w("backlog/B-001-alt.md", "# B-001 · Altes Ticket\n\n- **Status:** offen\n")
	w("sprints/README.md", roadmapMD)
	for _, st := range States {
		if err := os.MkdirAll(filepath.Join(root, "docs", "sprints", st), 0o755); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

func must(t *testing.T) func(string, error) string {
	return func(out string, err error) string {
		t.Helper()
		if err != nil {
			t.Fatal(err)
		}
		return out
	}
}

func doc(t *testing.T, root, rel string) string {
	t.Helper()
	text, _, err := readText(root, rel)
	if err != nil {
		t.Fatal(err)
	}
	return text
}

func TestTicketAnlegenAendernArchivieren(t *testing.T) {
	root := tempRepo(t)
	out := must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "neu", Title: "Neues Ticket",
		Fields: map[string]string{"Domäne": "SRV", "Typ": "Idee", "Prio": "hoch"}}))
	if !strings.Contains(out, "B-002 angelegt: docs/backlog/B-002-neu.md") {
		t.Fatalf("Antwort: %q", out)
	}
	must(t)(Section(root, "B-002", "Ausgangslage", "Heute fehlt es.\n\n### Detail\n\nmehr"))
	if text := doc(t, root, "backlog/B-002-neu.md"); !strings.Contains(text, "## Ausgangslage\n\nHeute fehlt es.\n\n### Detail\n\nmehr\n\n## Ziel") {
		t.Fatalf("Abschnitt:\n%s", text)
	}
	must(t)(Set(root, "B-002", map[string]string{"Status": "erledigt"}))
	if _, err := os.Stat(docsPath(root, "backlog/archiv/B-002-neu.md")); err != nil {
		t.Fatal("nicht archiviert:", err)
	}
	index := doc(t, root, "backlog/README.md")
	if !strings.HasSuffix(strings.TrimSpace(index), "| [B-002](archiv/B-002-neu.md) | SRV | Idee | hoch | erledigt | – | Neues Ticket |") {
		t.Fatalf("Index:\n%s", index)
	}
	checkConsistent(t, root)
}

func TestSprintUndSessionLebenslauf(t *testing.T) {
	root := tempRepo(t)
	must(t)(Create(root, NewDoc{Kind: "sprint", ID: "X1", Slug: "test", Title: "Test-Sprint", Fields: map[string]string{"Domäne": "SRV"}}))
	must(t)(Create(root, NewDoc{Kind: "session", ID: "X1.1", Slug: "eins", Title: "Erste"}))
	must(t)(Set(root, "X1.1", map[string]string{"Status": "fertig"}))
	readme := doc(t, root, "sprints/geplant/X1-test/README.md")
	if !strings.Contains(readme, "| X1.1 | `X1.1-eins.md` | Umsetzung | autonom | fertig |") {
		t.Fatalf("Session-Tabelle:\n%s", readme)
	}
	if _, err := Set(root, "X1", map[string]string{"Status": "aktiv"}); err == nil {
		t.Fatal("aktiv ohne Reife bereit")
	}
	must(t)(Set(root, "X1", map[string]string{"Reife": "bereit", "Spec": "freigegeben", "Freigabe": "2026-10-04 Test", "Status": "aktiv"}))
	roadmap := doc(t, root, "sprints/README.md")
	if !strings.Contains(roadmap, "| X1 | SRV | Test-Sprint | – | `aktiv/X1-test/` |\n\n## Offen am Gerät") {
		t.Fatalf("Fahrplan:\n%s", roadmap)
	}
	if _, err := Delete(root, "X1.1"); err == nil {
		t.Fatal("Session eines aktiven Sprints gelöscht")
	}
	must(t)(Set(root, "X1", map[string]string{"Status": "erledigt"}))
	roadmap = doc(t, root, "sprints/README.md")
	if !strings.Contains(roadmap, "| X1 | Test-Sprint | `erledigt/X1-test/` |") || strings.Count(roadmap, "| X1 |") != 1 {
		t.Fatalf("Fahrplan Erledigt:\n%s", roadmap)
	}
	if !strings.Contains(roadmap, "| X1.3 | Xbox |") {
		t.Fatal("Zeile in „Offen am Gerät“ mit gleichem Ordner verschwunden")
	}
	checkConsistent(t, root)
}

func TestEntwurfLoeschen(t *testing.T) {
	root := tempRepo(t)
	must(t)(Create(root, NewDoc{Kind: "sprint", ID: "X2", Slug: "weg", Title: "Weg", Fields: map[string]string{"Domäne": "SIM", "Einschiebbar": "ja"}}))
	must(t)(Create(root, NewDoc{Kind: "session", ID: "X2.1", Slug: "a", Title: "A"}))
	if !strings.Contains(doc(t, root, "sprints/README.md"), "| X2 | SIM | Weg | Entwurf | `geplant/X2-weg/` |\n\n## Erledigt") {
		t.Fatal("Einschiebbar-Zeile fehlt")
	}
	must(t)(Delete(root, "X2.1"))
	if strings.Contains(doc(t, root, "sprints/geplant/X2-weg/README.md"), "X2.1") {
		t.Fatal("Tabellenzeile bleibt")
	}
	must(t)(Delete(root, "X2"))
	if strings.Contains(doc(t, root, "sprints/README.md"), "X2") {
		t.Fatal("Fahrplan-Zeile bleibt")
	}
	if _, err := Delete(root, "B-001"); err == nil {
		t.Fatal("Ticket gelöscht")
	}
	checkConsistent(t, root)
}

// TestUngueltigeEingabeAendertNichts: Pfade, IDs und Werte von außen (AC-03).
func TestUngueltigeEingabeAendertNichts(t *testing.T) {
	root := tempRepo(t)
	must(t)(Create(root, NewDoc{Kind: "sprint", ID: "X3", Slug: "s", Title: "S", Fields: map[string]string{"Domäne": "SRV"}}))
	before := snapshot(t, root)
	abs := filepath.Join(root, "docs", "backlog", "B-001-alt.md")
	bad := map[string]func() error{
		"Set ../":     func() error { _, err := Set(root, "../B-001", map[string]string{"Status": "offen"}); return err },
		"Get absolut": func() error { _, err := Get(root, abs); return err },
		"Section ../": func() error { _, err := Section(root, "../../x", "Ziel", "x"); return err },
		"Slug ../":    func() error { _, err := Create(root, NewDoc{Kind: "ticket", Slug: "../böse", Title: "x"}); return err },
		"Art ../":     func() error { _, err := Create(root, NewDoc{Kind: "../../ticket", Slug: "a", Title: "x"}); return err },
		"Sprint-ID ../": func() error {
			_, err := Create(root, NewDoc{Kind: "sprint", ID: "../X9", Slug: "a", Title: "x"})
			return err
		},
		"unbekannte ID":    func() error { _, err := Set(root, "B-999", map[string]string{"Status": "offen"}); return err },
		"unbekanntes Feld": func() error { _, err := Set(root, "X3", map[string]string{"Pfad": "x"}); return err },
		"falscher Wert":    func() error { _, err := Set(root, "X3", map[string]string{"Status": "fertig"}); return err },
		"Domäne XYZ": func() error {
			_, err := Create(root, NewDoc{Kind: "ticket", Slug: "a", Title: "x", Fields: map[string]string{"Domäne": "XYZ", "Typ": "Idee", "Prio": "hoch"}})
			return err
		},
		"Wert mit |":            func() error { _, err := Set(root, "X3", map[string]string{"Tickets": "B-001 | x"}); return err },
		"Überschrift im Text":   func() error { _, err := Section(root, "X3", "Ziel", "## Neu"); return err },
		"unbekannter Abschnitt": func() error { _, err := Section(root, "X3", "Pfad", "x"); return err },
		"freigegeben ohne Text": func() error { _, err := Set(root, "X3", map[string]string{"Spec": "freigegeben"}); return err },
		"Ticket ohne Pflicht":   func() error { _, err := Create(root, NewDoc{Kind: "ticket", Slug: "a", Title: "x"}); return err },
		"Sprint doppelt": func() error {
			_, err := Create(root, NewDoc{Kind: "sprint", ID: "X3", Slug: "b", Title: "x", Fields: map[string]string{"Domäne": "SRV"}})
			return err
		},
	}
	for name, call := range bad {
		if err := call(); err == nil {
			t.Errorf("%s: kein Fehler", name)
		}
		if after := snapshot(t, root); after != before {
			t.Errorf("%s: Dateien geändert", name)
		}
	}
}

// snapshot ist ein Hash über Pfade und Inhalte unter root.
func snapshot(t *testing.T, root string) string {
	t.Helper()
	h := sha256.New()
	err := filepath.WalkDir(root, func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		b, err := os.ReadFile(p)
		_, _ = fmt.Fprintf(h, "%s\n%s\n", p, b)
		return err
	})
	if err != nil {
		t.Fatal(err)
	}
	return fmt.Sprintf("%x", h.Sum(nil))
}

var indexRow = regexp.MustCompile(`(?m)^\| \[(B-\d{3})\]\((.+?)\) \|.*\| (\S+) \| \S+ \| .* \|$`)

// checkConsistent prüft die Regeln aus tests/planning.test.ts, die die Schreibwege berühren: Index ↔ Tickets mit
// Status und Ordner, Sprint-Status ↔ Ordner, Fahrplan nennt jeden Ordner, Session-Tabelle ↔ Dateien.
func checkConsistent(t *testing.T, root string) {
	t.Helper()
	index := doc(t, root, "backlog/README.md")
	for _, m := range indexRow.FindAllStringSubmatch(index, -1) {
		if st := ParseTicket(doc(t, root, "backlog/"+m[2])).Status; st != m[3] {
			t.Errorf("%s: Index %s, Datei %s", m[1], m[3], st)
		}
		if archivedStatus := m[3] == "erledigt" || m[3] == "verworfen"; archivedStatus != strings.HasPrefix(m[2], "archiv/") {
			t.Errorf("%s: Ordner passt nicht zu %s", m[1], m[3])
		}
	}
	roadmap := doc(t, root, "sprints/README.md")
	d, err := Load(root)
	if err != nil {
		t.Fatal(err)
	}
	for _, s := range d.Sprints {
		r, _ := findSprint(root, s.ID)
		if f := headFields(doc(t, root, r.rel)); f["Status"] != r.state {
			t.Errorf("%s: Status %s im Ordner %s", s.ID, f["Status"], r.state)
		}
		if !strings.Contains(roadmap, "`"+strings.TrimPrefix(r.dir, "sprints/")+"/`") {
			t.Errorf("%s fehlt im Fahrplan", s.ID)
		}
		for _, x := range s.Sessions {
			if x.Text == "" {
				t.Errorf("%s: Datei %s fehlt", x.Nr, x.File)
			}
		}
	}
}

func TestSprintPrioFolgtTickets(t *testing.T) {
	root := tempRepo(t)
	must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "neu", Title: "Neu", Fields: map[string]string{"Domäne": "SRV", "Typ": "Idee", "Prio": "hoch"}}))
	must(t)(Create(root, NewDoc{Kind: "sprint", ID: "X3", Slug: "p", Title: "P", Fields: map[string]string{"Domäne": "SRV", "Tickets": "B-001, B-002 teils"}}))
	if !strings.Contains(doc(t, root, "sprints/geplant/X3-p/README.md"), "- **Prio:** hoch\n") {
		t.Fatal("Prio nicht aus den Tickets abgeleitet")
	}
	must(t)(Set(root, "X3", map[string]string{"Tickets": "B-001"}))
	if !strings.Contains(doc(t, root, "sprints/geplant/X3-p/README.md"), "- **Prio:** ?\n") {
		t.Fatal("Prio folgt geänderten Tickets nicht")
	}
	must(t)(Create(root, NewDoc{Kind: "sprint", ID: "X4", Slug: "q", Title: "Q", Fields: map[string]string{"Domäne": "SIM", "Tickets": "B-002"}}))
	if out := must(t)(List(root, Filter{Kind: "sprint"})); strings.Index(out, "X4 ") > strings.Index(out, "X3 ") {
		t.Fatalf("hohe Prio nicht zuerst:\n%s", out)
	}
}
