package hud

// Zeichnen der Tasten-Glyphen (B-911): mit geladenem Papierschnitt-Icon die Xbox-Taste als Bild bzw. die leere
// Tastenkappe mit der Beschriftung in Spielschrift darauf; ohne Icon malt das HUD die Glyphe selbst (Kreis, Kappe,
// Steuerkreuz) wie vor den Icons. Die breite Kappe wächst mit der Beschriftung (drei Streifen, Ecken unverzerrt); passt
// die Beschriftung auch dann nicht, wird die Schrift kleiner, nie abgeschnitten.

import (
	"image"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
)

// Maße der Kappen je Glyph-Höhe (Bild 32 px hoch): Schriftgröße der Beschriftung (Textfläche 21 px hoch), Textfläche
// der schmalen Kappe (23 px breit), Rand der breiten Kappe links und rechts zusammen (Kante 48 − 41 px und etwas Luft),
// ihre Breite von (wie das Bild) bis (gestreckt) und die Mitte der Textfläche von oben (y 3,5 … 25; darunter liegt der
// Kappenrand). paintLabelK ist die Schriftgröße in der gemalten Kappe (ohne Icon, wie vor den Icons).
const (
	glyphLabelK   = 0.54
	paintLabelK   = 0.5
	capNarrowText = 23.0 / 32
	capWideMargin = 12.0 / 32
	capWideMin    = 48.0 / 32
	capWideMax    = 3.25
	capTextMid    = 14.25 / 32
	capCornerPx   = 4 // Ecke der breiten Kappe in Bild-Pixeln, bleibt beim Strecken unverzerrt
)

// isKeyGlyph: g ist eine Tastatur-Taste (Kappe mit Beschriftung).
func isKeyGlyph(g glyph) bool { return strings.HasPrefix(g.key, "key:") }

// isShoulder: name ist eine Schultertaste (gemalt als Kappe mit ihrem Namen).
func isShoulder(name string) bool {
	return name == "LB" || name == "RB" || name == "LT" || name == "RT"
}

// capBox ist die Breite der Tastenkappe der Höhe gh und die Schriftgröße ihrer Beschriftung, die bei gh·glyphLabelK
// tw breit ist: Die schmale Kappe bleibt quadratisch, die breite wächst bis capWideMax; reicht das nicht, schrumpft
// die Schrift auf die Textfläche. Mit der Debug-Schrift (fixed, feste Größe) wächst die breite Kappe unbegrenzt.
func capBox(gh, tw float64, wide, fixed bool) (w, px float64) {
	px = gh * glyphLabelK
	if !wide {
		if a := gh * capNarrowText; tw > a && !fixed {
			px *= a / tw
		}
		return gh, px
	}
	m := gh * capWideMargin
	w = max(gh*capWideMin, tw+m)
	if !fixed && w > gh*capWideMax {
		w = gh * capWideMax
		px *= (w - m) / tw
	}
	return w, px
}

// glyphIcon liefert das geladene Papierschnitt-Icon von g (nil: nicht geladen oder ohne Bild, dann wird gemalt).
func (h *hud) glyphIcon(g glyph) *art.Sheet { return h.icon(g.icon) }

// glyphWidth ist die Breite der Glyph g bei Höhe gh, wie drawGlyph sie zeichnet; die Beschriftung misst es im Element
// slot des Spielers p.
func (h *hud) glyphWidth(f *art.Font, p int, slot textSlot, g glyph, gh float64) float64 {
	_, name, _ := strings.Cut(g.key, ":")
	icon := h.glyphIcon(g) != nil
	switch {
	case isKeyGlyph(g) && icon:
		tw, _ := h.paint.measureText(f, p, slot, g.label, gh*glyphLabelK)
		w, _ := capBox(gh, tw, g.icon == capWide, f.Fallback)
		return w
	case isKeyGlyph(g):
		tw, _ := h.paint.measureText(f, p, slot, g.label, gh*paintLabelK)
		return max(gh, tw+gh*0.5)
	case isShoulder(name) && !icon:
		tw, _ := h.paint.measureText(f, p, slot, name, gh*paintLabelK)
		return max(gh, tw+gh*0.5)
	}
	return gh
}

// drawGlyph zeichnet eine Glyph der Höhe gh, links an x, mittig an yMid, und liefert ihre Breite: mit geladenem Icon
// das Papierschnitt-Bild (iconGlyph), sonst gemalt (paintGlyph).
func (h *hud) drawGlyph(dst *ebiten.Image, f *art.Font, p int, slot textSlot, g glyph, x, yMid, gh float64) float64 {
	ic := h.glyphIcon(g)
	if ic == nil {
		return h.paintGlyph(dst, f, p, slot, g, x, yMid, gh)
	}
	var tw, th float64
	if isKeyGlyph(g) {
		tw, th = h.paint.measureText(f, p, slot, g.label, gh*glyphLabelK)
	}
	return h.iconGlyph(dst, f, ic, g, x, yMid, gh, tw, th)
}

