package backend

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"slices"
	"strings"

	"k3c/engine/sim"
)

// Werte-Overrides per Chat (WZ37.2, B-683, docs/rules/welten.md § 1): lesen, setzen und entfernen je Feld einer
// genannten Welt. Die Karte zeigt Basiswert → neuer Wert; geprüft wird wie im Editor und im Server (patchOverrides,
// sim.ParseWorld, checkWorldValues samt Kind-Welten), geschrieben nur auf dem Stand der Prüfung und nur das genannte Feld.

const overrideReadMax = 40 // Felder je Lese-Karte

func overrideActions() []actionSpec {
	return []actionSpec{
		{"override_lesen", "welt, datei? (z. B. buildings), pfad? (z. B. wall.cost.wood)", "Overrides einer Welt mit Basiswert zeigen (ohne pfad alle Overrides der Datei bzw. Welt)", prepareOverrideRead},
		{"override_setzen", "welt, datei, pfad, wert (JSON im Typ der Basis)", "Ein Override-Feld einer Welt setzen (Vorschau Basiswert → neuer Wert)", prepareOverrideSet},
		{"override_entfernen", "welt, datei, pfad", "Ein Override-Feld einer Welt entfernen (erbt wieder den Basiswert)", prepareOverrideReset},
	}
}

// overrideTarget ist das genannte Feld: Welt, Basis-Datei (Schlüssel in `overrides`) und Pfad mit Punkten.
type overrideTarget struct {
	World string `json:"welt"`
	File  string `json:"datei"`
	Path  string `json:"pfad"`
}

func (t overrideTarget) label() string { return t.World + " › " + t.File + " › " + t.Path }

// overrideState ist eine Welt zur Zeit der Prüfung: Inhalt, `overrides` in der Reihenfolge der Datei, Felder und Kinder.
type overrideState struct {
	id        string
	raw       []byte
	overrides json.RawMessage
	entries   []WorldValueEntry
	children  []string
}

// loadOverrideState liest die Welt id; nur eine gültige Welt (Prüfung des Ladens) wird bearbeitet.
func (a *App) loadOverrideState(id string) (overrideState, error) {
	if err := need("welt", id); err != nil {
		return overrideState{}, err
	}
	if err := knownWorld(a, id); err != nil {
		return overrideState{}, err
	}
	raw, err := os.ReadFile(a.worldFile(id))
	if err != nil {
		return overrideState{}, err
	}
	def, err := sim.ParseWorld(raw, id)
	if err != nil {
		return overrideState{}, err
	}
	base, err := sim.ResolveWorldFiles(a.baseData(), a.loadWorldDef, def.Extends)
	if err != nil {
		return overrideState{}, err
	}
	st := overrideState{id: id, raw: raw, entries: valueEntries(base, def.Overrides),
		children: sim.WorldChildren(a.allWorldDefs(), id)}
	if keys, vals := objectItems(raw); slices.Contains(keys, "overrides") {
		st.overrides = vals[slices.Index(keys, "overrides")]
	}
	return st, nil
}

// entry ist das Feld file › path (ohne Eintrag: Basis nennt es nicht, kein Override).
func (st overrideState) entry(file string, path []string) WorldValueEntry {
	for _, e := range st.entries {
		if e.File == file && slices.Equal(e.Path, path) {
			return e
		}
	}
	return WorldValueEntry{File: file, Path: path, Base: json.RawMessage("null")}
}

// isLeaf: file › path ist ein Feld des Wertesatzes (Blatt aus Basis oder Override), kein Teilpfad darüber.
func (st overrideState) isLeaf(file string, path []string) bool {
	return slices.ContainsFunc(st.entries, func(e WorldValueEntry) bool {
		return e.File == file && slices.Equal(e.Path, path)
	})
}

// splitPath zerlegt „wall.cost.wood“; leere Teile sind ein Fehler.
func splitPath(p string) ([]string, error) {
	parts := strings.Split(strings.TrimSpace(p), ".")
	if slices.Contains(parts, "") {
		return nil, fmt.Errorf("pfad %q ungültig (Teile mit Punkt getrennt, z. B. wall.cost.wood)", p)
	}
	return parts, nil
}

// prepareTarget liest die Welt und prüft Datei und Pfad des Feldes.
func (a *App) prepareTarget(t overrideTarget) (overrideState, []string, error) {
	st, err := a.loadOverrideState(t.World)
	if err != nil {
		return st, nil, err
	}
	if err := need("datei", t.File); err != nil {
		return st, nil, err
	}
	path, err := splitPath(t.Path)
	return st, path, err
}

