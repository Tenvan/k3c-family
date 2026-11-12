package menu

// Papier-Hintergrund der Menüs (B-812, Vorbild src/scenes/paperBackdrop.ts ohne Sterne und Wald, die sind Grafik):
// Nachthimmel, zwei geschnittene Papierbahnen mit heller Kante und ein Papiermond mit Schlagschatten. Die Lage rechnet
// backdropOf rein, painter zeichnet sie mit Rechtecken und einer Scheibe; je Bild entsteht nichts Neues.

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

// backdrop zeichnet den Hintergrund ins Fenster g.
func (p *painter) backdrop(dst *ebiten.Image, g geom) {
	b := backdropOf(g)
	r := g.win
	p.rect(dst, float64(r.Min.X), float64(r.Min.Y), float64(r.Dx()), float64(r.Dy()), colNight)
	for _, bd := range b.bands {
		for j := range bandSteps {
			x, w, y := b.segment(bd, j)
			p.rect(dst, x, y, w+1, float64(r.Max.Y)-y, bd.clr)
			p.rect(dst, x, y, w+1, 3*b.k, colEdge)
		}
	}
	for _, s := range shadowLayers {
		d := s.d * b.k
		p.disc(dst, b.moonX+d*0.6, b.moonY+d, b.moonR, fade(colShadow, s.alpha))
	}
	p.disc(dst, b.moonX, b.moonY, b.moonR, matLinen.licht)
	p.disc(dst, b.moonX-22*b.k, b.moonY-12*b.k, 50*b.k, colNight)
}

// disc zeichnet eine Scheibe mit Mittelpunkt (x, y) und Radius rad in clr (vormultipliziert).
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
	p.op.Filter = ebiten.FilterLinear
	dst.DrawImage(p.round, &p.op)
	p.op.Filter = ebiten.FilterNearest
}