// iconGlyph zeichnet g mit dem Icon ic: die Xbox-Taste als Quadrat, die Tastenkappe mit der Beschriftung (dunkel, ohne
// Kontur) mittig in ihrer Textfläche. tw und th sind die Maße der Beschriftung bei gh·glyphLabelK.
func (h *hud) iconGlyph(dst *ebiten.Image, f *art.Font, ic *art.Sheet, g glyph, x, yMid, gh, tw, th float64) float64 {
	y := yMid - gh/2
	if !isKeyGlyph(g) {
		drawIcon(dst, ic, x, y, gh)
		return gh
	}
	w, px := capBox(gh, tw, g.icon == capWide, f.Fallback)
	h.drawCap(dst, ic, x, y, w, gh)
	s := px / (gh * glyphLabelK)
	h.paint.label(dst, f, g.label, x+(w-tw*s)/2, y+gh*capTextMid-th*s/2, px, glyphColors["key"][1])
	return w
}

// drawCap zeichnet die Kappe ic auf (x, y, w, gh): in ihrer eigenen Breite als ein Bild, breiter in drei Streifen (Ecken
// unverzerrt, Mitte gestreckt). Die Streifen schneidet es einmal je Sheet (capCut).
func (h *hud) drawCap(dst *ebiten.Image, ic *art.Sheet, x, y, w, gh float64) {
	fr := ic.Frame(0)
	if fr == nil || ic.FW <= 2*capCornerPx || ic.FH <= 0 {
		return
	}
	pt := &h.paint
	s := gh / float64(ic.FH)
	if w <= float64(ic.FW)*s+0.5 {
		pt.stamp(dst, fr, x, y, w, gh, white)
		return
	}
	if h.capOf != ic {
		b := fr.Bounds()
		l, r := b.Min.X+capCornerPx, b.Max.X-capCornerPx
		for i, cut := range [3]image.Rectangle{{b.Min, image.Pt(l, b.Max.Y)}, {image.Pt(l, b.Min.Y), image.Pt(r, b.Max.Y)}, {image.Pt(r, b.Min.Y), b.Max}} {
			h.capCut[i] = fr.SubImage(cut).(*ebiten.Image)
		}
		h.capOf = ic
	}
	c := capCornerPx * s
	pt.stamp(dst, h.capCut[0], x, y, c, gh, white)
	pt.stamp(dst, h.capCut[1], x+c, y, w-2*c, gh, white)
	pt.stamp(dst, h.capCut[2], x+w-c, y, c, gh, white)
}

// paintGlyph malt eine Glyph ohne Icon: Kreis mit Buchstabe (A, X, Y) bzw. Menu-Strichen, Kappe mit Beschriftung
// (Schultertasten, Tastatur), Steuerkreuz (UP, DOWN).
func (h *hud) paintGlyph(dst *ebiten.Image, f *art.Font, p int, slot textSlot, g glyph, x, yMid, gh float64) float64 {
	_, name, _ := strings.Cut(g.key, ":")
	pt := &h.paint
	switch {
	case isKeyGlyph(g) || isShoulder(name):
		cols, label := glyphColors["pad"], name
		if isKeyGlyph(g) {
			cols, label = glyphColors["key"], g.label
		}
		tw, th := pt.measureText(f, p, slot, label, gh*paintLabelK)
		w := max(gh, tw+gh*0.5)
		r := rect{x, yMid - gh/2, w, gh}
		pt.fill(dst, r, cols[0])
		pt.frame(dst, r, 2, black)
		pt.text(dst, f, label, x+(w-tw)/2, yMid-th/2, gh*paintLabelK, cols[1])
		return w
	case name == "UP" || name == "DOWN":
		paintDPad(dst, pt, name == "UP", x, yMid, gh)
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

// paintDPad malt das Steuerkreuz der Höhe gh mit der Marke oben (up) oder unten.
func paintDPad(dst *ebiten.Image, pt *painter, up bool, x, yMid, gh float64) {
	s := gh / 3
	pt.fill(dst, rect{x + s - 2, yMid - gh/2, s + 4, gh}, black)
	pt.fill(dst, rect{x - 2, yMid - s/2 - 2, gh + 4, s + 4}, black)
	pt.fill(dst, rect{x + s, yMid - gh/2 + 2, s, gh - 4}, glyphColors["pad"][0])
	pt.fill(dst, rect{x, yMid - s/2, gh, s}, glyphColors["pad"][0])
	ty := yMid + s/2 + 1
	if up {
		ty = yMid - gh/2 + 5
	}
	pt.fill(dst, rect{x + s + 3, ty, s - 6, s - 6}, glyphColors["pad"][1])
}
