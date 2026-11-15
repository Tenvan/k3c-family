package simcli

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"maps"
	"slices"
	"strconv"
	"strings"
	"unicode/utf8"

	"k3c/data"
	"k3c/engine/level"
	"k3c/engine/sim"
)

// `k3c-sim world` (B-879): die Engine-Seite des Welten-Editors der Workbench. Die Welten liest k3c-sim aus data/worlds
// (K3C_DATA_DIR des Checkouts); es läuft keine Simulation, nur Level-Generator und die Prüfung der Welt-Werte.
//
//	{"op":"level","world","island","stage","seed"}  Level der Insel-Stufe: Layout, planned, warnings
//	{"op":"values","world"}                         Basis der Welt (Basis data/ + Eltern-Welten) je Datei, extends, overrides, children
//	{"op":"check","world","draft"}                  Entwurf der Welt-Datei prüfen wie der Server, mit allen Nachfolgern: warnings
//
// Antwort: {"event":"result","report":{…}}; ein Fehler (ungültiger Entwurf, unbekannte Insel) ist {"event":"error"}.

// worldInput ist die Eingabe von `k3c-sim world`.
type worldInput struct {
	Op     string          `json:"op"`
	World  string          `json:"world"`
	Island string          `json:"island"`
	Stage  int             `json:"stage"`
	Seed   string          `json:"seed"`
	Draft  json.RawMessage `json:"draft"`
}

// maxWorldSeedRunes ist die Länge des Seeds wie room.MaxSeedRunes.
const maxWorldSeedRunes = 64

func cmdWorld(args []string, v env) error {
	var in worldInput
	if err := readInput("world", args, v.stdin, &in); err != nil {
		return err
	}
	base := v.dataFiles()
	worlds, err := fs.Sub(base, "worlds")
	if err != nil {
		return err
	}
	var report any
	switch in.Op {
	case "level":
		report, err = worldLevel(base, worlds, in)
	case "values":
		report, err = worldValues(base, worlds, in.World)
	case "check":
		report, err = worldCheck(base, worlds, in.World, in.Draft)
	default:
		return usagef("unbekannte op %q (bekannt: level, values, check)", in.Op)
	}
	if err != nil {
		return err
	}
	v.out.Emit(map[string]any{"event": "result", "report": report})
	return nil
}

// worldLevelReport ist das Level wie im Spiel plus die geplanten Chunks der Stufe und die Warnungen der Prüfung.
// Natur (B-891): `nature` (im Layout) sind die geltenden Werte, `natureFixed` die in der Welt fest gesetzten,
// `natureTraits` und `natureWeights` die Eigenschaften und Gewichts-Gruppen des Bioms mit ihren Bereichen.
type worldLevelReport struct {
	level.Layout
	Planned       []level.PlannedChunk `json:"planned"`
	Warnings      []string             `json:"warnings"`
	Nature        level.NatureValues   `json:"nature"`
	NatureFixed   level.NatureValues   `json:"natureFixed"`
	NatureTraits  []natureTrait        `json:"natureTraits"`
	NatureWeights []natureWeights      `json:"natureWeights"`
}

type natureTrait struct {
	Key string `json:"key"`
	level.Trait
}

type natureWeights struct {
	Key     string         `json:"key"`
	Name    string         `json:"name"`
	Kind    string         `json:"kind"`
	Options []natureOption `json:"options"`
}

type natureOption struct {
	Key string `json:"key"`
	level.Range
}

// natureOf liefert Eigenschaften und Gewichts-Gruppen des Bioms in Datei-Reihenfolge (nie nil).
func natureOf(b level.Biome) ([]natureTrait, []natureWeights) {
	traits, groups := []natureTrait{}, []natureWeights{}
	for _, e := range b.Traits {
		traits = append(traits, natureTrait{e.Key, e.Value})
	}
	for _, g := range b.Weights {
		w := natureWeights{Key: g.Key, Name: g.Value.Name, Kind: g.Value.Kind, Options: []natureOption{}}
		for _, o := range g.Value.Options {
			w.Options = append(w.Options, natureOption{o.Key, o.Value})
		}
		groups = append(groups, w)
	}
	return traits, groups
}

func worldLevel(base data.FS, worlds fs.FS, in worldInput) (worldLevelReport, error) {
	if n := utf8.RuneCountInString(in.Seed); n == 0 || n > maxWorldSeedRunes {
		return worldLevelReport{}, fmt.Errorf("der Seed braucht 1 bis %d Zeichen", maxWorldSeedRunes)
	}
	def, err := sim.LoadWorld(worlds, in.World)
	if err != nil {
		return worldLevelReport{}, err
	}
	at := slices.IndexFunc(def.Islands, func(i sim.IslandDef) bool { return i.ID == in.Island })
	if at < 0 {
		return worldLevelReport{}, fmt.Errorf("die Insel %q gibt es nicht", in.Island)
	}
	isl := def.Islands[at]
	if in.Stage < 0 || in.Stage >= len(isl.Depths) {
		return worldLevelReport{}, fmt.Errorf("die Stufe %d gibt es auf der Insel %q nicht", in.Stage, in.Island)
	}
	depth := isl.Depths[in.Stage]
	b, err := biomeForDepth(base, depth)
	if err != nil {
		return worldLevelReport{}, err
	}
	key := strconv.Itoa(depth)
	vals, err := sim.ResolveWorldValues(base, func(w string) (sim.WorldDef, error) { return sim.LoadWorld(worlds, w) }, in.World)
	if err != nil {
		return worldLevelReport{}, err
	}
	opts := level.Options{Planned: isl.PlannedChunks[key], ChunkCount: isl.ChunkCounts[key], Nature: isl.NatureAt(depth), HubSites: vals.Hub.LevelSites()}
	layout, err := level.GenerateWith(b, in.Seed, opts)
	if err != nil {
		return worldLevelReport{}, fmt.Errorf("das Level ist nicht erzeugbar: %w", err)
	}
	r := worldLevelReport{Layout: layout, Planned: opts.Planned, Warnings: level.ValidateWith(layout, b, opts), Nature: layout.Nature, NatureFixed: opts.Nature}
	r.NatureTraits, r.NatureWeights = natureOf(b)
	if r.Planned == nil {
		r.Planned = []level.PlannedChunk{}
	}
	if r.Warnings == nil {
		r.Warnings = []string{}
	}
	return r, nil
}

