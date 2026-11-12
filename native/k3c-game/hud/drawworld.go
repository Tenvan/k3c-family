package hud

// Zeichnen an Orten der Welt (Aktions-Hinweis und geführte erste Nacht über dem Monarchen bzw. Objekt, Berufe und
// Händler als Labels) und des Radars. Welt-Texte erscheinen in fester Größe (Browser: scaleTexts gleicht den Zoom aus).

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// Radar-Farben (radarView.ts).
const (
	radarCastleC = 0xffd166
	radarPortalC = 0xb388ff
	radarExitC   = 0x6bd96b
	radarEnemyC  = 0xff4d4d
)

// screenAt ist der Fenster-Punkt h Units über dem Boden bei Welt-x x in der Zelle; oben bleibt Platz frei (Home-
// Button des Browsers, hier die oberen Zeilen). ok false außerhalb der Zelle.
func screenAt(c view.Cell, x, h, k float64) (sx, sy float64, ok bool) {
	sx = c.Camera.ScreenX(x)
	if sx < float64(c.Rect.Min.X) || sx > float64(c.Rect.Max.X) {
		return 0, 0, false
	}
	return sx, max(float64(c.Rect.Min.Y)+hintTopFreeRefPixel*k, c.ScreenY(h)), true
}

// visibleLabels hängt an dst die Lage der Labels, die in der Zelle sichtbar sind, in Reihenfolge von labels
// (Händler zuerst), höchstens maxLabels: Das Budget zählt erst nach der Sichtprüfung.
func visibleLabels(dst []labelSpot, labels []label, c view.Cell, k float64) []labelSpot {
	for i, l := range labels {
		if len(dst) >= maxLabels {
			break
		}
		if x, y, ok := screenAt(c, l.x, l.y, k); ok {
			dst = append(dst, labelSpot{i: i, x: x, y: y})
		}
	}
	return dst
}

// labelSpot ist ein sichtbares Label: Index in seatState.labels und Fenster-Punkt.
type labelSpot struct {
	i    int
	x, y float64
}

// drawWorldTexts zeichnet Labels, Aktions-Hinweis und Hinweis der ersten Nacht der Zelle.
func (h *hud) drawWorldTexts(dst *ebiten.Image, st *seatState, c view.Cell, k float64) {
	f := h.font()
	h.spots = visibleLabels(h.spots[:0], st.labels, c, k)
	for n, sp := range h.spots {
		l := st.labels[sp.i]
		if l.dot != 0 {
			h.paint.circle(dst, sp.x, sp.y, 12*k*0.5, rgb(l.dot, 1))
		}
		h.centered(dst, f, c.Player, slotLabel+textSlot(n), l.text, sp.x, sp.y-10*k, roleCitizen.px*k*0.75, roleCitizen.clr, k)
	}
	if st.hintS != "" {
		if x, y, ok := screenAt(c, st.x, hintAboveUnits, k); ok {
			h.centered(dst, f, c.Player, slotHint, st.hintS, x, y, roleHint.px*k, roleHint.clr, k)
		}
	}
	if st.guideS != "" {
		if x, y, ok := screenAt(c, st.guide.x, guideHeight(st.guide), k); ok {
			h.drawGuide(dst, f, c.Player, st, x, y, k)
		}
	}
}

// Farben der Bilder der Anleitung: Münze (Gold wie das Münzbild) und Papiermond auf Nachtblau (paperBackdrop.ts).
const (
	coinFill  = 0xffd23c
	coinRim   = 0xc48a1e
	moonLight = 0xf3ead4
	moonNight = 0x0b1026
)

// guideLayout legt Bild und Text eines Hinweises der Breite tw und Höhe th mittig über (x, y) (Unterkante): mit Bild
// steht links ein Quadrat der Texthöhe, dann ein Abstand, dann der Text; ohne Bild nur der Text (rein, getestet).
func guideLayout(x, y, tw, th float64, icon bool, k float64) (img rect, tx, ty float64) {
	ty = y - th
	if !icon {
		return rect{}, x - tw/2, ty
	}
	s, gapX := th, 8*k
	left := x - (s+gapX+tw)/2
	return rect{left, ty, s, s}, left + s + gapX, ty
}

