package fx

// Zeichnen der Status-Markierungen (statusLooks.ts › drawMark): Bodenflächen (Aura, Sturm) zuerst, Symbole danach;
// Drehung, Puls und Blitze allein aus der Zeit.

import (
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Takte der Status-Looks in Sekunden.
const (
	stunSpin       = 0.9
	stormFlash     = 0.22
	lastStandPulse = 0.6
	stormBoltPx    = 70.0
	stormBolts     = 3
)

// marks zeichnet die Bodenflächen (ground) bzw. die übrigen Markierungen der Zelle zur Zeit now.
func (pt *painter) marks(dst *ebiten.Image, list []mark, c view.Cell, now float64, ground bool) {
	for i := range list {
		m := &list[i]
		onGround := m.kind == statusAura || m.kind == statusStorm
		if onGround != ground || !c.Camera.Visible(m.x, m.radius/unitPx+1) {
			continue
		}
		x, y, s := at(c, m.x, m.y)
		pt.mark(dst, m, x, y, s, now)
	}
}

// mark zeichnet eine Markierung mit Mitte (x, y) in Fenster-px; s rechnet Welt-px in Fenster-px.
func (pt *painter) mark(dst *ebiten.Image, m *mark, x, y, s, now float64) {
	r := m.radius * s
	switch m.kind {
	case statusAura:
		pt.circle(dst, x, y, r, rgba(m.color, 0.14))
		pt.ringAt(dst, x, y, r, rgba(m.color, 0.5))
	case statusStorm:
		pt.storm(dst, x, y, r, s, now)
	case statusShield:
		pt.circle(dst, x, y, r, rgba(m.color, 0.28))
		pt.ringAt(dst, x, y, r, rgba(m.color, 0.8))
		bx, by := x-22*s, y-(104-shieldY)*s
		pt.stamp(dst, pt.pixel, bx, by, 44*s, 5*s, rgba(0, 0.6))
		pt.stamp(dst, pt.pixel, bx, by, 44*s*m.share, 5*s, rgba(m.color, 1))
	case statusLastStand:
		pulse := 0.5 + 0.5*math.Sin(now/lastStandPulse*math.Pi*2)
		pt.ringAt(dst, x, y, r, rgba(m.color, 0.4+0.5*pulse))
	case statusSlow:
		for i := range 3 {
			a := float64(i) / 3 * math.Pi
			dx, dy := math.Cos(a)*r, math.Sin(a)*r
			pt.line(dst, x-dx, y-dy, x+dx, y+dy, max(1, 3*s), rgba(m.color, 1))
		}
	case statusStun:
		for i := range 3 {
			a := now/stunSpin*math.Pi*2 + float64(i)/3*math.Pi*2
			pt.star(dst, x+math.Cos(a)*r, y+math.Sin(a)*r*0.35, 7*s, rgba(m.color, 1))
		}
	case statusTaunt:
		pt.stamp(dst, pt.pixel, x-2.5*s, y-r, 5*s, r*0.7, rgba(m.color, 1))
		pt.circle(dst, x, y+3*s, 3*s, rgba(m.color, 1))
	}
}

// star: kleiner Stern aus zwei Dreiecken (eines gespiegelt).
func (pt *painter) star(dst *ebiten.Image, x, y, size float64, clr color.RGBA) {
	pt.stamp(dst, pt.tri, x-size, y-size, 2*size, 1.6*size, clr)
	b := pt.tri.Bounds()
	pt.op.GeoM.Reset()
	pt.op.GeoM.Scale(2*size/float64(b.Dx()), -1.6*size/float64(b.Dy()))
	pt.op.GeoM.Translate(x-size, y+size)
	dst.DrawImage(pt.tri, &pt.op)
}

// storm: Bodenfläche mit Blitzen, die je stormFlash Sekunden an andere Stellen springen (aus der Zeit, kein Zufall).
func (pt *painter) storm(dst *ebiten.Image, x, y, r, s, now float64) {
	pt.circle(dst, x, y, r, rgba(stormFill, 0.2))
	step := int(math.Floor(now / stormFlash))
	clr := rgba(stormColor, 0.9)
	for i := range stormBolts {
		bx := x + (float64((step*7+i*13)%17)/8-1)*r*0.8
		top := y - stormBoltPx*s
		pt.line(dst, bx, top, bx+8*s, top+stormBoltPx*s*0.5, max(1, 3*s), clr)
		pt.line(dst, bx+8*s, top+stormBoltPx*s*0.5, bx-4*s, y, max(1, 3*s), clr)
	}
}
