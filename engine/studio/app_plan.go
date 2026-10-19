package studio

import (
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"strconv"

	"k3c/data"
	"k3c/engine/level"
	"k3c/engine/sim"
)

// Planung geplanter Chunks im Welt-Betrachter (WZ8.5, B-461): Laden, Prüfen und Speichern von
// data/islands.json › plannedChunks. Der Editor prüft den Entwurf bei jeder Änderung; gespeichert wird nur Gültiges.

// PlanIsland ist eine Insel mit ihren Tiefen und der gespeicherten Planung (Tiefe → Chunks).
type PlanIsland struct {
	ID      string                          `json:"id"`
	Name    string                          `json:"name"`
	Depths  []int                           `json:"depths"`
	Biomes  []string                        `json:"biomes"` // Biom je Tiefe, gleiche Reihenfolge wie Depths
	Planned map[string][]level.PlannedChunk `json:"planned"`
	// ChunkCounts sind die gespeicherten Chunk-Anzahlen (Tiefe → Inselgröße, WZ17.5); ohne Eintrag gilt die Biom-Länge.
	ChunkCounts map[string]int `json:"chunkCounts"`
}

// PlanView ist data/islands.json für den Editor. Stand ist der Hash der Datei beim Lesen; PlanSave braucht ihn.
type PlanView struct {
	Islands []PlanIsland     `json:"islands"`
	Sizes   []sim.IslandSize `json:"sizes"` // Inselgrößen zur Auswahl im Editor (data/island-sizes.json)
	Stand   string           `json:"stand"`
	Error   string           `json:"error"`
}

// PlanCheckView ist das Ergebnis der Prüfung eines Entwurfs: Slot nennt den Platz des Fehlers (0 = ohne Platz).
type PlanCheckView struct {
	OK    bool   `json:"ok"`
	Slot  int    `json:"slot"`
	Error string `json:"error"`
}

// PlanSaved ist das Ergebnis von PlanSave: neuer Stand und Hinweis, oder Error (Stale: Datei von außen geändert).
type PlanSaved struct {
	Stand string `json:"stand"`
	Hint  string `json:"hint"`
	Error string `json:"error"`
	Stale bool   `json:"stale"`
}

var planSlot = regexp.MustCompile(`auf Platz ([+-]\d+)`)

// biomeIDForDepth liefert das Biom der Tiefe aus den eingebetteten Daten.
func biomeIDForDepth(depth int) (level.Biome, error) {
	files, err := fs.Glob(data.Files, "biomes/*.json")
	if err != nil {
		return level.Biome{}, err
	}
	for _, f := range files {
		b, err := level.LoadBiome(filepath.Base(f[:len(f)-len(".json")]))
		if err == nil && b.Depth == depth {
			return b, nil
		}
	}
	return level.Biome{}, fmt.Errorf("kein Biom für Tiefe %d", depth)
}

func (a *App) readIslands() ([]byte, error) {
	return os.ReadFile(a.worldFile(a.activeWorld()))
}

// PlanLoad liest data/islands.json frisch (Binding).
func (a *App) PlanLoad() PlanView {
	view := PlanView{Islands: []PlanIsland{}, Sizes: sim.IslandSizes()}
	raw, err := a.readIslands()
	if err != nil {
		view.Error = err.Error()
		return view
	}
	var f struct {
		Islands []struct {
			ID      string                          `json:"id"`
			Name    string                          `json:"name"`
			Depths  []int                           `json:"depths"`
			Planned map[string][]level.PlannedChunk `json:"plannedChunks"`
			Counts  map[string]int                  `json:"chunkCounts"`
		} `json:"islands"`
	}
	if err := json.Unmarshal(raw, &f); err != nil {
		view.Error = fmt.Sprintf("%s: %v", islandsPath, err)
		return view
	}
	for _, file := range f.Islands {
		isl := PlanIsland{ID: file.ID, Name: file.Name, Depths: file.Depths, Planned: file.Planned, Biomes: []string{}, ChunkCounts: file.Counts}
		for _, d := range isl.Depths {
			b, err := biomeIDForDepth(d)
			if err != nil {
				view.Error = err.Error()
				return view
			}
			isl.Biomes = append(isl.Biomes, b.ID)
		}
		if isl.Planned == nil {
			isl.Planned = map[string][]level.PlannedChunk{}
		}
		if isl.ChunkCounts == nil {
			isl.ChunkCounts = map[string]int{}
		}
		view.Islands = append(view.Islands, isl)
	}
	view.Stand = planStand(raw)
	return view
}

