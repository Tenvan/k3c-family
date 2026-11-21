package sim

import (
	"encoding/json"
	"errors"
	"fmt"

	"k3c/engine/level"
)

// Lesen des Kopfes einer Welt-Datei (welten.md § 2): Rohform und strenges Dekodieren, ausgelagert aus island_worlds.go.

// Rohform der Datei: Zeiger erkennen fehlende Pflichtfelder, Rohdaten lassen jede Insel einzeln lesen.
type rawWorld struct {
	ID        *string                    `json:"id"`
	Name      *string                    `json:"name"`
	Notes     string                     `json:"notes"`
	Extends   *string                    `json:"extends"`
	Kind      *string                    `json:"kind"`
	Gewaesser string                     `json:"gewaesser"`
	Overrides map[string]json.RawMessage `json:"overrides"`
	Combat    *rawCombat                 `json:"combat"`
	Islands   *[]json.RawMessage         `json:"islands"`
	Story     json.RawMessage            `json:"story"`
	Skins     json.RawMessage            `json:"skins"`
}

type rawIsland struct {
	ID            *string                         `json:"id"`
	Name          *string                         `json:"name"`
	Etappe        *int                            `json:"etappe"`
	Depths        *[]int                          `json:"depths"`
	DepthScaling  json.RawMessage                 `json:"depthScaling"`
	PlannedChunks map[string][]level.PlannedChunk `json:"plannedChunks"`
	ChunkCounts   map[string]int                  `json:"chunkCounts"`
	EndBoss       string                          `json:"endBoss"`
	Nature        json.RawMessage                 `json:"nature"`
	NatureByDepth map[string]json.RawMessage      `json:"natureByDepth"`
	Story         json.RawMessage                 `json:"story"`
}

type rawScaling struct {
	HP     *float64 `json:"hp"`
	Damage *float64 `json:"damage"`
	Speed  *float64 `json:"speed"`
}

// decodeWorld liest den Kopf der Welt streng und prüft Pflichtfelder und die ID; die Inseln bleiben roh.
func decodeWorld(raw []byte, id string) (WorldDef, []json.RawMessage, error) {
	var v rawWorld
	if err := strictDecode(raw, &v, "id", "name", "notes", "extends", "kind", "gewaesser", "overrides", "combat", "islands", "story", "skins"); err != nil {
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
	if err := checkWorldLinks(id, v.Extends, v.Overrides); err != nil {
		return WorldDef{}, nil, err
	}
	kind, err := worldKind(v.Kind, v.Extends)
	if err != nil {
		return WorldDef{}, nil, err
	}
	combat, err := v.Combat.settings()
	if err != nil {
		return WorldDef{}, nil, err
	}
	w := WorldDef{ID: *v.ID, Name: *v.Name, Notes: v.Notes, Overrides: v.Overrides, Combat: combat, Kind: kind, Gewaesser: v.Gewaesser}
	if v.Extends != nil {
		w.Extends = *v.Extends
	}
	if w.Story, err = decodeWorldStory(v.Story); err != nil {
		return WorldDef{}, nil, err
	}
	if w.Skins, err = decodeWorldSkins(v.Skins); err != nil {
		return WorldDef{}, nil, err
	}
	return w, *v.Islands, nil
}
