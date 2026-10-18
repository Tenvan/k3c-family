package main

import (
	"k3c/engine/net"
	"k3c/engine/room"
)

// Live-Vorschau des Welt-Betrachters (B-460, WZ7.3): die Welt in dem Format, das ein Gerät im Spiel bekommt, aus der
// Vorschau-Funktion des Servers (net.Preview), direkt aufgerufen, ohne Raum, Spielstand und HTTP-Endpunkt.

// WorldPreviewArgs sind die Optionen der Live-Vorschau (JSON-Namen der Oberfläche; Nullwerte wie in room.PreviewOptions:
// höchste Hub-Stufe, Tag, alle Gebäude).
type WorldPreviewArgs struct {
	Seed     string          `json:"seed"`
	Source   WorldSource     `json:"source"`
	HubLevel int             `json:"hubLevel"`
	Phase    string          `json:"phase"`
	Progress float64         `json:"progress"`
	Off      map[string]bool `json:"buildings"` // Schlüssel kind@x; false schaltet den Platz aus
	// HubCandidate ist der Planer-Stand (Gebäude-Planer, WZ9.2), auch ungespeichert; leer = eingebettete data/hub.json.
	HubCandidate string `json:"hubCandidate"`
}

// WorldPreviewView ist Level und Zustand der Vorschau (Felder wie net.PreviewResult) oder ein Hinweis in Error.
type WorldPreviewView struct {
	net.PreviewResult
	Error string `json:"error"`
}

// WorldPreview baut die Vorschau-Welt (Binding). Die Quelle ist ein Biom oder Insel + Stufe, mit dem Entwurf der
// geplanten Chunks wie bei WorldLevel (dieselbe Quelle für Chunk- und Live-Vorschau, B-461/AC-11).
func (a *App) WorldPreview(args WorldPreviewArgs) WorldPreviewView {
	r, err := a.resolveSource(args.Source)
	if err != nil {
		return WorldPreviewView{Error: err.Error()}
	}
	res, err := net.Preview(room.PreviewOptions{
		Seed: args.Seed, Source: room.PreviewSource{Biome: r.biome}, HubLevel: args.HubLevel,
		Phase: args.Phase, Progress: args.Progress, Buildings: args.Off, Planned: r.planned,
		HubCandidate: []byte(args.HubCandidate), ChunkCount: r.count,
	})
	if err != nil {
		return WorldPreviewView{Error: err.Error()}
	}
	return WorldPreviewView{PreviewResult: res}
}
