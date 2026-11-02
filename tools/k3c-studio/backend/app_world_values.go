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

// WorldValuesView sind die Werte der gewählten Welt: Eltern-Welt, Nachfolger (alle Welten, die direkt oder über
// Zwischenwelten von ihr erben; ChildErrors nennt je ungültigem Nachfolger den Grund) und die Felder.
type WorldValuesView struct {
	World       string            `json:"world"`
	Extends     string            `json:"extends"`
	Children    []string          `json:"children"`
	ChildErrors map[string]string `json:"childErrors,omitempty"`
	Entries     []WorldValueEntry `json:"entries"`
	Error       string            `json:"error"`
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

// worldSnapshot ist der Stand aller Welten auf der Platte, einmal gelesen und je Prüfung wiederverwendet: defs die
// ladbaren, errs die, die ParseWorld ablehnt, extends der Elternverweis aller (auch der ungültigen, nur lesend).
type worldSnapshot struct {
	defs    map[string]sim.WorldDef
	errs    map[string]error
	extends map[string]string
}

// snapshotWorlds liest jede Welt genau einmal von der Platte.
func (a *App) snapshotWorlds() worldSnapshot {
	s := worldSnapshot{defs: map[string]sim.WorldDef{}, errs: map[string]error{}, extends: map[string]string{}}
	for _, w := range a.listWorlds() {
		raw, err := os.ReadFile(a.worldFile(w.ID))
		if err != nil {
			s.errs[w.ID] = fmt.Errorf("data/worlds/%s.json: %w", w.ID, err)
			continue
		}
		var head struct {
			Extends string `json:"extends"`
		}
		_ = json.Unmarshal(raw, &head)
		s.extends[w.ID] = head.Extends
		if def, err := sim.ParseWorld(raw, w.ID); err != nil {
			s.errs[w.ID] = err
		} else {
			s.defs[w.ID] = def
		}
	}
	return s
}

// lookup ist der Welten-Zugriff der Auflösung (sim.ResolveWorldValues) auf dem Snapshot.
func (s worldSnapshot) lookup(id string) (sim.WorldDef, error) {
	if def, ok := s.defs[id]; ok {
		return def, nil
	}
	if err, ok := s.errs[id]; ok {
		return sim.WorldDef{}, err
	}
	return sim.WorldDef{}, fmt.Errorf("data/worlds/%s.json: %w", id, fs.ErrNotExist)
}

// descendants sind alle Welten, die direkt oder über Zwischenwelten von id erben, sortiert (Zyklen enden an der Menge).
func (s worldSnapshot) descendants(id string) []string {
	seen := map[string]bool{id: true}
	for grew := true; grew; {
		grew = false
		for w, parent := range s.extends {
			if !seen[w] && seen[parent] {
				seen[w], grew = true, true
			}
		}
	}
	delete(seen, id)
	return slices.Sorted(maps.Keys(seen))
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
	snap := a.snapshotWorlds()
	view.Children = append(view.Children, snap.descendants(id)...)
	for _, kid := range view.Children {
		if bad := snap.errs[kid]; bad != nil {
			if view.ChildErrors == nil {
				view.ChildErrors = map[string]string{}
			}
			view.ChildErrors[kid] = bad.Error()
		}
	}
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
// von Objekten (je `id`, sonst Index wie der Merge der Engine, listIndex) werden durchlaufen. Kommentare (`_…`) entfallen.
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

// flattenList durchläuft eine Liste von Objekten je `id` (gibt es eine, sind nur Einträge mit `id` ansprechbar, wie in
// listIndex der Engine), sonst je Index; jede andere Liste ist ein Blatt.
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

// checkWorldValues prüft den Entwurf def wie der Server (sim.ResolveWorldValues): die Welt selbst und jeden Nachfolger
// (direkt oder über Zwischenwelten, eine Änderung wirkt auf sie, welten.md § 1). Ein Nachfolger blockiert nur, wenn der
// Entwurf ihn neu oder anders ungültig macht; war er mit dem Stand der Platte schon gleich ungültig oder lädt er nicht,
// kommt eine Warnung zurück. Alle Welten werden dafür einmal gelesen (Snapshot).
func (a *App) checkWorldValues(def sim.WorldDef) (warnings []string, err error) {
	snap := a.snapshotWorlds()
	draft := func(id string) (sim.WorldDef, error) {
		if id == def.ID {
			return def, nil
		}
		return snap.lookup(id)
	}
	base := a.baseData()
	if _, err := sim.ResolveWorldValues(base, draft, def.ID); err != nil {
		return nil, err
	}
	for _, kid := range snap.descendants(def.ID) {
		if bad := snap.errs[kid]; bad != nil {
			warnings = append(warnings, fmt.Sprintf("Welt %s erbt von %s, ist aber ungültig und nicht geprüft: %v", kid, def.ID, bad))
			continue
		}
		_, err := sim.ResolveWorldValues(base, draft, kid)
		if err == nil {
			continue
		}
		if _, was := sim.ResolveWorldValues(base, snap.lookup, kid); was != nil && was.Error() == err.Error() {
			warnings = append(warnings, fmt.Sprintf("Welt %s war schon vorher ungültig: %v", kid, err))
			continue
		}
		return warnings, err
	}
	return warnings, nil
}

// patchOverrides setzt `overrides` der Welt-Datei auf den Entwurf: fehlt er (null), bleibt die Datei; inhaltlich gleich
// bleibt sie byte-gleich; leer entfällt der Schlüssel. Ein neuer Schlüssel steht (wie bei setKey) am Ende des Objekts,
// hinter den Inseln; die Zeilenenden folgen der Datei (CRLF bleibt CRLF).
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
	text := b.Bytes()
	if bytes.Contains(raw, []byte("\r\n")) { // Zeilenenden der Datei beibehalten
		text = bytes.ReplaceAll(text, []byte("\n"), []byte("\r\n"))
	}
	return setKey(raw, whole, "overrides", text)
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
