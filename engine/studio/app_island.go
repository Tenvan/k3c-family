package studio

import (
	"bytes"
	"encoding/json"
	"fmt"
	"path/filepath"
)

// Inseln anlegen und entfernen (WZ18-Vorlauf, Welten Editor): erweitert die Liste `islands` in data/islands.json
// format-stabil (nur die Liste ändert sich), atomar und auf dem Stand aus PlanLoad. Eine neue Insel ist eine Kopie der
// letzten (Tiefen, Gegner-Skalierung) ohne Planung; die erste Insel bleibt immer.

// islandDef sind die Felder einer Insel, die beim Anlegen kopiert werden.
type islandDef struct {
	ID      string          `json:"id"`
	Depths  []int           `json:"depths"`
	Scaling json.RawMessage `json:"depthScaling"`
}

// islandList liest Bereich und Inhalt der Liste `islands`.
func islandList(raw []byte) (span, []islandDef, error) {
	list, ok, err := valueSpan(raw, span{0, len(raw)}, "islands")
	if err != nil || !ok {
		return span{}, nil, fmt.Errorf("%s: kein Feld islands (%v)", islandsPath, err)
	}
	var defs []islandDef
	if err := json.Unmarshal(raw[list.from:list.to], &defs); err != nil {
		return span{}, nil, fmt.Errorf("%s: %w", islandsPath, err)
	}
	return list, defs, nil
}

// addIsland hängt eine Insel als Kopie der letzten an die Liste an.
func addIsland(raw []byte) ([]byte, error) {
	list, defs, err := islandList(raw)
	if err != nil || len(defs) == 0 {
		return nil, fmt.Errorf("%s: keine Insel zum Kopieren (%v)", islandsPath, err)
	}
	last := defs[len(defs)-1]
	n := len(defs) + 1
	depths, _ := json.Marshal(last.Depths)
	entry := fmt.Sprintf(`{ "id": "island%d", "name": "Insel %d", "depths": %s, "depthScaling": %s, "plannedChunks": {} }`, n, n, bytes.TrimSpace(depths), bytes.TrimSpace(last.Scaling))
	at := list.to - 1 // vor der schließenden Klammer der Liste
	for at > list.from && bytes.ContainsRune([]byte(" \t\r\n"), rune(raw[at-1])) {
		at--
	}
	out := append(append(append([]byte{}, raw[:at]...), []byte(",\n    "+entry)...), raw[at:]...)
	if _, after, err := islandList(out); err != nil || len(after) != n {
		return nil, fmt.Errorf("%s: Insel nicht angelegt (%v)", islandsPath, err)
	}
	return out, nil
}

// removeIsland entfernt die Insel id samt dem Komma davor; die erste Insel bleibt.
func removeIsland(raw []byte, id string) ([]byte, error) {
	_, defs, err := islandList(raw)
	if err != nil {
		return nil, err
	}
	if len(defs) > 0 && defs[0].ID == id {
		return nil, fmt.Errorf("die erste Insel bleibt")
	}
	at, err := islandSpan(raw, id)
	if err != nil {
		return nil, err
	}
	from := at.from
	for from > 0 && raw[from-1] != ',' {
		from--
	}
	out := append(append([]byte{}, raw[:from-1]...), raw[at.to:]...)
	if _, after, err := islandList(out); err != nil || len(after) != len(defs)-1 {
		return nil, fmt.Errorf("%s: Insel nicht entfernt (%v)", islandsPath, err)
	}
	return out, nil
}

// planEdit liest islands.json, prüft den Stand, ändert den Inhalt mit edit und schreibt ihn atomar.
func (a *App) planEdit(stand string, edit func([]byte) ([]byte, error)) PlanSaved {
	raw, err := a.readIslands()
	if err != nil {
		return PlanSaved{Error: err.Error()}
	}
	if planStand(raw) != stand {
		return PlanSaved{Error: errPlanStale.Error(), Stale: true}
	}
	out, err := edit(raw)
	if err != nil {
		return PlanSaved{Error: err.Error()}
	}
	if err := writeAtomic(filepath.Join(a.root, islandsPath), out); err != nil {
		return PlanSaved{Error: err.Error()}
	}
	return PlanSaved{Stand: planStand(out), Hint: planRegHint}
}

// PlanIslandAdd legt eine weitere Insel an (Binding), nur auf dem Stand aus PlanLoad.
func (a *App) PlanIslandAdd(stand string) PlanSaved { return a.planEdit(stand, addIsland) }

// PlanIslandRemove entfernt die Insel id (Binding); die erste Insel bleibt, nur auf dem Stand aus PlanLoad.
func (a *App) PlanIslandRemove(id, stand string) PlanSaved {
	return a.planEdit(stand, func(raw []byte) ([]byte, error) { return removeIsland(raw, id) })
}
