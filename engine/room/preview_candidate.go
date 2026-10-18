package room

import (
	"fmt"
	"strings"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// Vorschau aus einer Kandidaten-hub.json (WZ9.1, B-462): der Gebäude-Planer zeigt seinen ungespeicherten Stand, ohne
// EXE-Neubau und ohne sim.UseData. Geplante Chunks (WZ8.4) gibt es im Branch noch nicht; ihr Vorrang
// (Kandidaten-Linien) kommt mit B-479.

// CandidateError: Die Kandidaten-hub.json ist kein JSON oder hat Befunde der Prüfung; es entsteht keine Welt.
type CandidateError struct {
	Findings []sim.HubFinding
}

func (e *CandidateError) Error() string {
	parts := make([]string, len(e.Findings))
	for i, f := range e.Findings {
		parts[i] = f.String()
	}
	return fmt.Sprintf("vorschau: Kandidaten-hub.json mit %d Befunden: %s", len(e.Findings), strings.Join(parts, "; "))
}

// candidateWorld liest und prüft den Kandidaten einmal und baut die Stufe über sim.CreateWorldWith (ohne Insel, wie
// die Vorschau ohne Kandidat). Liefert dazu die Zahl der Hub-Stufen des Kandidaten.
func candidateWorld(biome level.Biome, seed string, raw []byte) (*sim.World, int, error) {
	h, err := sim.ParseHub(raw)
	if err != nil {
		return nil, 0, &CandidateError{Findings: []sim.HubFinding{{Rule: sim.RuleJSON, Site: "hub.json", Detail: err.Error()}}}
	}
	if r := sim.ValidateHubData(h); len(r.Findings) > 0 {
		return nil, 0, &CandidateError{Findings: r.Findings}
	}
	w, err := sim.CreateWorldWith(biome, seed, sim.CreateOptions{Candidate: h})
	if err != nil {
		return nil, 0, fmt.Errorf("vorschau: Level aus dem Kandidaten nicht erzeugbar: %w", err)
	}
	return w, len(h.Levels), nil
}
