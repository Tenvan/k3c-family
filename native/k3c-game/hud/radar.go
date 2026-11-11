package hud

// Radar (Spiegel von src/scenes/hud/radar.ts, B-090): eine schmale Leiste über das ganze Level mit Burg, Portalen,
// Ausgang, Treppen, Gegnern und Monarchen sowie dem Kamera-Ausschnitt der Zelle. Reine Rechnung in einen
// wiederverwendeten Puffer; gezeichnet wird in paint.go.

import (
	"math"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// radarKind ist die Art eines Markers (Zeichenreihenfolge: Burg, Portale, Ausgang, Treppen, Gegner, Monarchen).
type radarKind int

const (
	radarCastle radarKind = iota
	radarPortal
	radarExit
	radarStairs
	radarEnemy
	radarPlayer
)

// radarMarker ist ein Marker: Lage 0..1 entlang der Levelbreite; nur bei Monarchen Index, eigen und gefallen.
type radarMarker struct {
	kind      radarKind
	pos       float64
	index     int
	own, down bool
}

// Maße der Leiste in px bei 1080 (radar.ts).
const (
	radarHeight   = 20.0
	radarMaxWidth = 640.0
)

// unitPos ist die Lage x in 0..1 bei Levelbreite width (Breite 0: 0).
func unitPos(x, width float64) float64 {
	if width <= 0 {
		return 0
	}
	return math.Min(1, math.Max(0, x/width))
}

// radarMarkers schreibt die Marker der Welt w in out (geleert); self ist der Monarch der Zelle (-1 = keiner). Die
// Monarchen stehen zuletzt (oben).
func radarMarkers(out []radarMarker, w *sim.World, self int) []radarMarker {
	out = out[:0]
	at := func(x float64) float64 { return unitPos(x, w.WidthUnits) }
	out = append(out, radarMarker{kind: radarCastle, pos: at(w.Castle.X)})
	for _, x := range w.Portals {
		out = append(out, radarMarker{kind: radarPortal, pos: at(x)})
	}
	for _, e := range w.Level.Entities {
		if e.Kind == "exit" {
			out = append(out, radarMarker{kind: radarExit, pos: at(e.X)})
		}
	}
	for _, s := range w.Sites {
		if s != nil && (s.Kind == "stairsUp" || s.Kind == "stairsDown") {
			out = append(out, radarMarker{kind: radarStairs, pos: at(s.X)})
		}
	}
	for _, e := range w.Enemies {
		if e != nil {
			out = append(out, radarMarker{kind: radarEnemy, pos: at(e.X)})
		}
	}
	for _, p := range w.Players {
		if p != nil {
			out = append(out, radarMarker{kind: radarPlayer, pos: at(p.X), index: p.Index, own: p.Index == self, down: p.RespawnIn > 0})
		}
	}
	return out
}

// radarView ist der sichtbare Ausschnitt der Kamera in 0..1.
func radarView(k view.Camera, width float64) (from, to float64) {
	l, r := k.Span()
	return unitPos(l, width), unitPos(r, width)
}

// radarSize ist die Größe der Leiste in einer Zelle der Breite cellW beim Maßstab k: halb so breit wie die Zelle,
// höchstens 640 px bei 1080.
func radarSize(cellW, k float64) (w, h float64) {
	return math.Min(cellW/2, radarMaxWidth*k), radarHeight * k
}
