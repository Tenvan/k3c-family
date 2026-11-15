package actors

// Anzeigen der Kampfregeln an den Figuren (KS5, B-934, docs/rules/kampf.md): Krone am Kopf des Monarchen und auf dem Kopf
// ihres Trägers (Gegner oder Mitspieler), Monarch ohne Krone, Banner am Monarchen, Schild des Gruppenführers und die
// dezente Gruppenmarke der Truppe. Fliegende und liegende Kronen, Portal-HP und Zahlstellen zeichnet props. Bis es
// Papierschnitt-Assets gibt (assets_*, 005), entstehen alle Formen aus Rechtecken und Scheiben wie die Balken (vorläufig).

import (
	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Maße in Spiel-Pixeln (wie marks.go), Farben als 0xRRGGBB.
const (
	crownFill  = 0xffd166
	crownEdge  = 0x9c6644
	crownAbove = 10.0 // Mitte der Krone über der Oberkante der Figur

	bareAlpha = 0.3 // Deckkraft des Spieler-Punkts ohne Krone, relativ zu markerAlpha

	bannerPoleW  = 3.0
	bannerFlagW  = 16.0
	bannerFlagH  = 12.0
	bannerPole   = 0x6b4226
	bannerOffset = 1.6 // Abstand des Stocks in Vielfachen des Last-Versatzes (hinter dem Rücken)

	shieldW, shieldH = 14.0, 18.0
	shieldFill       = 0x8d99ae
	shieldEdge       = 0x343a40
	shieldSide       = 16.0 // seitlicher Versatz des Schilds

	groupDot      = 6.0
	groupDotAlpha = 0.7
)

// crownBoxes ist eine Krone mit Mitte der Basis (cx, cy): Band und drei Zacken.
func crownBoxes(cx, cy float64) [4]box {
	var out [4]box
	for i, q := range view.CrownParts {
		out[i] = box{cx + q[0], cy + q[1], q[2], q[3]}
	}
	return out
}

// crown zeichnet eine Krone mit dunklem Rand; alpha ist die Deckkraft der Figur.
func (k *ink) crown(dst *ebiten.Image, a anchor, cx, cy, alpha float64) {
	bs := crownBoxes(cx, cy)
	for _, b := range bs {
		k.rect(dst, a, b, edgeWidth/2, tint(crownEdge, alpha))
	}
	for _, b := range bs {
		k.rect(dst, a, b, 0, tint(crownFill, alpha))
	}
}

// crownCarrier: Der Monarch p trägt die Krone eines Mitspielers (CrownPlayer, Träger ist er).
func crownCarrier(w *sim.World, p *sim.Player) bool {
	for _, o := range w.Players {
		if o != nil && o.Crown != nil && o.Crown.State == view.CrownPlayer && o.Crown.Carrier == p.ID {
			return true
		}
	}
	return false
}

// bannerBoxes sind Stock und Flagge des Banners eines Monarchen mit Oberkante topPx; die Flagge zeigt weg vom Rücken.
func bannerBoxes(topPx float64, left bool) (pole, flag box) {
	ox := LoadOffset(left) * bannerOffset
	dir := 1.0
	if ox < 0 {
		dir = -1
	}
	return box{ox, topPx / 2, bannerPoleW, topPx}, box{ox + dir*(bannerPoleW+bannerFlagW)/2, topPx - bannerFlagH/2, bannerFlagW, bannerFlagH}
}

// playerMarks zeichnet Krone (am Kopf bzw. getragen) und Banner des Monarchen.
func (d *drawer) playerMarks(dst *ebiten.Image, c view.Cell, fg *figure, p *sim.Player, w *sim.World) {
	a, ok := marksAnchor(c, fg, p.X)
	if !ok {
		return
	}
	top := d.topPx(fg)
	if p.Crown == nil || crownCarrier(w, p) {
		d.ink.crown(dst, a, 0, top+crownAbove, fg.ui)
	}
	if p.Banner != 0 {
		pole, flag := bannerBoxes(top, fg.left)
		d.ink.rect(dst, a, pole, 0, tint(bannerPole, fg.ui))
		d.ink.rect(dst, a, flag, edgeWidth/2, tint(crownEdge, fg.ui))
		d.ink.rect(dst, a, flag, 0, tint(PlayerColor(p.Index), fg.ui))
	}
}

// shieldBoxes ist das Schild des Gruppenführers neben einer Truppe mit Oberkante topPx: Fläche und runde Spitze.
func shieldBoxes(topPx float64, left bool) (body, tip box) {
	cx, cy := -LoadOffset(left)*shieldSide/loadSide, topPx*loadHeight
	return box{cx, cy + 3, shieldW, shieldH - 6}, box{cx, cy - shieldH/2 + 5, shieldW, 12}
}

// leaderMarks zeichnet das Schild des Gruppenführers, sonst bei zugeordneter Truppe die dezente Gruppenmarke über dem Kopf.
func (d *drawer) leaderMarks(dst *ebiten.Image, a anchor, fg *figure, t *sim.Troop, top float64) {
	switch {
	case t.Leader:
		body, tip := shieldBoxes(top, fg.left)
		d.ink.rect(dst, a, body, edgeWidth/2, tint(shieldEdge, fg.ui))
		d.ink.oval(dst, a, box{tip.cx, tip.cy, tip.w + edgeWidth, tip.h + edgeWidth}, tint(shieldEdge, fg.ui))
		d.ink.rect(dst, a, body, -edgeWidth/2, tint(shieldFill, fg.ui))
		d.ink.oval(dst, a, box{tip.cx, tip.cy, tip.w - edgeWidth, tip.h - edgeWidth}, tint(shieldFill, fg.ui))
	case t.Group != 0:
		d.ink.oval(dst, a, box{0, top + groupDot, groupDot, groupDot}, tint(PlayerColor(t.Group), groupDotAlpha*fg.ui))
	}
}
