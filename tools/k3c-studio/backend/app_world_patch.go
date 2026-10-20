package backend

import (
	"bytes"
	"encoding/json"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

// Format-stabiles Schreiben eines Welt-Entwurfs (WZ18.6): Nur geänderte Felder bekommen neue Bytes, eine Insel ohne
// Änderung bleibt byte-gleich, auch wenn sie in der Liste wandert; Planung und Chunk-Anzahl gehen mit ihrer Insel mit.

// newIslandText ist die Vorlage einer neuen Insel; applyDraft setzt danach ihre Felder aus dem Entwurf.
const newIslandText = `{ "id": "", "name": "", "depths": [], "depthScaling": { "hp": 1, "damage": 1, "speed": 1 }, "plannedChunks": {} }`

// islandSep trennt Inseln, wenn die Datei keine zwei Inseln als Vorbild hat.
const islandSep = ",\n    "

// islandItem ist ein Insel-Objekt der Liste `islands` mit Bereich und id.
type islandItem struct {
	at span
	id string
}

// islandItems liefert den Bereich der Liste `islands` und die Bereiche ihrer Inseln.
func islandItems(raw []byte) (span, []islandItem, error) {
	list, ok, err := valueSpan(raw, span{0, len(raw)}, "islands")
	if err != nil || !ok {
		return span{}, nil, fmt.Errorf("Welt-Datei: kein Feld islands (%v)", err)
	}
	dec := json.NewDecoder(bytes.NewReader(raw[list.from:list.to]))
	if tok, err := dec.Token(); err != nil || tok != json.Delim('[') {
		return span{}, nil, fmt.Errorf("Welt-Datei: islands ist keine Liste")
	}
	var items []islandItem
	for dec.More() {
		var v json.RawMessage
		if err := dec.Decode(&v); err != nil {
			return span{}, nil, err
		}
		end := list.from + int(dec.InputOffset())
		var head struct {
			ID string `json:"id"`
		}
		_ = json.Unmarshal(v, &head)
		items = append(items, islandItem{span{end - len(v), end}, head.ID})
	}
	return list, items, nil
}

// jsonText schreibt einen Wert als JSON ohne HTML-Ersatz (Umlaute und <> bleiben lesbar).
func jsonText(v any) []byte {
	var b bytes.Buffer
	enc := json.NewEncoder(&b)
	enc.SetEscapeHTML(false)
	_ = enc.Encode(v)
	return bytes.TrimRight(b.Bytes(), "\n")
}

// depthsText schreibt Tiefen wie die Datei: [0, 1, 4].
func depthsText(depths []int) []byte {
	parts := make([]string, len(depths))
	for i, d := range depths {
		parts[i] = strconv.Itoa(d)
	}
	return []byte("[" + strings.Join(parts, ", ") + "]")
}

// scalingText schreibt die Faktoren wie die Datei: { "hp": 1.5, "damage": 1.3, "speed": 1.1 }.
func scalingText(s DraftScaling) []byte {
	f := func(v float64) string { return strconv.FormatFloat(v, 'f', -1, 64) }
	return []byte(fmt.Sprintf(`{ "hp": %s, "damage": %s, "speed": %s }`, f(s.HP), f(s.Damage), f(s.Speed)))
}

// setKey setzt den Wert von key im Objekt obj; fehlt der Schlüssel, wird er vor der schließenden Klammer angehängt.
func setKey(raw []byte, obj span, key string, text []byte) ([]byte, error) {
	at, found, err := valueSpan(raw, obj, key)
	if err != nil {
		return nil, err
	}
	if !found {
		at = insertSpan(raw, obj)
		text = append([]byte(fmt.Sprintf(`, %q: `, key)), text...)
	}
	return slices.Concat(raw[:at.from], text, raw[at.to:]), nil
}

// patchField ist ein Feld der Insel: Schlüssel, ob die Datei ihn schon so hat, und der neue Text.
type patchField struct {
	key  string
	same bool
	text []byte
}

// given: der Entwurf bringt den Wert mit (nicht leer, nicht null).
func given(v json.RawMessage) bool { return len(v) > 0 && string(v) != "null" }

func compactJSON(v json.RawMessage) []byte {
	var b bytes.Buffer
	if json.Compact(&b, v) != nil {
		return v
	}
	return b.Bytes()
}

// patchIsland setzt id, Name, Tiefen und Skalierung einer Insel, nur wo sie vom Entwurf abweichen.
func patchIsland(text []byte, d IslandDraft) ([]byte, error) {
	var cur struct {
		ID      string        `json:"id"`
		Name    string        `json:"name"`
		Depths  []int         `json:"depths"`
		Scaling *DraftScaling `json:"depthScaling"`
		Planned json.RawMessage `json:"plannedChunks"`
		Counts  json.RawMessage `json:"chunkCounts"`
	}
	if err := json.Unmarshal(text, &cur); err != nil {
		return nil, fmt.Errorf("die Insel %q ist kein JSON-Objekt: %w", d.From, err)
	}
	fields := []patchField{
		{"id", cur.ID == d.ID, jsonText(d.ID)},
		{"name", cur.Name == d.Name, jsonText(d.Name)},
		{"depths", cur.Depths != nil && slices.Equal(cur.Depths, d.Depths), depthsText(d.Depths)},
		{"depthScaling", cur.Scaling != nil && *cur.Scaling == d.Scaling, scalingText(d.Scaling)},
	}
	if given(d.PlannedChunks) {
		fields = append(fields, patchField{"plannedChunks", len(cur.Planned) > 0 && sameJSON(cur.Planned, d.PlannedChunks), compactJSON(d.PlannedChunks)})
	}
	if given(d.ChunkCounts) {
		fields = append(fields, patchField{"chunkCounts", len(cur.Counts) > 0 && sameJSON(cur.Counts, d.ChunkCounts), compactJSON(d.ChunkCounts)})
	}
	var err error
	for _, f := range fields {
		if !f.same {
			if text, err = setKey(text, span{0, len(text)}, f.key, f.text); err != nil {
				return nil, err
			}
		}
	}
	return text, nil
}

// islandTexts liefert die Inseln des Entwurfs als Text: die Insel der Datei (From) bzw. die Vorlage, mit den Feldern des
// Entwurfs. inPlace ist wahr, wenn die Liste dieselben Inseln in derselben Reihenfolge hält.
func islandTexts(raw []byte, items []islandItem, d WorldDraft) (texts [][]byte, inPlace bool, err error) {
	inPlace = len(items) == len(d.Islands)
	used := map[string]bool{}
	for i, isl := range d.Islands {
		base := []byte(newIslandText)
		if isl.From != "" {
			at := slices.IndexFunc(items, func(it islandItem) bool { return it.id == isl.From })
			switch {
			case at < 0:
				return nil, false, fmt.Errorf("die Insel %q gibt es nicht in der Datei, bitte neu laden", isl.From)
			case used[isl.From]:
				return nil, false, fmt.Errorf("die Insel %q steht zweimal im Entwurf", isl.From)
			}
			used[isl.From] = true
			base = raw[items[at].at.from:items[at].at.to]
		}
		inPlace = inPlace && items[i].id == isl.From
		text, err := patchIsland(base, isl)
		if err != nil {
			return nil, false, err
		}
		texts = append(texts, text)
	}
	return texts, inPlace, nil
}

// spliceIslands setzt die Inseln in die Liste: an Ort und Stelle, sonst als neue Liste mit dem Trenner der Datei.
func spliceIslands(raw []byte, list span, items []islandItem, texts [][]byte, inPlace bool) []byte {
	if inPlace {
		out := raw
		for i := len(items) - 1; i >= 0; i-- {
			out = slices.Concat(out[:items[i].at.from], texts[i], out[items[i].at.to:])
		}
		return out
	}
	eol := "\n" // Zeilenende wie die Datei
	if bytes.Contains(raw, []byte("\r\n")) {
		eol = "\r\n"
	}
	open, closing, sep := []byte("["+eol+"    "), []byte(eol+"  ]"), []byte(strings.Replace(islandSep, "\n", eol, 1))
	if n := len(items); n > 0 {
		open, closing = raw[list.from:items[0].at.from], raw[items[n-1].at.to:list.to]
	}
	if len(items) > 1 {
		sep = raw[items[0].at.to:items[1].at.from]
	}
	return slices.Concat(raw[:list.from], open, bytes.Join(texts, sep), closing, raw[list.to:])
}

// patchHead setzt Name und Notizen der Welt, nur wo sie abweichen; fehlende Notizen ohne Text bleiben weg.
func patchHead(raw []byte, d WorldDraft) ([]byte, error) {
	var cur struct {
		Name  string  `json:"name"`
		Notes *string `json:"notes"`
	}
	if err := json.Unmarshal(raw, &cur); err != nil {
		return nil, fmt.Errorf("Welt-Datei: %w", err)
	}
	var err error
	if cur.Name != d.Name {
		if raw, err = setKey(raw, span{0, len(raw)}, "name", jsonText(d.Name)); err != nil {
			return nil, err
		}
	}
	if (cur.Notes == nil && d.Notes != "") || (cur.Notes != nil && *cur.Notes != d.Notes) {
		raw, err = setKey(raw, span{0, len(raw)}, "notes", jsonText(d.Notes))
	}
	return raw, err
}

// applyDraft schreibt den Entwurf in den Inhalt der Welt-Datei; ohne Änderung kommt derselbe Inhalt zurück.
func applyDraft(raw []byte, d WorldDraft) ([]byte, error) {
	list, items, err := islandItems(raw)
	if err != nil {
		return nil, err
	}
	texts, inPlace, err := islandTexts(raw, items, d)
	if err != nil {
		return nil, err
	}
	return patchHead(spliceIslands(raw, list, items, texts, inPlace), d)
}
