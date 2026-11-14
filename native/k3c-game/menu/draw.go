package menu

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// page ist, was eine Menü-Seite zeigt. Die Screens füllen sie in Draw nur bei neuer Version oder Sprache neu
// (Zwischenspeicher der Texte); painter zeichnet sie ohne weitere Rechnung.
type page struct {
	title, sub string
	rows       [maxRows]row
	n          int
	sel        int // sichtbare Zeile mit Auswahl, -1 = keine
	note       string
	bad        bool
	hint       string
	dim        bool // Hintergrund abgedunkelt (Optionen aus der Pause: ruhiger Grund)
	ask        string
	askRows    [2]string
	askSel     int

	ver  int    // Version des Screens, für die die Texte gelten
	lang string // Sprache der Texte
	ok   bool
}

// row ist eine Karte: Beschriftung links, Wert rechts, Papier-Farbe; off = ausgegraut bzw. als Problem markiert.
type row struct {
	label, value string
	mat          material
	off          bool
	art          string // Illustration `gruppe/id` (B-818), leer = nur Text
}

// stale meldet, ob die Texte für Version ver und Sprache lang neu gebaut werden müssen, und merkt sich beides.
func (p *page) stale(ver int, lang string) bool {
	if p.ok && p.ver == ver && p.lang == lang {
		return false
	}
	p.ver, p.lang, p.ok = ver, lang, true
	return true
}

// reset leert die Seite vor dem Neubau.
func (p *page) reset(title string) {
	*p = page{title: title, ver: p.ver, lang: p.lang, ok: p.ok}
}

// add hängt eine Zeile an (mehr als maxRows werden verworfen).
func (p *page) add(r row) {
	if p.n < maxRows {
		p.rows[p.n] = r
		p.n++
	}
}

// painter zeichnet Seiten: Rechtecke über ein 1×1-Bild, Text über die Schrift; die Debug-Schrift (fehlende
// Schriftdatei) wird über ein Zwischenbild vergrößert und eingefärbt. Bilder und Optionen werden wiederverwendet.
type painter struct {
	font    *art.Font
	lib     *art.Library // Illustrationen der Karten (B-818); nil = nur Text
	pixel   *ebiten.Image
	round   *ebiten.Image // Scheibe (Mond des Hintergrunds)
	scratch *ebiten.Image
	op      ebiten.DrawImageOptions
}

const scratchW, scratchH = 1024, 20

// rect füllt ein Rechteck mit clr (vormultipliziert).
func (p *painter) rect(dst *ebiten.Image, x, y, w, h float64, clr color.RGBA) {
	if p.pixel == nil {
		p.pixel = ebiten.NewImage(1, 1)
		p.pixel.Fill(color.White)
	}
	p.op.GeoM.Reset()
	p.op.GeoM.Scale(w, h)
	p.op.GeoM.Translate(x, y)
	p.op.ColorScale.Reset()
	p.op.ColorScale.ScaleWithColor(clr)
	dst.DrawImage(p.pixel, &p.op)
}

// measure liefert Breite und Höhe von s bei Größe size.
func (p *painter) measure(s string, size float64) (w, h float64) {
	w, h = p.font.Measure(s, size)
	if p.font.Fallback {
		k := textScale(size)
		return w * k, h * k
	}
	return w, h
}

// text zeichnet s mit der linken oberen Ecke bei (x, y).
func (p *painter) text(dst *ebiten.Image, s string, x, y, size float64, clr color.RGBA) {
	if s == "" {
		return
	}
	if !p.font.Fallback {
		p.font.Draw(dst, s, x, y, size, clr)
		return
	}
	if p.scratch == nil {
		p.scratch = ebiten.NewImage(scratchW, scratchH)
	}
	p.scratch.Clear()
	ebitenutil.DebugPrintAt(p.scratch, s, 0, 0)
	k := textScale(size)
	p.op.GeoM.Reset()
	p.op.GeoM.Scale(k, k)
	p.op.GeoM.Translate(x, y)
	p.op.ColorScale.Reset()
	p.op.ColorScale.ScaleWithColor(clr)
	dst.DrawImage(p.scratch, &p.op)
}

// charW ist die Breite eines Zeichens bei Größe size (Lage der Auswahl-Marke).
func (p *painter) charW(size float64) float64 {
	w, _ := p.measure(">", size)
	return w
}

// centered zeichnet s waagerecht mittig um cx, oben bei y.
func (p *painter) centered(dst *ebiten.Image, s string, cx, y, size float64, clr color.RGBA) {
	w, _ := p.measure(s, size)
	p.text(dst, s, cx-w/2, y, size, clr)
}

