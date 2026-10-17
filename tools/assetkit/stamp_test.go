package assets

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestFindRoot(t *testing.T) {
	root := t.TempDir()
	writeFile(t, filepath.Join(root, "go.mod"), "// Spiel\nmodule k3c\n\ngo 1.27.0\n")
	tool := filepath.Join(root, "tools", "k3c-studio")
	writeFile(t, filepath.Join(tool, "go.mod"), "module k3c/tools/k3c-studio\n")
	sub := filepath.Join(tool, "frontend")
	if err := os.MkdirAll(sub, 0o755); err != nil {
		t.Fatal(err)
	}
	// Das go.mod des Werkzeugs zählt nicht, erst das des Spiels. Einen Fall ohne Wurzel prüft der Test nicht: unter
	// `task` liegt TMP in .work/ im Repo, darüber stünde das echte go.mod; isGameModule deckt den Negativfall ab.
	if got, err := FindRoot(sub); err != nil || filepath.Clean(got) != filepath.Clean(root) {
		t.Errorf("FindRoot(%s) = %q, %v; erwartet %q", sub, got, err, root)
	}
	if isGameModule(filepath.Join(tool, "go.mod")) || isGameModule(filepath.Join(tool, "fehlt.mod")) {
		t.Error("fremdes oder fehlendes go.mod gilt als Spiel")
	}
}

// B-459/AC-07: Speichern auf veraltetem Stand wird abgelehnt und schreibt nichts; auf aktuellem Stand gibt es den
// neuen Stand zurück, mit dem das nächste Speichern gelingt.
func TestSetChoicesIfSchreibregel(t *testing.T) {
	root := repo(t)
	stand, err := Stamp(root)
	if err != nil {
		t.Fatal(err)
	}
	wahl := []Choice{{File: "grafik/town/well.png"}}
	slot, next, err := SetChoicesIf(root, "farm", wahl, stand)
	if err != nil || len(slot.Choices) != 1 || next == stand || next == "" {
		t.Fatalf("aktueller Stand: %+v %q %v", slot, next, err)
	}
	// Ein anderer Prozess (MCP) schreibt dazwischen.
	if _, err := SetChoices(root, "farm", []Choice{{File: "grafik/town/wagon.png"}}); err != nil {
		t.Fatal(err)
	}
	before, _ := os.ReadFile(filepath.Join(root, RegisterPath))
	if _, _, err := SetChoicesIf(root, "farm", wahl, next); !errors.Is(err, ErrStale) {
		t.Fatalf("veralteter Stand: Fehler %v, erwartet ErrStale", err)
	}
	after, _ := os.ReadFile(filepath.Join(root, RegisterPath))
	if string(before) != string(after) {
		t.Error("abgelehntes Speichern hat geschrieben")
	}
	if ErrStale.Error() != "Register geändert, bitte erneut speichern" {
		t.Errorf("Meldung: %q", ErrStale.Error())
	}
}

// B-459/AC-07: Eine Änderung durch einen anderen Schreiber löst onChange aus, ein Stand ohne Änderung nicht.
func TestWatchMeldetAenderung(t *testing.T) {
	root := repo(t)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	changed := make(chan struct{}, 4)
	Watch(ctx, root, 10*time.Millisecond, func() { changed <- struct{}{} })
	select {
	case <-changed:
		t.Fatal("onChange ohne Änderung")
	case <-time.After(60 * time.Millisecond):
	}
	if _, err := SetChoices(root, "farm", []Choice{{File: "grafik/town/well.png"}}); err != nil {
		t.Fatal(err)
	}
	select {
	case <-changed:
	case <-time.After(2 * time.Second):
		t.Fatal("keine Meldung nach Änderung")
	}
}

// Gleicher Inhalt mit neuer Änderungszeit ist keine Änderung (Hash entscheidet).
func TestWatchGleicherInhalt(t *testing.T) {
	root := repo(t)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	changed := make(chan struct{}, 4)
	Watch(ctx, root, 10*time.Millisecond, func() { changed <- struct{}{} })
	later := time.Now().Add(time.Minute)
	if err := os.Chtimes(filepath.Join(root, RegisterPath), later, later); err != nil {
		t.Fatal(err)
	}
	select {
	case <-changed:
		t.Fatal("onChange bei gleichem Inhalt")
	case <-time.After(80 * time.Millisecond):
	}
}
