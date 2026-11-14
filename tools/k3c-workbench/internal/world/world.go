// Package world ist das Backend des Welten-Editors der Workbench (B-879, Port aus dem früheren Asset-Studio, Commit
// 616b5afcb): Welten unter data/worlds/ auflisten und lesen, das Level einer Insel-Stufe für die Chunk-Vorschau holen
// und Werte einer Welt übersteuern (`overrides`, B-680, docs/rules/welten.md § 1). Die Workbench importiert den
// Engine-Kern nicht (B-726): Level, Basis-Werte und die Prüfung rechnet `bin/k3c-sim world` (Engine, engine.go).
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
}

// Island ist eine Insel der Welt mit dem Biom je Tiefe (gleiche Reihenfolge wie Depths), ihrer Planung und Chunk-Anzahl
// wie in der Datei.
type Island struct {
	ID          string                     `json:"id"`
	Name        string                     `json:"name"`
	Depths      []int                      `json:"depths"`
	Biomes      []string                   `json:"biomes"`
	Planned     map[string]json.RawMessage `json:"plannedChunks"`
	ChunkCounts map[string]int             `json:"chunkCounts"`
}

// Detail ist eine Welt mit Kopf und Inseln; Error nennt, warum sie nicht lesbar ist (dann ohne Inseln).
type Detail struct {
	ID      string   `json:"id"`
	Name    string   `json:"name"`
	Notes   string   `json:"notes"`
	Extends string   `json:"extends"`
	Islands []Island `json:"islands"`
	Error   string   `json:"error"`
}

// Editor ist der Welten-Editor eines Checkouts: Root ist die Repo-Wurzel, Engine die Engine-Seite (bin/k3c-sim world).
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

// List liefert alle Welten unter data/worlds/: die Standard-Welt zuerst, dann nach ID.
func (e Editor) List() []Info {
	entries, _ := os.ReadDir(filepath.Join(e.Root, worldsDir))
	out := []Info{}
	for _, d := range entries {
		id, ok := strings.CutSuffix(d.Name(), ".json")
		if d.IsDir() || !ok || !idPattern.MatchString(id) {
			continue
		}
		raw, _ := os.ReadFile(e.file(id))
		out = append(out, Info{ID: id, Name: nameOf(raw, id)})
	}
	sort.SliceStable(out, func(i, j int) bool {
		if (out[i].ID == standardID) != (out[j].ID == standardID) {
			return out[i].ID == standardID
		}
		return out[i].ID < out[j].ID
	})
	return out
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
	view := Detail{ID: id, Islands: []Island{}}
	raw, err := e.read(id)
	if err != nil {
		view.Error = err.Error()
		return view
	}
	var f struct {
		Name, Notes, Extends string
		Islands              []Island `json:"islands"`
	}
	if err := json.Unmarshal(raw, &f); err != nil {
		view.Error = fmt.Sprintf("Welt-Datei: %v", err)
		return view
	}
	view.Name, view.Notes, view.Extends = nameOf(raw, id), f.Notes, f.Extends
	biomes := e.biomesByDepth()
	for _, isl := range f.Islands {
		isl.Biomes = []string{}
		for _, d := range isl.Depths {
			isl.Biomes = append(isl.Biomes, biomes[d])
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
