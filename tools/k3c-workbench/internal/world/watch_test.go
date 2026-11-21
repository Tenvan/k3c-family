package world

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func writeFile(t *testing.T, path, text string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

// B-1216/AC-01: Eine geänderte Welt-Datei nennt ihre ID, eine Basis-Datei nennt keine ID (alle laden neu).
func TestChangedIDs(t *testing.T) {
	root := t.TempDir()
	writeFile(t, filepath.Join(root, "data", "worlds", "standard.json"), "{}")
	writeFile(t, filepath.Join(root, "data", "worlds", "nil.json"), "{}")
	writeFile(t, filepath.Join(root, "data", "buildings.json"), "{}")
	before := Snapshot(root)
	writeFile(t, filepath.Join(root, "data", "worlds", "nil.json"), `{"id":"nil"}`)
	writeFile(t, filepath.Join(root, "data", "worlds", "neu.json"), "{}")
	ids, all := ChangedIDs(before, Snapshot(root))
	if all || len(ids) != 2 || ids[0] != "neu" || ids[1] != "nil" {
		t.Fatalf("ids=%v all=%v, will [neu nil]", ids, all)
	}
	mid := Snapshot(root)
	writeFile(t, filepath.Join(root, "data", "buildings.json"), `{"a":1}`)
	if ids, all := ChangedIDs(mid, Snapshot(root)); !all || ids != nil {
		t.Fatalf("Basis-Datei: ids=%v all=%v, will alle", ids, all)
	}
	if ids, all := ChangedIDs(mid, mid); all || len(ids) != 0 {
		t.Fatalf("ohne Änderung: ids=%v all=%v", ids, all)
	}
	now := Snapshot(root)
	os.Remove(filepath.Join(root, "data", "worlds", "neu.json"))
	if ids, _ := ChangedIDs(now, Snapshot(root)); len(ids) != 1 || ids[0] != "neu" {
		t.Fatalf("gelöscht: %v", ids)
	}
}

func TestWatchMeldetAenderung(t *testing.T) {
	root := t.TempDir()
	file := filepath.Join(root, "data", "worlds", "standard.json")
	writeFile(t, file, "{}")
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	got := make(chan []string, 1)
	Watch(ctx, root, 10*time.Millisecond, func(ids []string) { got <- ids })
	writeFile(t, file, `{"id":"standard","name":"S"}`)
	select {
	case ids := <-got:
		if len(ids) != 1 || ids[0] != "standard" {
			t.Fatalf("ids=%v", ids)
		}
	case <-time.After(3 * time.Second):
		t.Fatal("keine Meldung")
	}
}
