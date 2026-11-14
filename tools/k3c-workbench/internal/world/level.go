package world

import (
	"context"
	"encoding/json"
	"time"

	"k3c/tools/k3c-workbench/internal/simexe"
)

// engineTimeout begrenzt einen Aufruf von bin/k3c-sim world (Level und Prüfung brauchen Millisekunden).
const engineTimeout = 30 * time.Second

// Engine ist die Engine-Seite des Editors: input ist das JSON von `k3c-sim world` ({"op": …}), die Antwort sein
// report. Ein Fehler ist die Meldung von k3c-sim (ungültiger Entwurf, unbekannte Insel) oder ein Startfehler.
type Engine func(ctx context.Context, input []byte) (json.RawMessage, error)

// SimExe ist die Engine über bin/k3c-sim des Checkouts root (B-726: die Workbench importiert den Engine-Kern nicht).
func SimExe(root string) Engine {
	return func(ctx context.Context, input []byte) (json.RawMessage, error) {
		ev, err := simexe.Call(ctx, root, []string{"world"}, input, engineTimeout)
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
	return v
}
