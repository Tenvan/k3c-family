package world

import (
	"bytes"
	"encoding/json"
	"errors"
	"io"
	"strconv"
	"strings"
)

// inline schreibt einen JSON-Wert in einer Zeile im Stil der Welt-Dateien: `{ "hp": 1.5, "damage": 1.3 }`, Listen von
// Zahlen `[0, 1, 2]`, Listen mit Objekten `[ { "slot": -7 } ]`. Schlüsselreihenfolge und Zahlen bleiben, wie sie sind;
// ein zweites inline über das Ergebnis ändert nichts.
func inline(raw []byte) (string, error) {
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.UseNumber()
	var b strings.Builder
	if err := inlineValue(dec, &b); err != nil {
		return "", err
	}
	if _, err := dec.Token(); err != io.EOF {
		return "", errors.New("kein gültiges JSON: Daten nach dem Ende")
	}
	return b.String(), nil
}

func quote(s string) string {
	var b bytes.Buffer
	enc := json.NewEncoder(&b)
	enc.SetEscapeHTML(false)
	_ = enc.Encode(s)
	return strings.TrimSuffix(b.String(), "\n")
}

func inlineValue(dec *json.Decoder, b *strings.Builder) error {
	tok, err := dec.Token()
	if err != nil {
		return err
	}
	switch t := tok.(type) {
	case json.Delim:
		if t == '{' {
			return inlineObject(dec, b)
		}
		return inlineArray(dec, b)
	case string:
		b.WriteString(quote(t))
	case json.Number:
		b.WriteString(t.String())
	case bool:
		b.WriteString(strconv.FormatBool(t))
	case nil:
		b.WriteString("null")
	}
	return nil
}

func inlineObject(dec *json.Decoder, b *strings.Builder) error {
	var parts []string
	for dec.More() {
		key, err := dec.Token()
		if err != nil {
			return err
		}
		var v strings.Builder
		if err := inlineValue(dec, &v); err != nil {
			return err
		}
		parts = append(parts, quote(key.(string))+": "+v.String())
	}
	if _, err := dec.Token(); err != nil {
		return err
	}
	if len(parts) == 0 {
		b.WriteString("{}")
		return nil
	}
	b.WriteString("{ " + strings.Join(parts, ", ") + " }")
	return nil
}

func inlineArray(dec *json.Decoder, b *strings.Builder) error {
	var parts []string
	nested := false
	for dec.More() {
		var v strings.Builder
		if err := inlineValue(dec, &v); err != nil {
			return err
		}
		nested = nested || strings.HasPrefix(v.String(), "{") || strings.HasPrefix(v.String(), "[")
		parts = append(parts, v.String())
	}
	if _, err := dec.Token(); err != nil {
		return err
	}
	switch {
	case len(parts) == 0:
		b.WriteString("[]")
	case nested:
		b.WriteString("[ " + strings.Join(parts, ", ") + " ]")
	default:
		b.WriteString("[" + strings.Join(parts, ", ") + "]")
	}
	return nil
}
