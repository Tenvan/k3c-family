package services

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/tools/k3c-dev/internal/console"
)

func writeFile(t *testing.T, root, rel, text string) {
	t.Helper()
	p := filepath.Join(root, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

// B-097: snapshot und changed erkennen neue, geänderte und gelöschte Dateien; Endungen, versteckte Ordner und
// node_modules zählen nicht.
func TestSnapshotUndChanged(t *testing.T) {
	root := t.TempDir()
	w := &Watch{Paths: []string{"engine", "go.mod"}, Ext: []string{".go"}}
	writeFile(t, root, "engine/a.go", "package a")
	writeFile(t, root, "engine/sub/b.go", "package b")
	writeFile(t, root, "engine/notiz.txt", "ignoriert")
	writeFile(t, root, "engine/.git/c.go", "versteckt")
	writeFile(t, root, "engine/node_modules/d.go", "fremd")
	writeFile(t, root, "go.mod", "module x") // einzeln genannte Dateien zählen ohne Endungsprüfung
	before := snapshot(root, w)
	if len(before) != 3 || before["engine/a.go"] == (fileSig{}) || before["engine/sub/b.go"] == (fileSig{}) || before["go.mod"] == (fileSig{}) {
		t.Fatalf("Stand: %v", before)
	}
	if got := changed(before, snapshot(root, w)); got != "" {
		t.Errorf("ohne Änderung: %q", got)
	}
	writeFile(t, root, "engine/notiz.txt", "anders")
	if got := changed(before, snapshot(root, w)); got != "" {
		t.Errorf("Datei mit anderer Endung: %q", got)
	}
	writeFile(t, root, "engine/a.go", "package a // neu, länger")
	if got := changed(before, snapshot(root, w)); got != "engine/a.go" {
		t.Errorf("geändert: %q", got)
	}
	writeFile(t, root, "engine/neu.go", "package neu")
	if got := changed(before, snapshot(root, w)); got != "engine/a.go" { // die erste in Namensreihenfolge
		t.Errorf("neu und geändert: %q", got)
	}
	if err := os.Remove(filepath.Join(root, "engine", "sub", "b.go")); err != nil {
		t.Fatal(err)
	}
	if got := changed(before, snapshot(root, w)); got == "" {
		t.Error("gelöschte Datei nicht erkannt")
	}
	if got := snapshot(root, &Watch{Paths: []string{"gibt-es-nicht"}}); len(got) != 0 {
		t.Errorf("fehlender Pfad: %v", got)
	}
}

func TestWatchWirdGeprueft(t *testing.T) {
	for _, bad := range []string{
		`"paths": []`, `"paths": [""]`, `"paths": ["/etc"]`, `"paths": [".."]`, `"paths": ["../x"]`, `"paths": ["a\\b"]`,
		`"paths": ["engine"], "ext": ["go"]`, `"paths": ["engine"], "ext": ["."]`, `"paths": ["engine"], "ext": [".a/b"]`,
	} {
		path := writeConfig(t, `[{"name":"A","command":["x"],"port":1,"health":"http","watch":{`+bad+`}}]`)
		if _, err := Load(path); err == nil {
			t.Errorf("%s: kein Fehler", bad)
		}
	}
	ok := writeConfig(t, `[{"name":"A","command":["x"],"port":1,"health":"http","watch":{"paths":["engine","go.mod"],"ext":[".go"]}}]`)
	list, err := Load(ok)
	if err != nil || list[0].Watch == nil || len(list[0].Watch.Paths) != 2 {
		t.Errorf("gültiger Eintrag: %+v, %v", list, err)
	}
}

func watchController(t *testing.T, f *fake, root string, w *Watch) (*Controller, *console.Store) {
	t.Helper()
	store := console.New(console.DefaultCapacity, nil)
	s := svc("Server", false)
	s.Watch = w
	c := New([]Service{s}, Options{Root: root, Console: store, Start: f.start, Check: f.check, Listen: f.listen, Sample: f.sample,
		KillPID: f.killPID, StartPoll: 2 * time.Millisecond, StartTimeout: 300 * time.Millisecond, WatchEvery: time.Hour,
		StopTimeout: time.Second, FilesEvery: 5 * time.Millisecond, FilesQuiet: 15 * time.Millisecond})
	return c, store
}

func touch(t *testing.T, root, rel, text string) {
	t.Helper()
	writeFile(t, root, rel, text)
}

// B-097/AC-01: Eine Änderung startet den laufenden Dienst neu (alter Prozess beendet, neuer läuft); mehrere schnelle
// Änderungen ergeben einen Neustart; Stop beendet die Beobachtung.
func TestDateiAenderungStartetNeu(t *testing.T) {
	root := t.TempDir()
	touch(t, root, "engine/a.go", "package a")
	f := &fake{}
	f.healthy.Store(true)
	c, store := watchController(t, f, root, &Watch{Paths: []string{"engine"}, Ext: []string{".go"}})
	if _, err := c.Start(t.Context(), "Server"); err != nil {
		t.Fatal(err)
	}
	first := f.last()

	touch(t, root, "engine/a.go", "package a // 1")
	touch(t, root, "engine/b.go", "package b")
	waitFor(t, "Neustart", func() bool { return f.count() == 2 && c.Statuses()[0].State == Running })
	if !first.killed.Load() {
		t.Error("der alte Prozess wurde nicht beendet")
	}
	time.Sleep(80 * time.Millisecond)
	if f.count() != 2 {
		t.Errorf("mehrere Änderungen auf einmal: %d Starts, erwartet 2", f.count())
	}
	if !consoleHas(store, "Neustart wegen Änderung an engine/a.go") {
		t.Error("Konsole nennt den Neustart nicht")
	}

	touch(t, root, "engine/notiz.txt", "keine Go-Datei")
	time.Sleep(60 * time.Millisecond)
	if f.count() != 2 {
		t.Errorf("Datei mit anderer Endung löste einen Neustart aus: %d", f.count())
	}

	if _, err := c.Stop(t.Context(), "Server", false); err != nil {
		t.Fatal(err)
	}
	touch(t, root, "engine/a.go", "package a // nach dem Stop")
	time.Sleep(80 * time.Millisecond)
	if f.count() != 2 || c.Statuses()[0].State != Stopped {
		t.Errorf("nach Stop: %d Starts, Zustand %s", f.count(), c.Statuses()[0].State)
	}
}

// B-097/AC-02: Scheitert ein Neustart (Build-Fehler: nicht gesund), bleibt die Beobachtung aktiv; die nächste Änderung
// (der Fix) startet den Dienst wieder.
func TestFehlgeschlagenerNeustartWirdBeiNaechsterAenderungWiederholt(t *testing.T) {
	root := t.TempDir()
	touch(t, root, "engine/a.go", "package a")
	f := &fake{}
	f.healthy.Store(true)
	c, _ := watchController(t, f, root, &Watch{Paths: []string{"engine"}, Ext: []string{".go"}})
	if _, err := c.Start(t.Context(), "Server"); err != nil {
		t.Fatal(err)
	}

	f.healthy.Store(false) // der neue Build ist kaputt: der Dienst wird nicht gesund
	touch(t, root, "engine/a.go", "package a // kaputt")
	waitFor(t, "fehlgeschlagen", func() bool { return c.Statuses()[0].State == Failed })

	f.healthy.Store(true)
	touch(t, root, "engine/a.go", "package a // repariert")
	waitFor(t, "läuft wieder", func() bool { return c.Statuses()[0].State == Running })
}

func TestDienstOhneWatchWirdNichtBeobachtet(t *testing.T) {
	root := t.TempDir()
	touch(t, root, "engine/a.go", "package a")
	f := &fake{}
	f.healthy.Store(true)
	c, _ := watchController(t, f, root, nil)
	if _, err := c.Start(t.Context(), "Server"); err != nil {
		t.Fatal(err)
	}
	touch(t, root, "engine/a.go", "package a // neu")
	time.Sleep(80 * time.Millisecond)
	if f.count() != 1 {
		t.Errorf("Dienst ohne watch startete neu: %d", f.count())
	}
}

func consoleHas(store *console.Store, part string) bool {
	lines, _ := store.Tail("Server", 50)
	for _, l := range lines {
		if strings.Contains(l.Text, part) {
			return true
		}
	}
	return false
}
