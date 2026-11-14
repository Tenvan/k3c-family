package world

import (
	"bytes"
	"encoding/json"
	"fmt"
	"reflect"
	"slices"
	"strconv"
	"strings"
)

// Format-stabiles Schreiben der Welt-Datei (aus dem früheren Asset-Studio, app_world_patch.go, plan_file.go): Nur der
// Wert von `overrides` bekommt neue Bytes, alle übrigen Felder der Datei bleiben byte-gleich.

// span ist ein Byte-Bereich [from, to) im Inhalt.
type span struct{ from, to int }

// valueSpan findet in einem JSON-Objekt (raw[outer.from:outer.to]) den Wert zu key; ok ist false, wenn der Schlüssel fehlt.
func valueSpan(raw []byte, outer span, key string) (span, bool, error) {
	dec := json.NewDecoder(bytes.NewReader(raw[outer.from:outer.to]))
	if tok, err := dec.Token(); err != nil || tok != json.Delim('{') {
		return span{}, false, fmt.Errorf("JSON-Objekt erwartet")
	}
	for dec.More() {
		tok, err := dec.Token()
		if err != nil {
			return span{}, false, err
		}
		var v json.RawMessage
		if err := dec.Decode(&v); err != nil {
			return span{}, false, err
		}
		if tok.(string) == key {
			end := int(dec.InputOffset())
			return span{outer.from + end - len(v), outer.from + end}, true, nil
		}
	}
	return span{}, false, nil
}

// insertSpan ist die Stelle vor der schließenden Klammer des Objekts, an der ein fehlendes Feld angehängt wird; Leerraum
// nach der Klammer (Zeilenende am Dateiende) wird übersprungen.
func insertSpan(raw []byte, obj span) span {
	space := func(b byte) bool { return b == ' ' || b == '\n' || b == '\r' || b == '\t' }
	end := obj.to
	for end > obj.from && space(raw[end-1]) {
		end--
	}
	end-- // die schließende Klammer
	for end > obj.from && space(raw[end-1]) {
		end--
	}
	return span{end, end}
}

// setKey setzt den Wert von key im Objekt obj; fehlt der Schlüssel, wird er vor der schließenden Klammer angehängt.
func setKey(raw []byte, obj span, key string, text []byte) ([]byte, error) {
	at, found, err := valueSpan(raw, obj, key)
	if err != nil {
		return nil, err
	}
	if !found {
		at = insertSpan(raw, obj)
		text = append(fmt.Appendf(nil, `, %q: `, key), text...)
	}
	return slices.Concat(raw[:at.from], text, raw[at.to:]), nil
}

// removeKey entfernt den Schlüssel key mit seinem Wert (Bereich at) und dem Komma davor bzw. danach.
func removeKey(raw []byte, at span, key string) []byte {
	from := bytes.LastIndex(raw[:at.from], []byte(strconv.Quote(key)))
	to := at.to
	i := from - 1
	for i >= 0 && strings.ContainsRune(" \t\r\n", rune(raw[i])) {
		i--
	}
	if i >= 0 && raw[i] == ',' {
		from = i
	} else {
		for to < len(raw) && strings.ContainsRune(" \t\r\n,", rune(raw[to])) {
			to++
		}
	}
	return slices.Concat(raw[:from], raw[to:])
}

// sameJSON: a und b bedeuten denselben Wert (Zahlenformat, Leerraum und Schlüsselreihenfolge egal).
func sameJSON(a, b []byte) bool {
	var x, y any
	return json.Unmarshal(a, &x) == nil && json.Unmarshal(b, &y) == nil && reflect.DeepEqual(x, y)
}

func compactJSON(v json.RawMessage) []byte {
	var b bytes.Buffer
	if json.Compact(&b, v) != nil {
		return v
	}
	return b.Bytes()
}

// patchOverrides setzt `overrides` der Welt-Datei auf ov (Objekt je Basis-Datei): inhaltlich gleich bleibt die Datei
// byte-gleich, leer entfällt der Schlüssel. Ein neuer Schlüssel steht am Ende des Objekts, hinter den Inseln; die
// Zeilenenden folgen der Datei (CRLF bleibt CRLF).
func patchOverrides(raw []byte, ov map[string]json.RawMessage) ([]byte, error) {
	whole := span{0, len(raw)}
	at, found, err := valueSpan(raw, whole, "overrides")
	if err != nil {
		return nil, err
	}
	text, err := json.Marshal(ov)
	if err != nil {
		return nil, err
	}
	switch {
	case found && sameJSON(raw[at.from:at.to], text):
		return raw, nil
	case !found && len(ov) == 0:
		return raw, nil
	case len(ov) == 0:
		return removeKey(raw, at, "overrides"), nil
	}
	var b bytes.Buffer
	if err := json.Indent(&b, text, "  ", "  "); err != nil {
		return nil, err
	}
	out := b.Bytes()
	if bytes.Contains(raw, []byte("\r\n")) { // Zeilenenden der Datei beibehalten
		out = bytes.ReplaceAll(out, []byte("\n"), []byte("\r\n"))
	}
	return setKey(raw, whole, "overrides", out)
}
