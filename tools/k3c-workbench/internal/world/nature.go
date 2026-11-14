package world

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"strconv"
	"strings"
)

// Natur-Werte im Welten-Editor (B-891, docs/rules/welten.md § 2.1): Die Seite zeigt je Insel-Stufe die geltenden Werte
// (fest oder aus dem Seed) und setzt einen Wert fest oder zurück. Geschrieben wird nur `natureByDepth` der Insel; alle
// übrigen Bytes der Welt-Datei bleiben gleich. Vor dem Schreiben prüft k3c-sim den Entwurf wie der Server.

// natureValues sind fest gesetzte Natur-Werte einer Tiefe (Form wie level.NatureValues, ohne Import des Engine-Kerns).
type natureValues struct {
	Traits  map[string]int            `json:"traits,omitempty"`
	Weights map[string]map[string]int `json:"weights,omitempty"`
}

func (v natureValues) empty() bool { return len(v.Traits) == 0 && len(v.Weights) == 0 }

// set setzt key (Eigenschaft oder „gruppe.variante“) auf value; nil entfernt den Eintrag, leere Gruppen entfallen.
func (v *natureValues) set(key string, value *int) {
	group, option, weight := strings.Cut(key, ".")
	switch {
	case !weight && value == nil:
		delete(v.Traits, key)
	case !weight:
		if v.Traits == nil {
			v.Traits = map[string]int{}
		}
		v.Traits[key] = *value
	case value == nil:
		delete(v.Weights[group], option)
		if len(v.Weights[group]) == 0 {
			delete(v.Weights, group)
		}
	default:
		if v.Weights == nil {
			v.Weights = map[string]map[string]int{}
		}
		if v.Weights[group] == nil {
			v.Weights[group] = map[string]int{}
		}
		v.Weights[group][option] = *value
	}
}

// islandHead ist der Teil einer Insel, den das Festsetzen braucht.
type islandHead struct {
	ID            string                  `json:"id"`
	Depths        []int                   `json:"depths"`
	NatureByDepth map[string]natureValues `json:"natureByDepth"`
}

// islandSpan findet die Insel id in `islands` der Welt-Datei: Byte-Bereich ihres Objekts und ihr Kopf.
func islandSpan(raw []byte, id string) (span, islandHead, error) {
	arr, found, err := valueSpan(raw, span{0, len(raw)}, "islands")
	if err != nil || !found {
		return span{}, islandHead{}, fmt.Errorf("islands fehlt in der Welt-Datei (%v)", err)
	}
	dec := json.NewDecoder(bytes.NewReader(raw[arr.from:arr.to]))
	if tok, err := dec.Token(); err != nil || tok != json.Delim('[') {
		return span{}, islandHead{}, fmt.Errorf("islands ist keine Liste")
	}
	for dec.More() {
		var v json.RawMessage
		if err := dec.Decode(&v); err != nil {
			return span{}, islandHead{}, err
		}
		var head islandHead
		if json.Unmarshal(v, &head) == nil && head.ID == id {
			end := arr.from + int(dec.InputOffset())
			return span{end - len(v), end}, head, nil
		}
	}
	return span{}, islandHead{}, fmt.Errorf("die Insel %q gibt es nicht", id)
}

// patchNature setzt in der Insel island den Natur-Wert key der Stufe stage (value nil = zurück auf „aus Seed“).
func patchNature(raw []byte, island string, stage int, key string, value *int) ([]byte, error) {
	obj, head, err := islandSpan(raw, island)
	if err != nil {
		return nil, err
	}
	if stage < 0 || stage >= len(head.Depths) {
		return nil, fmt.Errorf("die Stufe %d gibt es auf der Insel %q nicht", stage, island)
	}
	if key == "" || strings.Count(key, ".") > 1 {
		return nil, fmt.Errorf("ungültiger Natur-Schlüssel %q", key)
	}
	depth := strconv.Itoa(head.Depths[stage])
	nb := head.NatureByDepth
	if nb == nil {
		nb = map[string]natureValues{}
	}
	entry := nb[depth]
	entry.set(key, value)
	if entry.empty() {
		delete(nb, depth)
	} else {
		nb[depth] = entry
	}
	if len(nb) == 0 {
		at, found, err := valueSpan(raw, obj, "natureByDepth")
		if err != nil || !found {
			return raw, err
		}
		return removeKey(raw, at, "natureByDepth"), nil
	}
	text, err := json.Marshal(nb)
	if err != nil {
		return nil, err
	}
	return setKey(raw, obj, "natureByDepth", text)
}

// SetNature setzt den Natur-Wert key (Eigenschaft wie „forestDensity“ oder „gruppe.variante“ wie „treeSpecies.tanne“)
// der Insel island in der Stufe stage fest; value nil setzt ihn auf „aus Seed“ zurück. Ungültig (Bereich, unbekannter
// Schlüssel, alle Varianten 0) bleibt die Datei, wie sie ist, und Error nennt den Grund. Die Antwort ist das neue Level.
func (e Editor) SetNature(ctx context.Context, id, island string, stage int, key string, value *int, seed string) LevelView {
	writeMu.Lock()
	warnings, err := e.writeNature(ctx, id, island, stage, key, value)
	writeMu.Unlock()
	if err != nil {
		return e.withError(ctx, id, island, stage, seed, err)
	}
	v := e.Level(ctx, id, island, stage, seed)
	v.Warnings = append(v.Warnings, warnings...)
	return v
}

// withError ist das unveränderte Level mit dem Grund der Ablehnung.
func (e Editor) withError(ctx context.Context, id, island string, stage int, seed string, err error) LevelView {
	v := e.Level(ctx, id, island, stage, seed)
	v.Rejected = err.Error()
	return v
}

func (e Editor) writeNature(ctx context.Context, id, island string, stage int, key string, value *int) ([]string, error) {
	raw, err := e.read(id)
	if err != nil {
		return nil, err
	}
	out, err := patchNature(raw, island, stage, key, value)
	if err != nil || bytes.Equal(out, raw) {
		return nil, err
	}
	var check struct {
		Warnings []string `json:"warnings"`
	}
	if err := e.call(ctx, map[string]any{"op": "check", "world": id, "draft": json.RawMessage(out)}, &check); err != nil {
		return nil, err
	}
	return check.Warnings, writeAtomic(e.file(id), out)
}
