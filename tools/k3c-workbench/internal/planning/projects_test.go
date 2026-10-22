package planning

import (
	"slices"
	"strconv"
	"strings"
	"testing"
)

// projekteMD entspricht docs/projekte/README.md vor dem ersten Projekt (PJ1).
const projekteMD = "# Projekte\n\nRangliste aller Projekte.\n\n## Aktiv\n\n| Rang | Projekt | Ziel | Datei |\n|---|---|---|---|\n\n" +
	"Noch keine Projekte: Der Umzug der Planung folgt in PJ3 (B-359).\n\n## Ruht\n\n| Projekt | Grund | Datei |\n|---|---|---|\n\n" +
	"## Erledigt\n\n| Projekt | Datei |\n|---|---|\n"

func newProject(t *testing.T, root, id, slug string) {
	t.Helper()
	must(t)(Create(root, NewDoc{Kind: "projekt", ID: id, Slug: slug, Title: "Titel " + id}))
}

func newSprint(t *testing.T, root, id string, fields map[string]string) {
	t.Helper()
	f := map[string]string{"Domäne": "SRV"}
	for k, v := range fields {
		f[k] = v
	}
	must(t)(Create(root, NewDoc{Kind: "sprint", ID: id, Slug: "s", Title: "Sprint " + id, Fields: f}))
}

// ranks liefert Kürzel → Rang aus den Dateien.
func ranks(t *testing.T, root string) map[string]string {
	t.Helper()
	out := map[string]string{}
	for _, p := range loadProjects(root) {
		out[p.ID] = p.Rang
	}
	return out
}

func TestProjektAnlegenLesenListen(t *testing.T) {
	root := tempRepo(t)
	newProject(t, root, "GRA", "grafik")
	newProject(t, root, "SND", "sound")
	if got := ranks(t, root); got["GRA"] != "1" || got["SND"] != "2" {
		t.Fatalf("Ränge: %v", got)
	}
	overview := doc(t, root, "projekte/README.md")
	if !strings.Contains(overview, "| 1 | GRA · Titel GRA | – | [GRA-grafik.md](GRA-grafik.md) |\n| 2 | SND · Titel SND |") ||
		strings.Contains(overview, "Noch keine Projekte") || strings.Contains(overview, "\n\n\n") {
		t.Fatalf("Übersicht:\n%s", overview)
	}
	text := doc(t, root, "projekte/GRA-grafik.md")
	if strings.Contains(text, "SP00") || !strings.Contains(text, "- **Status:** aktiv\n- **Rang:** 1\n- **Ziel-Tickets:** –\n") {
		t.Fatalf("Projekt-Datei:\n%s", text)
	}
	if out := must(t)(Get(root, "GRA")); !strings.HasPrefix(out, "docs/projekte/GRA-grafik.md\n") {
		t.Fatalf("Get: %q", out)
	}
	must(t)(Section(root, "GRA", "Ziel", "Alles hübsch."))
	if !strings.Contains(doc(t, root, "projekte/GRA-grafik.md"), "## Ziel\n\nAlles hübsch.\n\n## Sprints") {
		t.Fatal("Abschnitt nicht ersetzt")
	}
	newSprint(t, root, "GR1", map[string]string{"Projekt": "GRA"})
	out := must(t)(List(root, Filter{Kind: "projekt"}))
	if !strings.HasPrefix(out, "1 GRA aktiv Sprints 0/1 · Titel GRA\n  GR1 geplant\n2 SND aktiv Sprints 0/0") {
		t.Fatalf("Liste:\n%s", out)
	}
	if out := must(t)(List(root, Filter{Projekt: "GRA"})); !strings.HasPrefix(out, "GR1 ") || strings.Contains(out, "B-001") {
		t.Fatalf("Filter projekt:\n%s", out)
	}
	checkProjects(t, root)
}