// checkPlan prüft einen Entwurf für Insel und Tiefe mit der Chunk-Anzahl count (0 = Biom-Länge): die Anzahl gegen die
// Inselgrößen, die Planung mit level.ValidatePlanFor (Plätze gegen die Anzahl, Mauerlinien die eingebetteten).
func (a *App) checkPlan(raw []byte, island string, depth int, chunks []level.PlannedChunk, count int) error {
	if err := checkIslandSize(count); err != nil {
		return err
	}
	var f struct {
		Islands []struct {
			ID     string `json:"id"`
			Depths []int  `json:"depths"`
		} `json:"islands"`
	}
	if err := json.Unmarshal(raw, &f); err != nil {
		return fmt.Errorf("%s: %w", islandsPath, err)
	}
	for _, isl := range f.Islands {
		if isl.ID != island {
			continue
		}
		for _, d := range isl.Depths {
			if d == depth {
				b, err := biomeIDForDepth(depth)
				if err != nil {
					return err
				}
				return level.ValidatePlanFor(b, chunks, level.DefaultWallLines(), count)
			}
		}
		return fmt.Errorf("die Tiefe %d gehört nicht zur Insel %q", depth, island)
	}
	return fmt.Errorf("die Insel %q gibt es nicht", island)
}

// PlanCheck prüft einen Entwurf (Planung und Chunk-Anzahl, 0 = Biom-Länge), ohne zu schreiben (Binding). Der Fehler
// nennt den Platz („auf Platz +3: …“); ein Fehler der Anzahl hat keinen Platz (Slot 0).
func (a *App) PlanCheck(island string, depth int, chunks []level.PlannedChunk, count int) PlanCheckView {
	raw, err := a.readIslands()
	if err != nil {
		return PlanCheckView{Error: err.Error()}
	}
	if err := a.checkPlan(raw, island, depth, chunks, count); err != nil {
		view := PlanCheckView{Error: err.Error()}
		if m := planSlot.FindStringSubmatch(err.Error()); m != nil {
			view.Slot, _ = strconv.Atoi(m[1])
		}
		return view
	}
	return PlanCheckView{OK: true}
}

// PlanSave ersetzt Planung und Chunk-Anzahl (0 = Biom-Länge, Eintrag entfällt) der Tiefe einer Insel in
// data/islands.json (Binding). Nur gültiger Entwurf, nur auf dem Stand aus PlanLoad (sonst Stale ohne Schreibzugriff),
// format-stabil und atomar in einem Schreibvorgang; danach der REG-Hinweis.
func (a *App) PlanSave(island string, depth int, chunks []level.PlannedChunk, count int, stand string) PlanSaved {
	path := a.worldFile(a.activeWorld())
	raw, err := a.readIslands()
	if err != nil {
		return PlanSaved{Error: err.Error()}
	}
	if planStand(raw) != stand {
		return PlanSaved{Error: errPlanStale.Error(), Stale: true}
	}
	if err := a.checkPlan(raw, island, depth, chunks, count); err != nil {
		return PlanSaved{Error: err.Error()}
	}
	out, err := patchPlanned(raw, island, depth, chunks)
	if err == nil {
		out, err = patchCounts(out, island, depth, count)
	}
	if err != nil {
		return PlanSaved{Error: err.Error()}
	}
	if string(out) != string(raw) {
		if err := writeAtomic(path, out); err != nil {
			return PlanSaved{Error: err.Error()}
		}
	}
	return PlanSaved{Stand: planStand(out), Hint: planRegHint}
}

// resolvedSource ist die aufgelöste Quelle: Biom, geplante Chunks und Chunk-Anzahl (0 = Biom-Länge).
type resolvedSource struct {
	biome   string
	planned []level.PlannedChunk
	count   int
}

// resolveSource löst die Quelle auf. Bei einer Insel wählt Stage den Index in ihren Tiefen; ohne Entwurf (nil) gelten
// die gespeicherte Planung und Chunk-Anzahl der Tiefe. Eine Anzahl, die keine Inselgröße ist, ist ein Fehler.
func (a *App) resolveSource(src WorldSource) (resolvedSource, error) {
	r, err := a.resolveIsland(src)
	if err != nil {
		return resolvedSource{}, err
	}
	if src.ChunkCount != nil {
		r.count = *src.ChunkCount
	}
	return r, checkIslandSize(r.count)
}

// resolveIsland löst Biom bzw. Insel und Stufe auf, mit Entwurf oder gespeicherter Planung und Anzahl.
func (a *App) resolveIsland(src WorldSource) (resolvedSource, error) {
	if src.Island == "" {
		if src.Stage != 0 {
			return resolvedSource{}, fmt.Errorf("eine Stufe braucht eine Insel")
		}
		r := resolvedSource{biome: src.Biome}
		if src.Planned != nil {
			r.planned = *src.Planned
		}
		return r, nil
	}
	view := a.PlanLoad()
	if view.Error != "" {
		return resolvedSource{}, errors.New(view.Error)
	}
	for _, isl := range view.Islands {
		if isl.ID != src.Island {
			continue
		}
		if src.Stage < 0 || src.Stage >= len(isl.Depths) {
			return resolvedSource{}, fmt.Errorf("die Stufe %d gibt es auf der Insel %q nicht", src.Stage, isl.ID)
		}
		depth := strconv.Itoa(isl.Depths[src.Stage])
		r := resolvedSource{biome: isl.Biomes[src.Stage], planned: isl.Planned[depth], count: isl.ChunkCounts[depth]}
		if src.Planned != nil {
			r.planned = *src.Planned
		}
		return r, nil
	}
	return resolvedSource{}, fmt.Errorf("die Insel %q gibt es nicht", src.Island)
}
