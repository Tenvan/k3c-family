package hud

// Zeichnen des Skill-Menüs (skillMenuView.ts) mittig in der Zelle, etwas hoch, mit Xbox-Glyphen vor belegten Skills
// am Pad (glyphView.ts). Schrift 36 px in breiten, 32 px in schmalen Zellen, kleiner bis das Menü passt, nie unter
// 28 px; mit der Debug-Schrift fest.

import (
	"math"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const (
	menuPxFull = 36.0
	menuPxHalf = 32.0
	menuPxMin  = 28.0
	menuPadX   = 24.0
	menuPadY   = 18.0
	menuBack   = 0x0c1024
)

// glyphPx ist die Glyph-Höhe zur Schriftgröße (glyphs.ts: 28 px → 36 px).
func glyphPx(fontPx float64) float64 { return math.Ceil(fontPx * 1.3) }

// menuSize wählt die Schriftgröße: Grundgröße base, verkleinert im Verhältnis, bis Breite w und Höhe h (gemessen bei
// base) in maxW × maxH passen, nie unter minPx.
func menuSize(base, w, h, maxW, maxH, minPx float64) float64 {
	if w <= 0 || h <= 0 {
		return base
	}
	px := base * min(1, maxW/w, maxH/h)
	return max(minPx, math.Floor(px/2)*2)
}

// Abstände im Skill-Menü je px Schriftgröße: unter der Kopfzeile, zwischen zwei Zeilen und vor dem Hinweis. Ohne sie
// berühren sich die Konturen der Texte (art.Outline8) und die Unterlängen von Go Regular (B-813).
const (
	menuHeadGap = 0.35
	menuRowGap  = 0.15
)

// menuBox ist die Lage des Skill-Menüs: Kasten, Kopfzeile, Zeilen und Hinweis untereinander, dazu Schriftgröße size
// und der Maßstab s der bei Grundgröße gemessenen Maße.
type menuBox struct {
	box, title, footer rect
	rows               [menuWindow]rect
	size, s            float64
}

// menuSizes sind Breite und Höhe von Kopfzeile (Index 0), n Zeilen (1 … n) und Hinweis (n+1) bei Grundgröße.
type menuSizes struct {
	w, h [menuWindow + 2]float64
	n    int
}

// layoutMenu legt das Menü mittig und etwas hoch in die Zelle cell (rein): Größe wie menuSize, mit der Debug-Schrift
// (fixed) fest; Kopfzeile, Zeilen und Hinweis überlappen nicht.
func layoutMenu(cell rect, k, base float64, m *menuSizes, fixed bool) menuBox {
	w, ht := 0.0, 0.0
	for i := range m.n + 2 {
		w, ht = max(w, m.w[i]), ht+m.h[i]
	}
	ht += (2*menuHeadGap + float64(max(0, m.n-1))*menuRowGap) * base
	out := menuBox{size: base, s: 1}
	if !fixed {
		out.size = menuSize(base, w+2*menuPadX*k, ht+2*menuPadY*k, cell.W*0.96, cell.H*0.9, menuPxMin*k)
		out.s = out.size / base
	}
	s := out.s
	out.box = rect{W: w*s + 2*menuPadX*k, H: ht*s + 2*menuPadY*k}
	out.box.X, out.box.Y = cell.X+(cell.W-out.box.W)/2, max(cell.Y, cell.Y+(cell.H-out.box.H)/2-40*k)
	x, y := out.box.X+menuPadX*k, out.box.Y+menuPadY*k
	out.title = rect{x, y, m.w[0] * s, m.h[0] * s}
	y += out.title.H + menuHeadGap*base*s
	for i := range m.n {
		out.rows[i] = rect{x, y, m.w[i+1] * s, m.h[i+1] * s}
		y += out.rows[i].H + menuRowGap*base*s
	}
	if m.n > 0 {
		y += (menuHeadGap - menuRowGap) * base * s
	}
	out.footer = rect{x, y, m.w[m.n+1] * s, m.h[m.n+1] * s}
	return out
}

// drawMenu misst das Menü bei Grundgröße, legt es ab (layoutMenu) und zeichnet Titel, Zeilen (Cursor hell) und Hinweis.
func (h *hud) drawMenu(dst *ebiten.Image, st *seatState, c view.Cell, k float64) {
	f, v, p := h.font(), &st.menuV, c.Player
	if f == nil {
		return
	}
	base := menuPxHalf * k
	if float64(c.Rect.Dx())/k >= 1400 {
		base = menuPxFull * k
	}
	m := menuSizes{n: v.n}
	m.w[0], m.h[0] = h.paint.measureText(f, p, slotMenuTitle, v.title, base)
	for i := range v.n {
		m.w[i+1], m.h[i+1] = h.paint.measureText(f, p, slotMenuRow+textSlot(i), v.rows[i].text, base)
		if v.rows[i].glyph.key != "" {
			m.w[i+1] += glyphPx(base) + 8*k
			m.h[i+1] = max(m.h[i+1], glyphPx(base))
		}
	}
	m.w[v.n+1], m.h[v.n+1] = h.paint.measureText(f, p, slotMenuFooter, v.footer, base)
	b := layoutMenu(rectOf(c.Rect), k, base, &m, f.Fallback)
	h.paint.fill(dst, b.box, rgb(menuBack, 0.8))
	h.paint.text(dst, f, v.title, b.title.X, b.title.Y, b.size, roleBoss.clr)
	for i := range v.n {
		r := b.rows[i]
		h.menuRowAt(dst, f, p, i, r.X, r.Y, r.H, b.size, k)
	}
	h.paint.text(dst, f, v.footer, b.footer.X, b.footer.Y, b.size*0.8, white)
}

// menuRowAt zeichnet Zeile i: Glyph (wenn belegt) und Text; die Zeile unter dem Cursor gelb.
func (h *hud) menuRowAt(dst *ebiten.Image, f *art.Font, p, i int, x, y, rowH, size, k float64) {
	row := &h.seats[p].menuV.rows[i]
	clr := white
	if row.selected {
		clr = yellow
	}
	if row.glyph.key != "" {
		gh := glyphPx(size)
		x += h.drawGlyph(dst, f, p, slotMenuGlyph+textSlot(i), row.glyph, x, y+rowH/2, gh) + 8*k
	}
	h.paint.text(dst, f, row.text, x, y+(rowH-size*1.2)/2, size, clr)
}

// drawGlyph zeichnet eine Glyph der Höhe gh, links an x, mittig an yMid, und liefert ihre Breite: Kreis mit Buchstabe
// (A, X, Y) bzw. Menu-Strichen, Kappe mit Beschriftung (Schultertasten, Tastatur), Steuerkreuz (UP, DOWN).
func (h *hud) drawGlyph(dst *ebiten.Image, f *art.Font, p int, slot textSlot, g glyph, x, yMid, gh float64) float64 {
	kind, name, _ := strings.Cut(g.key, ":")
	pt := &h.paint
	switch {
	case kind == "key" || name == "LB" || name == "RB" || name == "LT" || name == "RT":
		cols, label := glyphColors["pad"], name
		if kind == "key" {
			cols, label = glyphColors["key"], g.label
		}
		tw, th := pt.measureText(f, p, slot, label, gh*0.5)
		w := max(gh, tw+gh*0.5)
		r := rect{x, yMid - gh/2, w, gh}
		pt.fill(dst, r, cols[0])
		pt.frame(dst, r, 2, black)
		pt.text(dst, f, label, x+(w-tw)/2, yMid-th/2, gh*0.5, cols[1])
		return w
	case name == "UP" || name == "DOWN":
		s := gh / 3
		pt.fill(dst, rect{x + s - 2, yMid - gh/2, s + 4, gh}, black)
		pt.fill(dst, rect{x - 2, yMid - s/2 - 2, gh + 4, s + 4}, black)
		pt.fill(dst, rect{x + s, yMid - gh/2 + 2, s, gh - 4}, glyphColors["pad"][0])
		pt.fill(dst, rect{x, yMid - s/2, gh, s}, glyphColors["pad"][0])
		ty := yMid + s/2 + 1
		if name == "UP" {
			ty = yMid - gh/2 + 5
		}
		pt.fill(dst, rect{x + s + 3, ty, s - 6, s - 6}, glyphColors["pad"][1])
		return gh
	}
	cols, ok := glyphColors[name]
	if !ok {
		cols = glyphColors["pad"]
	}
	pt.circle(dst, x+gh/2, yMid, gh/2, black)
	pt.circle(dst, x+gh/2, yMid, gh/2-2, cols[0])
	if name == "MENU" {
		for _, dy := range [3]float64{-0.18, 0, 0.18} {
			pt.fill(dst, rect{x + gh*0.28, yMid + dy*gh - 1.5, gh * 0.44, 3}, cols[1])
		}
		return gh
	}
	tw, th := pt.measureText(f, p, slot, name, gh*0.6)
	pt.text(dst, f, name, x+(gh-tw)/2, yMid-th/2, gh*0.6, cols[1])
	return gh
}
