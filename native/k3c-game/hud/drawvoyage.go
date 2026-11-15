package hud

// Zeichnen der Schiffsfahrt im Vollbild (B-977, view.VoyageDrawer): Papier über das ganze Bild, darauf das Diorama der
// Seekarte (drawsea_paper.go) ohne Cursor, das Schiff auf dem Bogen von der verlassenen zur Zielinsel, die vordere
// Welle bewegt, die verlassene Insel mit Fahne, sobald das Schiff abgelegt hat. Nach der Fahrzeit liegt das Schiff am
// Ziel und schaukelt weiter. Je Bild keine Allokation über die Zeichenbefehle hinaus.

import (
	"image"
	"math"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

const clrVoyagePaper = 0xeee2c6 // Papier um die Tafel, wie das Ladebild

// seaClip hält den Ausschnitt des Bilds, auf den die bewegte Welle beschnitten wird (SubImage legt ein Bild an: nur
// bei neuem Ziel oder neuer Lage).
type seaClip struct {
	src *ebiten.Image
	r   image.Rectangle
	img *ebiten.Image
}

// of liefert den Ausschnitt r von dst.
func (c *seaClip) of(dst *ebiten.Image, r image.Rectangle) *ebiten.Image {
	if c.img == nil || c.src != dst || c.r != r {
		c.src, c.r, c.img = dst, r, dst.SubImage(r).(*ebiten.Image)
	}
	return c.img
}

// VoyageReady meldet, ob die Fahrt v zu zeichnen ist: Schrift, alle Assets der Karte und beide Inseln auf der Route
// (view.VoyageDrawer; sonst zeigt game den Ladebildschirm, B-977/R5).
func (h *hud) VoyageReady(f *view.Frame, v *view.Voyage) bool {
	if h.font() == nil || h.seaMap == nil || !h.voy.use(h.lang, f.Route, v) {
		return false
	}
	_, ok := h.seaArtOf(&h.voy.sea)
	return ok
}

// DrawVoyage zeichnet die Fahrt v über das ganze Bild dst; false = nichts gezeichnet (Schrift oder Asset fehlt).
func (h *hud) DrawVoyage(dst *ebiten.Image, f *view.Frame, v *view.Voyage) bool {
	y := &h.voy
	if h.font() == nil || !y.use(h.lang, f.Route, v) {
		return false
	}
	a, ok := h.seaArtOf(&y.sea)
	if !ok {
		return false
	}
	h.paint.stencils()
	cell := rectOf(dst.Bounds())
	k := voyageScale * cell.H / refHeight
	if y.sea.sizesK != k {
		y.sea.sizes, y.sea.sizesK = h.measureSea(&y.sea, 28*k), k
	}
	p := seaPaperPlan(y.sea.sizes, len(y.sea.rows), rect{cell.X, cell.Y + 40*k, cell.W, cell.H}, k) // mittig statt etwas hoch
	s := voyageEase(v.Elapsed, v.Duration)
	from := y.node(y.from)
	from.kind = seaTarget // am Liegeplatz noch ohne Fahne
	if s >= voyageFlagAt {
		from.kind = seaDone
	}
	ax, ay := h.berthOf(y.from, from, p)
	bx, by := h.berthOf(y.to, y.node(y.to), p)
	x := seaExtra{ship: true, lane: voyageLane(y.from.lane, y.to.lane, s), shift: voyageShift(voyageWave*p.u, v.Elapsed)}
	x.x, x.y = voyagePos(ax, ay, bx, by, voyageArc*p.u, voyageBob*p.u, v.Elapsed, v.Duration)
	x.front = y.clip.of(dst, image.Rect(int(p.box.X), int(p.front), int(math.Ceil(p.box.X+p.box.W)), int(math.Ceil(p.box.Y+p.box.H))))
	h.paint.fill(dst, cell, rgb(clrVoyagePaper, 1))
	h.drawSeaScene(dst, &y.sea, a, p, x)
	return true
}

// berthOf ist der Liegeplatz des Schiffs an der Insel nd am Platz s in der Lage p.
func (h *hud) berthOf(s seaSpot, nd *seaNode, p seaPlan) (x, y float64) {
	cx, wy := p.nodeX(s.col, s.lane, s.n), p.waterOf(s.lane)
	mini := h.seaSheet(h.seaMap.Island(nd.look.world))
	if mini == nil {
		return cx, wy // seaArtOf hat geprüft; ohne Sheet die Mitte der Insel
	}
	return seaBerth(seaIslandRect(mini, nd, cx, wy, p.u), cx, wy, p.u)
}
