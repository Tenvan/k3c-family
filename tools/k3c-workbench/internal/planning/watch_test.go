package planning

import (
	"context"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"testing"
	"time"
)

func write(t *testing.T, root, rel, text string) {
	t.Helper()
	p := filepath.Join(root, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

func TestStamp(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	write(t, root, "docs/sprints/aktiv/X1-a/README.md", "# X1")
	write(t, root, "docs/backlog/B-001-a.md", "# B-001")
	base := Stamp(root)
	if Stamp(root) != base {
		t.Fatal("unverändert muss gleich bleiben")
	}
	write(t, root, "docs/sprints/aktiv/X1-a/X1.1-s.md", "neu") // neue Session-Datei
	changed := Stamp(root)
	if changed == base {
		t.Fatal("neue Datei im aktiven Sprint")
	}
	write(t, root, "docs/backlog/archiv/B-000-x.md", "archiviert") // Archiv zählt nicht
	write(t, root, "src/main.ts", "egal")                          // außerhalb der Planung
	if Stamp(root) != changed {
		t.Fatal("Archiv und fremde Dateien dürfen den Stamp nicht ändern")
	}
	write(t, root, "docs/plan-weiterentwicklung.md", "Plan")
	if Stamp(root) == changed {
		t.Fatal("Plan-Dokument zählt")
	}
}

func TestWatch(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	write(t, root, "docs/backlog/B-001-a.md", "# B-001")
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	hit := make(chan struct{}, 4)
	Watch(ctx, root, 10*time.Millisecond, func() { hit <- struct{}{} })
	select {
	case <-hit:
		t.Fatal("der Ausgangsstand löst nichts aus")
	case <-time.After(60 * time.Millisecond):
	}
	write(t, root, "docs/backlog/B-002-b.md", "# B-002")
	select {
	case <-hit:
	case <-time.After(2 * time.Second):
		t.Fatal("Änderung nicht erkannt")
	}
}

func TestChangedIDs(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	write(t, root, "docs/sprints/aktiv/X1-a/README.md", "# X1")
	write(t, root, "docs/sprints/aktiv/X1-a/X1.1-s.md", "a")
	write(t, root, "docs/sprints/geplant/Y1-b/README.md", "# Y1")
	write(t, root, "docs/backlog/B-001-a.md", "# B-001")
	before := Snapshot(root)
	if ids := ChangedIDs(before, Snapshot(root)); ids != nil {
		t.Fatalf("keine Änderung: %v", ids)
	}
	write(t, root, "docs/sprints/aktiv/X1-a/X1.1-s.md", "ab") // Session, Größe ändert sich
	write(t, root, "docs/backlog/B-002-b.md", "# B-002")      // neues Ticket
	write(t, root, "docs/plan-weiterentwicklung.md", "Plan")  // Dokument
	_ = os.Remove(filepath.Join(root, "docs/backlog/B-001-a.md"))
	// Sprint geplant nach aktiv: beide Pfade nennen dieselbe ID
	if err := os.Rename(filepath.Join(root, "docs/sprints/geplant/Y1-b"), filepath.Join(root, "docs/sprints/aktiv/Y1-b")); err != nil {
		t.Fatal(err)
	}
	got := ChangedIDs(before, Snapshot(root))
	want := []string{"B-001", "B-002", "X1.1", "Y1", "plan"}
	if !slices.Equal(got, want) {
		t.Errorf("IDs %v, erwartet %v", got, want)
	}
}

func TestChangedIDsGrenzen(t *testing.T) {
	t.Parallel()
	cur := map[string]string{}
	for i := 0; i <= maxChangedIDs; i++ {
		cur["docs/backlog/B-"+strconv.Itoa(100+i)+"-x.md"] = "1|1"
	}
	if ChangedIDs(map[string]string{}, cur) != nil {
		t.Error("mehr als maxChangedIDs Dateien: keine IDs")
	}
	if ChangedIDs(map[string]string{}, map[string]string{"docs/sprints/aktiv/x/y/z.md": "1|1"}) != nil {
		t.Error("Datei ohne erkennbare ID: keine IDs")
	}
	idx := map[string]string{"docs/backlog/README.md": "1|1", "docs/sprints/aktiv/README.md": "1|1", "docs/backlog/B-005-x.md": "1|1"}
	if ids := ChangedIDs(map[string]string{}, idx); !slices.Equal(ids, []string{"B-005"}) {
		t.Errorf("Index-READMEs zählen nicht: %v", ids)
	}
	if ids := ChangedIDs(map[string]string{}, map[string]string{"docs/sprints/erledigt/Z9-x/": ""}); !slices.Equal(ids, []string{"Z9"}) {
		t.Errorf("erledigter Sprint: %v", ids)
	}
}

func TestWatchIDs(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	write(t, root, "docs/backlog/B-001-a.md", "# B-001")
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	hit := make(chan []string, 4)
	WatchIDs(ctx, root, 10*time.Millisecond, func(ids []string) { hit <- ids })
	write(t, root, "docs/backlog/B-002-b.md", "# B-002")
	select {
	case ids := <-hit:
		if !slices.Equal(ids, []string{"B-002"}) {
			t.Errorf("IDs %v", ids)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("Änderung nicht erkannt")
	}
}

// Projekte (docs/projekte) werden beobachtet: die Projekt-ID kommt, die Übersicht gilt als Index (B-909).
func TestChangedIDsProjekte(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	write(t, root, "docs/projekte/WZG-werkzeuge.md", "# WZG")
	write(t, root, "docs/projekte/README.md", "# Projekte")
	before := Snapshot(root)
	write(t, root, "docs/projekte/WZG-werkzeuge.md", "# WZG neu")
	write(t, root, "docs/projekte/README.md", "# Projekte neu")
	if got := ChangedIDs(before, Snapshot(root)); !slices.Equal(got, []string{"WZG"}) {
		t.Errorf("IDs %v, erwartet [WZG]", got)
	}
}

// Ein Sprintwechsel per plan_set ändert Ticket und Sprint-README: beide IDs kommen im selben Ereignis (die Oberfläche
// lädt dann die Liste neu, PlanSprint.tickets bleibt nicht veraltet).
func TestChangedIDsSprintwechselTicket(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	write(t, root, "docs/backlog/B-001-a.md", "- **Sprint:** X1")
	write(t, root, "docs/sprints/aktiv/X1-a/README.md", "Tickets: B-001")
	write(t, root, "docs/sprints/aktiv/Y1-b/README.md", "Tickets: –")
	before := Snapshot(root)
	write(t, root, "docs/backlog/B-001-a.md", "- **Sprint:** Y1 xx")
	write(t, root, "docs/sprints/aktiv/X1-a/README.md", "Tickets: –")
	write(t, root, "docs/sprints/aktiv/Y1-b/README.md", "Tickets: B-001")
	if got := ChangedIDs(before, Snapshot(root)); !slices.Equal(got, []string{"B-001", "X1", "Y1"}) {
		t.Errorf("IDs %v", got)
	}
}
