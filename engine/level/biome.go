package level

import (
	"bytes"
	"encoding/json"
	"fmt"

	"k3c/data"
)

// Range ist ein Bereich min..max (beide inklusive).
type Range struct {
	Min int `json:"min"`
	Max int `json:"max"`
}

// Biome enthält die Felder aus data/biomes/*.json, die Level-Generator und Simulation brauchen (Port von src/world/biome.ts).
// Einheiten sind float64 wie in JavaScript, damit z. B. `cw / 2` auch bei ungerader Breite gleich rechnet.
type Biome struct {
	ID              string  `json:"id"`
	Name            string  `json:"name"`
	Depth           int     `json:"depth"`
	LengthUnits     Range   `json:"lengthUnits"`
	ChunkWidthUnits float64 `json:"chunkWidthUnits"`
	HubWidthUnits   float64 `json:"hubWidthUnits"`
	ExitSide        string  `json:"exitSide"`
	Portals         struct {
		Count                   int     `json:"count"`
		MinDistanceFromHubUnits float64 `json:"minDistanceFromHubUnits"`
	} `json:"portals"`
	ChunkWeights      Ordered[float64]         `json:"chunkWeights"`
	EventChunks       map[string]Range         `json:"eventChunks"`
	SkillPoints       Range                    `json:"skillPoints"`
	ResourcesPerChunk Ordered[Ordered[[2]int]] `json:"resourcesPerChunk"`
	Cycle             Cycle                    `json:"cycle"`
	PrimaryResource   string                   `json:"primaryResource"`
	Enemies           struct {
		Portal []string `json:"portal"`
		Night  []string `json:"night"`
	} `json:"enemies"`
}

// Cycle ist der Zyklus eines Bioms: `dayNight` (Oberwelt) oder `aggressionPool` (unter Tage). Die Simulation
// (engine/sim) braucht ihn, der Generator nicht.
type Cycle struct {
	Type             string  `json:"type"`
	DayMinutes       float64 `json:"dayMinutes"`
	TwilightMinutes  float64 `json:"twilightMinutes"`
	NightMinutes     float64 `json:"nightMinutes"`
	PercentPerMinute float64 `json:"percentPerMinute"`
	PercentPerGather float64 `json:"percentPerGather"`
	PercentPerKill   float64 `json:"percentPerKill"`
}

// LoadBiome liest data/biomes/<id>.json.
func LoadBiome(id string) (Biome, error) {
	raw, err := data.Files.ReadFile("biomes/" + id + ".json")
	if err != nil {
		return Biome{}, err
	}
	var b Biome
	if err := json.Unmarshal(raw, &b); err != nil {
		return Biome{}, fmt.Errorf("biomes/%s.json: %w", id, err)
	}
	return b, nil
}

// Entry ist ein Schlüssel eines JSON-Objekts mit seinem Wert.
type Entry[V any] struct {
	Key   string
	Value V
}

// Ordered ist ein JSON-Objekt in Datei-Reihenfolge. TS durchläuft Objekte mit `Object.entries` in
// Einfügereihenfolge; eine Go-map hätte eine zufällige Reihenfolge und damit andere Level.
type Ordered[V any] []Entry[V]

// UnmarshalJSON liest die Schlüssel der Reihe nach.
func (o *Ordered[V]) UnmarshalJSON(raw []byte) error {
	dec := json.NewDecoder(bytes.NewReader(raw))
	if tok, err := dec.Token(); err != nil || tok != json.Delim('{') {
		return fmt.Errorf("JSON-Objekt erwartet: %s", raw)
	}
	*o = (*o)[:0]
	for dec.More() {
		tok, err := dec.Token()
		if err != nil {
			return err
		}
		var v V
		if err := dec.Decode(&v); err != nil {
			return err
		}
		*o = append(*o, Entry[V]{Key: tok.(string), Value: v})
	}
	return nil
}

// Get liefert den Wert zu key.
func (o Ordered[V]) Get(key string) (V, bool) {
	for _, e := range o {
		if e.Key == key {
			return e.Value, true
		}
	}
	var zero V
	return zero, false
}
