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
}

// WorldPreviewView ist Level und Zustand der Vorschau (Felder wie net.PreviewResult) oder ein Hinweis in Error.
type WorldPreviewView struct {
	net.PreviewResult
	Error string `json:"error"`
}

// WorldPreview baut die Vorschau-Welt (Binding). Die Quelle ist in WZ7 das Biom; Insel + Stufe folgt mit WZ8.5.
func (a *App) WorldPreview(args WorldPreviewArgs) WorldPreviewView {
	if args.Source.Island != "" || args.Source.Stage != 0 {
		return WorldPreviewView{Error: "Quelle Insel + Stufe folgt mit WZ8.5, bisher nur das Biom"}
	}
	res, err := net.Preview(room.PreviewOptions{
		Seed: args.Seed, Source: room.PreviewSource{Biome: args.Source.Biome}, HubLevel: args.HubLevel,
		Phase: args.Phase, Progress: args.Progress, Buildings: args.Off,
	})
	if err != nil {
		return WorldPreviewView{Error: err.Error()}
	}
	return WorldPreviewView{PreviewResult: res}
}
