// Package world ist das Backend des Welten-Editors des Studios (B-879, Port aus dem früheren Asset-Studio, Commit
// 616b5afcb): Welten unter data/worlds/ auflisten und lesen, das Level einer Insel-Stufe für die Chunk-Vorschau holen
// und Werte einer Welt übersteuern (`overrides`, B-680, docs/rules/welten.md § 1). Das Studio importiert den
// Engine-Kern nicht selbst (B-924: über engine/simcli): Level, Basis-Werte und die Prüfung rechnet `simcli world` im Prozess (Engine, level.go).
// Geschrieben wird nur die Welt-Datei, format-stabil und atomar; es läuft keine Simulation.
package world

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// worldsDir ist der Ordner der Welt-Dateien unter der Repo-Wurzel; die Standard-Welt ist standard.json darin.
const worldsDir = "data/worlds"

// standardID ist die Welt, die das Spiel ohne Wahl lädt; sie steht in der Liste oben.
const standardID = "standard"

// idPattern ist das Muster einer Welt-ID (gleich dem Dateinamen ohne .json, wie sim.LoadWorld); nur solche Dateien
// gelten als Welt.
var idPattern = regexp.MustCompile(`^[a-z0-9][a-z0-9-]{0,31}$`)

// Info ist eine Welt der Liste.
type Info struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	// Missing: feste Welt (data/paper.json › weltbilder) ohne Datei; sie erscheint mit dem Hinweis „noch nicht
	// umgesetzt“ (B-1168, WZ62.1) und lässt sich nicht öffnen.
	Missing bool `json:"missing,omitempty"`
	// Kind: Art der Welt ("spinoff" | "new", B-1217); leer bei fehlender Datei.
	Kind string `json:"kind,omitempty"`
	// Gewaesser: Untertitel der Welt in der Liste.
	Gewaesser string `json:"gewaesser,omitempty"`
}

// MissingNote ist der Hinweis an einer festen Welt ohne Datei.
const MissingNote = "noch nicht umgesetzt"

// Island ist eine Insel der Welt mit dem Biom je Tiefe (gleiche Reihenfolge wie Depths), ihrer Planung und Chunk-Anzahl
// wie in der Datei.
type Island struct {
	ID          string                     `json:"id"`
	Name        string                     `json:"name"`
	Etappe      *int                       `json:"etappe,omitempty"`
	Depths      []int                      `json:"depths"`
	Biomes      []string                   `json:"biomes"`
	Planned     map[string]json.RawMessage `json:"plannedChunks"`
	ChunkCounts map[string]int             `json:"chunkCounts"`
	// Story ist die Geschichte der Insel (story.go, B-975); nil = fehlt.
	Story *IslandStory `json:"story,omitempty"`
}

// Detail ist eine Welt mit Kopf und Inseln; Error nennt, warum sie nicht lesbar ist (dann ohne Inseln).
type Detail struct {
	ID      string   `json:"id"`
	Name    string   `json:"name"`
	Notes   string   `json:"notes"`
	Extends string   `json:"extends"`
	// Kind ist die Art der Welt: "spinoff" (Ableger oder Episode) oder "new" (komplett neu); fehlt das Feld in der Datei,
	// gilt spinoff mit extends, sonst new (B-1217, wie sim.worldKind).
	Kind    string   `json:"kind"`
	// Gewaesser: Gewässer der Welt (Fluss, See, Küste, Meer); Untertitel in der Weltenliste.
	Gewaesser string `json:"gewaesser"`
	Islands []Island `json:"islands"`
	// Story ist die Geschichte der Welt (story.go, B-975); nil = fehlt.
	Story *Story `json:"story,omitempty"`
	// BiomeInfo: Palette und Portal-HP je Biom-ID aus data/biomes/*.json (nur die Biome der Inseln).
	BiomeInfo map[string]BiomeInfo `json:"biomeInfo"`
	Error     string               `json:"error"`
}

