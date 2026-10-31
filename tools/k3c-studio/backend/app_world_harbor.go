package backend

import (
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"slices"
	"strings"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// Hafen und vollständige Fehlerliste einer Welt (B-686): Das Studio ergänzt einen fehlenden Hafen-Chunk selbst
// (geplante-chunks.md § 10: jede Insel außer der letzten plant in Tiefe 0 einen `harbor`), deterministisch auf dem
// äußersten freien Platz links, den die Prüfung der Planung annimmt. Die Prüfung des Ladens (sim.ParseWorld) meldet nur
// den ersten Fehler; worldProblems sammelt alle über Proben je Insel mit derselben Prüfung.

// harborDepth ist die Tiefe des Hafens (geplante-chunks.md § 10, sim.hasHarbor).
const harborDepth = 0

// harborSlotsMax begrenzt die Suche nach einem Platz (weiter reicht keine Inselgröße).
const harborSlotsMax = 96

func hasHarborIn(plan []level.PlannedChunk) bool {
	return slices.ContainsFunc(plan, func(c level.PlannedChunk) bool { return c.Kind == level.HarborKind })
}

// harborSlot ist der äußerste Platz links, auf dem ein Hafen zur Planung plan der Tiefe 0 mit der Anzahl count passt.
func harborSlot(plan []level.PlannedChunk, count int) (int, bool) {
	b, err := biomeIDForDepth(harborDepth)
	if err != nil {
		return 0, false
	}
	for k := harborSlotsMax; k >= 1; k-- {
		cand := append(slices.Clone(plan), level.PlannedChunk{Slot: -k, Kind: level.HarborKind})
		if level.ValidatePlanFor(b, cand, count) == nil {
			return -k, true
		}
	}
	return 0, false
}

// withHarbor ergänzt den Hafen in plan (Tiefe 0) der Insel name, falls er fehlt; note nennt den Platz ("" = nichts ergänzt).
func withHarbor(name string, plan []level.PlannedChunk, count int) ([]level.PlannedChunk, string) {
	if hasHarborIn(plan) {
		return plan, ""
	}
	slot, ok := harborSlot(plan, count)
	if !ok {
		return plan, ""
	}
	return append(slices.Clone(plan), level.PlannedChunk{Slot: slot, Kind: level.HarborKind}),
		fmt.Sprintf("Hafen ergänzt: Insel %s, Tiefe 0, Platz %+d (geplante-chunks.md § 10)", name, slot)
}

// draftPlan liefert Planung und Chunk-Anzahlen einer Insel des Entwurfs: aus dem Entwurf, sonst aus der Datei (From).
func draftPlan(isl IslandDraft, file []PlanIsland) (map[string][]level.PlannedChunk, map[string]int, bool) {
	planned, counts := map[string][]level.PlannedChunk{}, map[string]int{}
	at := slices.IndexFunc(file, func(f PlanIsland) bool { return isl.From != "" && f.ID == isl.From })
	if at >= 0 {
		planned, counts = maps.Clone(file[at].Planned), maps.Clone(file[at].ChunkCounts)
	}
	if given(isl.PlannedChunks) {
		planned = map[string][]level.PlannedChunk{}
		if json.Unmarshal(isl.PlannedChunks, &planned) != nil {
			return nil, nil, false
		}
	}
	if given(isl.ChunkCounts) {
		counts = map[string]int{}
		if json.Unmarshal(isl.ChunkCounts, &counts) != nil {
			return nil, nil, false
		}
	}
	if planned == nil {
		planned = map[string][]level.PlannedChunk{}
	}
	return planned, counts, true
}

// withHarbors ergänzt in jeder Insel außer der letzten, die Tiefe 0 hat, einen fehlenden Hafen; notes nennt die Ergänzungen.
func (a *App) withHarbors(d WorldDraft) (WorldDraft, []string) {
	if len(d.Islands) < 2 {
		return d, nil
	}
	file := a.PlanLoad().Islands
	d.Islands = slices.Clone(d.Islands)
	var notes []string
	for i := range d.Islands[:len(d.Islands)-1] {
		isl := &d.Islands[i]
		if !slices.Contains(isl.Depths, harborDepth) {
			continue
		}
		planned, counts, ok := draftPlan(*isl, file)
		if !ok {
			continue
		}
		plan, note := withHarbor(isl.ID, planned["0"], counts["0"])
		if note == "" {
			continue
		}
		planned["0"] = plan
		isl.PlannedChunks = jsonText(planned)
		notes = append(notes, note)
	}
	return d, notes
}

// chunksHarbor ergänzt in der Planung chunks der Tiefe 0 einer Insel, die nicht die letzte ist, einen fehlenden Hafen.
func (a *App) chunksHarbor(view PlanView, island string, depth int, chunks *[]level.PlannedChunk, count int) string {
	at := slices.IndexFunc(view.Islands, func(i PlanIsland) bool { return i.ID == island })
	if depth != harborDepth || at < 0 || at == len(view.Islands)-1 {
		return ""
	}
	plan, note := withHarbor(island, *chunks, count)
	*chunks = plan
	return note
}

// planWorldError: Fehler der Prüfung des Ladens für die Welt-Datei, wie sie nach dem Speichern der Planung wäre (nil = gültig).
func (a *App) planWorldError(island string, depth int, chunks []level.PlannedChunk, count int) error {
	id := a.activeWorld()
	raw, err := a.readIslands()
	if err != nil {
		return err
	}
	out, err := patchPlanned(raw, island, depth, chunks)
	if err == nil {
		out, err = patchCounts(out, island, depth, count)
	}
	if err != nil {
		return err
	}
	if list := worldProblems(out, id); len(list) > 0 {
		return errors.New(strings.Join(list, "; "))
	}
	return nil
}

// worldProblems liefert alle Fehler der Prüfung des Ladens für die Welt-Datei file der Welt id (leer = gültig).
func worldProblems(file []byte, id string) []string {
	_, err := sim.ParseWorld(file, id)
	if err == nil {
		return nil
	}
	first := err.Error()
	var w struct {
		Islands []json.RawMessage `json:"islands"`
	}
	if json.Unmarshal(file, &w) != nil || len(w.Islands) == 0 {
		return []string{first}
	}
	var out []string
	seen := map[string]bool{}
	for i, isl := range w.Islands {
		out = append(out, islandProblems(isl, i, i < len(w.Islands)-1, id, seen)...)
	}
	if !slices.Contains(out, first) {
		out = append([]string{first}, out...)
	}
	return out
}

// islandProblems prüft eine Insel allein (Probe-Welt mit nur ihr) und, wenn sie nicht die letzte ist, mit einer Kopie
// dahinter (Hafen-Regel); doppelte ids meldet sie wie die Prüfung des Ladens.
func islandProblems(isl json.RawMessage, i int, notLast bool, id string, seen map[string]bool) []string {
	var head struct {
		ID string `json:"id"`
	}
	_ = json.Unmarshal(isl, &head)
	if head.ID != "" && seen[head.ID] {
		return []string{fmt.Sprintf("%s: Insel %s: doppelt", worldRel(id), head.ID)}
	}
	seen[head.ID] = true
	nr := func(err error) []string { // die Probe zählt ab 1, gemeldet wird der Platz in der Welt
		return []string{strings.Replace(err.Error(), "Insel Nr. 1:", fmt.Sprintf("Insel Nr. %d:", i+1), 1)}
	}
	if err := probeWorld(id, isl); err != nil {
		return nr(err)
	}
	if !notLast {
		return nil
	}
	twin, ok := renamed(isl, head.ID)
	if !ok {
		return nil
	}
	if err := probeWorld(id, isl, twin); err != nil {
		return nr(err)
	}
	return nil
}

func probeWorld(id string, islands ...json.RawMessage) error {
	raw, err := json.Marshal(map[string]any{"id": id, "name": "probe", "islands": islands})
	if err != nil {
		return err
	}
	_, err = sim.ParseWorld(raw, id)
	return err
}

// renamed ist die Insel mit einer anderen id (für die Probe der Hafen-Regel).
func renamed(isl json.RawMessage, id string) (json.RawMessage, bool) {
	var m map[string]json.RawMessage
	if json.Unmarshal(isl, &m) != nil {
		return nil, false
	}
	other := "zz-probe"
	if id == other {
		other = "zz-probe-2"
	}
	m["id"] = jsonText(other)
	out, err := json.Marshal(m)
	return out, err == nil
}

// worldRel ist der Pfad der Welt-Datei im Repo, wie die Meldungen der Prüfung ihn nennen.
func worldRel(id string) string {
	if id == standardWorld {
		return islandsPath
	}
	return worldsDir + "/" + id + ".json"
}
