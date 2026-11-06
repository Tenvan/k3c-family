package review

import (
	"bytes"
	"encoding/json"
	"errors"
	"io"
)

// stilAenderung beschreibt, was sich an der Stil-Datei gegenüber der Basis geändert hat: alles (Papier, Licht …)
// oder nur einzelne Materialien und Einzeltöne. Ein Asset ist betroffen, wenn es einen geänderten Namen nutzt
// (Fixture: Stil `stein` heller trifft die Mauer, nicht den Monarchen; B-701 D-Zeile zu `stil` je Asset).
type stilAenderung struct {
	alle  bool
	namen map[string]bool
}

// namensListen sind die Abschnitte der Stil-Datei, deren Einträge Specs per Namen nutzen.
var namensListen = map[string]bool{"materialien": true, "einzeltoene": true}

func vergleicheStil(alt, neu []byte) (stilAenderung, error) {
	var a, n map[string]any
	if err := json.Unmarshal(alt, &a); err != nil {
		return stilAenderung{}, err
	}
	if err := json.Unmarshal(neu, &n); err != nil {
		return stilAenderung{}, err
	}
	s := stilAenderung{namen: map[string]bool{}}
	for k := range vereinigung(a, n) {
		if k == "$schema" {
			continue
		}
		if !namensListen[k] {
			s.alle = s.alle || Kanonisch(a[k]) != Kanonisch(n[k])
			continue
		}
		ma, _ := a[k].(map[string]any)
		mn, _ := n[k].(map[string]any)
		for name := range vereinigung(ma, mn) {
			if Kanonisch(ma[name]) != Kanonisch(mn[name]) {
				s.namen[name] = true
			}
		}
	}
	return s, nil
}

func vereinigung(a, b map[string]any) map[string]bool {
	keys := map[string]bool{}
	for k := range a {
		keys[k] = true
	}
	for k := range b {
		keys[k] = true
	}
	return keys
}

// betrifft meldet, ob die Stiländerung die Spec trifft: alles geändert oder ein genutzter Name (jeder Text der Spec
// zählt, so sind Lagen, Umhang und Varianten erfasst).
func (s stilAenderung) betrifft(spec []byte) bool {
	if s.alle {
		return true
	}
	if len(s.namen) == 0 {
		return false
	}
	var v any
	if json.Unmarshal(spec, &v) != nil {
		return true
	}
	return nutztNamen(v, s.namen)
}

func nutztNamen(v any, namen map[string]bool) bool {
	switch w := v.(type) {
	case string:
		return namen[w]
	case []any:
		for _, e := range w {
			if nutztNamen(e, namen) {
				return true
			}
		}
	case map[string]any:
		for _, e := range w {
			if nutztNamen(e, namen) {
				return true
			}
		}
	}
	return false
}

// ErsteAnimation liefert den ersten Schlüssel von `animationen` in Datei-Reihenfolge; leer ohne Animation.
func ErsteAnimation(spec []byte) (string, error) {
	namen, err := Animationen(spec)
	if err != nil || len(namen) == 0 {
		return "", err
	}
	return namen[0], nil
}

// Animationen liefert alle Animationsnamen in Datei-Reihenfolge (Go-Maps kennen keine).
func Animationen(spec []byte) ([]string, error) {
	var top map[string]json.RawMessage
	if err := json.Unmarshal(spec, &top); err != nil {
		return nil, err
	}
	raw, ok := top["animationen"]
	if !ok {
		return nil, nil
	}
	dec := json.NewDecoder(bytes.NewReader(raw))
	if _, err := dec.Token(); err != nil {
		return nil, err
	}
	var namen []string
	for dec.More() {
		key, err := dec.Token()
		if err != nil {
			return nil, err
		}
		name, _ := key.(string)
		namen = append(namen, name)
		var skip json.RawMessage
		if err := dec.Decode(&skip); err != nil && !errors.Is(err, io.EOF) {
			return nil, err
		}
	}
	return namen, nil
}
