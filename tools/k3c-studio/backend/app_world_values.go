package backend

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/fs"
	"maps"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"strings"

	"k3c/data"
	"k3c/engine/sim"
)

// Werte-Overrides im Welten Editor (WZ37.1, B-682, docs/rules/welten.md § 1): Die Ansicht „Werte“ zeigt je Feld den
// Basiswert (Basis data/, darüber die Overrides der Eltern-Welten, sim.ResolveWorldFiles) und den Override der Welt.
// Geprüft wird wie im Server (sim.ResolveWorldValues) für die Welt und alle Welten, die von ihr erben.

// WorldValueEntry ist ein Feld einer Basis-Datei: File ist der Schlüssel in `overrides` (z. B. buildings), Path der Weg
// darin wie im Override (Listen von Objekten je id bzw. Index). Base ist null, wenn die Basis das Feld nicht nennt;
// Override fehlt, wenn die gespeicherte Welt es erbt.
type WorldValueEntry struct {
	File     string          `json:"file"`
	Path     []string        `json:"path"`
	Base     json.RawMessage `json:"base"`
	Override json.RawMessage `json:"override,omitempty"`
}

// WorldValuesView sind die Werte der gewählten Welt: Eltern-Welt, Kind-Welten (erben direkt von ihr) und die Felder.
type WorldValuesView struct {
	World    string            `json:"world"`
	Extends  string            `json:"extends"`
	Children []string          `json:"children"`
	Entries  []WorldValueEntry `json:"entries"`
	Error    string            `json:"error"`
}

// baseData ist die Basis wie im Server: data/ des Repos, ohne Basis-Dateien (Tests) die eingebetteten.
func (a *App) baseData() fs.FS {
	if _, err := os.Stat(filepath.Join(a.root, "data", "buildings.json")); err == nil {
		return os.DirFS(filepath.Join(a.root, "data"))
	}
	return data.Files
}

// loadWorldDef liest die Welt id von der Platte und prüft sie wie beim Laden.
func (a *App) loadWorldDef(id string) (sim.WorldDef, error) {
	raw, err := os.ReadFile(a.worldFile(id))
	if err != nil {
		return sim.WorldDef{}, fmt.Errorf("data/worlds/%s.json: %w", id, err)
	}
	return sim.ParseWorld(raw, id)
}

// allWorldDefs sind alle lesbaren Welten (eine ungültige fehlt, sie meldet ihr eigener Editor).
func (a *App) allWorldDefs() []sim.WorldDef {
	var out []sim.WorldDef
	for _, w := range a.listWorlds() {
		if def, err := a.loadWorldDef(w.ID); err == nil {
			out = append(out, def)
		}
	}
	return out
}

// PlanWorldValues liefert die Werte der gewählten Welt für die Ansicht „Werte“ (Binding).
func (a *App) PlanWorldValues() WorldValuesView {
	id := a.activeWorld()
	view := WorldValuesView{World: id, Children: []string{}, Entries: []WorldValueEntry{}}
	def, err := a.loadWorldDef(id)
	if err != nil {
		view.Error = err.Error()
		return view
	}
	view.Extends = def.Extends
	view.Children = append(view.Children, sim.WorldChildren(a.allWorldDefs(), id)...)
	base, err := sim.ResolveWorldFiles(a.baseData(), a.loadWorldDef, def.Extends)
	if err != nil {
		view.Error = err.Error()
		return view
	}
	view.Entries = valueEntries(base, def.Overrides)
	return view
}

// valueEntries: je Basis-Datei (sortiert) ihre Felder in der Reihenfolge der Datei mit dem Override der Welt, danach
// übersteuerte Felder, die die Basis nicht nennt.
func valueEntries(base map[string][]byte, overrides map[string]json.RawMessage) []WorldValueEntry {
	out := []WorldValueEntry{}
	for _, file := range slices.Sorted(maps.Keys(base)) {
		own := map[string]json.RawMessage{}
		flattenJSON(overrides[file], nil, func(p []string, v json.RawMessage) { own[pathKey(p)] = v })
		flattenJSON(base[file], nil, func(p []string, v json.RawMessage) {
			out = append(out, WorldValueEntry{File: file, Path: p, Base: v, Override: own[pathKey(p)]})
			delete(own, pathKey(p))
		})
		flattenJSON(overrides[file], nil, func(p []string, v json.RawMessage) {
			if _, left := own[pathKey(p)]; left {
				out = append(out, WorldValueEntry{File: file, Path: p, Base: json.RawMessage("null"), Override: v})
			}
		})
	}
	return out
}