func prepareOverrideSet(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		overrideTarget
		Value json.RawMessage `json:"wert"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Override setzen: " + f.label()}
	st, path, err := a.prepareTarget(f.overrideTarget)
	if err != nil {
		return p, err
	}
	if !given(f.Value) {
		return p, errors.New(`feld "wert" fehlt (null ist kein Wert; zurücksetzen mit override_entfernen)`)
	}
	return a.planOverride(p, st, f.File, path, compactJSON(f.Value))
}

func prepareOverrideReset(a *App, raw json.RawMessage) (actionPlan, error) {
	var f overrideTarget
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Override entfernen: " + f.label()}
	st, path, err := a.prepareTarget(f)
	if err != nil {
		return p, err
	}
	if st.entry(f.File, path).Override == nil {
		return p, fmt.Errorf("kein Override an %s: das Feld erbt schon den Basiswert", f.label())
	}
	return a.planOverride(p, st, f.File, path, nil)
}

// planOverride bereitet das Setzen (value) bzw. Entfernen (nil) des Feldes vor: Vorschau, Prüfung wie Editor und Server,
// Lauf auf dem geprüften Stand. Ein Wert gleich dem Basiswert ist kein Override (wie im Editor).
func (a *App) planOverride(p actionPlan, st overrideState, file string, path []string, value json.RawMessage) (actionPlan, error) {
	if !st.isLeaf(file, path) {
		return p, fmt.Errorf("%s › %s ist kein Feld des Wertesatzes (Blatt, z. B. wall.cost.wood): ein Teilpfad würde Geschwister-Overrides löschen", st.id, strings.Join(path, "."))
	}
	e := st.entry(file, path)
	shown := value
	if value == nil || sameJSON(value, e.Base) {
		value, shown = nil, e.Base
	}
	field := file + " › " + strings.Join(path, ".")
	p.preview = []ActionPreview{{Field: field, Base: valueText(e.Base), Old: valueText(e.Override), New: valueText(shown)}}
	p.details = []string{fmt.Sprintf("Welt %s: %s, Basis %s → neu %s", st.id, field, valueText(e.Base), valueText(shown))}
	if len(st.children) > 0 {
		p.details = append(p.details, "wirkt auch auf die Kind-Welten: "+strings.Join(st.children, ", "))
	}
	ov := withLeaf(st.overrides, append([]string{file}, path...), value)
	if ov == nil {
		ov = json.RawMessage("{}")
	}
	out, err := a.overridesFile(st.raw, st.id, ov)
	if err != nil {
		return p, err
	}
	if bytes.Equal(out, st.raw) {
		return p, fmt.Errorf("keine Änderung an %s › %s", st.id, field)
	}
	stand := planStand(st.raw)
	p.run = func() (string, error) { return a.saveOverrides(st.id, ov, stand) }
	return p, nil
}

// overridesFile setzt `overrides` der Welt-Datei und prüft wie draftFile (Laden, Werte der Welt und ihrer Kinder).
func (a *App) overridesFile(raw []byte, id string, ov json.RawMessage) ([]byte, error) {
	out, err := patchOverrides(raw, ov)
	if err != nil {
		return nil, err
	}
	def, err := sim.ParseWorld(out, id)
	if err == nil {
		err = a.checkWorldValues(def)
	}
	return out, err
}

// saveOverrides schreibt die Overrides der Welt id, nur auf dem Stand stand (sonst nichts) und nur gültig, atomar.
func (a *App) saveOverrides(id string, ov json.RawMessage, stand string) (string, error) {
	a.save.Lock()
	defer a.save.Unlock()
	path := a.worldFile(id)
	raw, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	if planStand(raw) != stand {
		return "", errors.New(staleError(id))
	}
	out, err := a.overridesFile(raw, id, ov)
	if err != nil {
		return "", err
	}
	if err := writeAtomic(path, out); err != nil {
		return "", err
	}
	return strings.TrimSpace("gespeichert: Welt " + id + ". " + planRegHint), nil
}

// withLeaf liefert das Objekt node mit value am Pfad (nil entfernt das Blatt); leere Objekte entfallen (nil), die
// Reihenfolge der übrigen Schlüssel bleibt.
func withLeaf(node json.RawMessage, path []string, value json.RawMessage) json.RawMessage {
	if len(path) == 0 {
		return value
	}
	keys, vals := objectItems(node)
	i := slices.Index(keys, path[0])
	var child json.RawMessage
	if i >= 0 {
		child = vals[i]
	}
	next := withLeaf(child, path[1:], value)
	switch {
	case i >= 0 && next == nil:
		keys, vals = slices.Delete(keys, i, i+1), slices.Delete(vals, i, i+1)
	case i >= 0:
		vals[i] = next
	case next != nil:
		keys, vals = append(keys, path[0]), append(vals, next)
	}
	if len(keys) == 0 {
		return nil
	}
	var b bytes.Buffer
	b.WriteByte('{')
	for j, k := range keys {
		if j > 0 {
			b.WriteByte(',')
		}
		b.Write(jsonText(k))
		b.WriteByte(':')
		b.Write(vals[j])
	}
	b.WriteByte('}')
	return b.Bytes()
}

// valueText zeigt einen Wert der Vorschau: ohne Wert „–“.
func valueText(v json.RawMessage) string {
	if !given(v) {
		return "–"
	}
	return string(compactJSON(v))
}

func prepareOverrideRead(a *App, raw json.RawMessage) (actionPlan, error) {
	var f overrideTarget
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Overrides lesen: " + f.World}
	st, err := a.loadOverrideState(f.World)
	if err != nil {
		return p, err
	}
	var path []string
	if f.Path != "" {
		if path, err = splitPath(f.Path); err != nil {
			return p, err
		}
	}
	for _, e := range st.entries {
		if (f.File == "" || e.File == f.File) && ((path == nil && e.Override != nil) || slices.Equal(e.Path, path)) {
			p.preview = append(p.preview, readPreview(e))
		}
	}
	if path != nil && len(p.preview) == 0 {
		return p, fmt.Errorf("unbekanntes Feld %s", f.label())
	}
	p.details = []string{fmt.Sprintf("Welt %s: %d Felder", st.id, len(p.preview))}
	if len(p.preview) > overrideReadMax {
		p.preview, p.details[0] = p.preview[:overrideReadMax], p.details[0]+fmt.Sprintf(" (die ersten %d)", overrideReadMax)
	}
	if len(p.preview) == 0 {
		p.details = append(p.details, "keine Overrides: alle Werte geerbt")
	}
	summary := strings.Join(p.details, "; ")
	p.run = func() (string, error) { return summary, nil }
	return p, nil
}

func readPreview(e WorldValueEntry) ActionPreview {
	pv := ActionPreview{Field: e.File + " › " + strings.Join(e.Path, "."), Base: valueText(e.Base), New: "geerbt"}
	if e.Override != nil {
		pv.New = valueText(e.Override)
	}
	return pv
}
