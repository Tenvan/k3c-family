package backend

import (
	"encoding/json"
	"os"
	"regexp"
	"slices"
	"strings"

	"k3c/engine/sim"
)

// Antwort des Modells prüfen (B-488, WZ19.3): Die Antwort ist Daten. Das JSON wird aus dem Text gelöst (auch aus einem
// Code-Block) und streng wie beim Laden einer Welt geprüft (sim.ParseWorld). Lesbare Welten kommen als Entwurf zurück,
// auch mit Prüffehler (der Grund steht am Entwurf, der Editor zeigt ihn am Feld); nicht lesbare oder Antworten mit
// unbekannten Feldern kommen ohne Entwurf. Geschrieben wird nichts.

// WorldAnswerView ist die Prüfung einer Antwort. Draft ist gesetzt, wenn die Welt lesbar ist; OK nur, wenn sie
// außerdem die Prüfung des Ladens besteht.
type WorldAnswerView struct {
	OK    bool        `json:"ok"`
	Error string      `json:"error"`
	Draft *WorldDraft `json:"draft"`
}

const answerUnreadable = "Antwort nicht lesbar"

var fencedJSON = regexp.MustCompile("(?s)```[a-zA-Z]*\\s*(\\{.*?\\})\\s*```")

// extractJSON löst das JSON-Objekt aus der Antwort: erst ein Code-Block, sonst vom ersten „{“ bis zum passenden „}“.
func extractJSON(text string) (string, bool) {
	if m := fencedJSON.FindStringSubmatch(text); m != nil {
		return m[1], true
	}
	start := strings.IndexByte(text, '{')
	if start < 0 {
		return "", false
	}
	dec := json.NewDecoder(strings.NewReader(text[start:]))
	var raw json.RawMessage
	if dec.Decode(&raw) != nil {
		return "", false
	}
	return string(raw), true
}

// PlanWorldAnswer prüft die Antwort des Modells und liefert sie als Entwurf (Binding); schreibt nichts.
func (a *App) PlanWorldAnswer(text string) WorldAnswerView {
	raw, ok := extractJSON(text)
	if !ok {
		return WorldAnswerView{Error: answerUnreadable + ": kein JSON gefunden"}
	}
	var head struct {
		ID string `json:"id"`
	}
	if err := json.Unmarshal([]byte(raw), &head); err != nil {
		return WorldAnswerView{Error: answerUnreadable + ": " + err.Error()}
	}
	id := head.ID
	if id == "" {
		id = "antwort"
	}
	_, err := sim.ParseWorld([]byte(raw), id)
	if err == nil {
		d, derr := draftOfAnswer(raw, a.fileIslandIDs())
		if derr != nil {
			return WorldAnswerView{Error: answerUnreadable + ": " + derr.Error()}
		}
		return WorldAnswerView{OK: true, Draft: d}
	}
	msg := err.Error()
	if strings.Contains(msg, "unbekanntes Feld") || strings.Contains(msg, "kein gültiges JSON") {
		return WorldAnswerView{Error: answerUnreadable + ": " + msg}
	}
	d, derr := draftOfAnswer(raw, a.fileIslandIDs())
	if derr != nil {
		return WorldAnswerView{Error: answerUnreadable + ": " + derr.Error()}
	}
	return WorldAnswerView{Error: msg, Draft: d}
}

// fileIslandIDs liefert die Insel-ids der gewählten Welt-Datei (leer, wenn sie nicht lesbar ist).
func (a *App) fileIslandIDs() []string {
	raw, err := os.ReadFile(a.worldFile(a.activeWorld()))
	if err != nil {
		return nil
	}
	_, items, err := islandItems(raw)
	if err != nil {
		return nil
	}
	ids := make([]string, len(items))
	for i, it := range items {
		ids[i] = it.id
	}
	return ids
}

// draftOfAnswer macht aus der Welt-Datei der Antwort einen Entwurf: Inseln mit einer id, die schon in der Datei steht
// (existing), behalten diese als From und damit ihre Planung, soweit die Antwort keine mitbringt; alle anderen sind neu.
// Die ID der Antwort ersetzt nicht die der gewählten Welt (bleibt Dateiname).
func draftOfAnswer(raw string, existing []string) (*WorldDraft, error) {
	var w struct {
		Name    string `json:"name"`
		Notes   string `json:"notes"`
		Islands []struct {
			ID      string          `json:"id"`
			Name    string          `json:"name"`
			Depths  []int           `json:"depths"`
			Scaling DraftScaling    `json:"depthScaling"`
			Planned json.RawMessage `json:"plannedChunks"`
			Counts  json.RawMessage `json:"chunkCounts"`
		} `json:"islands"`
	}
	dec := json.NewDecoder(strings.NewReader(raw))
	if err := dec.Decode(&w); err != nil {
		return nil, err
	}
	d := &WorldDraft{Name: w.Name, Notes: w.Notes, Islands: []IslandDraft{}}
	for _, i := range w.Islands {
		isl := IslandDraft{ID: i.ID, Name: i.Name, Depths: i.Depths, Scaling: i.Scaling, PlannedChunks: i.Planned, ChunkCounts: i.Counts}
		if i.ID != "" && slices.Contains(existing, i.ID) && !slices.ContainsFunc(d.Islands, func(o IslandDraft) bool { return o.From == i.ID }) {
			isl.From = i.ID
		}
		if isl.Depths == nil {
			isl.Depths = []int{}
		}
		d.Islands = append(d.Islands, isl)
	}
	return d, nil
}
