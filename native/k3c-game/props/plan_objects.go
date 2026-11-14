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
			if n := treeAt(w.Nodes, x); n != nil {
				p.felledTree(n, x)
				continue
			}
			p.sprite(seedlingLook, x, 0, animPhase(x, 8))
		}
	}
}

// felledTree: niedrige Pappel auf dem Platz; beim Fällen schrumpft sie und der Balken läuft (worldRenderer.ts ›
// updateNode, wie bei den Waldbäumen in nodes).
func (p *planner) felledTree(n *sim.ResourceNode, x float64) {
	_, h, ok := p.size(plantTreeLook)
	if !ok {
		return
	}
	p.body = append(p.body, item{shape: shapeSprite, look: plantTreeLook, x: x, phase: animPhase(x, 8), sy: fellShrink(n.Progress)})
	if n.Progress > 0 {
		p.bar(x, min(h, pennantMax)+px(28), px(50), n.Progress, gold)
	}
}

// fellShrink ist der vertikale Maßstab eines Baums beim Fällen: bis zu 30 % niedriger (worldRenderer.ts ›
// updateNode: setScale(1, 1 - progress * 0.3)).
func fellShrink(progress float64) float64 { return 1 - progress*0.3 }

// treeAt liefert den Baum-Knoten, der auf dem Platz x steht (nil: keiner).
func treeAt(nodes []*sim.ResourceNode, x float64) *sim.ResourceNode {
	for _, n := range nodes {
		if n.Kind == "tree" && math.Abs(n.X-x) < plotTolerance {
			return n
		}
	}
	return nil
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
