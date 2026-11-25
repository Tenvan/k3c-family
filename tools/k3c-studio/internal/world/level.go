package world

import (
	"context"
	"encoding/json"
	"time"

	"k3c/tools/k3c-studio/internal/gameexe"
)

// engineTimeout begrenzt einen Aufruf von sim world (Level und Prüfung brauchen Millisekunden).
const engineTimeout = 30 * time.Second

// Engine ist die Engine-Seite des Editors: input ist das JSON von `k3c-game sim world` ({"op": …}), die Antwort sein
// report. Ein Fehler ist die Meldung der Spiel-EXE (ungültiger Entwurf, unbekannte Insel) oder ein Startfehler.
type Engine func(ctx context.Context, input []byte) (json.RawMessage, error)

// Exe ist die Engine über die Spiel-EXE des Checkouts root (`k3c-game sim world`, RU34.3, internal/gameexe); ist sie nicht
// gebaut oder veraltet, nennt der Fehler den Bau-Befehl. Kein Rückfall auf die Go-Engine.
func Exe(root string) Engine {
	return func(ctx context.Context, input []byte) (json.RawMessage, error) {
		ctx, cancel := context.WithTimeout(ctx, engineTimeout)
		defer cancel()
		ev, err := gameexe.Call(ctx, gameexe.Options{Root: root, Sub: "world", Input: input})
		return ev.Report, err
	}
}

// call schickt eine op an die Engine und liest den report nach dst.
func (e Editor) call(ctx context.Context, req map[string]any, dst any) error {
	input, err := json.Marshal(req)
	if err != nil {
		return err
	}
	report, err := e.Engine(ctx, input)
	if err != nil {
		return err
	}
	return json.Unmarshal(report, dst)
}

// LevelView ist das Level wie im Spiel (Layout als JSON-Felder) plus die geplanten Chunks der Stufe und die Warnungen
// der Spielbarkeits-Prüfung. Error ist der Hinweis für die Seite, dann fehlt das Layout.
type LevelView struct {
	Seed            string            `json:"seed"`
	BiomeID         string            `json:"biomeId"`
	WidthUnits      float64           `json:"widthUnits"`
	ChunkWidthUnits float64           `json:"chunkWidthUnits"`
	HubCenterUnits  float64           `json:"hubCenterUnits"`
	Chunks          []json.RawMessage `json:"chunks"`
	Entities        []json.RawMessage `json:"entities"`
	Planned         []json.RawMessage `json:"planned"`
	Warnings        []string          `json:"warnings"`
	Error           string            `json:"error"`
	// Natur (B-891): geltende Werte, fest gesetzte Werte, Eigenschaften und Gewichts-Gruppen des Bioms (von der Spiel-EXE).
	Nature        json.RawMessage `json:"nature"`
	NatureFixed   json.RawMessage `json:"natureFixed"`
	NatureTraits  json.RawMessage `json:"natureTraits"`
	NatureWeights json.RawMessage `json:"natureWeights"`
	// Rejected ist der Grund, warum ein Festsetzen abgelehnt wurde (SetNature); das Level ist dann das alte.
	Rejected string `json:"rejected,omitempty"`
}

// Level holt das Level der Insel island in der Stufe stage (Index in ihren Tiefen) der Welt id zum Seed, mit der
// gespeicherten Planung und Chunk-Anzahl der Tiefe. Der Level-Generator ist deterministisch; es läuft keine Simulation.
func (e Editor) Level(ctx context.Context, id, island string, stage int, seed string) LevelView {
	var v LevelView
	err := checkID(id)
	if err == nil {
		err = e.call(ctx, map[string]any{"op": "level", "world": id, "island": island, "stage": stage, "seed": seed}, &v)
	}
	if err != nil {
		v = LevelView{Error: err.Error()}
	}
	for _, list := range []*[]json.RawMessage{&v.Chunks, &v.Entities, &v.Planned} {
		if *list == nil {
			*list = []json.RawMessage{} // immer Arrays, nie null
		}
	}
	if v.Warnings == nil {
		v.Warnings = []string{}
	}
	for _, f := range []struct {
		p     *json.RawMessage
		empty string
	}{{&v.Nature, "{}"}, {&v.NatureFixed, "{}"}, {&v.NatureTraits, "[]"}, {&v.NatureWeights, "[]"}} {
		if len(*f.p) == 0 || string(*f.p) == "null" {
			*f.p = json.RawMessage(f.empty)
		}
	}
	return v
}
