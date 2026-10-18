package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"reflect"
	"strconv"

	"k3c/engine/level"
)

// Schreiben der Planung in data/islands.json (WZ8.5, B-461/AC-12): format-stabil (nur der Wert von `plannedChunks` der
// betroffenen Insel wird ersetzt, alle übrigen Bytes bleiben), atomar (Temp-Datei, dann umbenennen) und mit Stand-Prüfung.

// islandsPath ist data/islands.json relativ zur Repo-Wurzel.
const islandsPath = "data/islands.json"

// planRegHint ist der Hinweis nach jedem Speichern: Eine gespeicherte Planung ist eine REG-Änderung durch 🧑.
const planRegHint = "REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung"

// errPlanStale lehnt ein Speichern auf veraltetem Stand ab; der Entwurf bleibt in der Oberfläche.
type planStaleError struct{}

func (planStaleError) Error() string { return "data/islands.json geändert, bitte neu laden" }

var errPlanStale error = planStaleError{}

// planStand ist der Stand der Datei: SHA-256 des Inhalts (hex).
func planStand(raw []byte) string {
	sum := sha256.Sum256(raw)
	return hex.EncodeToString(sum[:])
}

// planned liest die Planung der Insel id (Tiefe → Chunks) aus dem Inhalt von islands.json.
func plannedOf(raw []byte, id string) (map[string][]level.PlannedChunk, error) {
	var f struct {
		Islands []struct {
			ID      string                          `json:"id"`
			Planned map[string][]level.PlannedChunk `json:"plannedChunks"`
		} `json:"islands"`
	}
	if err := json.Unmarshal(raw, &f); err != nil {
		return nil, fmt.Errorf("%s: %w", islandsPath, err)
	}
	for _, isl := range f.Islands {
		if isl.ID == id {
			return isl.Planned, nil
		}
	}
	return nil, fmt.Errorf("die Insel %q gibt es nicht in %s", id, islandsPath)
}

// span ist ein Byte-Bereich [from, to) im Inhalt.
type span struct{ from, to int }

// valueSpan findet in einem JSON-Objekt (raw[outer.from:outer.to]) den Wert zu key; ok ist false, wenn der Schlüssel fehlt.
func valueSpan(raw []byte, outer span, key string) (span, bool, error) {
	dec := json.NewDecoder(bytes.NewReader(raw[outer.from:outer.to]))
	if tok, err := dec.Token(); err != nil || tok != json.Delim('{') {
		return span{}, false, fmt.Errorf("JSON-Objekt erwartet")
	}
	for dec.More() {
		tok, err := dec.Token()
		if err != nil {
			return span{}, false, err
		}
		var v json.RawMessage
		if err := dec.Decode(&v); err != nil {
			return span{}, false, err
		}
		if tok.(string) == key {
			end := int(dec.InputOffset())
			return span{outer.from + end - len(v), outer.from + end}, true, nil
		}
	}
	return span{}, false, nil
}

// islandSpan findet das Objekt der Insel id im Array `islands`.
func islandSpan(raw []byte, id string) (span, error) {
	root := span{0, len(raw)}
	list, ok, err := valueSpan(raw, root, "islands")
	if err != nil || !ok {
		return span{}, fmt.Errorf("%s: kein Feld islands (%v)", islandsPath, err)
	}
	dec := json.NewDecoder(bytes.NewReader(raw[list.from:list.to]))
	if tok, err := dec.Token(); err != nil || tok != json.Delim('[') {
		return span{}, fmt.Errorf("%s: islands ist keine Liste", islandsPath)
	}
	for dec.More() {
		var v json.RawMessage
		if err := dec.Decode(&v); err != nil {
			return span{}, err
		}
		end := int(dec.InputOffset())
		var head struct {
			ID string `json:"id"`
		}
		if json.Unmarshal(v, &head) == nil && head.ID == id {
			return span{list.from + end - len(v), list.from + end}, nil
		}
	}
	return span{}, fmt.Errorf("die Insel %q gibt es nicht in %s", id, islandsPath)
}

// encodePlanned schreibt die Planung kompakt, Tiefen aufsteigend (wie die Datei sie hält).
func encodePlanned(planned map[string][]level.PlannedChunk) ([]byte, error) {
	var b bytes.Buffer
	b.WriteByte('{')
	for i, key := range sortedDepths(planned) {
		if i > 0 {
			b.WriteString(", ")
		}
		var item bytes.Buffer
		enc := json.NewEncoder(&item)
		enc.SetEscapeHTML(false)
		if err := enc.Encode(planned[key]); err != nil {
			return nil, err
		}
		fmt.Fprintf(&b, "%q: %s", key, bytes.TrimRight(item.Bytes(), "\n"))
	}
	b.WriteByte('}')
	return b.Bytes(), nil
}

