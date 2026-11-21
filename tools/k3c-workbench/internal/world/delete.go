package world

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

// Delete löscht die Datei der Welt id (WZ62.6, B-1172). Abgelehnt werden die festen Welten (Standard und jede Welt in
// data/paper.json › weltbilder), eine fehlende Datei und Welten mit Erben (andere Welten, deren `extends` auf id zeigt;
// die Meldung nennt sie). Die Datei wird endgültig gelöscht, sie liegt in Git. Spielstände unter saves/, die die Welt
// nutzen, bleiben unberührt; ihre Namen stehen als Warnung in der Antwort.
func (e Editor) Delete(id string) ([]string, error) {
	if err := checkID(id); err != nil {
		return nil, err
	}
	if id == standardID || slices.Contains(e.fixedIDs(), id) {
		return nil, fmt.Errorf("die Welt %q ist eine feste Welt und lässt sich nicht löschen", id)
	}
	// ohne lesbare paper.json ist nicht klar, welche Welten fest sind: lieber nichts löschen
	if _, err := os.Stat(filepath.Join(e.Root, "data", "paper.json")); err != nil {
		return nil, fmt.Errorf("data/paper.json fehlt: die festen Welten sind unbekannt, nichts gelöscht")
	}
	writeMu.Lock()
	defer writeMu.Unlock()
	if _, err := os.Stat(e.file(id)); err != nil {
		return nil, fmt.Errorf("%s/%s.json gibt es nicht", worldsDir, id)
	}
	if heirs := e.heirs(id); len(heirs) > 0 {
		return nil, fmt.Errorf("die Welt %q hat Erben (%s); erst deren extends ändern oder sie löschen", id, strings.Join(heirs, ", "))
	}
	if err := os.Remove(e.file(id)); err != nil {
		return nil, fmt.Errorf("%s/%s.json: %w", worldsDir, id, err)
	}
	var warnings []string
	if saves := e.savesUsing(id); len(saves) > 0 {
		warnings = append(warnings, fmt.Sprintf("Spielstände nutzen die gelöschte Welt und lassen sich nicht mehr laden: %s", strings.Join(saves, ", ")))
	}
	return warnings, nil
}

// heirs liefert die Welten (IDs, sortiert), deren `extends` auf id zeigt.
func (e Editor) heirs(id string) []string {
	var out []string
	for _, w := range e.List() {
		if w.Missing || w.ID == id {
			continue
		}
		raw, err := os.ReadFile(e.file(w.ID))
		if err != nil {
			continue
		}
		var head struct {
			Extends string `json:"extends"`
		}
		if json.Unmarshal(raw, &head) == nil && head.Extends == id {
			out = append(out, w.ID)
		}
	}
	return out
}

// savesUsing liefert die Namen der Spielstände unter saves/, deren Feld `world` id ist (Stände ohne Feld gelten als standard).
func (e Editor) savesUsing(id string) []string {
	files, _ := filepath.Glob(filepath.Join(e.Root, "saves", "*.json"))
	var out []string
	for _, f := range files {
		raw, err := os.ReadFile(f)
		if err != nil {
			continue
		}
		var head struct {
			World string `json:"world"`
		}
		if json.Unmarshal(raw, &head) == nil && head.World == id {
			out = append(out, strings.TrimSuffix(filepath.Base(f), ".json"))
		}
	}
	slices.Sort(out)
	return out
}

// DeleteView löscht wie Delete und liefert die Antwort für die Seite (nie ein Go-Fehler).
func (e Editor) DeleteView(id string) CreateView {
	warnings, err := e.Delete(id)
	v := CreateView{ID: id, Warnings: warnings}
	if v.Warnings == nil {
		v.Warnings = []string{}
	}
	if err != nil {
		v.Error = err.Error()
	}
	return v
}

