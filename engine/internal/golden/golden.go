// Package golden vergleicht Go-Ergebnisse Feld für Feld mit den Golden-Daten aus testdata/golden/ (B-043).
package golden

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"slices"
	"strconv"
	"testing"
)

// Tree wandelt v über JSON in einen generischen Baum, wie ihn json.Unmarshal liefert (Zahlen als float64).
func Tree(t testing.TB, v any) any {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	var out any
	if err := json.Unmarshal(raw, &out); err != nil {
		t.Fatal(err)
	}
	return out
}

// Raw ist v als JSON in der Feldreihenfolge von Go (für -update: ein Abzug des Zustands zu diesem Zeitpunkt).
func Raw(t testing.TB, v any) json.RawMessage {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	return raw
}

// Diff liefert den Pfad der ersten Abweichung zwischen zwei JSON-Bäumen ("" = gleich). Zahlen als float64.
func Diff(at string, want, got any) string {
	switch w := want.(type) {
	case map[string]any:
		g, ok := got.(map[string]any)
		if !ok {
			return at
		}
		keys := make([]string, 0, len(w)+len(g))
		for k := range w {
			keys = append(keys, k)
		}
		for k := range g {
			keys = append(keys, k)
		}
		slices.Sort(keys)
		for _, k := range slices.Compact(keys) {
			_, inW := w[k]
			_, inG := g[k]
			if inW != inG {
				return fmt.Sprintf("%s.%s (Schlüssel nur auf einer Seite)", at, k)
			}
			if d := Diff(at+"."+k, w[k], g[k]); d != "" {
				return d
			}
		}
	case []any:
		g, ok := got.([]any)
		if !ok || len(g) != len(w) {
			return fmt.Sprintf("%s (Länge)", at)
		}
		for i := range w {
			if d := Diff(at+"["+strconv.Itoa(i)+"]", w[i], g[i]); d != "" {
				return d
			}
		}
	default:
		if want != got {
			return fmt.Sprintf("%s = %v, erwartet %v", at, got, want)
		}
	}
	return ""
}

// Update ist der Schalter -update (B-137, `task golden:update`): Die Golden-Tests schreiben dann den Ist-Zustand in
// ihre Datei, statt zu vergleichen. Nur für gewollte Regel- oder Wertänderungen; Ablauf in docs/arbeitsweise.md.
var Update = flag.Bool("update", false, "Golden-Dateien aus dem Ist-Zustand neu schreiben (task golden:update)")

// File ist eine Golden-Datei in ihrer Form: oberste Ebene ein Objekt, ein Schlüssel je Zeile, Werte kompakt; Listen,
// die schon über mehrere Zeilen stehen, behalten einen Eintrag je Zeile.
type File struct {
	keys []string
	vals map[string]json.RawMessage
	crlf bool // Zeilenende der gelesenen Datei (Windows-Checkout): ein Lauf ohne Änderung ändert nichts
}

// ReadFile liest eine Golden-Datei mit der Reihenfolge ihrer Schlüssel.
func ReadFile(t testing.TB, path string) *File {
	t.Helper()
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	f := &File{vals: map[string]json.RawMessage{}, crlf: bytes.Contains(raw, []byte("\r\n"))}
	dec := json.NewDecoder(bytes.NewReader(raw))
	if tok, err := dec.Token(); err != nil || tok != json.Delim('{') {
		t.Fatalf("%s: kein Objekt (%v)", path, err)
	}
	for dec.More() {
		tok, err := dec.Token()
		var v json.RawMessage
		if err == nil {
			err = dec.Decode(&v)
		}
		if err != nil {
			t.Fatalf("%s: %v", path, err)
		}
		key := tok.(string)
		f.keys = append(f.keys, key)
		f.vals[key] = v
	}
	return f
}

// Set ersetzt den Wert von key, behält aber die alten Bytes, wenn der Inhalt gleich ist (kein Diff ohne Änderung).
func (f *File) Set(t testing.TB, key string, v any) {
	t.Helper()
	if old, ok := f.vals[key]; ok && bytes.ContainsRune(old, '\n') {
		var items []json.RawMessage
		if err := json.Unmarshal(keep(t, nil, v), &items); err != nil {
			t.Fatalf("%s: keine Liste: %v", key, err)
		}
		f.vals[key] = lines(t, old, items)
		return
	}
	f.vals[key] = keep(t, f.vals[key], v)
}

// lines setzt eine Liste mit einem Eintrag je Zeile; gleiche Einträge behalten ihre Bytes.
func lines(t testing.TB, old json.RawMessage, items []json.RawMessage) json.RawMessage {
	t.Helper()
	var prev []json.RawMessage
	_ = json.Unmarshal(old, &prev)
	var b bytes.Buffer
	b.WriteString("[\n")
	for i, it := range items {
		var o json.RawMessage
		if i < len(prev) {
			o = prev[i]
		}
		b.WriteString("    ")
		b.Write(keep(t, o, it))
		if i < len(items)-1 {
			b.WriteByte(',')
		}
		b.WriteByte('\n')
	}
	b.WriteString("  ]")
	return b.Bytes()
}

// keep liefert old, wenn v denselben Inhalt hat, sonst v kompakt (ohne HTML-Escapes, wie die Dateien).
func keep(t testing.TB, old json.RawMessage, v any) json.RawMessage {
	t.Helper()
	if old != nil && Diff("", Tree(t, old), Tree(t, v)) == "" {
		return old
	}
	var b bytes.Buffer
	enc := json.NewEncoder(&b)
	enc.SetEscapeHTML(false)
	if err := enc.Encode(v); err != nil {
		t.Fatal(err)
	}
	return bytes.TrimRight(b.Bytes(), "\n")
}

// Write schreibt die Datei (Schlüssel und Zeilenende wie gelesen).
func (f *File) Write(t testing.TB, path string) {
	t.Helper()
	var b bytes.Buffer
	b.WriteString("{\n")
	for i, k := range f.keys {
		name, _ := json.Marshal(k)
		fmt.Fprintf(&b, "  %s: %s", name, f.vals[k])
		if i < len(f.keys)-1 {
			b.WriteByte(',')
		}
		b.WriteByte('\n')
	}
	b.WriteString("}\n")
	out := b.Bytes()
	if f.crlf {
		out = bytes.ReplaceAll(out, []byte("\n"), []byte("\r\n"))
	}
	if err := os.WriteFile(path, out, 0o644); err != nil {
		t.Fatal(err)
	}
}
