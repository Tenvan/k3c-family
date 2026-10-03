package planning

import (
	"os"
	"path/filepath"
	"testing"
)

const sprint = "# SP11 · SRV · Raspberry Pi\n\n- **Status:** aktiv\n- **Domäne:** SRV\n- **Tickets:** B-028, B-035\n- **Spec:** freigegeben\n\n" +
	"## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n" +
	"| SP11.1 | `a.md` | Umsetzung | autonom | fertig |\n| SP11.2 | `b.md` | Workshop | Mensch | offen |\n\n" +
	"## Abnahme\n\n| X.1 | `a` | b | c | d |\n"

const draft = "# F1 · REG · Zielkorridore\n\n- **Reife:** Entwurf\n\n## Sessions\n\n- F1.1 🧑 Workshop Zielkorridore (AC-01)\n"

func TestParseSprint(t *testing.T) {
	s := ParseSprint(sprint, "SP11-raspberry-pi", "aktiv")
	if s.ID != "SP11" || s.Domain != "SRV" || s.Title != "Raspberry Pi" || len(s.Tickets) != 2 {
		t.Fatalf("Kopf: %+v", s)
	}
	if len(s.Sessions) != 2 || s.Sessions[0].Status != "fertig" || s.Sessions[1].Agent != "Mensch" {
		t.Fatalf("Sessions (Tabelle unter Abnahme zählt nicht): %+v", s.Sessions)
	}
	d := ParseSprint(draft, "F1-x", "geplant")
	if len(d.Sessions) != 1 || d.Sessions[0].Status != "entwurf" || d.Reife != "Entwurf" {
		t.Fatalf("Entwurf: %+v", d)
	}
}

func TestLoad(t *testing.T) {
	root := t.TempDir()
	w := func(rel, text string) {
		p := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	w("docs/sprints/aktiv/SP11-pi/README.md", sprint)
	w("docs/sprints/geplant/F1-x/README.md", draft)
	w("docs/sprints/erledigt/M1-a/README.md", "# M1")
	w("docs/backlog/B-090-radar.md", "# B-090 · Radar\n\n- **Domäne:** CLI\n- **Status:** eingeplant\n- **Sprint:** U1\n")
	w("docs/backlog/README.md", "kein Ticket")
	d, err := Load(root)
	if err != nil || len(d.Sprints) != 2 || len(d.Tickets) != 1 || d.Done != 1 || d.Tickets[0].Title != "Radar" {
		t.Fatalf("%+v %v", d, err)
	}
	if _, err := Load(t.TempDir()); err == nil {
		t.Fatal("falsche Wurzel muss scheitern")
	}
	if _, err := Doc(root, "../x"); err == nil {
		t.Fatal("Dokumentname nur aus der Liste")
	}
}

// TestRepo liest die echten Docs des Repos: das Format der Vorlagen darf nicht still aus dem Parser fallen.
func TestRepo(t *testing.T) {
	root := filepath.Join("..", "..", "..", "..")
	if _, err := os.Stat(filepath.Join(root, "docs", "sprints")); err != nil {
		t.Skip("kein Repo über dem Modul")
	}
	d, err := Load(root)
	if err != nil || len(d.Sprints) == 0 || len(d.Tickets) == 0 {
		t.Fatalf("%v %d %d", err, len(d.Sprints), len(d.Tickets))
	}
	for _, s := range d.Sprints {
		if s.Title == "" || len(s.Sessions) == 0 {
			t.Errorf("Sprint %s ohne Titel oder Sessions: %+v", s.ID, s)
		}
	}
	for _, tk := range d.Tickets {
		if tk.Nr == "" || tk.Title == "" || tk.Status == "" {
			t.Errorf("Ticket unvollständig: %+v", tk)
		}
	}
	if text, err := Doc(root, "plan"); err != nil || text == "" {
		t.Errorf("Plan: %v", err)
	}
}