func TestProjektRangLueckenlos(t *testing.T) {
	root := tempRepo(t)
	for _, id := range []string{"GRA", "SND", "BAL"} {
		newProject(t, root, id, strings.ToLower(id))
	}
	steps := []struct {
		id     string
		values map[string]string
		want   string // Ränge GRA, SND, BAL
	}{
		{"BAL", map[string]string{"Rang": "1"}, "2 3 1"},
		{"GRA", map[string]string{"Rang": "3"}, "3 2 1"},
		{"SND", map[string]string{"Status": "ruht"}, "2 – 1"},
		{"BAL", map[string]string{"Status": "erledigt"}, "1 – –"},
		{"SND", map[string]string{"Status": "aktiv"}, "1 2 –"},
		{"SND", map[string]string{"Rang": "1"}, "2 1 –"},
	}
	for _, s := range steps {
		must(t)(Set(root, s.id, s.values))
		r := ranks(t, root)
		if got := r["GRA"] + " " + r["SND"] + " " + r["BAL"]; got != s.want {
			t.Fatalf("%s %v: Ränge %s, erwartet %s", s.id, s.values, got, s.want)
		}
		checkProjects(t, root)
	}
	overview := doc(t, root, "projekte/README.md")
	if !strings.Contains(overview, "## Erledigt\n\n| Projekt | Datei |\n|---|---|\n| BAL · Titel BAL | [BAL-bal.md](BAL-bal.md) |") {
		t.Fatalf("Übersicht Erledigt:\n%s", overview)
	}
}

func TestProjektOhneRangABN(t *testing.T) {
	root := tempRepo(t)
	newProject(t, root, "GRA", "grafik")
	newProject(t, root, "ABN", "abnahmen")
	newProject(t, root, "SND", "sound")
	if r := ranks(t, root); r["GRA"] != "1" || r["ABN"] != "–" || r["SND"] != "2" {
		t.Fatalf("Ränge: %v", r)
	}
	if _, err := Set(root, "ABN", map[string]string{"Rang": "1"}); err == nil {
		t.Fatal("ABN mit Rang")
	}
	if out := must(t)(List(root, Filter{Kind: "projekt"})); !strings.Contains(out, "2 SND aktiv Sprints 0/0 · Titel SND\n– ABN aktiv") {
		t.Fatalf("ABN nach den Rängen:\n%s", out)
	}
}

func TestProjektZuordnung(t *testing.T) {
	root := tempRepo(t)
	newProject(t, root, "GRA", "grafik")
	newProject(t, root, "SND", "sound")
	newSprint(t, root, "X1", nil)
	newSprint(t, root, "X2", map[string]string{"Projekt": "GRA"})
	must(t)(Set(root, "X1", map[string]string{"Projekt": "GRA"}))
	gra := func() []string { p, _ := findProject(loadProjects(root), "GRA"); return p.Sprints }
	if got := gra(); !slices.Equal(got, []string{"X2", "X1"}) {
		t.Fatalf("Tabelle GRA: %v", got)
	}
	must(t)(Set(root, "GRA", map[string]string{"Sprints": "X1, X2"}))
	if got := gra(); !slices.Equal(got, []string{"X1", "X2"}) {
		t.Fatalf("Reihenfolge: %v", got)
	}
	must(t)(Set(root, "X1", map[string]string{"Reife": "bereit", "Spec": "freigegeben", "Freigabe": "2026-10-08 Test", "Status": "aktiv"}))
	if !strings.Contains(doc(t, root, "projekte/GRA-grafik.md"), "| X1 | Sprint X1 | aktiv |\n| X2 | Sprint X2 | geplant |") {
		t.Fatal("Status des Sprints nicht nachgezogen")
	}
	must(t)(Set(root, "X2", map[string]string{"Projekt": "SND"}))
	if got := gra(); !slices.Equal(got, []string{"X1"}) {
		t.Fatalf("X2 bleibt bei GRA: %v", got)
	}
	must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "ton", Title: "Ton", Fields: map[string]string{"Domäne": "CLI", "Typ": "Idee",
		"Prio": "hoch", "Projekt": "SND"}}))
	if _, err := Delete(root, "SND"); err == nil {
		t.Fatal("Projekt mit Sprint gelöscht")
	}
	must(t)(Delete(root, "X2"))
	if _, err := Delete(root, "SND"); err == nil {
		t.Fatal("Projekt mit Ticket gelöscht")
	}
	must(t)(Set(root, "B-002", map[string]string{"Projekt": "–"}))
	must(t)(Delete(root, "SND"))
	if strings.Contains(doc(t, root, "projekte/README.md"), "SND") {
		t.Fatal("Übersicht nennt gelöschtes Projekt")
	}
	checkProjects(t, root)
	checkConsistent(t, root)
}

