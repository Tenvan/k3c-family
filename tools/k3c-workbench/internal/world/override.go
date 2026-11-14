package world

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"sync"

	"k3c/engine/store"
)

// writeMu reiht schreibende Aufrufe: zwei gleichzeitige Overrides auf dieselbe Datei verlieren keinen Wert.
var writeMu sync.Mutex

// SetOverride übersteuert das Feld base/path der Welt id mit value (JSON). null oder ein Wert gleich dem Basiswert
// entfernt den Override (das Feld erbt wieder). Nur Felder der Ansicht „Werte“ sind erlaubt. Vor dem Schreiben prüft
// k3c-sim den Entwurf wie der Server (sim.ResolveWorldValues) für die Welt und alle Nachfolger; ungültig bleibt die
// Datei, wie sie ist, und Error nennt den Grund. Die Antwort ist die neue Ansicht.
func (e Editor) SetOverride(ctx context.Context, id, base string, path []string, value json.RawMessage) ValuesView {
	writeMu.Lock()
	defer writeMu.Unlock()
	view := e.Values(ctx, id)
	if view.Error != "" {
		return view
	}
	at := slices.IndexFunc(view.Entries, func(v ValueEntry) bool { return v.File == base && slices.Equal(v.Path, path) })
	if at < 0 {
		view.Error = fmt.Sprintf("das Feld %s › %s gibt es nicht", base, strings.Join(path, "."))
		return view
	}
	warnings, err := e.writeOverride(ctx, id, base, path, value, view.Entries[at].Base)
	if err != nil {
		view.Error = err.Error()
		return view
	}
	next := e.Values(ctx, id)
	next.Warnings = warnings
	return next
}

// writeOverride setzt das Blatt, lässt den Entwurf prüfen und schreibt die Datei nur bei einer inhaltlichen Änderung.
func (e Editor) writeOverride(ctx context.Context, id, base string, path []string, value, baseValue json.RawMessage) ([]string, error) {
	raw, err := e.read(id)
	if err != nil {
		return nil, err
	}
	var head struct {
		Overrides map[string]json.RawMessage `json:"overrides"`
	}
	if err := json.Unmarshal(raw, &head); err != nil {
		return nil, fmt.Errorf("Welt-Datei: %w", err)
	}
	reset := !given(value) || sameJSON(value, baseValue)
	ov, err := withLeaf(head.Overrides, base, path, value, reset)
	if err != nil {
		return nil, err
	}
	out, err := patchOverrides(raw, ov)
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

// given: der Wert ist gesetzt (nicht leer, nicht null).
func given(v json.RawMessage) bool {
	t := bytes.TrimSpace(v)
	return len(t) > 0 && string(t) != "null"
}

// withLeaf liefert die Overrides mit dem Blatt base/path = value (reset entfernt es); leere Objekte entfallen.
func withLeaf(ov map[string]json.RawMessage, base string, path []string, value json.RawMessage, reset bool) (map[string]json.RawMessage, error) {
	if len(path) == 0 {
		return nil, errors.New("der Pfad fehlt")
	}
	tree := map[string]any{}
	if given(ov[base]) {
		if err := decodeNumbers(ov[base], &tree); err != nil {
			return nil, fmt.Errorf("overrides › %s: %w", base, err)
		}
	}
	var leaf any
	if !reset {
		if err := decodeNumbers(value, &leaf); err != nil {
			return nil, fmt.Errorf("der Wert ist kein JSON: %w", err)
		}
	}
	setPath(tree, path, leaf, reset)
	out := maps.Clone(ov)
	if out == nil {
		out = map[string]json.RawMessage{}
	}
	if len(tree) == 0 {
		delete(out, base)
		return out, nil
	}
	text, err := json.Marshal(tree)
	if err != nil {
		return nil, err
	}
	out[base] = text
	return out, nil
}

// decodeNumbers liest JSON und lässt Zahlen, wie sie sind (1.50 bleibt 1.50, große Ganzzahlen bleiben genau).
func decodeNumbers(raw []byte, dst any) error {
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.UseNumber()
	return dec.Decode(dst)
}

// setPath setzt oder entfernt das Blatt am Pfad und räumt leer gewordene Objekte auf.
func setPath(node map[string]any, path []string, leaf any, reset bool) {
	k := path[0]
	if len(path) == 1 {
		if reset {
			delete(node, k)
		} else {
			node[k] = leaf
		}
		return
	}
	child, ok := node[k].(map[string]any)
	if !ok {
		if reset {
			return
		}
		child = map[string]any{}
		node[k] = child
	}
	setPath(child, path[1:], leaf, reset)
	if len(child) == 0 {
		delete(node, k)
	}
}

// writeAtomic schreibt über eine Temp-Datei im selben Ordner, damit nie eine halbe Datei liegt; die Rechte bleiben. Das
// Präfix .tmp- ignoriert der Datei-Watch des Servers (engine/watch), erst das Umbenennen zählt.
func writeAtomic(path string, content []byte) error {
	tmp, err := os.CreateTemp(filepath.Dir(path), ".tmp-world-*")
	if err != nil {
		return err
	}
	if info, err := os.Stat(path); err == nil {
		_ = tmp.Chmod(info.Mode().Perm())
	}
	_, werr := tmp.Write(content)
	cerr := tmp.Close()
	if werr != nil || cerr != nil {
		_ = os.Remove(tmp.Name())
		return fmt.Errorf("schreiben: %v %v", werr, cerr)
	}
	if err := store.Rename(tmp.Name(), path); err != nil {
		_ = os.Remove(tmp.Name())
		return err
	}
	return nil
}
