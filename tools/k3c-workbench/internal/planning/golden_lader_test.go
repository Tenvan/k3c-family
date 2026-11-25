package planning

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"testing"
	"time"
)

// Golden-Fälle (RU68): Das Format steht in testdata/planung/README.md. Dieser Lader liest nur Dateien unter
// testdata/planung/ (JSON, Text, Markdown); kein Go-Typ ist Teil des Formats, ein Rust-Test liest dieselben Dateien.

const (
	goldenDir     = "../../../../testdata/planung"
	goldenUpdate  = "UPDATE_GOLDEN"
	goldenVorlage = "_vorlagen"
	errPrefix     = "FEHLER: "
	maskToday     = "<HEUTE>"
	maskRoot      = "<WURZEL>"
)

// goldenAufruf ist aufruf.json: ein Werkzeug der Planung mit seinen Argumenten wie im MCP-Tool.
type goldenAufruf struct {
	Werkzeug string          `json:"werkzeug"`
	Args     json.RawMessage `json:"args"`
}

var reCreated = regexp.MustCompile(`(\*\*Erstellt:\*\* )(\d{4}-\d{2}-\d{2})`)

// ladeFall liest aufruf.json eines Falls (Ordner dir).
func ladeFall(dir string) (goldenAufruf, error) {
	var a goldenAufruf
	b, err := os.ReadFile(filepath.Join(dir, "aufruf.json"))
	if err != nil {
		return a, err
	}
	if err := json.Unmarshal(b, &a); err != nil {
		return a, fmt.Errorf("aufruf.json: %w", err)
	}
	if a.Werkzeug == "" {
		return a, fmt.Errorf("aufruf.json: werkzeug fehlt")
	}
	return a, nil
}

// goldenFaelle sind die Fall-Ordner (ohne die mit `_` beginnenden Hilfsordner), nach Name.
func goldenFaelle(t *testing.T) []string {
	t.Helper()
	entries, err := os.ReadDir(goldenDir)
	if err != nil {
		t.Fatal(err)
	}
	var names []string
	for _, e := range entries {
		if e.IsDir() && !strings.HasPrefix(e.Name(), "_") {
			names = append(names, e.Name())
		}
	}
	sort.Strings(names)
	return names
}

// kopiereBaum kopiert alle Dateien von src nach dst (Pfade bleiben, Ordner entstehen nach Bedarf).
func kopiereBaum(src, dst string) error {
	return filepath.WalkDir(src, func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		rel, err := filepath.Rel(src, p)
		if err != nil {
			return err
		}
		b, err := os.ReadFile(p)
		if err != nil {
			return err
		}
		target := filepath.Join(dst, rel)
		if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
			return err
		}
		return os.WriteFile(target, b, 0o644)
	})
}

// baueVorher legt die Wurzel eines Falls an: vorher/ plus die festen Vorlagen unter docs/vorlagen/. Git kennt keine
// leeren Ordner, darum entstehen docs/sprints/<Zustand>/ hier.
func baueVorher(t *testing.T, fall string) string {
	t.Helper()
	root := t.TempDir()
	if err := kopiereBaum(filepath.Join(goldenDir, fall, "vorher"), root); err != nil {
		t.Fatal(err)
	}
	if err := kopiereBaum(filepath.Join(goldenDir, goldenVorlage), filepath.Join(root, "docs", "vorlagen")); err != nil {
		t.Fatal(err)
	}
	for _, st := range States {
		if err := os.MkdirAll(filepath.Join(root, "docs", "sprints", st), 0o755); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

// maskiere macht Antwort und Dateien vergleichbar: LF statt CRLF, das heutige Datum der Zeile „Erstellt“ und der Pfad der
// Wurzel durch Platzhalter.
func maskiere(text, root string) string {
	text = strings.ReplaceAll(text, "\r\n", "\n")
	for _, r := range []string{root, filepath.ToSlash(root)} {
		text = strings.ReplaceAll(text, r, maskRoot)
	}
	today := time.Now().Format("2006-01-02")
	return reCreated.ReplaceAllStringFunc(text, func(m string) string {
		if sub := reCreated.FindStringSubmatch(m); sub[2] == today {
			return sub[1] + maskToday
		}
		return m
	})
}

// leseBaum liest alle Dateien unter root als Pfad (mit /) → Text; docs/vorlagen/ gehört nicht zum Vergleich.
func leseBaum(root, mask string) (map[string]string, error) {
	files := map[string]string{}
	err := filepath.WalkDir(root, func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		rel, err := filepath.Rel(root, p)
		if err != nil {
			return err
		}
		rel = filepath.ToSlash(rel)
		if strings.HasPrefix(rel, "docs/vorlagen/") {
			return nil
		}
		b, err := os.ReadFile(p)
		if err != nil {
			return err
		}
		files[rel] = maskiere(string(b), mask)
		return nil
	})
	return files, err
}

// ersteAbweichung nennt die erste Zeile, in der zwei Texte sich unterscheiden.
func ersteAbweichung(want, got string) string {
	w, g := strings.Split(want, "\n"), strings.Split(got, "\n")
	for i := 0; i < len(w) || i < len(g); i++ {
		var a, b string
		if i < len(w) {
			a = w[i]
		}
		if i < len(g) {
			b = g[i]
		}
		if i >= len(w) || i >= len(g) || a != b {
			return fmt.Sprintf("Zeile %d:\n  erwartet: %q\n  erhalten: %q", i+1, a, b)
		}
	}
	return "gleich"
}

// vergleiche prüft den Dateibaum unter root gegen nachher/ des Falls; Abweichungen meldet es mit t.Errorf.
func vergleiche(t *testing.T, root, nachher string) {
	t.Helper()
	got, err := leseBaum(root, root)
	if err != nil {
		t.Fatal(err)
	}
	want, err := leseBaum(nachher, root)
	if err != nil {
		t.Fatal(err)
	}
	for rel, w := range want {
		g, ok := got[rel]
		switch {
		case !ok:
			t.Errorf("Datei fehlt: %s", rel)
		case g != w:
			t.Errorf("Datei %s weicht ab, %s", rel, ersteAbweichung(w, g))
		}
	}
	for rel := range got {
		if _, ok := want[rel]; !ok {
			t.Errorf("Datei zu viel: %s", rel)
		}
	}
}

// schreibeNachher ersetzt nachher/ und antwort.txt eines Falls (nur mit UPDATE_GOLDEN=1, bewusst, Begründung im Commit).
func schreibeNachher(t *testing.T, fall, root, antwort string) {
	t.Helper()
	dir := filepath.Join(goldenDir, fall)
	got, err := leseBaum(root, root)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.RemoveAll(filepath.Join(dir, "nachher")); err != nil {
		t.Fatal(err)
	}
	for rel, text := range got {
		p := filepath.Join(dir, "nachher", filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	if err := os.WriteFile(filepath.Join(dir, "antwort.txt"), []byte(antwort), 0o644); err != nil {
		t.Fatal(err)
	}
}
