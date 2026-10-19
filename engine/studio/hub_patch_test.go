package studio

import (
	"encoding/json"
	"slices"
	"strings"
	"testing"
)

// WZ9.3 (B-462/AC-06): format-stabiles Schreiben, unabhängig von der Form des Kandidaten.

// lineDiff liefert die entfernten und hinzugekommenen Zeilen von a nach b (längste gemeinsame Teilfolge, ohne CR).
func lineDiff(a, b string) (removed, added []string) {
	x, y := strings.Split(strings.ReplaceAll(a, "\r\n", "\n"), "\n"), strings.Split(strings.ReplaceAll(b, "\r\n", "\n"), "\n")
	lcs := make([][]int, len(x)+1)
	for i := range lcs {
		lcs[i] = make([]int, len(y)+1)
	}
	for i := len(x) - 1; i >= 0; i-- {
		for j := len(y) - 1; j >= 0; j-- {
			if x[i] == y[j] {
				lcs[i][j] = lcs[i+1][j+1] + 1
			} else {
				lcs[i][j] = max(lcs[i+1][j], lcs[i][j+1])
			}
		}
	}
	i, j := 0, 0
	for i < len(x) || j < len(y) {
		switch {
		case i < len(x) && j < len(y) && x[i] == y[j]:
			i, j = i+1, j+1
		case j < len(y) && (i == len(x) || lcs[i][j+1] >= lcs[i+1][j]):
			added, j = append(added, y[j]), j+1
		default:
			removed, i = append(removed, x[i]), i+1
		}
	}
	return removed, added
}

// remarshal ist raw nach edit, kompakt und mit sortierten Schlüsseln (eine ganz andere Form als die Datei).
func remarshal(t *testing.T, raw string, edit func(m map[string]any)) string {
	t.Helper()
	var m map[string]any
	if err := json.Unmarshal([]byte(raw), &m); err != nil {
		t.Fatal(err)
	}
	edit(m)
	out, err := json.Marshal(m)
	if err != nil {
		t.Fatal(err)
	}
	return string(out)
}

func site(m map[string]any, list string, i int) map[string]any {
	return m[list].([]any)[i].(map[string]any)
}

func patched(t *testing.T, old, cand string) string {
	t.Helper()
	out, err := patchJSON([]byte(old), []byte(cand))
	if err != nil {
		t.Fatal(err)
	}
	return string(out)
}

const patchDoc = "{\r\n  \"a\": [\r\n    { \"k\": \"x\", \"n\": 1.50 },\r\n    { \"k\": \"y\", \"n\": 2 }\r\n  ],\r\n  \"b\": { \"u\": [1, 2, 3], \"g\": 2 },\r\n  \"c\": \"ü\"\r\n}\r\n"

func TestPatchUnveraendertIstByteGleich(t *testing.T) {
	cand := remarshal(t, patchDoc, func(map[string]any) {})
	if got := patched(t, patchDoc, cand); got != patchDoc {
		t.Fatalf("unverändert, aber anders geschrieben:\n%s", got)
	}
}

func TestPatchEineZahlEineZeile(t *testing.T) {
	cand := remarshal(t, patchDoc, func(m map[string]any) { site(m, "a", 1)["n"] = 3 })
	got := patched(t, patchDoc, cand)
	removed, added := lineDiff(patchDoc, got)
	if !slices.Equal(removed, []string{`    { "k": "y", "n": 2 }`}) || !slices.Equal(added, []string{`    { "k": "y", "n": 3 }`}) {
		t.Fatalf("Diff: -%q +%q", removed, added)
	}
}

func TestPatchSchluesselUndEintraege(t *testing.T) {
	cand := remarshal(t, patchDoc, func(m map[string]any) {
		site(m, "a", 0)["d"] = true
		m["a"] = append(m["a"].([]any), map[string]any{"k": "z"})
		m["b"].(map[string]any)["u"] = []any{1, 3}
	})
	got := patched(t, patchDoc, cand)
	for _, want := range []string{`    { "k": "x", "n": 1.50, "d": true },`, `    { "k": "y", "n": 2 },`, `    { "k": "z" }` + "\r\n  ],", `"u": [1, 3]`} {
		if !strings.Contains(got, want) {
			t.Errorf("fehlt %q in\n%s", want, got)
		}
	}
	if !strings.HasSuffix(got, "\"c\": \"ü\"\r\n}\r\n") {
		t.Errorf("Ende verändert:\n%s", got)
	}
}

func TestPatchEintragEntfernen(t *testing.T) {
	cand := remarshal(t, patchDoc, func(m map[string]any) { m["a"] = m["a"].([]any)[1:] })
	removed, added := lineDiff(patchDoc, patched(t, patchDoc, cand))
	if !slices.Equal(removed, []string{`    { "k": "x", "n": 1.50 },`}) || len(added) != 0 {
		t.Fatalf("Diff: -%q +%q", removed, added)
	}
}

func TestPatchMehrzeiligerNeuerEintrag(t *testing.T) {
	const doc = "{\n  \"s\": [\n    {\n      \"k\": \"x\"\n    }\n  ]\n}\n"
	got := patched(t, doc, `{"s":[{"k":"x"},{"k":"y","h":2}]}`)
	const want = "{\n  \"s\": [\n    {\n      \"k\": \"x\"\n    },\n    {\n      \"k\": \"y\",\n      \"h\": 2\n    }\n  ]\n}\n"
	if got != want {
		t.Fatalf("got\n%s\nwant\n%s", got, want)
	}
}

func TestPatchUngueltigerKandidat(t *testing.T) {
	if _, err := patchJSON([]byte(patchDoc), []byte(`{"a": [`)); err == nil {
		t.Fatal("kaputter Kandidat ohne Fehler")
	}
}

// Tabellenfälle: Zeilenenden, Zahlenformat, Umlaute, doppelter Schlüssel. want leer = nur Bedeutung gleich cand.
func TestPatchFaelle(t *testing.T) {
	for _, c := range []struct{ name, old, cand, want string }{
		{"LF bleibt LF", "{\n  \"a\": [1, 2],\n  \"b\": 1\n}\n", `{"a":[1,2,3],"b":1}`, "{\n  \"a\": [1, 2, 3],\n  \"b\": 1\n}\n"},
		{"CRLF bleibt CRLF", "{\r\n  \"s\": [\r\n    { \"k\": 1 }\r\n  ]\r\n}\r\n", `{"s":[{"k":1},{"k":2}]}`, "{\r\n  \"s\": [\r\n    { \"k\": 1 },\r\n    { \"k\": 2 }\r\n  ]\r\n}\r\n"},
		{"44.0 gleich 44", "{ \"w\": 44.0, \"g\": 2 }\n", `{"w":44,"g":3}`, "{ \"w\": 44.0, \"g\": 3 }\n"},
		{"Umlaut im Klartext", "{\n  \"n\": \"Rüstkammer\",\n  \"x\": 1\n}\n", `{"n":"Rüstkammer","x":1,"m":"Lager für Erz"}`, "{\n  \"n\": \"Rüstkammer\",\n  \"x\": 1,\n  \"m\": \"Lager für Erz\"\n}\n"},
		{"doppelter Schlüssel", "{ \"a\": 1, \"a\": 2 }\n", `{"a":3}`, ""},
	} {
		t.Run(c.name, func(t *testing.T) {
			got := patched(t, c.old, c.cand)
			if c.want != "" && got != c.want {
				t.Fatalf("got %q, want %q", got, c.want)
			}
			if !sameJSON([]byte(got), []byte(c.cand)) {
				t.Fatalf("Bedeutung weicht ab: %q", got)
			}
		})
	}
}
