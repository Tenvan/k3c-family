package planning

import (
	"os"
	"path/filepath"
	"reflect"
	"testing"
)

func TestDiffData(t *testing.T) {
	base := Data{
		Projects: []Project{{ID: "TST", Status: "aktiv", Rang: "8"}},
		Sprints:  []Sprint{{ID: "TR4", Status: "geplant", Sessions: []Session{{Nr: "TR4.1", Status: "offen"}}}},
		Tickets:  []Ticket{{Nr: "B-1", Status: "offen", Prio: "hoch"}},
	}
	wt := Data{
		Projects: []Project{{ID: "TST", Status: "ruht", Rang: "8"}},
		Sprints: []Sprint{
			{ID: "TR4", Status: "geplant", Sessions: []Session{{Nr: "TR4.1", Status: "fertig"}, {Nr: "TR4.2", Titel: "neu"}}},
			{ID: "TR5", Title: "Dummy"},
		},
		Tickets: []Ticket{{Nr: "B-1", Status: "offen", Prio: "hoch"}, {Nr: "B-2", Title: "x"}},
	}
	got := map[string]string{}
	for _, c := range diffData(base, wt) {
		got[c.Item+" "+c.ID] = c.Kind + ": " + c.Text
	}
	want := map[string]string{
		"Projekt TST":   "geändert: Status aktiv → ruht",
		"Session TR4.1": "geändert: Status offen → fertig",
		"Session TR4.2": "neu: neu",
		"Sprint TR5":    "neu: Dummy",
		"Ticket B-2":    "neu: x",
	}
	if len(got) != len(want) {
		t.Fatalf("got %v", got)
	}
	for k, v := range want {
		if got[k] != v {
			t.Errorf("%s = %q, want %q", k, got[k], v)
		}
	}
	if len(diffData(base, base)) != 0 {
		t.Error("gleiche Stände ergeben keine Unterschiede")
	}
}

func TestParseWorktreePaths(t *testing.T) {
	out := "worktree C:/k3c\nHEAD 1\nbranch refs/heads/develop\n\nworktree C:/k3c/wt\nbranch refs/heads/claude/x\n"
	got := parseWorktreePaths(out, "C:/k3c")
	if len(got) != 1 || got[0].branch != "claude/x" {
		t.Fatalf("got %v", got)
	}
}

