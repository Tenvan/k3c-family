package room

import (
	"fmt"
	"regexp"
	"strconv"
	"unicode/utf8"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// Vorschau-Welt für den Welt-Betrachter im Asset-Studio (WZ7.1, B-460): eine Welt in dem Format, das auch ein Raum
// hat, aber ohne Raum, Spielstand und globalen Zustand. Die Serialisierung (level und snap) liegt in engine/net.

// maxPreviewSeedRunes ist die Länge des Seeds wie bei GET /api/level.
const maxPreviewSeedRunes = 64

var previewBiome = regexp.MustCompile(`^[a-z0-9-]{1,32}$`)

// PreviewSource ist die Quelle des Levels. In WZ7 ist es das Biom; Insel + Stufe kommt mit WZ8.5 als weitere Felder
// (ein Aufruf mit nur Biome verhält sich dann unverändert).
type PreviewSource struct {
	Biome string
}

// PreviewOptions beschreiben eine Vorschau-Welt. Der Nullwert jedes Feldes außer Seed und Source ist das Standardbild:
// höchste Hub-Stufe, Tag mit Fortschritt 0, alle Gebäude. Neue Felder (WZ8.4 „geplante Chunks“, WZ9
// „Kandidaten-hub.json“) kommen hinzu, ohne bestehende Aufrufer zu ändern.
type PreviewOptions struct {
	Seed     string
	Source   PreviewSource
	HubLevel int // 1 bis Anzahl hub.json › levels; 0 = die höchste
	// Phase und Progress: Tageszeit als Sim-Phase (day, dusk, night; leer = day) und Fortschritt in [0, 1).
	Phase    string
	Progress float64
	// Buildings: false schaltet einen Platz aus (Schlüssel PreviewSiteKey); fehlende und unbekannte Schlüssel
	// bleiben an bzw. werden ignoriert.
	Buildings map[string]bool
	// HubCandidate: Kandidaten-hub.json als Bytes (Gebäude-Planer, WZ9.1, B-462); leer = eingebettete data/hub.json.
	// Geprüft mit sim.ValidateHub (Befunde → *CandidateError), gebaut über sim.CreateOptions; HubLevel 0 ist dann
	// die höchste Stufe des Kandidaten. Nur Parameter, kein globaler Zustand.
	HubCandidate []byte
}

// PreviewSiteKey ist der Schlüssel eines Bauplatzes in PreviewOptions.Buildings: Art und X-Position in Units.
func PreviewSiteKey(kind string, x float64) string {
	return kind + "@" + strconv.FormatFloat(x, 'f', -1, 64)
}

// BuildPreview erzeugt die Welt zu den Optionen: Level aus Seed und Biom, ohne Bürger und Gegner, auf der Hub-Stufe
// ausgebaut (sim.PreviewHubLevel), Tageszeit gesetzt (sim.PreviewCycle), abgewählte Plätze entfernt. Gleiche Optionen
// ergeben dieselbe Welt; die Funktion schreibt nichts und ändert keinen Paketzustand.
func BuildPreview(o PreviewOptions) (*sim.World, error) {
	if n := utf8.RuneCountInString(o.Seed); n == 0 || n > maxPreviewSeedRunes {
		return nil, fmt.Errorf("vorschau: Seed braucht 1 bis %d Zeichen", maxPreviewSeedRunes)
	}
	if o.Source.Biome == "" {
		return nil, fmt.Errorf("vorschau: Quelle fehlt (Biom)")
	}
	if !previewBiome.MatchString(o.Source.Biome) {
		return nil, fmt.Errorf("vorschau: ungültiges Biom %q", o.Source.Biome)
	}
	biome, err := level.LoadBiome(o.Source.Biome)
	if err != nil {
		return nil, fmt.Errorf("vorschau: Biom %q nicht ladbar: %w", o.Source.Biome, err)
	}
	w, top, err := previewWorld(biome, o)
	if err != nil {
		return nil, err
	}
	hubLevel := o.HubLevel
	if hubLevel == 0 {
		hubLevel = top
	}
	phase := o.Phase
	if phase == "" {
		phase = "day"
	}
	w.Troops, w.Enemies, w.SpawnQueue = []*sim.Troop{}, []*sim.Enemy{}, []sim.QueuedSpawn{}
	if err := sim.PreviewHubLevel(w, hubLevel); err != nil {
		return nil, err
	}
	if err := sim.PreviewCycle(w, phase, o.Progress); err != nil {
		return nil, err
	}
	sites := make([]*sim.Site, 0, len(w.Sites)) // erst nach dem Ausbau entfernen: die Stufen der übrigen bleiben
	for _, s := range w.Sites {
		if on, set := o.Buildings[PreviewSiteKey(s.Kind, s.X)]; !set || on {
			sites = append(sites, s)
		}
	}
	w.Sites = sites
	return w, nil
}

// previewWorld erzeugt die frische Welt und nennt die höchste Hub-Stufe: ohne Kandidat aus der eingebetteten
// hub.json (wie vor WZ9.1), mit Kandidat aus ihm (candidateWorld).
func previewWorld(biome level.Biome, o PreviewOptions) (*sim.World, int, error) {
	if len(o.HubCandidate) > 0 {
		return candidateWorld(biome, o.Seed, o.HubCandidate)
	}
	data, err := fullData()
	if err != nil {
		return nil, 0, err
	}
	w, err := sim.CreateWorld(biome, o.Seed, sim.Options{CycleSpeed: 1})
	if err != nil {
		return nil, 0, fmt.Errorf("vorschau: Level nicht erzeugbar: %w", err)
	}
	return w, data.hub, nil
}