// page zeichnet die Seite pg ins Fenster der Zelle c.
func (p *painter) page(dst *ebiten.Image, c view.Cell, pg *page) {
	g := geomOf(c.Rect)
	r := c.Rect
	p.backdrop(dst, g)
	if pg.dim {
		p.rect(dst, float64(r.Min.X), float64(r.Min.Y), float64(r.Dx()), float64(r.Dy()), colDim)
	}
	cx := float64(r.Min.X) + float64(r.Dx())/2
	p.centered(dst, pg.title, cx, g.titleY, g.titlePx, colTitle)
	p.centered(dst, pg.sub, cx, g.subY, g.smallPx, colText)
	for i := range pg.n {
		p.card(dst, g, i, &pg.rows[i], i == pg.sel)
	}
	noteClr := colText
	if pg.bad {
		noteClr = colBad
	}
	p.centered(dst, pg.note, cx, g.noteY, g.smallPx, noteClr)
	p.centered(dst, pg.hint, cx, g.hintY, g.smallPx, colText)
	if pg.ask != "" {
		p.askBox(dst, g, pg)
	}
}

// card zeichnet die Karte der sichtbaren Zeile i: Schatten, Papier, Auswahl-Rahmen, Leinen-Schild mit Text.
func (p *painter) card(dst *ebiten.Image, g geom, i int, rw *row, selected bool) {
	x, y, w, h := g.rowRect(i)
	p.paper(dst, g, x, y, w, h, rw.mat, rw.off, selected)
	in := g.stripInset()
	if selected {
		p.text(dst, ">", x-4*in-p.charW(g.rowPx), y+(h-g.rowPx)/2, g.rowPx, colTitle)
	}
	p.rect(dst, x+in, y+in, w-2*in, h-2*in, styleOf(rw.mat, rw.off, selected).strip)
	clr := colLine
	if rw.off {
		clr = colLineOff
	}
	lx := x + 3*in
	if ah := h - 2*in; p.artBox(dst, rw.art, x+2*in, y+in, 1.4*ah, ah) {
		lx += 1.4*ah + in // Text rechts neben der Illustration
	}
	_, th := p.measure(rw.label, g.rowPx)
	ty := y + (h-th)/2
	p.text(dst, rw.label, lx, ty, g.rowPx, clr)
	vw, _ := p.measure(rw.value, g.rowPx)
	p.text(dst, rw.value, x+w-3*in-vw, ty, g.rowPx, clr)
}

// cardStyle ist das Aussehen einer Karte (wie paperCard.ts): Papierton, Kante unten, Leinen-Schild und ob der goldene
// Auswahl-Rahmen sie umgibt.
type cardStyle struct {
	fill, edge, strip color.RGBA
	frame             bool
}

// styleOf liefert das Aussehen einer Karte aus Material m: ausgegraut aus Stoff, gewählt heller mit Rahmen und
// goldhellem Schild (rein, getestet).
func styleOf(m material, off, selected bool) cardStyle {
	if off {
		m = matCloth
	}
	st := cardStyle{fill: m.basis, edge: m.schatten, strip: matLinen.basis}
	if selected {
		st.fill, st.strip, st.frame = m.licht, colPicked, true
	}
	return st
}

// paper zeichnet ein Papier-Rechteck mit Schlagschatten; ausgewählt heller mit goldenem Rahmen.
func (p *painter) paper(dst *ebiten.Image, g geom, x, y, w, h float64, m material, off, selected bool) {
	for _, s := range shadowLayers {
		d := s.d * g.k
		p.rect(dst, x+d, y+d, w, h, fade(colShadow, s.alpha))
	}
	st := styleOf(m, off, selected)
	if st.frame {
		b := frameWidth(g)
		p.rect(dst, x-b, y-b, w+2*b, h+2*b, matGold.licht)
		p.rect(dst, x-b/2, y-b/2, w+b, h+b, colLine)
	}
	p.rect(dst, x, y, w, h, st.fill)
	p.rect(dst, x, y+h-3*g.k, w, 3*g.k, st.edge)
}

// frameWidth ist die Breite des Auswahl-Rahmens um eine Karte (kleiner als der Zeilenabstand: berührt keine Nachbarin).
func frameWidth(g geom) float64 { return 0.9 * g.rowGap }

// askBox zeichnet die Rückfrage: Kasten mit Frage und zwei Knöpfen nebeneinander.
func (p *painter) askBox(dst *ebiten.Image, g geom, pg *page) {
	x, y, w, h := g.askRect()
	p.rect(dst, float64(g.win.Min.X), float64(g.win.Min.Y), float64(g.win.Dx()), float64(g.win.Dy()), colDim)
	p.paper(dst, g, x, y, w, h, matLinen, false, false)
	p.centered(dst, pg.ask, x+w/2, y+0.6*g.rowH, g.smallPx, colLine)
	bw := (w - 3*g.rowGap*2) / 2
	for i, label := range pg.askRows {
		bx := x + 2*g.rowGap + float64(i)*(bw+2*g.rowGap)
		by := y + h - 1.6*g.rowH
		m := matBlue
		if i == 1 {
			m = matRed
		}
		p.paper(dst, g, bx, by, bw, g.rowH, m, false, i == pg.askSel)
		in := g.stripInset()
		p.rect(dst, bx+in, by+in, bw-2*in, g.rowH-2*in, matLinen.licht)
		_, th := p.measure(label, g.rowPx)
		p.centered(dst, label, bx+bw/2, by+(g.rowH-th)/2, g.rowPx, colLine)
	}
}