// sortedDepths liefert die Schlüssel (Tiefen) einer Tabelle je Tiefe, nach Zahl aufsteigend.
func sortedDepths[V any](byDepth map[string]V) []string {
	keys := make([]string, 0, len(byDepth))
	for k := range byDepth {
		keys = append(keys, k)
	}
	for i := 1; i < len(keys); i++ { // wenige Schlüssel: Einfügesortierung nach Zahl
		for j := i; j > 0 && depthOf(keys[j]) < depthOf(keys[j-1]); j-- {
			keys[j], keys[j-1] = keys[j-1], keys[j]
		}
	}
	return keys
}

func depthOf(key string) int {
	n, _ := strconv.Atoi(key)
	return n
}

// nextPlanned ist die Planung nach dem Ersetzen der Tiefe depth (leere Listen entfallen); same ist wahr, wenn sich
// inhaltlich nichts ändert.
func nextPlanned(old map[string][]level.PlannedChunk, depth int, chunks []level.PlannedChunk) (next map[string][]level.PlannedChunk, same bool) {
	oldClean, next := map[string][]level.PlannedChunk{}, map[string][]level.PlannedChunk{}
	for k, v := range old {
		if len(v) > 0 {
			oldClean[k], next[k] = v, v
		}
	}
	if len(chunks) > 0 {
		next[strconv.Itoa(depth)] = chunks
	} else {
		delete(next, strconv.Itoa(depth))
	}
	return next, reflect.DeepEqual(oldClean, next)
}

// insertSpan ist die Stelle vor der schließenden Klammer der Insel, an der ein fehlendes Feld angehängt wird.
func insertSpan(raw []byte, island span) span {
	end := island.to - 1
	for end > island.from && (raw[end-1] == ' ' || raw[end-1] == '\n' || raw[end-1] == '\r' || raw[end-1] == '\t') {
		end--
	}
	return span{end, end}
}

// patchPlanned ersetzt die Planung der Tiefe depth der Insel id und lässt alle übrigen Bytes stehen. Ohne inhaltliche
// Änderung kommt derselbe Inhalt zurück (byte-gleich).
func patchPlanned(raw []byte, id string, depth int, chunks []level.PlannedChunk) ([]byte, error) {
	old, err := plannedOf(raw, id)
	if err != nil {
		return nil, err
	}
	next, same := nextPlanned(old, depth, chunks)
	if same {
		return raw, nil
	}
	text, err := encodePlanned(next)
	if err != nil {
		return nil, err
	}
	return replaceField(raw, id, "plannedChunks", text)
}

// replaceField ersetzt den Wert des Feldes key der Insel id durch text; fehlt das Feld, wird es vor der schließenden
// Klammer der Insel angehängt. Alle übrigen Bytes bleiben.
func replaceField(raw []byte, id, key string, text []byte) ([]byte, error) {
	island, err := islandSpan(raw, id)
	if err != nil {
		return nil, err
	}
	at, found, err := valueSpan(raw, island, key)
	if err != nil {
		return nil, err
	}
	if !found {
		at = insertSpan(raw, island)
		text = append([]byte(fmt.Sprintf(`, %q: `, key)), text...)
	}
	return append(append(append([]byte{}, raw[:at.from]...), text...), raw[at.to:]...), nil
}

// writeAtomic schreibt über eine Temp-Datei im selben Ordner, damit nie eine halbe Datei liegt.
func writeAtomic(path string, data []byte) error {
	tmp, err := os.CreateTemp(filepath.Dir(path), ".islands-*")
	if err != nil {
		return err
	}
	if info, err := os.Stat(path); err == nil { // die Rechte der Datei bleiben
		_ = tmp.Chmod(info.Mode().Perm())
	}
	_, werr := tmp.Write(data)
	cerr := tmp.Close()
	if werr != nil || cerr != nil {
		_ = os.Remove(tmp.Name())
		return fmt.Errorf("schreiben: %v %v", werr, cerr)
	}
	if err := os.Rename(tmp.Name(), path); err != nil {
		_ = os.Remove(tmp.Name())
		return err
	}
	return nil
}
