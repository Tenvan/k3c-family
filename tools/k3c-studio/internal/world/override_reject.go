package world

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"maps"
	"slices"
	"strings"
)

// Ablehnung eines Overrides (B-986): Die Meldung nennt, was an der Stelle erlaubt ist. Die Dateien und Felder stammen
// aus der Basis der Engine (`values`), gesperrte Felder aus deren Prüfung (welten.md § 1.3); keine zweite Liste.

// maxFieldsShown ist die Zahl der Felder einer Ebene in einer Meldung; mehr werden mit `…` abgekürzt.
const maxFieldsShown = 12

// rejectOverride erklärt, warum das Feld base › path kein Blatt der Ansicht „Werte“ ist: unbekannte Datei (mit den
// erlaubten Dateien), Objekt oder Liste statt Blatt (gesperrt, wenn die Engine es so meldet, sonst mit den Feldern der
// Ebene) oder unbekanntes Feld (mit den Feldern der Ebene).
func (e Editor) rejectOverride(ctx context.Context, id string, entries []ValueEntry, base string, path []string) error {
	var files []string
	for _, v := range entries {
		if !slices.Contains(files, v.File) {
			files = append(files, v.File)
		}
	}
	if !slices.Contains(files, base) {
		list := strings.Join(files, ", ")
		if engine := e.probeFiles(ctx, id, base); engine != "" {
			list = engine // die Engine kennt auch Dateien ohne Blatt in der Ansicht (welt-events, RU40.4)
		}
		return fmt.Errorf("unbekannte Datei %q (erlaubt: %s)", base, list)
	}
	if len(path) == 0 {
		return fmt.Errorf("der Pfad fehlt (Felder von %s: %s)", base, shown(levelFields(entries, base, nil)))
	}
	// Gesperrt kann ein Feld auch unterhalb der Stelle sein, die die Ansicht kennt: erst die Engine fragen.
	if err := e.probeLocked(ctx, id, base, path); err != nil {
		return err
	}
	where := base + " › " + strings.Join(path, " › ")
	if below := levelFields(entries, base, path); len(below) > 0 {
		return fmt.Errorf("%s ist ein Objekt oder eine Liste, übersteuerbar sind nur die Felder darunter (%s)", where, shown(below))
	}
	for k := len(path) - 1; k >= 0; k-- {
		if fields := levelFields(entries, base, path[:k]); len(fields) > 0 {
			at := base
			if k > 0 {
				at += " › " + strings.Join(path[:k], " › ")
			}
			return fmt.Errorf("unbekanntes Feld %q in %s (Felder: %s)", path[k], at, shown(fields))
		}
	}
	return fmt.Errorf("unbekanntes Feld %q in %s", strings.Join(path, " › "), base)
}

// probeLocked lässt die Engine einen Entwurf mit leerem Objekt am Pfad prüfen; nur die Ablehnung „nicht übersteuerbar“
// (welten.md § 1.3) kommt zurück, andere Fehler des Entwurfs sind hier ohne Belang.
func (e Editor) probeLocked(ctx context.Context, id, base string, path []string) error {
	if len(path) > 1 {
		if err := e.probeLocked(ctx, id, base, path[:1]); err != nil {
			return err
		}
	}
	raw, out, err := e.draftOverride(id, base, path, json.RawMessage("{}"), nil)
	if err != nil || bytes.Equal(raw, out) {
		return nil
	}
	if _, err := e.checkDraft(ctx, id, out); err != nil && strings.Contains(err.Error(), "nicht übersteuerbar") {
		return err
	}
	return nil
}

// probeFiles lässt die Engine einen Entwurf mit leerem Override der Datei base prüfen und liefert die Liste aus ihrer
// Ablehnung „(erlaubt: …)“; leer, wenn die Engine keine solche Liste nennt.
func (e Editor) probeFiles(ctx context.Context, id, base string) string {
	raw, err := e.read(id)
	if err != nil {
		return ""
	}
	var head struct {
		Overrides map[string]json.RawMessage `json:"overrides"`
	}
	if json.Unmarshal(raw, &head) != nil {
		return ""
	}
	ov := maps.Clone(head.Overrides)
	if ov == nil {
		ov = map[string]json.RawMessage{}
	}
	ov[base] = json.RawMessage("{}")
	out, err := patchOverrides(raw, ov)
	if err != nil {
		return ""
	}
	_, err = e.checkDraft(ctx, id, out)
	if err == nil {
		return ""
	}
	_, list, ok := strings.Cut(err.Error(), "(erlaubt: ")
	if !ok {
		return ""
	}
	list, _, _ = strings.Cut(list, ")")
	return strings.TrimSpace(list)
}

// levelFields sind die Namen unter dem Pfad prefix der Datei file in der Reihenfolge der Datei, ohne Wiederholung.
func levelFields(entries []ValueEntry, file string, prefix []string) []string {
	var out []string
	for _, v := range entries {
		if v.File != file || len(v.Path) <= len(prefix) || !slices.Equal(v.Path[:len(prefix)], prefix) {
			continue
		}
		if name := v.Path[len(prefix)]; !slices.Contains(out, name) {
			out = append(out, name)
		}
	}
	return out
}

// shown verbindet höchstens maxFieldsShown Namen, danach `…`.
func shown(names []string) string {
	if len(names) > maxFieldsShown {
		return strings.Join(names[:maxFieldsShown], ", ") + ", …"
	}
	return strings.Join(names, ", ")
}
