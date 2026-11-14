package menu

// Papier-Hintergrund der Menüs (B-812, Vorbild src/scenes/paperBackdrop.ts ohne Sterne und Wald, die sind Grafik):
// Nachthimmel, zwei geschnittene Papierbahnen mit heller Kante und ein Papiermond mit Schlagschatten. Die Lage rechnet
// backdropOf und shapes rein, painter zeichnet die Formen (erst Rechtecke, dann Scheiben); je Bild entsteht nichts Neues.

import (
	"image"
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"
)

// Töne des Hintergrunds: Himmel und Bahnen wie paperBackdrop.ts (HUB_COLORS.night, Bahnen 0x1b2550 und 0x3b3a6b), der
// Mond aus materialien.leinen, sein Schatten aus papier.lage.farbe von public/grafik/stil.paper.json.
var (
	colShadow = rgb(0x1a0f14)
	colBand1  = rgb(0x1b2550)
	colBand2  = rgb(0x3b3a6b)
	colEdge   = fade(color.RGBA{255, 255, 255, 255}, 0.14)
)

// bandSteps ist der Schnitt der Oberkante einer Bahn: Versatz je Abschnitt in px bei 1080 (die zweite Bahn liest die
// Folge versetzt, damit die Kanten nicht parallel laufen).
var bandSteps = [...]float64{0, -6, 4, -3, 7, -5, 2, -8, 5, -2, 6, -4}

// band ist eine Papierbahn: Oberkante (ohne Schnitt) in px und Farbe; sie reicht bis zum unteren Rand.
type band struct {
	top   float64
	clr   color.RGBA
	shift int // Versatz in bandSteps
}

// backdrop ist die Lage des Hintergrunds in einem Fenster.
type backdrop struct {
	win          image.Rectangle
	k            float64
	bands        [2]band
	moonX, moonY float64
	moonR        float64
}

// backdropOf rechnet die Lage für das Fenster der Seite g (Browser: 1920 × 1080, Bahnen ab 430 und 690 px, Mond bei
// 1660/150 mit Radius 56).
func backdropOf(g geom) backdrop {
	w, top := float64(g.win.Dx()), float64(g.win.Min.Y)
	return backdrop{
		win: g.win, k: g.k,
		bands: [2]band{{top + 430*g.k, colBand1, 0}, {top + 690*g.k, colBand2, 5}},
		moonX: float64(g.win.Min.X) + w*1660/1920, moonY: top + 150*g.k, moonR: 56 * g.k,
	}
}

// segment liefert Abschnitt j einer Bahn: linke Kante, Breite und Oberkante.
func (b backdrop) segment(bd band, j int) (x, w, y float64) {
	w = float64(b.win.Dx()) / float64(len(bandSteps))
	return float64(b.win.Min.X) + float64(j)*w, w, bd.top + bandSteps[(j+bd.shift)%len(bandSteps)]*b.k
}

// discPx ist die Größe der Scheiben-Vorlage (Mond); gestreckt gezeichnet.
const discPx = 64

// shape ist eine Form des Hintergrunds: Rechteck (Ecke x, y, Größe w × h) oder Scheibe (Mittelpunkt x, y, Radius w).
type shape struct {
	disc       bool
	x, y, w, h float64
	clr        color.RGBA
}

// filter ist der Filter der Form: Scheiben glatt (Linear), Rechtecke scharf (Nearest).
func (s shape) filter() ebiten.Filter {
	if s.disc {
		return ebiten.FilterLinear
	}
	return ebiten.FilterNearest
}

// backdropShapes ist die Zahl der Formen: Himmel, je Bahn-Abschnitt Fläche und Kante, Mondschatten, Mond, Ausschnitt.
const backdropShapes = 1 + 2*2*len(bandSteps) + len(shadowLayers) + 2

// shapes füllt out in Zeichenreihenfolge: erst alle Rechtecke, dann alle Scheiben. So wechselt der Filter je Bild nur
// einmal und Ebitengine kann die Draw-Aufrufe bündeln (B-820); Mond und Bahnen überlappen nicht.
func (b backdrop) shapes(out *[backdropShapes]shape) {
	r := b.win
	out[0] = shape{x: float64(r.Min.X), y: float64(r.Min.Y), w: float64(r.Dx()), h: float64(r.Dy()), clr: colNight}
	n := 1
	for _, bd := range b.bands {
		for j := range bandSteps {
			x, w, y := b.segment(bd, j)
			out[n] = shape{x: x, y: y, w: w + 1, h: float64(r.Max.Y) - y, clr: bd.clr}
			out[n+1] = shape{x: x, y: y, w: w + 1, h: 3 * b.k, clr: colEdge}
			n += 2
		}
	}
	for _, s := range shadowLayers {
		d := s.d * b.k
		out[n] = shape{disc: true, x: b.moonX + d*0.6, y: b.moonY + d, w: b.moonR, clr: fade(colShadow, s.alpha)}
		n++
	}
	out[n] = shape{disc: true, x: b.moonX, y: b.moonY, w: b.moonR, clr: matLinen.licht}
	out[n+1] = shape{disc: true, x: b.moonX - 22*b.k, y: b.moonY - 12*b.k, w: 50 * b.k, clr: colNight}
}

// backdrop zeichnet den Hintergrund ins Fenster g; danach gilt wieder FilterNearest für die übrigen Zeichnungen.
func (p *painter) backdrop(dst *ebiten.Image, g geom) {
	var s [backdropShapes]shape
	backdropOf(g).shapes(&s)
	for i := range s {
		p.op.Filter = s[i].filter()
		if s[i].disc {
			p.disc(dst, s[i].x, s[i].y, s[i].w, s[i].clr)
		} else {
			p.rect(dst, s[i].x, s[i].y, s[i].w, s[i].h, s[i].clr)
		}
	}
	p.op.Filter = ebiten.FilterNearest
}

// disc zeichnet eine Scheibe mit Mittelpunkt (x, y) und Radius rad in clr (vormultipliziert) mit dem gesetzten Filter.
func (p *painter) disc(dst *ebiten.Image, x, y, rad float64, clr color.RGBA) {
	if p.round == nil {
		p.round = ebiten.NewImage(discPx, discPx)
		vector.FillCircle(p.round, discPx/2, discPx/2, discPx/2, color.White, true)
	}
	p.op.GeoM.Reset()
	p.op.GeoM.Scale(2*rad/discPx, 2*rad/discPx)
	p.op.GeoM.Translate(x-rad, y-rad)
	p.op.ColorScale.Reset()
	p.op.ColorScale.ScaleWithColor(clr)
	dst.DrawImage(p.round, &p.op)
}
