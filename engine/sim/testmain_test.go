package sim

import (
	"os"
	"testing"

	"k3c/engine/level"
)

// TestMain rechnet die Tests dieses Pakets mit der Standard-Welt von vor WL2 (eine Insel, ohne Chunk-Anzahl). Die
// eingebettete Standard-Welt hat seit B-1215 fünf Inseln und Chunk-Anzahlen; die Go-Engine ist eingefroren (Entscheidung
// 013), ihre Mechanik-Tests brauchen die letzte Insel als Ziel der Siegvarianten und sollen nicht an den Daten hängen,
// die für das Rust-Spiel wachsen. Wer die eingebetteten Daten prüft, liest sie über data.Files.
func TestMain(m *testing.M) {
	isl := standardWorld.Islands[0]
	isl.ChunkCounts = nil
	isl.PlannedChunks = map[string][]level.PlannedChunk{"0": {{Slot: -7, Kind: "harbor"}}}
	standardWorld.Islands = []IslandDef{isl}
	os.Exit(m.Run())
}
