package sim

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/fs"
	"regexp"
	"slices"
	"strconv"
	"strings"

	"k3c/data"
	"k3c/engine/level"
)

// Welten als Dateien (docs/rules/welten.md, B-488): Eine Welt ist eine Datei data/worlds/<id>.json mit ihren Inseln in
// Spielreihenfolge. Gelesen wird streng (unbekannte Felder und fehlende Pflichtfelder sind Fehler), geprüft wird jede
// Insel (checkIsland); bei einem Fehler wird die Welt nicht geladen. Eingebettet ist nur die Standard-Welt.

// StandardWorldID ist die ID der Standard-Welt (data/worlds/standard.json).
const StandardWorldID = "standard"

// WorldDef ist eine Welt aus einer Welt-Datei (im Go-Code ist `World` eine Stufe).
type WorldDef struct {
	ID, Name, Notes string
	Islands         []IslandDef
}

// idPattern gilt für Welt- und Insel-IDs (welten.md § 4).
var idPattern = regexp.MustCompile(`^[a-z0-9][a-z0-9-]{0,31}$`)

// standardWorld ist die eingebettete Standard-Welt; ohne eigene Welt spielt die Sim sie.
var standardWorld = mustStandardWorld()

func mustStandardWorld() WorldDef {
	worlds, err := fs.Sub(data.Files, "worlds")
	if err == nil {
		var w WorldDef
		if w, err = LoadWorld(worlds, StandardWorldID); err == nil {
			return w
		}
	}
	panic(err)
}

// StandardWorld liefert die eingebettete Standard-Welt. Die Inselliste ist kopiert, Tiefen und Planung der Inseln
// teilen sich die Aufrufer mit der Sim: nur lesen.
func StandardWorld() WorldDef {
	w := standardWorld
	w.Islands = slices.Clone(w.Islands)
	return w
}

// LoadWorld liest die Welt id aus fsys (Datei <id>.json im Wurzelordner von fsys) und prüft sie wie ParseWorld.
// Eine ID außerhalb des Musters wird abgelehnt, bevor gelesen wird.
func LoadWorld(fsys fs.FS, id string) (WorldDef, error) {
	if err := checkWorldID(id); err != nil {
		return WorldDef{}, err
	}
	raw, err := fs.ReadFile(fsys, id+".json")
	if err != nil {
		return WorldDef{}, fmt.Errorf("%s: %w", worldPath(id), err)
	}
	return ParseWorld(raw, id)
}

// ParseWorld liest die Bytes der Welt-Datei der Welt id streng und prüft sie (welten.md § 3). Fehler nennen Datei,
// Insel, Tiefe und Platz, soweit vorhanden: `<Datei>: Insel <insel-id>: <Grund>`.
func ParseWorld(raw []byte, id string) (WorldDef, error) {
	if err := checkWorldID(id); err != nil {
		return WorldDef{}, err
	}
	w, islands, err := decodeWorld(raw, id)
	if err == nil {
		w.Islands, err = decodeIslands(islands)
	}
	if err != nil {
		return WorldDef{}, fmt.Errorf("%s: %w", worldPath(id), err)
	}
	return w, nil
}

func checkWorldID(id string) error {
	if !idPattern.MatchString(id) {
		return fmt.Errorf("Welt-ID %q ungültig (a-z, 0-9, -, höchstens 32 Zeichen)", id)
	}
	return nil
}

func worldPath(id string) string { return "data/worlds/" + id + ".json" }

// Rohform der Datei: Zeiger erkennen fehlende Pflichtfelder, Rohdaten lassen jede Insel einzeln lesen.
type rawWorld struct {
	ID      *string            `json:"id"`
	Name    *string            `json:"name"`
	Notes   string             `json:"notes"`
	Islands *[]json.RawMessage `json:"islands"`
}

type rawIsland struct {
	ID            *string                         `json:"id"`
	Name          *string                         `json:"name"`
	Depths        *[]int                          `json:"depths"`
	DepthScaling  json.RawMessage                 `json:"depthScaling"`
	PlannedChunks map[string][]level.PlannedChunk `json:"plannedChunks"`
	ChunkCounts   map[string]int                  `json:"chunkCounts"`
}

type rawScaling struct {
	HP     *float64 `json:"hp"`
	Damage *float64 `json:"damage"`
	Speed  *float64 `json:"speed"`
}

// decodeWorld liest den Kopf der Welt streng und prüft Pflichtfelder und die ID; die Inseln bleiben roh.
func decodeWorld(raw []byte, id string) (WorldDef, []json.RawMessage, error) {
	var v rawWorld
	if err := strictDecode(raw, &v, "id", "name", "notes", "islands"); err != nil {
		return WorldDef{}, nil, err
	}
	switch {
	case v.ID == nil:
		return WorldDef{}, nil, missingField("id")
	case v.Name == nil:
		return WorldDef{}, nil, missingField("name")
	case v.Islands == nil:
		return WorldDef{}, nil, missingField("islands")
	case *v.ID != id:
		return WorldDef{}, nil, fmt.Errorf("id %q passt nicht zum Dateinamen", *v.ID)
	case *v.Name == "":
		return WorldDef{}, nil, errors.New("name ist leer")
	case len(*v.Islands) == 0:
		return WorldDef{}, nil, errors.New("keine Insel")
	}
	return WorldDef{ID: *v.ID, Name: *v.Name, Notes: v.Notes}, *v.Islands, nil
}

