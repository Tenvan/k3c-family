package devtools

// Geometrie des Debug-Overlays (F10, Seite Geometrie, B-888): Rahmen um Figuren, Münzen und Bauplätze (dort die
// Zahl-Reichweite aus data/economy.json), Linien an den Chunk-Grenzen und Weltkanten, Kamera-Mitte. Die Simulation ist
// eindimensional (Welt-x in Units); die Höhen der Rahmen sind feste Anzeige-Werte. Reine Rechnung in Fenster-Pixeln.

import (
	"encoding/json"

	"k3c/data"
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Arten der Debug-Formen (Farbe).
const (
	geoMonarch = iota
	geoTroop
	geoEnemy
	geoCoin
	geoSite
	geoChunk
	geoEdge
	geoCamera
	geoKinds
)

// geoHeight ist die Höhe der Rahmen je Art in Units (Linien: ganze Zelle).
var geoHeight = [geoKinds]float64{geoMonarch: 2.2, geoTroop: 1.6, geoEnemy: 1.6, geoCoin: 0.4, geoSite: 0.3}

// geoHalf ist die halbe Breite der Figuren-Rahmen in Units (Bauplätze: Zahl-Reichweite).
var geoHalf = [geoKinds]float64{geoMonarch: 0.5, geoTroop: 0.4, geoEnemy: 0.45, geoCoin: 0.2}

// shape ist eine Form in Fenster-Pixeln: Rahmen (x0 < x1) oder senkrechte Linie (x0 == x1).
type shape struct {
	kind           int
	x0, y0, x1, y1 float64
}

// payRange ist die Zahl-Reichweite der Bauplätze (economy.json › payRangeUnits; Fehler: 2).
var payRange = loadPayRange()

func loadPayRange() float64 {
	var v struct {
		PayRangeUnits float64 `json:"payRangeUnits"`
	}
	raw, err := data.Files.ReadFile("economy.json")
	if err != nil || json.Unmarshal(raw, &v) != nil || v.PayRangeUnits <= 0 {
		return 2
	}
	return v.PayRangeUnits
}

// geometry hängt die sichtbaren Formen der Welt w in der Zelle c an dst.
func geometry(dst []shape, w *sim.World, c view.Cell) []shape {
	k := c.Camera
	ground := float64(c.GroundY())
	add := func(kind int, x, half float64) {
		if !k.Visible(x, half) {
			return
		}
		dst = append(dst, shape{kind, k.ScreenX(x - half), c.ScreenY(geoHeight[kind]), k.ScreenX(x + half), ground})
	}
	for _, p := range w.Players {
		add(geoMonarch, p.X, geoHalf[geoMonarch])
	}
	for _, t := range w.Troops {
		add(geoTroop, t.X, geoHalf[geoTroop])
	}
	for _, e := range w.Enemies {
		add(geoEnemy, e.X, geoHalf[geoEnemy])
	}
	for _, m := range w.Coins {
		add(geoCoin, m.X, geoHalf[geoCoin])
	}
	for _, s := range w.Sites {
		add(geoSite, s.X, payRange)
	}
	return lines(dst, w, c)
}

// lines hängt die Chunk-Grenzen, die Weltkanten und die Kamera-Mitte als Linien über die ganze Zelle an.
func lines(dst []shape, w *sim.World, c view.Cell) []shape {
	k := c.Camera
	top, bottom := float64(c.Rect.Min.Y), float64(c.Rect.Max.Y)
	line := func(kind int, x float64) { dst = append(dst, shape{kind, x, top, x, bottom}) }
	for _, ch := range w.Level.Chunks {
		if x := ch.StartUnits; x > 0 && k.Visible(x, 0) {
			line(geoChunk, k.ScreenX(x))
		}
	}
	for _, x := range []float64{0, w.WidthUnits} {
		if k.Visible(x, 0) {
			line(geoEdge, k.ScreenX(x))
		}
	}
	line(geoCamera, k.Center)
	return dst
}
