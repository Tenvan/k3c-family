package world

import (
	"bytes"
	"context"
	"encoding/json"
	"maps"
	"slices"
	"strconv"
	"strings"
)

// Werte-Overrides (B-680, B-682, docs/rules/welten.md § 1): Die Ansicht „Werte“ zeigt je Feld den Basiswert (Basis
// data/, darüber die Overrides der Eltern-Welten; gerechnet von k3c-sim world values) und den Override der Welt.

// ValueEntry ist ein Feld einer Basis-Datei: File ist der Schlüssel in `overrides` (z. B. buildings), Path der Weg darin
// wie im Override (Listen von Objekten je id bzw. Index). Base ist null, wenn die Basis das Feld nicht nennt; Override
// fehlt, wenn die Welt es erbt.
type ValueEntry struct {
	File     string          `json:"file"`
	Path     []string        `json:"path"`
	Base     json.RawMessage `json:"base"`
	Override json.RawMessage `json:"override,omitempty"`
}

// ValuesView sind die Werte der Welt: Eltern-Welt, Nachfolger (alle Welten, die direkt oder über Zwischenwelten von ihr
// erben) und die Felder. Error nennt einen Lese- oder Prüffehler, Warnings Nachfolger, die schon vorher ungültig waren.
type ValuesView struct {
	World    string       `json:"world"`
	Extends  string       `json:"extends"`
	Children []string     `json:"children"`
	Entries  []ValueEntry `json:"entries"`
	Warnings []string     `json:"warnings,omitempty"`
	Error    string       `json:"error"`
}

// valuesReport ist die Antwort von `k3c-sim world` op values.
type valuesReport struct {
	Extends   string                     `json:"extends"`
	Children  []string                   `json:"children"`
	Base      map[string]json.RawMessage `json:"base"`
	Overrides map[string]json.RawMessage `json:"overrides"`
}

// Values liefert die Werte der Welt id für die Ansicht „Werte“.
func (e Editor) Values(ctx context.Context, id string) ValuesView {
	view := ValuesView{World: id, Children: []string{}, Entries: []ValueEntry{}}
	var r valuesReport
	err := checkID(id)
	if err == nil {
		err = e.call(ctx, map[string]any{"op": "values", "world": id}, &r)
	}
	if err != nil {
		view.Error = err.Error()
		return view
	}
	view.Extends = r.Extends
	view.Children = append(view.Children, r.Children...)
	view.Entries = valueEntries(r.Base, r.Overrides)
	return view
}

// valueEntries: je Basis-Datei (sortiert) ihre Felder in der Reihenfolge der Datei mit dem Override der Welt, danach
// übersteuerte Felder, die die Basis nicht nennt.
func valueEntries(base, overrides map[string]json.RawMessage) []ValueEntry {
	out := []ValueEntry{}
	for _, f := range slices.Sorted(maps.Keys(base)) {
		own := map[string]json.RawMessage{}
		flattenJSON(overrides[f], nil, func(p []string, v json.RawMessage) { own[pathKey(p)] = v })
		flattenJSON(base[f], nil, func(p []string, v json.RawMessage) {
			out = append(out, ValueEntry{File: f, Path: p, Base: v, Override: own[pathKey(p)]})
			delete(own, pathKey(p))
		})
		flattenJSON(overrides[f], nil, func(p []string, v json.RawMessage) {
			if _, left := own[pathKey(p)]; left {
				out = append(out, ValueEntry{File: f, Path: p, Base: json.RawMessage("null"), Override: v})
			}
		})
	}
	return out
}

func pathKey(p []string) string { return strings.Join(p, "\x00") }

// flattenJSON ruft emit für jedes Blatt von raw: Zahl, Text, true/false und Listen von Werten (ganz); Objekte und Listen
// von Objekten (je `id`, sonst Index wie der Merge der Engine) werden durchlaufen. Kommentare (`_…`) entfallen.
func flattenJSON(raw json.RawMessage, path []string, emit func([]string, json.RawMessage)) {
	flattenSkip(raw, path, "", emit)
}

// flattenSkip wie flattenJSON; skip ist ein Schlüssel des Objekts raw, der nicht ausgegeben wird (die `id` eines
// Listeneintrags ist sein Schlüssel im Override, kein Feld).
func flattenSkip(raw json.RawMessage, path []string, skip string, emit func([]string, json.RawMessage)) {
	raw = bytes.TrimSpace(raw)
	if len(raw) == 0 {
		return
	}
	switch raw[0] {
	case '{':
		keys, vals := objectItems(raw)
		for i, k := range keys {
			if !strings.HasPrefix(k, "_") && k != skip {
				flattenJSON(vals[i], append(slices.Clone(path), k), emit)
			}
		}
	case '[':
		flattenList(raw, path, emit)
	default:
		emit(path, raw)
	}
}

// flattenList durchläuft eine Liste von Objekten je `id` (gibt es eine, sind nur Einträge mit `id` ansprechbar), sonst
// je Index; jede andere Liste ist ein Blatt.
func flattenList(raw json.RawMessage, path []string, emit func([]string, json.RawMessage)) {
	var items []json.RawMessage
	if json.Unmarshal(raw, &items) != nil || len(items) == 0 || bytes.TrimSpace(items[0])[0] != '{' {
		emit(path, compactJSON(raw))
		return
	}
	ids := make([]*string, len(items))
	byID := false
	for i, it := range items {
		ids[i] = itemID(it)
		byID = byID || ids[i] != nil
	}
	for i, it := range items {
		switch {
		case byID && ids[i] != nil:
			flattenSkip(it, append(slices.Clone(path), *ids[i]), "id", emit)
		case !byID:
			flattenSkip(it, append(slices.Clone(path), strconv.Itoa(i)), "", emit)
		}
	}
}

// itemID ist die `id` (Text) eines Listeneintrags, sonst nil.
func itemID(raw json.RawMessage) *string {
	var head struct {
		ID any `json:"id"`
	}
	if json.Unmarshal(raw, &head) != nil {
		return nil
	}
	id, ok := head.ID.(string)
	if !ok {
		return nil
	}
	return &id
}

// objectItems liefert die Schlüssel und Werte eines JSON-Objekts in der Reihenfolge der Datei.
func objectItems(raw []byte) ([]string, []json.RawMessage) {
	dec := json.NewDecoder(bytes.NewReader(raw))
	if tok, err := dec.Token(); err != nil || tok != json.Delim('{') {
		return nil, nil
	}
	var keys []string
	var vals []json.RawMessage
	for dec.More() {
		tok, err := dec.Token()
		if err != nil {
			return keys, vals
		}
		var v json.RawMessage
		if err := dec.Decode(&v); err != nil {
			return keys, vals
		}
		keys, vals = append(keys, tok.(string)), append(vals, v)
	}
	return keys, vals
}
