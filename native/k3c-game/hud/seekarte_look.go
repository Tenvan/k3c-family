package hud

// Aussehen einer Insel im Diorama der Seekarte (B-964/R2): ohne neues Asset aus den Angaben der Welt-Datei. Größe der
// Miniatur aus der Inselgröße (größte Chunk-Anzahl), Zahl der Schichten unter der Insel aus der Stufenzahl, Aufsatz aus
// einem festen Hash der Insel-ID. Kein Zufall und kein Zustand: Dieselbe Insel sieht immer gleich aus, bei beiden
// Spielern. Reine Logik ohne Ebitengine; das HUD rechnet damit keine Regel, es wählt nur Bilder.

import "k3c/native/k3c-game/view"

const (
	// Chunk-Anzahlen der kleinsten und größten Inselgröße (data/island-sizes.json: sehr klein, sehr groß).
	seaChunksMin, seaChunksMax = 16.0, 100.0
	// Maßstab der Miniatur bei kleinster und größter Insel; ohne Angabe gilt die Mitte.
	seaScaleMin, seaScaleMax = 0.7, 1.1
	// seaLayersMax: mehr Schichten als diese passen nicht zwischen Insel und nächste Welle.
	seaLayersMax = 5
)

// seaLook ist das Aussehen einer Insel: Welt-ID der Miniatur, Maßstab, Schichten unter der Insel, Wert für den Aufsatz.
type seaLook struct {
	world  string
	scale  float64
	layers int
	seed   uint32
}

// lookOf leitet das Aussehen aus den Angaben der Route ab.
func lookOf(isl view.RouteIsland) seaLook {
	return seaLook{world: isl.World, scale: seaScale(isl.Chunks), layers: seaLayers(isl.Stages), seed: seaSeed(isl.ID)}
}

// seaScale ist der Maßstab der Miniatur zur Chunk-Anzahl: linear zwischen kleinster und größter Inselgröße, darüber und
// darunter begrenzt; ohne Angabe (<= 0, Länge des Bioms) die Mitte.
func seaScale(chunks int) float64 {
	if chunks <= 0 {
		return (seaScaleMin + seaScaleMax) / 2
	}
	t := (float64(chunks) - seaChunksMin) / (seaChunksMax - seaChunksMin)
	return seaScaleMin + min(1, max(0, t))*(seaScaleMax-seaScaleMin)
}

// seaLayers ist die Zahl der sichtbaren Schichten unter der Insel: je Stufe eine, mindestens eine, höchstens seaLayersMax.
func seaLayers(stages int) int {
	return min(seaLayersMax, max(1, stages))
}

// seaSeed ist der feste Hash der Insel-ID (FNV-1a, 32 Bit), ohne Allokation.
func seaSeed(id string) uint32 {
	h := uint32(2166136261)
	for i := range len(id) {
		h = (h ^ uint32(id[i])) * 16777619
	}
	return h
}
