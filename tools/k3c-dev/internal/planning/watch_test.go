package planning

import (
	"context"
	"os"
	"path/filepath"
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
