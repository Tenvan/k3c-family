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

// drawMenu misst das Menü bei Grundgröße, wählt die Größe und zeichnet Titel, Zeilen (Cursor hell) und Hinweis.
func (h *hud) drawMenu(dst *ebiten.Image, st *seatState, c view.Cell, k float64) {
	f, v, p := h.font(), &st.menuV, c.Player
	if f == nil {
		return
	}
	base := menuPxHalf * k
	if float64(c.Rect.Dx())/k >= 1400 {
		base = menuPxFull * k
	}
	var ws [menuWindow + 2]float64
	var hs [menuWindow + 2]float64
	ws[0], hs[0] = h.paint.measureText(f, p, slotMenuTitle, v.title, base)
	for i := range v.n {
		ws[i+1], hs[i+1] = h.paint.measureText(f, p, slotMenuRow+textSlot(i), v.rows[i].text, base)
		if v.rows[i].glyph.key != "" {
			ws[i+1] += glyphPx(base) + 8*k
			hs[i+1] = max(hs[i+1], glyphPx(base))
		}
	}
	ws[v.n+1], hs[v.n+1] = h.paint.measureText(f, p, slotMenuFooter, v.footer, base)
	w, ht := 0.0, 0.0
	for i := range v.n + 2 {
		w, ht = max(w, ws[i]), ht+hs[i]
	}
	cell := rectOf(c.Rect)
	size := menuSize(base, w+2*menuPadX*k, ht+2*menuPadY*k, cell.W*0.96, cell.H*0.9, menuPxMin*k)
	if f.Fallback {
		size = base
	}
	s := size / base
	if f.Fallback {
		s = 1
	}
	box := rect{W: w*s + 2*menuPadX*k, H: ht*s + 2*menuPadY*k}
	box.X, box.Y = cell.X+(cell.W-box.W)/2, max(cell.Y, cell.Y+(cell.H-box.H)/2-40*k)
	h.paint.fill(dst, box, rgb(menuBack, 0.8))
	x, y := box.X+menuPadX*k, box.Y+menuPadY*k
	h.paint.text(dst, f, v.title, x, y, size, roleBoss.clr)
	y += hs[0] * s
	for i := range v.n {
		h.menuRowAt(dst, f, p, i, x, y, hs[i+1]*s, size, k)
		y += hs[i+1] * s
	}
	h.paint.text(dst, f, v.footer, x, y, size*0.8, white)
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
