package net

import (
	"k3c/engine/level"
	"k3c/engine/room"
)

// PreviewResult ist die Vorschau-Welt in dem Format, das ein Gerät im Spiel bekommt: Level wie die Nachricht `level`
// (Feld layout), State wie der volle Zustand der Nachricht `snap` (Feld s). Ohne Bürger und Gegner (WZ7.1, B-460).
type PreviewResult struct {
	Level level.Layout   `json:"level"`
	State map[string]any `json:"state"`
}

// Preview baut die Welt zu den Optionen (room.BuildPreview) und serialisiert sie über stateOf. Kein Raum, kein Spielstand,
// kein HTTP-Endpunkt und kein globaler Zustand: der Aufrufer (das Asset-Studio) ruft die Funktion direkt auf.
func Preview(o room.PreviewOptions) (PreviewResult, error) {
	w, err := room.BuildPreview(o)
	if err != nil {
		return PreviewResult{}, err
	}
	return PreviewResult{Level: w.Level, State: stateOf(w, 0, false)}, nil
}