// biomeForDepth ist das Biom der Tiefe depth (Feld depth der Biom-Dateien).
func biomeForDepth(files data.FS, depth int) (level.Biome, error) {
	entries, err := files.ReadDir("biomes")
	if err != nil {
		return level.Biome{}, err
	}
	for _, e := range entries {
		b, err := loadBiomeFrom(files, e.Name())
		if err == nil && b.Depth == depth {
			return b, nil
		}
	}
	return level.Biome{}, fmt.Errorf("kein Biom für Tiefe %d", depth)
}

// worldValuesReport ist die Basis der Welt je Basis-Datei (ohne die eigenen Overrides), ihre Overrides und die
// Nachfolger (alle Welten, die direkt oder über Zwischenwelten von ihr erben; dieselben, die check prüft).
type worldValuesReport struct {
	Extends   string                     `json:"extends"`
	Children  []string                   `json:"children"`
	Base      map[string]json.RawMessage `json:"base"`
	Overrides map[string]json.RawMessage `json:"overrides"`
}

func worldValues(base data.FS, worlds fs.FS, id string) (worldValuesReport, error) {
	def, err := sim.LoadWorld(worlds, id)
	if err != nil {
		return worldValuesReport{}, err
	}
	files, err := sim.ResolveWorldFiles(base, func(w string) (sim.WorldDef, error) { return sim.LoadWorld(worlds, w) }, def.Extends)
	if err != nil {
		return worldValuesReport{}, err
	}
	r := worldValuesReport{Extends: def.Extends, Children: append([]string{}, descendants(worlds, id)...), Base: map[string]json.RawMessage{}, Overrides: def.Overrides}
	for k, v := range files {
		r.Base[k] = v
	}
	if r.Overrides == nil {
		r.Overrides = map[string]json.RawMessage{}
	}
	return r, nil
}

// worldCheck prüft den Entwurf der Welt-Datei wie der Server (sim.ResolveWorldValues): die Welt und jeden Nachfolger.
// Ein Nachfolger blockiert nur, wenn der Entwurf ihn neu oder anders ungültig macht; war er schon gleich ungültig oder
// lädt er nicht, kommt eine Warnung zurück.
func worldCheck(base data.FS, worlds fs.FS, id string, draft json.RawMessage) (map[string][]string, error) {
	def, err := sim.ParseWorld(draft, id)
	if err != nil {
		return nil, err
	}
	disk := func(w string) (sim.WorldDef, error) { return sim.LoadWorld(worlds, w) }
	withDraft := func(w string) (sim.WorldDef, error) {
		if w == id {
			return def, nil
		}
		return disk(w)
	}
	if _, err := sim.ResolveWorldValues(base, withDraft, id); err != nil {
		return nil, err
	}
	warnings := []string{}
	for _, kid := range descendants(worlds, id) {
		if _, bad := disk(kid); bad != nil {
			warnings = append(warnings, fmt.Sprintf("Welt %s erbt von %s, ist aber ungültig und nicht geprüft: %v", kid, id, bad))
			continue
		}
		_, err := sim.ResolveWorldValues(base, withDraft, kid)
		if err == nil {
			continue
		}
		if _, was := sim.ResolveWorldValues(base, disk, kid); was != nil && was.Error() == err.Error() {
			warnings = append(warnings, fmt.Sprintf("Welt %s war schon vorher ungültig: %v", kid, err))
			continue
		}
		return nil, err
	}
	return map[string][]string{"warnings": warnings}, nil
}

// descendants sind alle Welten unter worlds, die direkt oder über Zwischenwelten von id erben, sortiert.
func descendants(worlds fs.FS, id string) []string {
	extends := map[string]string{}
	entries, _ := fs.ReadDir(worlds, ".")
	for _, e := range entries {
		w, ok := strings.CutSuffix(e.Name(), ".json")
		if !ok || e.IsDir() {
			continue
		}
		raw, err := fs.ReadFile(worlds, e.Name())
		var head struct {
			Extends string `json:"extends"`
		}
		if err == nil && json.Unmarshal(raw, &head) == nil {
			extends[w] = head.Extends
		}
	}
	seen := map[string]bool{id: true}
	for grew := true; grew; {
		grew = false
		for w, parent := range extends {
			if !seen[w] && seen[parent] {
				seen[w], grew = true, true
			}
		}
	}
	delete(seen, id)
	return slices.Sorted(maps.Keys(seen))
}

// loadBiomeFrom liest eine Biom-Datei (Name mit .json) aus files.
func loadBiomeFrom(files data.FS, name string) (level.Biome, error) {
	raw, err := files.ReadFile("biomes/" + name)
	if err != nil {
		return level.Biome{}, err
	}
	var b level.Biome
	err = json.Unmarshal(raw, &b)
	return b, err
}