// Gegen die Merge-Base: Der Vorsprung der Wurzel (B-006) erscheint nicht, nur was der Worktree selbst ändert (B-007).
func TestCheckWorktreesMergeBase(t *testing.T) {
	root := gitRepo(t)
	wt := filepath.Join(t.TempDir(), "wt")
	git(t, root, "worktree", "add", "-q", "-b", "claude/x", wt)
	if err := os.WriteFile(filepath.Join(root, "docs", "backlog", "B-006-wurzel.md"), []byte("# B-006 · In der Wurzel\n\n- **Status:** offen\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	git(t, root, "add", "-A")
	git(t, root, "commit", "-q", "-m", "wurzel weiter")
	for _, st := range States { // git legt leere Ordner nicht an
		if err := os.MkdirAll(filepath.Join(wt, "docs", "sprints", st), 0o755); err != nil {
			t.Fatal(err)
		}
	}
	if err := os.WriteFile(filepath.Join(wt, "docs", "backlog", "B-007-worktree.md"), []byte("# B-007 · Im Worktree\n\n- **Status:** offen\n"), 0o644); err != nil {
		t.Fatal(err)
	}

	diffs, err := CheckWorktrees(root)
	if err != nil {
		t.Fatal(err)
	}
	if len(diffs) != 1 || diffs[0].Error != "" {
		t.Fatalf("got %+v", diffs)
	}
	cs := diffs[0].Changes
	if len(cs) != 1 || cs[0].ID != "B-007" || cs[0].Kind != "neu" {
		t.Fatalf("nur B-007 erwartet, got %+v", cs)
	}
}

// M1: Ein im Worktree archiviertes Ticket (erledigt) erscheint als Statuswechsel, nicht als Nichts.
func TestCheckWorktreesArchiviertesTicket(t *testing.T) {
	root := gitRepo(t)
	wt := filepath.Join(t.TempDir(), "wt")
	git(t, root, "worktree", "add", "-q", "-b", "claude/y", wt)
	for _, st := range States {
		if err := os.MkdirAll(filepath.Join(wt, "docs", "sprints", st), 0o755); err != nil {
			t.Fatal(err)
		}
	}
	arch := filepath.Join(wt, "docs", "backlog", "archiv")
	if err := os.MkdirAll(arch, 0o755); err != nil {
		t.Fatal(err)
	}
	old := filepath.Join(wt, "docs", "backlog", "B-001-alt.md")
	if err := os.Remove(old); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(arch, "B-001-alt.md"), []byte("# B-001 · Altes Ticket\n\n- **Status:** erledigt\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	diffs, err := CheckWorktrees(root)
	if err != nil || len(diffs) != 1 || diffs[0].Error != "" {
		t.Fatalf("got %+v, %v", diffs, err)
	}
	cs := diffs[0].Changes
	if len(cs) != 1 || cs[0].ID != "B-001" || cs[0].Text != "Status offen → erledigt" {
		t.Fatalf("Statuswechsel erwartet, got %+v", cs)
	}
}

// M1: Was im Worktree fehlt, wird als „entfernt“ gemeldet.
func TestDiffDataEntfernt(t *testing.T) {
	base := Data{
		Projects: []Project{{ID: "TST"}},
		Sprints:  []Sprint{{ID: "TR4", Sessions: []Session{{Nr: "TR4.1"}}}, {ID: "TR9"}},
		Tickets:  []Ticket{{Nr: "B-1"}},
	}
	wt := Data{Sprints: []Sprint{{ID: "TR4"}}}
	got := map[string]string{}
	for _, c := range diffData(base, wt) {
		got[c.Item+" "+c.ID] = c.Kind
	}
	for _, k := range []string{"Projekt TST", "Session TR4.1", "Sprint TR9", "Ticket B-1"} {
		if got[k] != "entfernt" {
			t.Errorf("%s: %q, want entfernt", k, got[k])
		}
	}
}

// M3: Scheitert git, kommt ein Fehler zurück statt einer leeren Liste.
func TestCheckWorktreesGitFehler(t *testing.T) {
	if _, err := CheckWorktrees(filepath.Join(t.TempDir(), "gibt-es-nicht")); err == nil {
		t.Fatal("Fehler erwartet")
	}
}

// Reihenfolge und alle verglichenen Felder: je Art erst der Worktree (neu oder Feldänderungen in fester Reihenfolge,
// beim Sprint danach die Sessions), dann das Entfernte; Arten in der Folge Projekt, Sprint, Ticket (B-465).
func TestDiffDataReihenfolge(t *testing.T) {
	base := Data{
		Projects: []Project{{ID: "WZG", Title: "Werkzeuge", Status: "ruht", Rang: "2"}, {ID: "ALT", Title: "Alt"}},
		Sprints: []Sprint{{ID: "WZ6", Title: "Studio", Status: "geplant", Project: "–",
			Sessions: []Session{{Nr: "WZ6.1", Status: "offen", Titel: "Glossar"}}}},
		Tickets: []Ticket{{Nr: "B-1", Title: "Eins", Status: "offen", Prio: "mittel", Sprint: "–"}, {Nr: "B-2", Title: "Zwei"}},
	}
	wt := Data{
		Projects: []Project{{ID: "NEU", Title: "Neu"}, {ID: "WZG", Title: "Werkzeuge", Status: "aktiv", Rang: "1"}},
		Sprints: []Sprint{{ID: "WZ6", Title: "Studio", Status: "aktiv", Project: "WZG",
			Sessions: []Session{{Nr: "WZ6.1", Status: "fertig", Titel: "Glossar"}}}},
		Tickets: []Ticket{{Nr: "B-1", Title: "Eins", Status: "eingeplant", Prio: "hoch", Sprint: "WZ6"}},
	}
	want := []Change{
		{"neu", "Projekt", "NEU", "Neu"},
		{"geändert", "Projekt", "WZG", "Status ruht → aktiv"},
		{"geändert", "Projekt", "WZG", "Rang 2 → 1"},
		{"entfernt", "Projekt", "ALT", "Alt"},
		{"geändert", "Sprint", "WZ6", "Status geplant → aktiv"},
		{"geändert", "Sprint", "WZ6", "Projekt – → WZG"},
		{"geändert", "Session", "WZ6.1", "Status offen → fertig"},
		{"geändert", "Ticket", "B-1", "Status offen → eingeplant"},
		{"geändert", "Ticket", "B-1", "Prio mittel → hoch"},
		{"geändert", "Ticket", "B-1", "Sprint – → WZ6"},
		{"entfernt", "Ticket", "B-2", "Zwei"},
	}
	if got := diffData(base, wt); !reflect.DeepEqual(got, want) {
		t.Errorf("diffData:\n got %v\nwant %v", got, want)
	}
}
