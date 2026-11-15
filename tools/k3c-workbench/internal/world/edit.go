package world

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

// Welten anlegen und bearbeiten (B-954, MCP-Tools world_* und „Welt anlegen“ der Seite). Geschrieben wird nur die
// Welt-Datei unter data/worlds/, atomar und format-stabil: Nur die geänderten Werte bekommen neue Bytes (Stil der
// Datei: ein Wert eine Zeile, `inline`). Vor dem Schreiben prüft k3c-sim den Entwurf wie das Laden im Spiel
// (sim.ParseWorld, Werte aller Nachfolger); ungültig bleibt die Datei, wie sie ist, und der Fehler nennt den Grund.

// headerKeys sind die Felder des Kopfs, die world_set ändert (Reihenfolge wie in der Datei); `islands` kommt dazu.
var headerKeys = []string{"name", "notes", "extends", "combat"}

// defaultScaling ist die Tiefen-Skalierung einer neuen Insel (wie die Standard-Welt, Insel 1).
const defaultScaling = `{"hp": 1.5, "damage": 1.3, "speed": 1.1}`

// eol gibt s mit den Zeilenenden der Datei zurück (CRLF bleibt CRLF).
func eol(raw []byte, s string) []byte {
	if bytes.Contains(raw, []byte("\r\n")) {
		s = strings.ReplaceAll(s, "\n", "\r\n")
	}
	return []byte(s)
}

// commit lässt den Entwurf out der Welt id prüfen und schreibt ihn atomar; die Antwort sind die Warnungen der Prüfung.
func (e Editor) commit(ctx context.Context, id string, out []byte) ([]string, error) {
	var check struct {
		Warnings []string `json:"warnings"`
	}
	if err := e.call(ctx, map[string]any{"op": "check", "world": id, "draft": json.RawMessage(out)}, &check); err != nil {
		return nil, err
	}
	if err := os.MkdirAll(filepath.Dir(e.file(id)), 0o755); err != nil {
		return nil, err
	}
	return check.Warnings, writeAtomic(e.file(id), out)
}

// Create legt die Welt id an: eine Insel „island1“ (Tiefen 0-4, bis zum Endboss), Kampf groups/5, optional von extends erbend. Eine
// vorhandene Welt wird nie überschrieben.
func (e Editor) Create(ctx context.Context, id, name, extends string) ([]string, error) {
	if err := checkID(id); err != nil {
		return nil, err
	}
	if strings.TrimSpace(name) == "" {
		return nil, errors.New("der Name fehlt")
	}
	writeMu.Lock()
	defer writeMu.Unlock()
	if _, err := os.Stat(e.file(id)); err == nil {
		return nil, fmt.Errorf("%s/%s.json gibt es schon (ändern mit world_set)", worldsDir, id)
	}
	head := fmt.Sprintf("{\n  \"id\": %s,\n  \"name\": %s,\n", quote(id), quote(strings.TrimSpace(name)))
	if extends != "" {
		if err := checkID(extends); err != nil {
			return nil, fmt.Errorf("extends: %w", err)
		}
		head += fmt.Sprintf("  \"extends\": %s,\n", quote(extends))
	}
	head += "  \"combat\": { \"mode\": \"groups\", \"groupSize\": 5 },\n  \"islands\": [\n    " + newIsland("island1", "Insel 1", "[0, 1, 2, 3, 4]", nil) + "\n  ]\n}\n"
	return e.commit(ctx, id, []byte(head))
}

