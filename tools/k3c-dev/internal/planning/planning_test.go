package planning

import (
	"os"
	"path/filepath"
	"strings"
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
	w("docs/sprints/aktiv/SP11-pi/a.md", "# SP11.1 · Image")
	w("docs/sprints/geplant/F1-x/README.md", draft)
	w("docs/sprints/erledigt/M1-a/README.md", "# M1 · DEV · Alt")
	w("docs/backlog/B-090-radar.md", "# B-090 · Radar\n\n- **Domäne:** CLI\n- **Status:** eingeplant\n- **Sprint:** U1\n")
	w("docs/backlog/README.md", "kein Ticket")
	d, err := Load(root)
	if err != nil || len(d.Sprints) != 3 || len(d.Tickets) != 1 || d.Done != 1 || d.Tickets[0].Title != "Radar" {
		t.Fatalf("%+v %v", d, err)
	}
	if d.Sprints[0].Sessions[0].Text != "# SP11.1 · Image" || d.Sprints[0].Sessions[1].Text != "" {
		t.Fatalf("Session-Text aus der Spalte Datei: %+v", d.Sprints[0].Sessions)
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
		if s.Title == "" || len(s.Sessions) == 0 && s.Status != "erledigt" { // Altlasten ohne Session-Tabelle
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

func TestWorktrees(t *testing.T) {
	b := parseWorktrees("worktree C:/k3c\nHEAD 1\nbranch refs/heads/develop\n\nworktree C:/w1\nbranch refs/heads/sprint/s4\n\n" +
		"worktree C:/w2\nbranch refs/heads/gr4-4-work\n\nworktree C:/w3\ndetached\n")
	if len(b) != 3 {
		t.Fatalf("%v", b)
	}
	sp := []Sprint{{ID: "S4"}, {ID: "GR4"}, {ID: "S40"}, {ID: "F1"}}
	markWorktrees(sp, b)
	if sp[0].Worktree != "sprint/s4" || sp[1].Worktree != "gr4-4-work" || sp[2].Worktree != "" || sp[3].Worktree != "" {
		t.Fatalf("%+v", sp)
	}
}

// TestDocs: Das Glossar erscheint nur mit Datei, die Reihenfolge folgt DocOrder (B-211/AC-04).
func TestDocs(t *testing.T) {
	root := t.TempDir()
	if got := strings.Join(Available(root), ","); got != "" {
		t.Fatalf("ohne Dateien keine Dokumente: %q", got)
	}
	for _, f := range []string{"glossar.md", "plan-weiterentwicklung.md"} {
		if err := os.MkdirAll(filepath.Join(root, "docs"), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(root, "docs", f), []byte("# "+f), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	if got := strings.Join(Available(root), ","); got != "plan,glossar" {
		t.Fatalf("Dokumente: %q", got)
	}
	if text, err := Doc(root, "glossar"); err != nil || text != "# glossar.md" {
		t.Fatalf("Glossar: %q %v", text, err)
	}
}