// drawGuide zeichnet den Hinweis der ersten Nacht: Bild (Münze, Mond) und Text, mit der Debug-Schrift auf dunkler Fläche.
func (h *hud) drawGuide(dst *ebiten.Image, f *art.Font, p int, st *seatState, x, y, k float64) {
	size := rolePlayer.px * k
	tw, th := h.paint.measureText(f, p, slotGuide, st.guideS, size)
	if tw == 0 {
		return
	}
	icon := guideImage(st.guide.id, h.paint.disc != nil)
	img, tx, ty := guideLayout(x, y, tw, th, icon != iconNone, k)
	if f.Fallback {
		h.paint.fill(dst, rect{tx - boxPadX*k, ty - boxPadY*k, tw + 2*boxPadX*k, th + 2*boxPadY*k}, rgb(0, boxAlpha))
	}
	h.drawIcon(dst, icon, img)
	h.paint.text(dst, f, st.guideS, tx, ty, size, rolePlayer.clr)
}

// drawIcon zeichnet das Bild der Anleitung in r: Münze mit Rand und Glanz, Mond als Sichel auf einer Nachtscheibe.
func (h *hud) drawIcon(dst *ebiten.Image, icon guideIcon, r rect) {
	pt := &h.paint
	cx, cy, rad := r.X+r.W/2, r.Y+r.H/2, r.W/2
	switch icon {
	case iconCoin:
		pt.circle(dst, cx, cy, rad, rgb(coinRim, 1))
		pt.circle(dst, cx, cy, rad*0.78, rgb(coinFill, 1))
		pt.circle(dst, cx-rad*0.25, cy-rad*0.25, rad*0.22, rgb(0xffffff, 0.6))
	case iconMoon:
		pt.circle(dst, cx, cy, rad, rgb(moonNight, 0.9))
		pt.circle(dst, cx, cy, rad*0.72, rgb(moonLight, 1))
		pt.circle(dst, cx-rad*0.3, cy-rad*0.18, rad*0.62, rgb(moonNight, 1))
	}
}

// centered zeichnet einen Text mittig über (x, y) (Unterkante); mit der Debug-Schrift auf dunkler Fläche.
func (h *hud) centered(dst *ebiten.Image, f *art.Font, p int, slot textSlot, s string, x, y, size float64, clr color.RGBA, k float64) {
	w, ht := h.paint.measureText(f, p, slot, s, size)
	if w == 0 {
		return
	}
	tx, ty := x-w/2, y-ht
	if f.Fallback {
		h.paint.fill(dst, rect{tx - boxPadX*k, ty - boxPadY*k, w + 2*boxPadX*k, ht + 2*boxPadY*k}, rgb(0, boxAlpha))
	}
	h.paint.text(dst, f, s, tx, ty, size, clr)
}

// drawRadar zeichnet Leiste, Marker und Kamera-Ausschnitt.
func (h *hud) drawRadar(dst *ebiten.Image, st *seatState, c view.Cell, r rect, k float64) {
	pt := &h.paint
	pt.fill(dst, r, rgb(0, 0.5))
	pt.frame(dst, r, 1, rgb(0xffffff, 0.35))
	y := r.Y + r.H/2
	for _, m := range st.radar {
		x := r.X + m.pos*r.W
		switch m.kind {
		case radarPlayer:
			a := 1.0
			if m.down {
				a = 0.4
			}
			rad := 6 * k
			if m.own {
				rad = 8 * k
				pt.ringAt(dst, x, y, rad+1*k, rgb(0xffffff, a))
			}
			pt.circle(dst, x, y, rad, rgb(playerColors[m.index%len(playerColors)], a))
		case radarEnemy:
			pt.circle(dst, x, y, 3*k, rgb(radarEnemyC, 1))
		case radarCastle:
			pt.fill(dst, rect{x - 5*k, y - 5*k, 10 * k, 10 * k}, rgb(radarCastleC, 1))
		case radarPortal:
			pt.stamp(dst, pt.tri, x-6*k, y-7*k, 12*k, 13*k, rgb(radarPortalC, 1))
		default:
			pt.stamp(dst, pt.tri, x-6*k, y-7*k, 12*k, 13*k, rgb(radarExitC, 1))
		}
	}
	from, to := radarView(c.Camera, st.widthUnits)
	pt.frame(dst, rect{r.X + from*r.W, r.Y + 1, max(2, (to-from)*r.W), r.H - 2}, 2*k, rgb(0xffffff, 0.9))
}
