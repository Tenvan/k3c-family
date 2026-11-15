package menu

import "github.com/hajimehoshi/ebiten/v2"

// Das große Weltbild der Weltauswahl (B-938): Papier-Rahmen mit dem Titelbild der Welt (Querformat), darüber auf einem
// dunklen Band der Weltname als Schrift, links und rechts die Blätter-Pfeile. Fehlt das Bild, zeigt der Rahmen nur den
// Namen.

// heroRect ist der Rahmen des Weltbilds: Breite der Karten, von deren Oberkante bis kurz über die Meldungszeile.
func (g geom) heroRect() (x, y, w, h float64) {
	return g.rowX, g.rowTop, g.rowW, g.noteY - g.rowTop - 2*g.rowGap
}

// hero zeichnet das große Weltbild von h.
func (p *painter) hero(dst *ebiten.Image, g geom, h *hero) {
	x, y, w, ht := g.heroRect()
	p.paper(dst, g, x, y, w, ht, matLinen, false, true)
	in := g.stripInset()
	p.artBox(dst, h.art, x+in, y+in, w-2*in, ht-2*in)
	band := 1.9 * g.rowPx
	p.rect(dst, x+in, y+ht-in-band, w-2*in, band, colDim)
	p.centered(dst, h.name, x+w/2, y+ht-in-band+(band-1.3*g.rowPx)/2, 1.3*g.rowPx, colTitle)
	_, ah := p.measure(">", g.titlePx)
	ay := y + (ht-ah)/2
	if h.left {
		p.text(dst, "<", x-2*in-p.charW(g.titlePx), ay, g.titlePx, colTitle)
	}
	if h.right {
		p.text(dst, ">", x+w+2*in, ay, g.titlePx, colTitle)
	}
}
