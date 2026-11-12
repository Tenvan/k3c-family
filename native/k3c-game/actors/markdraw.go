package actors

// Zeichnen der Anzeigen (marks.go): weißer Bildpunkt und weiße Scheibe als Vorlagen, eingefärbt über ColorScale; je
// Bild entsteht nichts Neues. Die Vorlagen entstehen beim ersten Zeichnen.

import (
	"image"
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

const stencilPx = 64

// marksHalf: halbe Breite der Anzeigen in Units (Balken 50 px, Punkt 56 px) für die Sichtprüfung.
const marksHalf = 2.0

// ink hält die Vorlagen und die Optionen der Anzeigen.
type ink struct {
	pixel, disc *ebiten.Image
	op          ebiten.DrawImageOptions
}

func newInk() *ink {
	k := &ink{}
	white := ebiten.NewImage(3, 3)
	white.Fill(color.White)
	k.pixel = white.SubImage(image.Rect(1, 1, 2, 2)).(*ebiten.Image) // Rand weg: kein Ausbluten beim Filtern
	const r = stencilPx / 2
	k.disc = ebiten.NewImage(stencilPx, stencilPx)
	vector.FillCircle(k.disc, r, r, r, color.White, true)
	k.op.Filter = ebiten.FilterLinear
	return k
}

// tint ist 0xRRGGBB mit Deckkraft a (0 … 1), vormultipliziert.
func tint(hex uint32, a float64) color.RGBA {
	a = math.Max(0, math.Min(1, a))
	return color.RGBA{uint8(float64(hex>>16&0xff) * a), uint8(float64(hex>>8&0xff) * a), uint8(float64(hex&0xff) * a), uint8(255 * a)}
}

// stamp zeichnet die Vorlage img in das Rechteck mit linker oberer Ecke (x, y) und Größe w × h.
func (k *ink) stamp(dst, img *ebiten.Image, x, y, w, h float64, clr color.RGBA) {
	if w <= 0 || h <= 0 || clr.A == 0 {
		return
	}
	b := img.Bounds()
	k.op.GeoM.Reset()
	k.op.GeoM.Scale(w/float64(b.Dx()), h/float64(b.Dy()))
	k.op.GeoM.Translate(x, y)
	k.op.ColorScale.Reset()
	k.op.ColorScale.ScaleWithColor(clr)
	dst.DrawImage(img, &k.op)
}

// anchor ist der Bezugspunkt der Anzeigen einer Figur: Fußpunkt (sx, sy) in Fenster-px, s Fenster-px je Spiel-Pixel.
type anchor struct{ sx, sy, s float64 }

// rect füllt b (px über dem Fußpunkt, y nach oben) mit clr; grow vergrößert es rundum um so viele px.
func (k *ink) rect(dst *ebiten.Image, a anchor, b box, grow float64, clr color.RGBA) {
	w, h := b.w+2*grow, b.h+2*grow
	k.stamp(dst, k.pixel, a.sx+(b.cx-w/2)*a.s, a.sy-(b.cy+h/2)*a.s, w*a.s, h*a.s, clr)
}

// oval füllt die Ellipse in b.
func (k *ink) oval(dst *ebiten.Image, a anchor, b box, clr color.RGBA) {
	k.stamp(dst, k.disc, a.sx+(b.cx-b.w/2)*a.s, a.sy-(b.cy+b.h/2)*a.s, b.w*a.s, b.h*a.s, clr)
}

// bar zeichnet den Lebensbalken: schwarzer Hintergrund, Füllung in der Farbe des Balkens.
func (k *ink) bar(dst *ebiten.Image, a anchor, l barLook, topPx, frac, alpha float64) {
	back, fill := barBoxes(l, topPx, frac)
	k.rect(dst, a, back, 0, tint(barBack, barBackAlpha*alpha))
	k.rect(dst, a, fill, 0, tint(l.color, alpha))
}

// marksAnchor liefert den Bezugspunkt der Anzeigen der Figur fg bei Welt-x x, wenn sie in die Zelle ragen.
func marksAnchor(c view.Cell, fg *figure, x float64) (anchor, bool) {
	if !c.Camera.Visible(x, marksHalf) {
		return anchor{}, false
	}
	return anchor{c.Camera.ScreenX(x), c.ScreenY(fg.lift), c.Camera.Scale / unitPx}, true
}

// topPx ist die Oberkante der gezeichneten Figur über dem Fußpunkt in Spiel-Pixeln (Rig: Rahmen-Oberkante, beritten
// samt Reittier; Sheet: Höhe über dem Fuß-Ursprung).
func (d *drawer) topPx(fg *figure) float64 {
	if fg.actor != nil {
		size, frame := rigSize(fg)
		return -frame[1] * size * unitPx
	}
	if d.lib != nil {
		if s := d.lib.Sheet(fg.look.Asset, fg.pose.Anim); s != nil {
			return fg.look.Height * s.OriginY * unitPx
		}
	}
	return fg.look.Height * unitPx
}

// troopMarks zeichnet Last und Lebensbalken einer Truppe.
func (d *drawer) troopMarks(dst *ebiten.Image, c view.Cell, fg *figure, t *sim.Troop) {
	a, ok := marksAnchor(c, fg, t.X)
	if !ok {
		return
	}
	top := d.topPx(fg)
	if Carrying(t.Job) {
		b := loadBox(top, fg.left)
		d.ink.rect(dst, a, b, edgeWidth/2, tint(loadEdge, fg.ui))
		d.ink.rect(dst, a, b, -edgeWidth/2, tint(loadFill, fg.ui))
	}
	if BarVisible(t.HP, t.MaxHP) {
		d.ink.bar(dst, a, troopBar, top, BarFraction(t.HP, t.MaxHP), fg.ui)
	}
}

// enemyMarks zeichnet Goldkreis und Lebensbalken eines Gegners.
func (d *drawer) enemyMarks(dst *ebiten.Image, c view.Cell, fg *figure, e *sim.Enemy) {
	a, ok := marksAnchor(c, fg, e.X)
	if !ok {
		return
	}
	top := d.topPx(fg)
	if HasLoot(e.CarriedGold) {
		cx, cy := lootCenter(top)
		d.ink.oval(dst, a, box{cx, cy, 2*lootRadius + edgeWidth, 2*lootRadius + edgeWidth}, tint(lootEdge, fg.ui))
		d.ink.oval(dst, a, box{cx, cy, 2*lootRadius - edgeWidth, 2*lootRadius - edgeWidth}, tint(lootFill, fg.ui))
	}
	if BarVisible(e.HP, e.MaxHP) {
		d.ink.bar(dst, a, enemyBar, top, BarFraction(e.HP, e.MaxHP), fg.ui)
	}
}

// playerMarker zeichnet den Punkt in der Spielerfarbe unter den Füßen (vor der Figur).
func (d *drawer) playerMarker(dst *ebiten.Image, c view.Cell, fg *figure, p *sim.Player) {
	if a, ok := marksAnchor(c, fg, p.X); ok {
		d.ink.oval(dst, a, markerBox(), tint(PlayerColor(p.Index), markerAlpha*fg.ui))
	}
}

// playerBarMark zeichnet den Lebensbalken des lebenden, verletzten Monarchen (nach der Figur).
func (d *drawer) playerBarMark(dst *ebiten.Image, c view.Cell, fg *figure, p *sim.Player) {
	if !PlayerBarVisible(p.HP, p.MaxHP, p.RespawnIn) {
		return
	}
	if a, ok := marksAnchor(c, fg, p.X); ok {
		d.ink.bar(dst, a, playerBar, d.topPx(fg), BarFraction(p.HP, p.MaxHP), fg.ui)
	}
}
