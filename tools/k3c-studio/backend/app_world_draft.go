package backend

import (
	"bytes"
	"encoding/json"
	"os"
	"strconv"
	"strings"

	"k3c/engine/sim"
)

// Entwurf einer Welt im Welten Editor (WZ18.6, B-488/AC-05, docs/rules/welten.md): Name, Notizen und die Inseln mit
// Name, id, Tiefen und Skalierung; Planung und Chunk-Anzahl bleiben beim Editor für geplante Chunks (PlanSave). Geprüft
// wird mit der Prüfung des Ladens (sim.ParseWorld), gespeichert nur Gültiges, atomar, format-stabil und auf dem Stand aus
// PlanLoad.

// DraftScaling sind die Faktoren der Skalierung je Tiefe einer Insel (depthScaling).
type DraftScaling struct {
	HP     float64 `json:"hp"`
	Damage float64 `json:"damage"`
	Speed  float64 `json:"speed"`
}

// IslandDraft ist eine Insel im Entwurf. From ist ihre id in der Datei (leer = neue Insel); ihre Planung und
// Chunk-Anzahl gehen mit, auch wenn sich die id ändert.
type IslandDraft struct {
	From    string       `json:"from"`
	ID      string       `json:"id"`
	Name    string       `json:"name"`
	Depths  []int        `json:"depths"`
	Scaling DraftScaling `json:"depthScaling"`
	// PlannedChunks und ChunkCounts sind nur gesetzt, wenn der Entwurf sie mitbringt (Antwort aus dem Chat, WZ19.3);
	// fehlen sie, bleibt die Planung der Insel in der Datei, wie sie ist.
	PlannedChunks json.RawMessage `json:"plannedChunks,omitempty"`
	ChunkCounts   json.RawMessage `json:"chunkCounts,omitempty"`
}

// WorldDraft ist der Entwurf der gewählten Welt; die id bleibt (gleich dem Dateinamen).
type WorldDraft struct {
	Name    string        `json:"name"`
	Notes   string        `json:"notes"`
	Islands []IslandDraft `json:"islands"`
}

// WorldCheckView ist die Prüfung eines Entwurfs: Island ist der Platz der Insel im Entwurf (-1 = die Welt), Field das
// Feld (id, name, depths, depthScaling, plan bzw. bei der Welt name, islands; leer = ohne Feld). Preview ist der Inhalt
// der Datei, wie Speichern ihn schreiben würde.
type WorldCheckView struct {
	OK      bool   `json:"ok"`
	Error   string `json:"error"`
	Island  int    `json:"island"`
	Field   string `json:"field"`
	Preview string `json:"preview"`
}

// draftFile liefert den Inhalt der Datei mit dem Entwurf und prüft ihn wie beim Laden.
func draftFile(raw []byte, id string, d WorldDraft) ([]byte, error) {
	out, err := applyDraft(raw, d)
	if err != nil {
		return nil, err
	}
	_, err = sim.ParseWorld(out, id)
	return out, err
}

// PlanWorldCheck prüft den Entwurf der gewählten Welt, ohne zu schreiben (Binding).
func (a *App) PlanWorldCheck(d WorldDraft) WorldCheckView {
	id := a.activeWorld()
	raw, err := os.ReadFile(a.worldFile(id))
	if err != nil {
		return WorldCheckView{Island: -1, Error: err.Error()}
	}
	out, err := draftFile(raw, id, d)
	view := WorldCheckView{OK: err == nil, Island: -1, Preview: string(out)}
	if err != nil {
		view.Error = err.Error()
		view.Island, view.Field = fieldOf(view.Error, d)
	}
	return view
}

// PlanWorldSave speichert den Entwurf der gewählten Welt (Binding): nur gültig, nur auf dem Stand aus PlanLoad (sonst
// Stale ohne Schreibzugriff), format-stabil und atomar; danach der REG-Hinweis.
func (a *App) PlanWorldSave(d WorldDraft, stand string) PlanSaved {
	a.save.Lock()
	defer a.save.Unlock()
	id := a.activeWorld() // einmal gelesen: Lesen, Prüfen und Schreiben gelten derselben Welt
	path := a.worldFile(id)
	raw, err := os.ReadFile(path)
	if err != nil {
		return PlanSaved{Error: err.Error()}
	}
	if planStand(raw) != stand {
		return PlanSaved{Error: staleError(id), Stale: true}
	}
	out, err := draftFile(raw, id, d)
	if err != nil {
		return PlanSaved{Error: err.Error()}
	}
	if !bytes.Equal(out, raw) {
		if err := writeAtomic(path, out); err != nil {
			return PlanSaved{Error: err.Error()}
		}
	}
	return PlanSaved{Stand: planStand(out), Hint: planRegHint}
}

// fieldOf ordnet eine Meldung der Prüfung (`<Datei>: Insel <insel>: <Grund>` bzw. `<Datei>: <Grund>`) Insel und Feld zu.
func fieldOf(msg string, d WorldDraft) (int, string) {
	_, rest, _ := strings.Cut(msg, ".json: ")
	label, reason, ok := strings.Cut(strings.TrimPrefix(rest, "Insel "), ": ")
	if !strings.HasPrefix(rest, "Insel ") || !ok {
		switch {
		case strings.HasPrefix(rest, "name"):
			return -1, "name"
		case rest == "keine Insel" || strings.HasPrefix(rest, "islands"):
			return -1, "islands"
		}
		return -1, ""
	}
	return islandIndex(label, reason == "doppelt", d), islandField(reason)
}

// islandIndex findet die Insel der Meldung im Entwurf: „Nr. n“, eine id oder eine id in Anführungszeichen; bei
// „doppelt“ die zweite Insel mit dieser id.
func islandIndex(label string, second bool, d WorldDraft) int {
	if n, ok := strings.CutPrefix(label, "Nr. "); ok {
		i, _ := strconv.Atoi(n)
		return i - 1
	}
	if id, err := strconv.Unquote(label); err == nil {
		label = id
	}
	found := -1
	for i, isl := range d.Islands {
		if isl.ID != label {
			continue
		}
		if !second || found >= 0 {
			return i
		}
		found = i
	}
	return found
}

// islandField ist das Feld einer Insel zum Grund der Prüfung.
func islandField(reason string) string {
	switch {
	case strings.HasPrefix(reason, "plannedChunks") || strings.HasPrefix(reason, "chunkCounts"):
		return "plan"
	case strings.HasPrefix(reason, "depthScaling"):
		return "depthScaling"
	case strings.HasPrefix(reason, "id") || reason == "doppelt":
		return "id"
	case strings.HasPrefix(reason, "name"):
		return "name"
	}
	return "depths"
}