// Editor ist der Welten-Editor eines Checkouts: Root ist die Repo-Wurzel, Engine die Engine-Seite (simcli world).
type Editor struct {
	Root   string
	Engine Engine
}

// file ist der Pfad der Datei einer Welt; id ist vorher gegen idPattern geprüft.
func (e Editor) file(id string) string { return filepath.Join(e.Root, worldsDir, id+".json") }

// checkID lehnt eine ID außerhalb des Musters ab, bevor ein Pfad entsteht.
func checkID(id string) error {
	if !idPattern.MatchString(id) {
		return fmt.Errorf("ungültige Welt-ID %q", id)
	}
	return nil
}

// read liest die Datei der Welt id nach Prüfung der ID.
func (e Editor) read(id string) ([]byte, error) {
	if err := checkID(id); err != nil {
		return nil, err
	}
	raw, err := os.ReadFile(e.file(id))
	if err != nil {
		return nil, fmt.Errorf("%s/%s.json: %w", worldsDir, id, err)
	}
	return raw, nil
}

// List liefert die festen Welten (Schlüssel von data/paper.json › weltbilder in Datei-Reihenfolge; ohne Datei mit
// Missing) und danach jede weitere Datei unter data/worlds/ nach ID. Die Standard-Welt steht zuerst, auch ohne
// Eintrag in paper.json. Ursache von B-1168: Die Liste las früher nur die Dateien und ließ feste Welten ohne Datei weg.
func (e Editor) List() []Info {
	entries, _ := os.ReadDir(filepath.Join(e.Root, worldsDir))
	files := map[string]Info{}
	for _, d := range entries {
		id, ok := strings.CutSuffix(d.Name(), ".json")
		if d.IsDir() || !ok || !idPattern.MatchString(id) {
			continue
		}
		raw, _ := os.ReadFile(e.file(id))
		files[id] = Info{ID: id, Name: nameOf(raw, id), Kind: kindOfFile(raw), Gewaesser: gewaesserOf(raw)}
	}
	out := []Info{}
	add := func(id string) {
		if info, ok := files[id]; ok {
			out = append(out, info)
			delete(files, id)
		} else {
			out = append(out, Info{ID: id, Name: id, Missing: true})
		}
	}
	add(standardID)
	for _, id := range e.fixedIDs() {
		if id != standardID {
			add(id)
		}
	}
	rest := make([]string, 0, len(files))
	for id := range files {
		rest = append(rest, id)
	}
	sort.Strings(rest)
	for _, id := range rest {
		add(id)
	}
	return out
}

// fixedIDs liefert die IDs der festen Welten aus data/paper.json › weltbilder in Datei-Reihenfolge (fehlt die Datei
// oder der Schlüssel: keine).
func (e Editor) fixedIDs() []string {
	raw, err := os.ReadFile(filepath.Join(e.Root, "data", "paper.json"))
	if err != nil {
		return nil
	}
	var doc struct {
		Weltbilder json.RawMessage `json:"weltbilder"`
	}
	if json.Unmarshal(raw, &doc) != nil || len(doc.Weltbilder) == 0 {
		return nil
	}
	dec := json.NewDecoder(strings.NewReader(string(doc.Weltbilder)))
	if tok, err := dec.Token(); err != nil || tok != json.Delim('{') {
		return nil
	}
	var ids []string
	for dec.More() {
		key, err := dec.Token()
		name, ok := key.(string)
		if err != nil || !ok {
			break
		}
		if idPattern.MatchString(name) {
			ids = append(ids, name)
		}
		var skip json.RawMessage
		if dec.Decode(&skip) != nil {
			break
		}
	}
	return ids
}

func nameOf(raw []byte, id string) string {
	var head struct {
		Name string `json:"name"`
	}
	if json.Unmarshal(raw, &head) == nil && head.Name != "" {
		return head.Name
	}
	return id
}