func pathKey(p []string) string { return strings.Join(p, "\x00") }

// flattenJSON ruft emit für jedes Blatt von raw: Zahl, Text, true/false und Listen von Werten (ganz); Objekte und Listen
// von Objekten (je `id`, sonst Index wie der Merge der Engine) werden durchlaufen. Kommentare (`_…`) entfallen.
func flattenJSON(raw json.RawMessage, path []string, emit func([]string, json.RawMessage)) {
	raw = bytes.TrimSpace(raw)
	if len(raw) == 0 {
		return
	}
	switch raw[0] {
	case '{':
		keys, vals := objectItems(raw)
		for i, k := range keys {
			if !strings.HasPrefix(k, "_") {
				flattenJSON(vals[i], append(slices.Clone(path), k), emit)
			}
		}
	case '[':
		var items []json.RawMessage
		if json.Unmarshal(raw, &items) != nil || len(items) == 0 || bytes.TrimSpace(items[0])[0] != '{' {
			emit(path, compactJSON(raw))
			return
		}
		for i, it := range items {
			var head struct {
				ID *string `json:"id"`
			}
			key := strconv.Itoa(i)
			if json.Unmarshal(it, &head) == nil && head.ID != nil {
				key = *head.ID
			}
			flattenJSON(it, append(slices.Clone(path), key), emit)
		}
	default:
		emit(path, raw)
	}
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

// checkWorldValues prüft den Entwurf def wie der Server (sim.ResolveWorldValues): die Welt selbst und jede Welt, die
// über `extends` von ihr erbt (eine Änderung wirkt auf die Kinder, welten.md § 1).
func (a *App) checkWorldValues(def sim.WorldDef) error {
	lookup := func(id string) (sim.WorldDef, error) {
		if id == def.ID {
			return def, nil
		}
		return a.loadWorldDef(id)
	}
	ids := []string{def.ID}
	for _, w := range a.allWorldDefs() {
		if w.ID != def.ID && inherits(w, def.ID, lookup) {
			ids = append(ids, w.ID)
		}
	}
	for _, id := range ids {
		if _, err := sim.ResolveWorldValues(a.baseData(), lookup, id); err != nil {
			return err
		}
	}
	return nil
}

// inherits: w erbt über ihre Kette (höchstens 8 Schritte, Zyklen meldet die Auflösung) von parent.
func inherits(w sim.WorldDef, parent string, lookup func(string) (sim.WorldDef, error)) bool {
	for i := 0; i < 8 && w.Extends != ""; i++ {
		if w.Extends == parent {
			return true
		}
		next, err := lookup(w.Extends)
		if err != nil {
			return false
		}
		w = next
	}
	return false
}

// patchOverrides setzt `overrides` der Welt-Datei auf den Entwurf: fehlt er (null), bleibt die Datei; inhaltlich gleich
// bleibt sie byte-gleich; leer entfällt der Schlüssel.
func patchOverrides(raw []byte, ov json.RawMessage) ([]byte, error) {
	if !given(ov) {
		return raw, nil
	}
	var want map[string]json.RawMessage
	if err := json.Unmarshal(ov, &want); err != nil {
		return nil, fmt.Errorf("overrides: Objekt je Basis-Datei erwartet: %w", err)
	}
	whole := span{0, len(raw)}
	at, found, err := valueSpan(raw, whole, "overrides")
	switch {
	case err != nil:
		return nil, err
	case found && sameJSON(raw[at.from:at.to], ov):
		return raw, nil
	case !found && len(want) == 0:
		return raw, nil
	case len(want) == 0:
		return removeKey(raw, at, "overrides"), nil
	}
	var b bytes.Buffer
	if err := json.Indent(&b, compactJSON(ov), "  ", "  "); err != nil {
		return nil, err
	}
	return setKey(raw, whole, "overrides", b.Bytes())
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
