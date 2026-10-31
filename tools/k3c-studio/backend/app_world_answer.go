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
	Hint  string      `json:"hint,omitempty"` // was das Studio ergänzt hat (Hafen, B-686)
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
	raw, ok := extractJSON(withoutFence(effectiveAnswer(text), actionsFence)) // ein Aktionsblock (B-679) ist nicht die Welt
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
	if _, err := sim.ParseWorld([]byte(raw), id); err != nil {
		if msg := err.Error(); strings.Contains(msg, "unbekanntes Feld") || strings.Contains(msg, "kein gültiges JSON") {
			return WorldAnswerView{Error: answerUnreadable + ": " + msg}
		}
	}
	d, derr := draftOfAnswer(raw, a.fileIslandIDs())
	if derr != nil {
		return WorldAnswerView{Error: answerUnreadable + ": " + derr.Error()}
	}
	// geprüft wird, was Speichern in die gewählte Welt schreiben würde (Meldung mit deren Datei, B-686), mit ergänztem Hafen
	full, notes := a.withHarbors(*d)
	chk := a.PlanWorldCheck(full)
	return WorldAnswerView{OK: chk.OK, Error: chk.Error, Draft: &full, Hint: strings.Join(notes, "; ")}
}

// chatFixEnd schließt den Hinweis eines Korrekturversuchs (chat_world.go); danach folgt der neue Vorschlag.
const chatFixEnd = "(neuer Vorschlag folgt)"

// effectiveAnswer ist der Teil der Antwort nach dem letzten Korrekturversuch des Studios (die ganze Antwort ohne einen).
func effectiveAnswer(text string) string {
	if i := strings.LastIndex(text, chatFixEnd); i >= 0 {
		return text[i+len(chatFixEnd):]
	}
	return text
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