// Load liest die Welt id mit ihren Inseln; das Biom je Tiefe kommt aus data/biomes/*.json (Feld depth).
func (e Editor) Load(id string) Detail {
	view := Detail{ID: id, Islands: []Island{}, BiomeInfo: map[string]BiomeInfo{}}
	raw, err := e.read(id)
	if err != nil {
		view.Error = err.Error()
		return view
	}
	var f struct {
		Name, Notes, Extends, Kind, Gewaesser string
		Islands              []Island `json:"islands"`
		Story                *Story   `json:"story"`
	}
	if err := json.Unmarshal(raw, &f); err != nil {
		view.Error = fmt.Sprintf("Welt-Datei: %v", err)
		return view
	}
	view.Name, view.Notes, view.Extends, view.Story = nameOf(raw, id), f.Notes, f.Extends, f.Story
	view.Kind, view.Gewaesser = kindOf(f.Kind, f.Extends), f.Gewaesser
	biomes := e.biomesByDepth()
	for _, isl := range f.Islands {
		isl.Biomes = []string{}
		for _, d := range isl.Depths {
			isl.Biomes = append(isl.Biomes, biomes[d])
			if _, ok := view.BiomeInfo[biomes[d]]; !ok && biomes[d] != "" {
				view.BiomeInfo[biomes[d]] = e.biomeInfo(biomes[d])
			}
		}
		if isl.Planned == nil {
			isl.Planned = map[string]json.RawMessage{}
		}
		if isl.ChunkCounts == nil {
			isl.ChunkCounts = map[string]int{}
		}
		view.Islands = append(view.Islands, isl)
	}
	return view
}

// biomesByDepth ordnet jeder Tiefe die ID ihres Bioms zu (Dateiname unter data/biomes/, Feld depth).
func (e Editor) biomesByDepth() map[int]string {
	out := map[int]string{}
	files, _ := filepath.Glob(filepath.Join(e.Root, "data", "biomes", "*.json"))
	for _, f := range files {
		raw, err := os.ReadFile(f)
		var b struct {
			Depth *int `json:"depth"`
		}
		if err == nil && json.Unmarshal(raw, &b) == nil && b.Depth != nil {
			out[*b.Depth] = strings.TrimSuffix(filepath.Base(f), ".json")
		}
	}
	return out
}

// BiomeInfo sind die Anzeige-Werte eines Bioms aus data/biomes/<id>.json: Palette des Querschnitts und Portal-HP.
type BiomeInfo struct {
	Palette  map[string]string `json:"palette"`
	PortalHP int               `json:"portalHp"`
}

// biomeInfo liest Palette (palette) und Portal-HP (portals.hp) des Bioms id; fehlt etwas, bleibt es leer bzw. 0.
func (e Editor) biomeInfo(id string) BiomeInfo {
	info := BiomeInfo{Palette: map[string]string{}}
	raw, err := os.ReadFile(filepath.Join(e.Root, "data", "biomes", id+".json"))
	if err != nil {
		return info
	}
	var b struct {
		Palette map[string]string `json:"palette"`
		Portals struct {
			HP int `json:"hp"`
		} `json:"portals"`
	}
	if json.Unmarshal(raw, &b) == nil {
		if b.Palette != nil {
			info.Palette = b.Palette
		}
		info.PortalHP = b.Portals.HP
	}
	return info
}

// kindOf löst die Art der Welt auf wie sim.worldKind: ohne Angabe spinoff mit extends, sonst new.
func kindOf(kind, extends string) string {
	if kind != "" {
		return kind
	}
	if extends != "" {
		return "spinoff"
	}
	return "new"
}

// kindOfFile liest die Art der Welt aus den Rohdaten der Datei (Feld kind, sonst aus extends).
func kindOfFile(raw []byte) string {
	var h struct{ Kind, Extends string }
	_ = json.Unmarshal(raw, &h)
	return kindOf(h.Kind, h.Extends)
}

// gewaesserOf liest das Gewässer aus den Rohdaten der Datei.
func gewaesserOf(raw []byte) string {
	var h struct{ Gewaesser string }
	_ = json.Unmarshal(raw, &h)
	return h.Gewaesser
}
