package hud

// Vorrats-Anzeige mit Icons (B-890): je Rohstoff Icon und Zahl in einer Fläche, Skill-Punkte als Text dahinter. Ein
// voller Rohstoff steht in Orange. Fehlt das Icon (ohne Grafik), steht der Name vor der Zahl.

import (
	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
)

// iconScale ist die Kantenlänge eines Icons als Vielfaches der Schriftgröße (lesbar aus TV-Abstand).
const iconScale = 1.5

// fullColor ist die Farbe eines vollen Rohstoffs.
var fullColor = rgb(0xf4a261, 1)

// icon liefert das Icon asset (nil ohne Grafik oder wenn es nicht geladen ist).
func (h *hud) icon(asset string) *art.Sheet {
	if h.lib == nil || asset == "" {
		return nil
	}
	return h.lib.Sheet(asset, iconAnim)
}

// partLabel ist der Text eines Teils: die Zahl, ohne Icon mit dem Namen davor.
func (h *hud) partLabel(p stockPart, lang string) (string, *art.Sheet) {
	ic := h.icon(resourceIcons[p.res])
	if ic == nil && p.res != "" {
		return resName(lang, p.res) + " " + p.text, nil
	}
	return p.text, ic
}

// stockSize ist die Größe der Vorrats-Fläche bei Schriftgröße px und Maßstab k.
func (h *hud) stockSize(st *seatState, f *art.Font, player int, px, k float64) (float64, float64) {
	if len(st.parts) == 0 {
		return 0, 0
	}
	w, ht := 0.0, px*iconScale
	for i, p := range st.parts {
		label, ic := h.partLabel(p, h.lang)
		tw, th := h.paint.measureText(f, player, slotStock+textSlot(i), label, px)
		if ic != nil {
			w += px*iconScale + spaceW(px)
		}
		w += tw + 3*spaceW(px)
		ht = max(ht, th)
	}
	return boxSize(w, ht, k)
}

// drawStock zeichnet die Vorrats-Fläche in r.
func (h *hud) drawStock(dst *ebiten.Image, st *seatState, f *art.Font, player int, r rect, px, k float64) {
	pt := &h.paint
	pt.fill(dst, r, rgb(0, boxAlpha))
	x, y := r.X+boxPadX*k, r.Y+boxPadY*k
	inner := r.H - 2*boxPadY*k
	for i, p := range st.parts {
		label, ic := h.partLabel(p, h.lang)
		if ic != nil {
			s := px * iconScale
			drawIcon(dst, ic, x, y+(inner-s)/2, s)
			x += s + spaceW(px)
		}
		tw, th := pt.measureText(f, player, slotStock+textSlot(i), label, px)
		clr := roleShared.clr
		if p.full {
			clr = fullColor
		}
		pt.text(dst, f, label, x, y+(inner-th)/2, px, clr)
		x += tw + 3*spaceW(px)
	}
}

// drawIcon zeichnet Bild 0 des Icons ic als Quadrat der Kante s mit der linken oberen Ecke (x, y).
func drawIcon(dst *ebiten.Image, ic *art.Sheet, x, y, s float64) {
	if ic.FW <= 0 || ic.FH <= 0 {
		return
	}
	var g ebiten.GeoM
	g.Scale(s/float64(ic.FW), s/float64(ic.FH))
	g.Translate(x, y)
	ic.Draw(dst, 0, g)
}