// Set ändert Kopf und Inseln der Welt id nach patch (JSON-Objekt): name, notes, extends, combat (null entfernt das Feld)
// und islands (Liste; je Eintrag Insel per id ändern oder anhängen, siehe patchIslands).
func (e Editor) Set(ctx context.Context, id string, patch json.RawMessage) ([]string, error) {
	var p map[string]json.RawMessage
	if err := json.Unmarshal(patch, &p); err != nil || p == nil {
		return nil, errors.New("patch muss ein JSON-Objekt sein")
	}
	for k := range p {
		if !slices.Contains(headerKeys, k) && k != "islands" {
			return nil, fmt.Errorf("unbekanntes Feld %q im patch (erlaubt: %s, islands)", k, strings.Join(headerKeys, ", "))
		}
	}
	writeMu.Lock()
	defer writeMu.Unlock()
	raw, err := e.read(id)
	if err != nil {
		return nil, err
	}
	out := raw
	for _, k := range headerKeys {
		if v, ok := p[k]; ok {
			if out, err = setField(out, span{0, len(out)}, k, v, true); err != nil {
				return nil, err
			}
		}
	}
	if list, ok := p["islands"]; ok {
		if out, err = e.patchIslands(out, id, list); err != nil {
			return nil, err
		}
	}
	return e.commitChange(ctx, id, raw, out)
}

// commitChange schreibt nur eine inhaltliche Änderung (sonst bleibt die Datei, wie sie ist).
func (e Editor) commitChange(ctx context.Context, id string, raw, out []byte) ([]string, error) {
	if bytes.Equal(raw, out) {
		return nil, nil
	}
	return e.commit(ctx, id, out)
}

// Check prüft die Datei der Welt id wie das Laden im Spiel und mit allen Nachfolgern; die Antwort sind die Warnungen.
func (e Editor) Check(ctx context.Context, id string) ([]string, error) {
	raw, err := e.read(id)
	if err != nil {
		return nil, err
	}
	var check struct {
		Warnings []string `json:"warnings"`
	}
	if err := e.call(ctx, map[string]any{"op": "check", "world": id, "draft": json.RawMessage(raw)}, &check); err != nil {
		return nil, err
	}
	return check.Warnings, nil
}

// Raw ist der Text der Welt-Datei.
func (e Editor) Raw(id string) (string, error) {
	raw, err := e.read(id)
	return string(raw), err
}

// setField setzt key im Objekt obj (null oder leer entfernt es). Ein neues Feld der obersten Ebene (top) steht vor
// `islands`, in einer Insel am Ende der Zeile; ein inhaltlich gleicher Wert lässt die Bytes, wie sie sind.
func setField(raw []byte, obj span, key string, v json.RawMessage, top bool) ([]byte, error) {
	at, found, err := valueSpan(raw, obj, key)
	if err != nil {
		return nil, err
	}
	if !given(v) {
		if !found {
			return raw, nil
		}
		return removeKey(raw, at, key), nil
	}
	text, err := inline(v)
	if err != nil {
		return nil, fmt.Errorf("%s: %w", key, err)
	}
	if found && sameJSON(raw[at.from:at.to], v) {
		return raw, nil
	}
	if found || !top {
		return setKey(raw, obj, key, []byte(text))
	}
	isl, ok, err := valueSpan(raw, obj, "islands")
	if err != nil || !ok {
		return setKey(raw, obj, key, []byte(text))
	}
	k := bytes.LastIndex(raw[:isl.from], []byte(`"islands"`))
	return slices.Concat(raw[:k], eol(raw, fmt.Sprintf("%s: %s,\n  ", quote(key), text)), raw[k:]), nil
}

// CreateView ist die Antwort von „Welt anlegen“ der Seite: ID der neuen Welt, Warnungen der Prüfung oder der Fehler.
type CreateView struct {
	ID       string   `json:"id"`
	Warnings []string `json:"warnings"`
	Error    string   `json:"error"`
}

// CreateView legt die Welt wie Create an und liefert die Antwort für die Seite (nie ein Go-Fehler).
func (e Editor) CreateView(ctx context.Context, id, name, extends string) CreateView {
	warnings, err := e.Create(ctx, id, name, extends)
	v := CreateView{ID: id, Warnings: warnings}
	if v.Warnings == nil {
		v.Warnings = []string{}
	}
	if err != nil {
		v.Error = err.Error()
	}
	return v
}
