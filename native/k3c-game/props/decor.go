package props

// Darstellung der Natur-Eigenschaften (Sprint BI2, B-902 bis B-906): Eigenschaften mit `display: true` aus
// data/biomes zeigen sich als Formen der Zeichenliste. Der Wert gilt je Stufe (World.Level.Nature, B-891); fehlt er,
// gilt der Default des Bioms, ein Biom ohne die Eigenschaft zeigt nichts davon. Orte und Formen kommen aus der
// Position (mix), kein Zufall. decor läuft vor allen anderen props, liegt also hinter ihnen.

import (
	"image/color"
	"math"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// viewTop ist die sichtbare Welthöhe über dem Boden in Units (game.ViewUnits: 420 px bei 32 px je Unit).
const viewTop = 420.0 / 32

// Raster der Decken-Zacken und der Tropfsteine in Units.
const (
	ceilingStep = 1.5
	stalStep    = 2.0
)

var (
	ceilingRock = color.RGBA{36, 36, 46, 255}
	stalColor   = color.RGBA{74, 72, 86, 255}
)

// trait liefert den geltenden Wert der Eigenschaft key, begrenzt auf ihren Bereich; ok = false, wenn das Biom sie
// nicht hat.
func trait(w *sim.World, key string) (int, bool) {
	t, ok := w.Biome.Traits.Get(key)
	if !ok {
		return 0, false
	}
	v, set := w.Level.Nature.Traits[key]
	if !set {
		v = t.Default
	}
	return min(max(v, t.Min), t.Max), true
}

// span ist der sichtbare Bereich der Kamera k innerhalb der Welt, um margin erweitert, Anfang auf das Raster step
// gelegt; ok = false, wenn nichts davon in der Welt liegt. Kamera ohne Maßstab = ganze Welt (wie visible).
func span(w *sim.World, k view.Camera, step, margin float64) (from, to float64, ok bool) {
	l, r := 0.0, w.WidthUnits
	if k.Scale != 0 {
		l, r = k.Span()
	}
	l, r = max(l-margin, 0), min(r+margin, w.WidthUnits)
	if l >= r {
		return 0, 0, false
	}
	return math.Floor(l/step) * step, r, true
}

// cellHash ist die Mischung des Rasterplatzes bei x (Raster step) mit salt: gleicher Platz, gleicher Wert.
func cellHash(x, step, salt float64) uint32 { return mix(math.Round(x/step)*7 + salt) }

// ceilingBottom ist die Unterkante der Decke in Units über dem Boden zum Wert v der Deckenhöhe (1 tief, 3 hoch):
// Dicke 3,5 Units bei 1, 2,15 bei 2, 0,8 bei 3.
func ceilingBottom(v int) float64 { return viewTop - (3.5 - float64(v-1)*1.35) }

// decor plant die Darstellung der Natur-Eigenschaften der Welt w für Kamera k.
func (p *planner) decor(w *sim.World, k view.Camera) {
	p.ceiling(w, k)
	p.stalactites(w, k)
}

// ceiling: Felsdecke über die sichtbare Breite bis zur Unterkante der Deckenhöhe, mit Zacken nach unten (B-902).
func (p *planner) ceiling(w *sim.World, k view.Camera) {
	v, ok := trait(w, "ceilingHeight")
	if !ok {
		return
	}
	from, to, ok := span(w, k, ceilingStep, 2)
	if !ok {
		return
	}
	y := ceilingBottom(v)
	p.rect(&p.body, (from+to)/2, y, to-from, viewTop+2-y, ceilingRock)
	for x := from; x < to; x += ceilingStep {
		h := cellHash(x, ceilingStep, 101)
		depth := 0.3 + float64(h%6)/10
		width := ceilingStep * (1.1 + float64(h>>8%4)/10)
		p.body = append(p.body, item{shape: shapeTri, x: x + ceilingStep/2, y: y + 0.05, w: width, h: -depth, clr: ceilingRock})
	}
}

// stalactites: Tropfsteine hängen von der Decke, an jedem Rasterplatz mit mix % 6 < Wert (B-903); so bleiben die
// Plätze eines kleineren Werts bei einem größeren erhalten. Ohne Deckenhöhe hängen sie an der Decke mit Wert 2.
func (p *planner) stalactites(w *sim.World, k view.Camera) {
	v, ok := trait(w, "stalactites")
	if !ok || v <= 0 {
		return
	}
	c, ok := trait(w, "ceilingHeight")
	if !ok {
		c = 2
	}
	from, to, ok := span(w, k, stalStep, 1)
	if !ok {
		return
	}
	y := ceilingBottom(c) + 0.1
	for x := from; x < to; x += stalStep {
		h := cellHash(x, stalStep, 211)
		if h%6 >= uint32(v) {
			continue
		}
		length := 0.6 + float64(h>>4%13)/10
		width := 0.4 + float64(h>>8%5)/10
		off := float64(h>>12%7)/10 - 0.3
		p.body = append(p.body, item{shape: shapeTri, x: x + stalStep/2 + off, y: y, w: width, h: -length, clr: stalColor})
	}
}
