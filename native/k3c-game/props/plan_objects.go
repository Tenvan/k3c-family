package props

// Zeichenliste der Plantage, Truhen, Sterne und Münzen (worldRenderer.ts). Die Fackeln
// (Halter und Flamme) zeichnet world über der Lichtkarte (lightView.ts › drawTorches, NG2.3).

import (
	"image/color"
	"math"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// plantation: Setzling auf leeren Plätzen, niedrige Pappel auf gewachsenen (GEB5, vor der Baumschule).
func (p *planner) plantation(w *sim.World, k view.Camera) {
	pl := buildings["nursery"].Plantation
	for _, s := range w.Sites {
		if s.Kind != "nursery" || s.State != "built" || !visible(k, s.X, pl.StripUnits+1) {
			continue
		}
		for i := range pl.Slots {
			x := plotX(s.X, i, pl.Slots, pl.StripUnits)
			l := seedlingLook
			if treeAt(w.Nodes, x) {
				l = plantTreeLook
			}
			p.sprite(l, x, 0, animPhase(x, 8))
		}
	}
}

// treeAt meldet, ob ein Baum-Knoten auf dem Platz x steht.
func treeAt(nodes []*sim.ResourceNode, x float64) bool {
	for _, n := range nodes {
		if n.Kind == "tree" && math.Abs(n.X-x) < plotTolerance {
			return true
		}
	}
	return false
}

// pickups: Truhen geschlossen, Sterne darüber; aufgehobene Truhen kurz offen.
func (p *planner) pickups(w *sim.World, info *stageInfo, k view.Camera, clock float64) {
	for _, pk := range w.Pickups {
		l, lift := pickupLook(pk.Kind)
		if !l.ok() || !visible(k, pk.X, 1) {
			continue
		}
		if !p.sprite(l, pk.X, lift, animPhase(pk.X, 8)) {
			p.rect(&p.body, pk.X, lift, px(44), px(36), color.RGBA{156, 102, 68, 255})
		}
	}
	if info == nil {
		return
	}
	for _, o := range info.openings {
		if a := openAlpha(o, clock); a > 0 && p.sprite(chestOpenLook, o.x, 0, 0) {
			p.body[len(p.body)-1].alpha = a
		}
	}
}

// coins: Münzen am Boden (Papier-Münze, ohne Grafik ein goldener Kreis).
func (p *planner) coins(w *sim.World, k view.Camera) {
	for _, c := range w.Coins {
		if visible(k, c.X, 0.5) && !p.sprite(coinLook, c.X, 0, animPhase(c.X, 8)) {
			p.body = append(p.body, item{shape: shapeCircle, x: c.X, y: px(8), w: px(8), clr: gold})
		}
	}
}
