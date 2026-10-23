package assets

import (
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
)

// WZ25.4 (B-511/AC-05): Jede Datei, die das Studio in data/ oder public/ schreibt, entsteht per Temp-Datei und Umbenennen
// (bzw. Link). Der Server beobachtet beide Ordner und prüft nach 2 s Ruhe; eine halbe Datei darf nie dort liegen.

// Schreibwege ohne Atomarität sind nur für Ziele außerhalb der beobachteten Ordner erlaubt (Datei → Grund).
var nonAtomicAllowed = map[string]string{
	"k3c-studio/backend/rpc.go":  "saveBody schreibt den Upload in ein Temp-Verzeichnis außerhalb des Repos",
	"windowstate/windowstate.go": "Fensterzustand unter <UserConfigDir>/k3c",
}

// goFiles liefert die Go-Dateien (ohne Tests, Oberfläche und Testdaten) unter den Ordnern dirs, Schlüssel = Pfad ab tools/.
func goFiles(t *testing.T, dirs ...string) map[string]string {
	t.Helper()
	out := map[string]string{}
	skip := map[string]bool{"node_modules": true, "frontend": true, "testdata": true}
	for _, dir := range dirs {
		err := filepath.WalkDir(dir, func(path string, d os.DirEntry, err error) error {
			switch {
			case err != nil:
				return err
			case d.IsDir() && skip[d.Name()]:
				return filepath.SkipDir
			case d.IsDir() || !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go"):
				return nil
			}
			raw, rerr := os.ReadFile(path)
			out[strings.TrimPrefix(filepath.ToSlash(filepath.Clean(path)), "../")] = string(raw)
			return rerr
		})
		if err != nil {
			t.Fatal(err)
		}
	}
	return out
}

func TestSchreibwegeDesStudiosSindAtomar(t *testing.T) {
	files := goFiles(t, ".", "../k3c-studio")
	if len(files) < 10 {
		t.Fatalf("nur %d Dateien gelesen: Pfade stimmen nicht", len(files))
	}
	for rel, src := range files {
		for _, call := range []string{"os.WriteFile(", "os.Create(", "os.OpenFile(", "ioutil.WriteFile("} {
			if strings.Contains(src, call) && nonAtomicAllowed[rel] == "" {
				t.Errorf("%s ruft %s: schreibt nicht atomar (Temp-Datei und Umbenennen, writeAtomic); außerhalb von data/ und public/ in nonAtomicAllowed mit Grund eintragen", rel, call)
			}
		}
	}
}

// Der Import legt nie eine halbe Datei ab, hinterlässt keine Temp-Dateien und überschreibt keine vorhandene; gleichzeitige
// Importe mit demselben Titel bekommen verschiedene Namen mit vollständigem Inhalt.
func TestImportSchreibtAtomar(t *testing.T) {
	root := t.TempDir()
	const n = 12
	data := func(i int) []byte { return []byte(strings.Repeat(string(rune('a'+i)), 4096)) }
	names := make([]string, n)
	var wg sync.WaitGroup
	for i := range n {
		wg.Add(1)
		go func() {
			defer wg.Done()
			var err error
			if names[i], err = storeImport(root, KindSound, "Gleicher Titel", ".ogg", data(i)); err != nil {
				t.Error(err)
			}
		}()
	}
	wg.Wait()
	seen := map[string]bool{}
	for i, name := range names {
		if seen[name] {
			t.Errorf("Name %s doppelt vergeben", name)
		}
		seen[name] = true
		got, err := os.ReadFile(filepath.Join(root, "public", filepath.FromSlash(name)))
		if err != nil || string(got) != string(data(i)) {
			t.Errorf("%s: Inhalt unvollständig oder überschrieben (%d Bytes, %v)", name, len(got), err)
		}
	}
	entries, err := os.ReadDir(filepath.Join(root, "public", filepath.FromSlash(importDir[KindSound])))
	if err != nil {
		t.Fatal(err)
	}
	for _, e := range entries {
		if strings.HasPrefix(e.Name(), ".import-") || strings.HasPrefix(e.Name(), ".tmp") {
			t.Errorf("Temp-Datei %s bleibt liegen", e.Name())
		}
	}
	if len(entries) != n {
		t.Errorf("%d Dateien, erwartet %d", len(entries), n)
	}
}
