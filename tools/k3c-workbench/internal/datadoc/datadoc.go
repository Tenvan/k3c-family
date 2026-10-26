// Package datadoc kennt die Dateien in data/, die das Balancing beeinflussen, und ihre Beschreibungen unter
// docs/data/ (WZ28.10, B-608, B-610). Die Balancing-Seite zeigt daraus die Liste, die Beschreibung als Popup und die
// Zahlenwerte einer Datei als Pfade in der Form von internal/balance/vary.go (`datei.json:a.b.c`).
package datadoc

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strconv"
	"strings"
)

// Leser einer Datei (Spalte „gelesen von“).
const (
	BySim    = "Simulation"
	ByLevel  = "Level-Generator"
	ByTester = "Balancing-Tester"
)

// entry ist eine Datei der Liste: Name wie in data/, Beschreibung docs/data/<Doc>.md, Leser. Vary: Varianten wirken,
// weil sim.UseData die Datei neu lädt (engine/sim/data.go; internal/balance/vary.go › withData); TestVaryWieUseData.
type entry struct {
	Name   string
	Doc    string
	ReadBy []string
	Vary   bool
}

// files sind alle Dateien, die Simulation, Level-Generator oder Balancing-Tester lesen. TestListeWieLoader hält die
// Liste gleich mit den Dateinamen, die engine/ lädt.
var files = []entry{
	{"economy.json", "economy", []string{BySim, ByLevel}, true},
	{"buildings.json", "buildings", []string{BySim, ByLevel}, true},
	{"hub.json", "hub", []string{BySim, ByLevel}, true},
	{"troops.json", "troops", []string{BySim}, true},
	{"enemies.json", "enemies", []string{BySim}, true},
	{"waves.json", "waves", []string{BySim}, true},
	{"difficulty.json", "difficulty", []string{BySim}, false},
	{"bosses.json", "bosses", []string{BySim}, false},
	{"guards.json", "guards", []string{ByLevel}, false},
	{"events.json", "events", []string{BySim}, false},
	{"goals.json", "goals", []string{BySim}, false},
	{"island-sizes.json", "island-sizes", []string{BySim}, false},
	{"ruins.json", "ruins", []string{ByLevel}, false},
	{"monarch.json", "monarch", []string{BySim}, true},
	{"biomes/*.json", "biomes", []string{BySim, ByLevel}, false},
	{"worlds/*.json", "worlds", []string{BySim}, false},
	{"balance-targets.json", "balance-targets", []string{ByTester}, false},
	{"balance-hints.json", "balance-hints", []string{ByTester}, false},
	{"ship.json", "ship", []string{ByTester}, false},
}

// File ist eine Zeile der Liste auf der Balancing-Seite.
type File struct {
	Name    string   `json:"name"`
	Doc     string   `json:"doc"`
	ReadBy  []string `json:"readBy"`
	Purpose string   `json:"purpose"`
	Key     string   `json:"key"`
	// Vary: die Zahlenwerte taugen als Variante (Dateien, die sim.UseData neu lädt).
	Vary    bool `json:"vary"`
	Missing bool `json:"missing"`
}

// Value ist ein Zahlenwert einer Datei mit seinem Pfad (`a.b.0.c`).
type Value struct {
	Path  string  `json:"path"`
	Value float64 `json:"value"`
}

var (
	purposeLine = regexp.MustCompile(`(?m)^- \*\*Zweck:\*\*\s*(.+)$`)
	keyLine     = regexp.MustCompile(`(?m)^- \*\*Wichtigste Werte:\*\*\s*(.+)$`)
	docName     = regexp.MustCompile(`^[a-z0-9-]{1,40}$`)
)

// List liefert alle Dateien mit Zweck und wichtigsten Werten aus docs/data/; fehlt eine Beschreibung, ist Missing gesetzt.
func List(root string) []File {
	out := make([]File, 0, len(files))
	for _, e := range files {
		f := File{Name: e.Name, Doc: e.Doc, ReadBy: e.ReadBy, Vary: e.Vary}
		md, err := Doc(root, e.Doc)
		if err != nil {
			f.Missing = true
		} else {
			f.Purpose = firstMatch(purposeLine, md)
			f.Key = firstMatch(keyLine, md)
		}
		out = append(out, f)
	}
	return out
}

func firstMatch(re *regexp.Regexp, s string) string {
	if m := re.FindStringSubmatch(s); m != nil {
		return strings.TrimSpace(m[1])
	}
	return ""
}

// Doc liest docs/data/<name>.md; nur Namen der Liste, nie ein anderer Pfad.
func Doc(root, name string) (string, error) {
	if !docName.MatchString(name) || !slices.ContainsFunc(files, func(e entry) bool { return e.Doc == name }) {
		return "", fmt.Errorf("keine Beschreibung %q: nur die Dateien der Liste", name)
	}
	raw, err := os.ReadFile(filepath.Join(root, "docs", "data", name+".md"))
	if errors.Is(err, os.ErrNotExist) {
		return "", fmt.Errorf("docs/data/%s.md fehlt noch", name)
	}
	return string(raw), err
}

// Values liefert alle Zahlenwerte einer variierbaren Datei (z. B. economy.json) in Dateireihenfolge.
func Values(root, name string) ([]Value, error) {
	i := slices.IndexFunc(files, func(e entry) bool { return e.Name == name })
	if i < 0 || !files[i].Vary {
		return nil, fmt.Errorf("%q: nur variierbare Dateien der Liste", name)
	}
	raw, err := os.ReadFile(filepath.Join(root, "data", name))
	if err != nil {
		return nil, err
	}
	var out []Value
	err = walkNumbers(raw, func(path string, v float64) { out = append(out, Value{path, v}) })
	return out, err
}

// walkNumbers ruft fn für jede Zahl der JSON in Dateireihenfolge (Token-weise, damit die Reihenfolge der Schlüssel bleibt).
func walkNumbers(raw []byte, fn func(path string, v float64)) error {
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.UseNumber()
	return walk(dec, "", fn)
}

func walk(dec *json.Decoder, path string, fn func(string, float64)) error {
	tok, err := dec.Token()
	if err != nil {
		return err
	}
	switch t := tok.(type) {
	case json.Number:
		if v, err := t.Float64(); err == nil {
			fn(path, v)
		}
	case json.Delim:
		if t == '{' {
			for dec.More() {
				key, err := dec.Token()
				if err != nil {
					return err
				}
				if err := walk(dec, join(path, fmt.Sprint(key)), fn); err != nil {
					return err
				}
			}
		} else {
			for i := 0; dec.More(); i++ {
				if err := walk(dec, join(path, strconv.Itoa(i)), fn); err != nil {
					return err
				}
			}
		}
		_, err = dec.Token() // schließende Klammer
	}
	return err
}

func join(path, key string) string {
	if path == "" {
		return key
	}
	return path + "." + key
}