// decodeIslands liest und prüft die Inseln der Reihe nach; die erste verletzte Regel wird gemeldet. Ohne ID heißt die
// Insel in der Meldung „Nr. <Platz in der Liste>“.
func decodeIslands(islands []json.RawMessage) ([]IslandDef, error) {
	var out []IslandDef
	for i, raw := range islands {
		var r rawIsland
		var d IslandDef
		err := strictDecode(raw, &r, "id", "name", "depths", "depthScaling", "plannedChunks", "chunkCounts")
		if err == nil {
			d, err = r.def()
		}
		if err == nil {
			err = checkIslandIn(d, out)
		}
		if err != nil {
			return nil, islandError{islandLabel(raw, i), err}
		}
		out = append(out, d)
	}
	for i, d := range out[:len(out)-1] {
		if !hasHarbor(d) {
			return nil, islandError{islandLabel(islands[i], i), errors.New("kein Hafen in den geplanten Chunks der Tiefe 0, die Insel ist nicht die letzte der Welt (geplante-chunks.md § 10)")}
		}
	}
	return out, nil
}

// hasHarbor: Die Insel plant in Tiefe 0 einen Hafen-Chunk (BAS, B-637).
func hasHarbor(d IslandDef) bool {
	return slices.ContainsFunc(d.PlannedChunks["0"], func(c level.PlannedChunk) bool { return c.Kind == level.HarborKind })
}

// islandLabel nennt die Insel in Meldungen: ihre ID, außerhalb des Musters in Anführungszeichen, ohne ID ihr Platz in
// der Liste („Nr. 2“).
func islandLabel(raw []byte, i int) string {
	var head struct {
		ID *string `json:"id"`
	}
	switch {
	case json.Unmarshal(raw, &head) != nil || head.ID == nil:
		return "Nr. " + strconv.Itoa(i+1)
	case idPattern.MatchString(*head.ID):
		return *head.ID
	}
	return strconv.Quote(*head.ID)
}

// checkIslandIn: ID nach Muster und eindeutig unter den Inseln davor, dann checkIsland.
func checkIslandIn(d IslandDef, before []IslandDef) error {
	switch {
	case !idPattern.MatchString(d.ID):
		return errors.New("id ungültig (a-z, 0-9, -, höchstens 32 Zeichen)")
	case slices.ContainsFunc(before, func(o IslandDef) bool { return o.ID == d.ID }):
		return errors.New("doppelt")
	}
	return checkIsland(d)
}

// def prüft die Pflichtfelder der Insel und liefert sie als IslandDef.
func (r rawIsland) def() (IslandDef, error) {
	var s rawScaling
	missing := ""
	switch {
	case r.ID == nil:
		missing = "id"
	case r.Name == nil:
		missing = "name"
	case r.Depths == nil:
		missing = "depths"
	case r.DepthScaling == nil:
		missing = "depthScaling"
	}
	if missing != "" {
		return IslandDef{}, missingField(missing)
	}
	if err := strictDecode(r.DepthScaling, &s, "hp", "damage", "speed"); err != nil {
		return IslandDef{}, fmt.Errorf("depthScaling: %w", err)
	}
	switch {
	case s.HP == nil:
		missing = "depthScaling.hp"
	case s.Damage == nil:
		missing = "depthScaling.damage"
	case s.Speed == nil:
		missing = "depthScaling.speed"
	}
	if missing != "" {
		return IslandDef{}, missingField(missing)
	}
	d := IslandDef{ID: *r.ID, Name: *r.Name, Depths: *r.Depths, PlannedChunks: r.PlannedChunks, ChunkCounts: r.ChunkCounts}
	d.DepthScaling.HP, d.DepthScaling.Damage, d.DepthScaling.Speed = *s.HP, *s.Damage, *s.Speed
	return d, nil
}

// strictDecode liest genau ein JSON-Objekt nach dst: Schlüssel exakt aus fields (Groß-/Kleinschreibung zählt, keiner
// doppelt), unbekannte Felder und Daten nach dem Ende sind Fehler.
func strictDecode(raw []byte, dst any, fields ...string) error {
	if err := checkKeys(raw, fields); err != nil {
		return err
	}
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.DisallowUnknownFields()
	if err := dec.Decode(dst); err != nil {
		if field, ok := strings.CutPrefix(err.Error(), "json: unknown field "); ok {
			return fmt.Errorf("unbekanntes Feld %s", field)
		}
		return fmt.Errorf("kein gültiges JSON: %w", err)
	}
	if _, err := dec.Token(); err != io.EOF {
		return errors.New("kein gültiges JSON: Daten nach dem Ende")
	}
	return nil
}

// checkKeys prüft die Schlüssel der obersten Ebene des Objekts raw; ist raw kein Objekt, meldet das Decode den Fehler.
func checkKeys(raw []byte, fields []string) error {
	dec := json.NewDecoder(bytes.NewReader(raw))
	if t, err := dec.Token(); err != nil || t != json.Delim('{') {
		return nil
	}
	var seen []string
	for dec.More() {
		t, err := dec.Token()
		if err != nil {
			return nil
		}
		key, _ := t.(string)
		switch {
		case !slices.Contains(fields, key):
			return fmt.Errorf("unbekanntes Feld %q", key)
		case slices.Contains(seen, key):
			return fmt.Errorf("doppeltes Feld %q", key)
		}
		seen = append(seen, key)
		var skip json.RawMessage
		if dec.Decode(&skip) != nil {
			return nil
		}
	}
	return nil
}

// missingField meldet ein fehlendes Pflichtfeld (welten.md § 2: nicht als „Faktoren > 0“ o. ä.).
func missingField(name string) error { return fmt.Errorf("%s fehlt (Pflichtfeld)", name) }

// islandError nennt die Insel vor dem Grund: `Insel <insel-id>: <Grund>` (welten.md § 3).
type islandError struct {
	island string
	err    error
}

func (e islandError) Error() string { return "Insel " + e.island + ": " + e.err.Error() }
func (e islandError) Unwrap() error { return e.err }