// TestProjektAblehnungAendertNichts: B-357 › Ausnahme- und Fehlerfälle.
func TestProjektAblehnungAendertNichts(t *testing.T) {
	root := tempRepo(t)
	newProject(t, root, "GRA", "grafik")
	newProject(t, root, "SND", "sound")
	newProject(t, root, "BAL", "bal")
	must(t)(Set(root, "BAL", map[string]string{"Status": "ruht"}))
	newSprint(t, root, "X1", map[string]string{"Projekt": "GRA"})
	newSprint(t, root, "X2", map[string]string{"Projekt": "SND"})
	must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "t", Title: "T", Fields: map[string]string{"Domäne": "SRV", "Typ": "Idee", "Prio": "hoch"}}))
	before := snapshot(t, root)
	set := func(id string, v map[string]string) func() error {
		return func() error { _, err := Set(root, id, v); return err }
	}
	bad := map[string]func() error{
		"unbekanntes Projekt am Sprint": set("X1", map[string]string{"Projekt": "XYZ"}),
		"unbekanntes Projekt am Ticket": set("B-002", map[string]string{"Projekt": "XYZ"}),
		"Rang 0":                        set("GRA", map[string]string{"Rang": "0"}),
		"Rang über n":                   set("GRA", map[string]string{"Rang": "3"}),
		"Rang keine Zahl":               set("GRA", map[string]string{"Rang": "eins"}),
		"Rang ruhend":                   set("BAL", map[string]string{"Rang": "1"}),
		"Sprints fremd":                 set("GRA", map[string]string{"Sprints": "X1, X2"}),
		"Sprints unvollständig":         set("GRA", map[string]string{"Sprints": "X2"}),
		"Status falsch":                 set("GRA", map[string]string{"Status": "fertig"}),
		"Löschen mit Sprint":            func() error { _, err := Delete(root, "GRA"); return err },
		"Kürzel falsch": func() error {
			_, err := Create(root, NewDoc{Kind: "projekt", ID: "GR", Slug: "x", Title: "x"})
			return err
		},
		"Kürzel doppelt": func() error {
			_, err := Create(root, NewDoc{Kind: "projekt", ID: "GRA", Slug: "x", Title: "x"})
			return err
		},
		"Rang beim Anlegen": func() error {
			_, err := Create(root, NewDoc{Kind: "projekt", ID: "NEU", Slug: "x", Title: "x", Fields: map[string]string{"Rang": "1"}})
			return err
		},
		"Sprint mit unbekanntem Projekt": func() error {
			_, err := Create(root, NewDoc{Kind: "sprint", ID: "X9", Slug: "x", Title: "x", Fields: map[string]string{"Domäne": "SRV", "Projekt": "XYZ"}})
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

// checkProjects bildet die Regeln aus tests/planningProjects.test.ts nach: Ränge der aktiven Projekte lückenlos ab 1
// (ABN ohne Rang), ruhende und erledigte mit `–`; Sprint ↔ Sprint-Tabelle in beide Richtungen; Übersicht nennt jede
// Datei; Tickets nennen nur bestehende Projekte.
func checkProjects(t *testing.T, root string) {
	t.Helper()
	d, err := Load(root)
	if err != nil {
		t.Fatal(err)
	}
	checkRanks(t, d.Projects)
	checkLinks(t, root, d)
}

func checkRanks(t *testing.T, ps []Project) {
	t.Helper()
	var got []int
	for _, p := range ps {
		if p.Status != "aktiv" || p.ID == unranked {
			if p.Rang != "–" {
				t.Errorf("%s: %s mit Rang %s", p.ID, p.Status, p.Rang)
			}
			continue
		}
		n, _ := strconv.Atoi(p.Rang)
		got = append(got, n)
	}
	for i, n := range got {
		if n != i+1 {
			t.Errorf("Ränge %v nicht lückenlos ab 1", got)
			break
		}
	}
}

func checkLinks(t *testing.T, root string, d Data) {
	t.Helper()
	overview := doc(t, root, "projekte/README.md")
	byID := map[string]Project{}
	for _, p := range d.Projects {
		byID[p.ID] = p
		if !strings.Contains(overview, p.File) {
			t.Errorf("Übersicht ohne %s", p.File)
		}
	}
	sprintProject := map[string]string{}
	for _, s := range d.Sprints {
		sprintProject[s.ID] = s.Project
		if p, ok := byID[s.Project]; s.Project != "–" && (!ok || !slices.Contains(p.Sprints, s.ID)) {
			t.Errorf("%s: fehlt in der Tabelle von %s", s.ID, s.Project)
		}
	}
	for _, p := range d.Projects {
		for _, id := range p.Sprints {
			if sprintProject[id] != p.ID {
				t.Errorf("%s: %s nennt Projekt %q", p.ID, id, sprintProject[id])
			}
		}
	}
	for _, tk := range d.Tickets {
		if _, ok := byID[tk.Project]; tk.Project != "–" && tk.Project != "" && !ok {
			t.Errorf("%s: Projekt %s fehlt", tk.Nr, tk.Project)
		}
	}
}
