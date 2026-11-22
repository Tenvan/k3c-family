package datadoc

import (
	"encoding/json"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"testing"
)

// WZ28.11/AC-12 (B-608): Jede Datei der Liste hat eine Beschreibung unter docs/data/ mit Zweck; jeder Pfad der
// Aufbau-Tabelle und der wichtigsten Werte existiert in der Datei (bei Ordnern in mindestens einer), jedes
// Varianten-Beispiel `datei.json:pfad=wert` zeigt auf eine Zahl.

var (
	tablePath = regexp.MustCompile("^\\|\\s*`([^`]+)`\\s*\\|")
	tickPath  = regexp.MustCompile("`([^`]+)`")
	example   = regexp.MustCompile("`([a-z0-9-]+\\.json):([^`=]+)=[^`]+`")
)

func TestBeschreibungenVollstaendig(t *testing.T) {
	for _, f := range List(root) {
		if f.Missing {
			t.Errorf("docs/data/%s.md fehlt (Beschreibung für data/%s)", f.Doc, f.Name)
			continue
		}
		if f.Purpose == "" || f.Key == "" {
			t.Errorf("docs/data/%s.md: Zeilen „Zweck“ und „Wichtigste Werte“ nötig", f.Doc)
		}
		md, _ := Doc(root, f.Doc)
		docs := load(t, f.Name)
		check := func(where, p string) {
			if !anyHas(docs, p) {
				t.Errorf("docs/data/%s.md (%s): Pfad %q fehlt in data/%s", f.Doc, where, p, f.Name)
			}
		}
		for _, p := range sectionPaths(md) {
			check("Aufbau", p)
		}
		for _, m := range tickPath.FindAllStringSubmatch(f.Key, -1) {
			check("Wichtigste Werte", m[1])
		}
		for _, m := range example.FindAllStringSubmatch(md, -1) {
			if !f.Vary || m[1] != f.Name {
				t.Errorf("docs/data/%s.md: Beispiel %s gehört nicht zu einer variierbaren data/%s", f.Doc, m[0], f.Name)
				continue
			}
			if v, ok := lookup(docs[0], strings.Split(m[2], ".")); !ok || !isNumber(v) {
				t.Errorf("docs/data/%s.md: Beispiel %s zeigt nicht auf eine Zahl", f.Doc, m[0])
			}
		}
	}
}

// sectionPaths liefert die Pfade der ersten Tabellenspalte unter „## Aufbau“.
func sectionPaths(md string) []string {
	var out []string
	in := false
	for line := range strings.SplitSeq(md, "\n") {
		if strings.HasPrefix(line, "## ") {
			in = strings.TrimSpace(line) == "## Aufbau"
			continue
		}
		if m := tablePath.FindStringSubmatch(line); in && m != nil {
			out = append(out, m[1])
		}
	}
	return out
}

func load(t *testing.T, name string) []any {
	t.Helper()
	paths, _ := filepath.Glob(filepath.Join(root, "data", name))
	var out []any
	for _, p := range paths {
		raw, err := os.ReadFile(p)
		if err != nil {
			t.Fatal(err)
		}
		var v any
		if err := json.Unmarshal(raw, &v); err != nil {
			t.Fatalf("%s: %v", p, err)
		}
		out = append(out, v)
	}
	if len(out) == 0 {
		t.Fatalf("data/%s fehlt", name)
	}
	return out
}

func anyHas(docs []any, path string) bool {
	for _, d := range docs {
		if has(d, strings.Split(path, ".")) {
			return true
		}
	}
	return false
}

// has: Pfad existiert; `*` passt auf jeden Schlüssel bzw. Index (mindestens einer muss weiterführen).
func has(v any, keys []string) bool {
	if len(keys) == 0 {
		return true
	}
	if keys[0] != "*" {
		next, ok := lookup(v, keys[:1])
		return ok && has(next, keys[1:])
	}
	switch x := v.(type) {
	case map[string]any:
		for _, c := range x {
			if has(c, keys[1:]) {
				return true
			}
		}
	case []any:
		for _, c := range x {
			if has(c, keys[1:]) {
				return true
			}
		}
	}
	return false
}

func lookup(v any, keys []string) (any, bool) {
	for _, k := range keys {
		switch x := v.(type) {
		case map[string]any:
			c, ok := x[k]
			if !ok {
				return nil, false
			}
			v = c
		case []any:
			i, err := strconv.Atoi(k)
			if err != nil || i < 0 || i >= len(x) {
				return nil, false
			}
			v = x[i]
		default:
			return nil, false
		}
	}
	return v, true
}

func isNumber(v any) bool {
	_, ok := v.(float64)
	return ok
}
