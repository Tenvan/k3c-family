package view

import (
	"slices"

	"github.com/hajimehoshi/ebiten/v2"
)

// Layer ist die feste Zeichen-Ebene eines Drawers. Die Reihenfolge der Konstanten ist die Zeichenreihenfolge (hinten
// nach vorn); neue Ebenen gibt es nur über die NG1-Nachfolge.
type Layer int

const (
	LayerFar       Layer = iota // ferne Kulisse, steht (Parallaxe 0)
	LayerMid                    // mittlere Kulisse, scrollt langsam
	LayerNear                   // nahe Kulisse
	LayerGround                 // Boden und Natur am Boden
	LayerBuildings              // Bauten, Bauplätze, Burg (props)
	LayerActors                 // Figuren: Monarchen, Truppen, Gegner (actors)
	LayerRiver                  // Fluss im unteren Drittel der Zelle, vor allem anderen der Welt (world)
	LayerEffects                // Effekte: Geschosse, Treffer, Zauber (fx)
	LayerHUD                    // Anzeigen je Zelle (hud)
	LayerOverlay                // einmal je Fenster über allen Zellen, z. B. FPS, Hinweise (Cell = Fenster)
	layerCount
)

var layerNames = [layerCount]string{"far", "mid", "near", "ground", "buildings", "actors", "river", "effects", "hud", "overlay"}

func (l Layer) String() string {
	if !l.Valid() {
		return "invalid"
	}
	return layerNames[l]
}

// Valid meldet, ob l eine der festen Ebenen ist.
func (l Layer) Valid() bool { return l >= 0 && l < layerCount }

// PerCell meldet, ob die Ebene je Zelle gezeichnet wird (alle außer LayerOverlay).
func (l Layer) PerCell() bool { return l.Valid() && l != LayerOverlay }

// Layers liefert alle Ebenen in Zeichenreihenfolge.
func Layers() []Layer {
	out := make([]Layer, layerCount)
	for i := range out {
		out[i] = Layer(i)
	}
	return out
}

// Drawer zeichnet einen Teil des Frames auf genau einer Ebene. dst ist bei Zellen-Ebenen der Ausschnitt der Zelle
// (SubImage, beschnitten, Koordinaten bleiben Fenster-Koordinaten), bei LayerOverlay das ganze Fenster mit
// c = WindowCell. Ein Fachpaket mit mehreren Ebenen liefert mehrere Drawer.
type Drawer interface {
	Layer() Layer
	Draw(dst *ebiten.Image, f *Frame, c Cell)
}

// Listener bekommt jeden Frame genau einmal, bevor er gezeichnet wird (Ton, Effekte mit Zustand, Hinweise des HUD).
// Ein Drawer mit eigenem Zustand über Bilder hinweg erfüllt zusätzlich Listener; game meldet ihn dann selbst an.
type Listener interface {
	Listen(f *Frame)
}

// Sorted liefert die Drawer stabil nach Ebene sortiert (gleiche Ebene: Reihenfolge der Anmeldung); ungültige Ebenen
// fallen heraus.
func Sorted(ds []Drawer) []Drawer {
	out := make([]Drawer, 0, len(ds))
	for _, d := range ds {
		if d != nil && d.Layer().Valid() {
			out = append(out, d)
		}
	}
	slices.SortStableFunc(out, func(a, b Drawer) int { return int(a.Layer()) - int(b.Layer()) })
	return out
}
