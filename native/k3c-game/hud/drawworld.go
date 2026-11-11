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
			h.centered(dst, f, c.Player, slotGuide, st.guideS, x, y, rolePlayer.px*k, rolePlayer.clr, k)
		}
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
