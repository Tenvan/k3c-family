package paperassets

import (
	"bytes"
	"encoding/json"
	"fmt"
	"regexp"
)

// feld ist ein Schlüssel eines JSON-Objekts mit rohem Wert; die Reihenfolge bleibt erhalten.
type feld struct {
	key string
	val json.RawMessage
}

// objekt zerlegt raw in seine Felder; ok = false, wenn raw kein Objekt ist.
func objekt(raw json.RawMessage) ([]feld, bool, error) {
	raw = bytes.TrimSpace(raw)
	if len(raw) == 0 || raw[0] != '{' {
		return nil, false, nil
	}
	dec := json.NewDecoder(bytes.NewReader(raw))
	if _, err := dec.Token(); err != nil {
		return nil, false, err
	}
	var out []feld
	for dec.More() {
		tok, err := dec.Token()
		if err != nil {
			return nil, false, err
		}
		var v json.RawMessage
		if err := dec.Decode(&v); err != nil {
			return nil, false, err
		}
		out = append(out, feld{key: tok.(string), val: v})
	}
	if _, err := dec.Token(); err != nil {
		return nil, false, err
	}
	return out, true, nil
}

func istNull(raw json.RawMessage) bool { return bytes.Equal(bytes.TrimSpace(raw), []byte("null")) }

// MergePatch wendet einen JSON-Merge-Patch (RFC 7396) an: null löscht, Objekte rekursiv, alles andere ersetzt.
// Die Reihenfolge der Schlüssel des Ziels bleibt, neue kommen hinten an; Zahlen bleiben roh.
func MergePatch(ziel, patch json.RawMessage) (json.RawMessage, error) {
	if !json.Valid(patch) {
		return nil, fmt.Errorf("Patch ist kein gültiges JSON") //nolint:staticcheck // Satz für die Antwort
	}
	pf, ok, err := objekt(patch)
	if err != nil {
		return nil, err
	}
	if !ok {
		return kompakt(patch)
	}
	zf, _, err := objekt(ziel)
	if err != nil {
		return nil, err
	}
	for _, p := range pf {
		i := index(zf, p.key)
		if istNull(p.val) {
			if i >= 0 {
				zf = append(zf[:i], zf[i+1:]...)
			}
			continue
		}
		var alt json.RawMessage
		if i >= 0 {
			alt = zf[i].val
		}
		neu, err := MergePatch(alt, p.val)
		if err != nil {
			return nil, err
		}
		if i >= 0 {
			zf[i].val = neu
		} else {
			zf = append(zf, feld{key: p.key, val: neu})
		}
	}
	return schreibeObjekt(zf)
}

func index(f []feld, key string) int {
	for i := range f {
		if f[i].key == key {
			return i
		}
	}
	return -1
}

func schreibeObjekt(f []feld) (json.RawMessage, error) {
	var b bytes.Buffer
	b.WriteByte('{')
	for i, x := range f {
		if i > 0 {
			b.WriteByte(',')
		}
		k, _ := json.Marshal(x.key)
		b.Write(k)
		b.WriteByte(':')
		v, err := kompakt(x.val)
		if err != nil {
			return nil, err
		}
		b.Write(v)
	}
	b.WriteByte('}')
	return b.Bytes(), nil
}

func kompakt(raw json.RawMessage) (json.RawMessage, error) {
	var b bytes.Buffer
	if err := json.Compact(&b, raw); err != nil {
		return nil, err
	}
	return b.Bytes(), nil
}

// zahlenListe ist eine eingerückte Liste nur aus Zahlen; Format schreibt sie in eine Zeile wie aus-prototyp.ts.
var zahlenListe = regexp.MustCompile(`\[\s*(-?[\d.eE+-]+(?:,\s*-?[\d.eE+-]+)*)\s*\]`)
var trenner = regexp.MustCompile(`,\s*`)

// Format rückt eine Spec wie die Fixtures ein (2 Leerzeichen, Zahlenlisten in einer Zeile, Zeilenende am Schluss).
func Format(raw json.RawMessage) ([]byte, error) {
	var b bytes.Buffer
	if err := json.Indent(&b, raw, "", "  "); err != nil {
		return nil, err
	}
	out := zahlenListe.ReplaceAllFunc(b.Bytes(), func(m []byte) []byte {
		inner := zahlenListe.FindSubmatch(m)[1]
		return append(append([]byte("["), trenner.ReplaceAll(inner, []byte(", "))...), ']')
	})
	return append(out, '\n'), nil
}
