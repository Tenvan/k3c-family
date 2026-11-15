package hud

// Seekarte als Papierschnitt-Diorama (B-964): Meer aus Papierschichten, die Etappen als Spalten von links nach rechts,
// je Insel eine Miniatur ihrer Welt (Größe, Schichten darunter und Aufsatz aus seekarte_look.go), das Schiff an der
// aktuellen Insel, Fahne = abgeschlossen, Nebel und blass = gesperrt. Gelber Rahmen: Cursor dieses Spielers; weißer
// Rahmen: gewähltes Ziel (gilt für alle). Die Inseln einer Etappe stehen hintereinander, jede Reihe hinter ihrer eigenen
// Welle. Fehlt ein Asset, zeichnet drawSea die Formen (drawsea.go). Maße in Asset-Pixeln, u rechnet sie in Bildpunkte
// um; je Bild keine Allokation (Sheets sind Nachschlagen in der Bibliothek, Flächen kommen aus dem painter).

import (
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
)

const (
	seaAnim      = "stand" // Animation von Meer und Miniaturen; Schiff, Aufsätze und Zeichen haben iconAnim
	seaColW      = 340.0   // Breite einer Etappen-Spalte
	seaLaneH     = 90.0    // Abstand der Wasserlinien zweier Reihen
	seaTopH      = 172.0   // Platz über der Wasserlinie der hintersten Reihe (höchste Miniatur)
	seaWaveUp    = 30.0    // Oberkante einer Welle über der Wasserlinie ihrer Reihe
	seaFrontDrop = 36.0    // Oberkante der vordersten Welle unter der letzten Wasserlinie (ihr freier Rand liegt auf der Welle davor)
	seaLayerH    = 96.0    // Höhe einer Papierschicht des Meers
	seaStagger   = 150.0   // seitlicher Versatz der Inseln einer Etappe von Reihe zu Reihe (im Wechsel links, rechts)
	seaSlabH     = 7.0     // Höhe einer Schicht unter der Insel
	clrSeaSky    = 0xdfeaf0
	clrSeaSlabA  = 0xb08a5e
	clrSeaSlabB  = 0x8f6d4a
)

// seaArt sind die Sheets der Karte außer Miniaturen und Aufsätzen (die hängen an der Insel).
type seaArt struct {
	back, mid, front, ship, flag, fog *art.Sheet
}

// seaSheet liefert das Sheet des Assets (nil ohne Grafik, ungeladen oder leer). Ohne Allokation.
func (h *hud) seaSheet(asset string) *art.Sheet {
	if h.lib == nil || asset == "" {
		return nil
	}
	s := h.lib.Sheet(asset, seaAnim)
	if s == nil {
		s = h.lib.Sheet(asset, iconAnim)
	}
	if s == nil || s.FW <= 0 || s.FH <= 0 || s.Frame(0) == nil {
		return nil
	}
	return s
}

// seaArtOf liefert die Sheets der Karte v; ok nur, wenn alle da sind, auch Miniatur und Aufsatz jeder Insel (R5: sonst
// Formen).
func (h *hud) seaArtOf(v *seaView) (seaArt, bool) {
	m := h.seaMap
	if m == nil || h.lib == nil {
		return seaArt{}, false
	}
	a := seaArt{back: h.seaSheet(m.Sea.Back), mid: h.seaSheet(m.Sea.Mid), front: h.seaSheet(m.Sea.Front),
		ship: h.seaSheet(m.Ship), flag: h.seaSheet(m.Marks.Done), fog: h.seaSheet(m.Marks.Locked)}
	if a.back == nil || a.mid == nil || a.front == nil || a.ship == nil || a.flag == nil || a.fog == nil {
		return a, false
	}
	for i := range v.rows {
		for j := range v.rows[i].nodes {
			l := &v.rows[i].nodes[j].look
			if h.seaSheet(m.Island(l.world)) == nil || h.seaSheet(m.Topper(l.seed)) == nil {
				return a, false
			}
		}
	}
	return a, true
}

// seaPlan ist die Lage des Dioramas in der Zelle: Tafel, Bildpunkte je Asset-Pixel u, Schriftgröße b, Rand pad, linke
// Kante der ersten Spalte x0, Wasserlinie der hintersten Reihe water, Oberkante der vordersten Welle front, Zahl der
// Reihen lanes.
type seaPlan struct {
	box              rect
	u, b, pad        float64
	x0, water, front float64
	lanes            int
}

// seaPaperPlan legt das Diorama mittig und etwas hoch in die Zelle; passt es nicht, schrumpft es samt Schrift (nie über
// die Zelle hinaus). m sind die bei Schriftgröße 28·k gemessenen Texte.
func seaPaperPlan(m seaSizes, cols int, cell rect, k float64) seaPlan {
	p := seaPlan{u: k, b: 28 * k, lanes: max(1, m.maxNodes)}
	p.pad = 0.6 * p.b
	head := p.pad + 2.5*p.b // Titel und Etappen-Zeile
	colsW := float64(cols) * seaColW * p.u
	if p.lanes > 1 {
		colsW += seaStagger * p.u // Versatz der Reihen
	}
	w := max(colsW, 2*p.pad+m.titleW, 2*p.pad+m.hintW, 2*p.pad+m.nameW)
	sea := seaTopH + float64(p.lanes-1)*seaLaneH + seaFrontDrop + seaLayerH
	ht := head + sea*p.u
	s := min(1, cell.W*0.96/w, cell.H*0.9/ht)
	p.u, p.b, p.pad, head, w, ht = p.u*s, p.b*s, p.pad*s, head*s, w*s, ht*s
	p.box = rect{X: cell.X + (cell.W-w)/2, Y: max(cell.Y, cell.Y+(cell.H-ht)/2-40*k), W: w, H: ht}
	p.x0 = p.box.X + (w-float64(cols)*seaColW*p.u)/2 // Spalten mittig, der Versatz liegt je zur Hälfte links und rechts
	p.water = p.box.Y + head + seaTopH*p.u
	p.front = p.water + (float64(p.lanes-1)*seaLaneH+seaFrontDrop)*p.u
	return p
}

// waterOf ist die Wasserlinie der Reihe lane (0 = hinten).
func (p seaPlan) waterOf(lane int) float64 { return p.water + float64(lane)*seaLaneH*p.u }

// nodeX ist die Mitte der Insel in Spalte col und Reihe lane von n Inseln der Etappe: eine allein mittig, mehrere im
// Wechsel links und rechts der Mitte, damit die vordere die hintere nicht verdeckt.
func (p seaPlan) nodeX(col, lane, n int) float64 {
	x := p.x0 + (float64(col)+0.5)*seaColW*p.u
	if n > 1 {
		x += (float64(lane%2) - 0.5) * seaStagger * p.u
	}
	return x
}

// seaExtra sind die Zusätze der Schiffsfahrt im Diorama (drawvoyage.go): das fahrende Schiff mit Fußpunkt (x, y),
// gezeichnet nach den Inseln der Reihe lane (es liegt vor ihnen), und die bewegte vordere Welle (um shift Bildpunkte versetzt, beschnitten auf
// den Ausschnitt front). Der Nullwert zeichnet die Karte am Hafen.
type seaExtra struct {
	ship  bool
	lane  int
	x, y  float64
	front *ebiten.Image
	shift float64
}

// drawSeaPaper zeichnet das Diorama der Karte v mit den Sheets a in die Zelle cell.
func (h *hud) drawSeaPaper(dst *ebiten.Image, v *seaView, a seaArt, cell rect, k float64) {
	h.drawSeaScene(dst, v, a, seaPaperPlan(v.sizes, len(v.rows), cell, k), seaExtra{})
}

// drawSeaScene zeichnet das Diorama der Karte v in der Lage p, von hinten nach vorn: je Reihe Welle, Inseln und das
// fahrende Schiff, dann die vordere Welle, Rahmen und Texte.
func (h *hud) drawSeaScene(dst *ebiten.Image, v *seaView, a seaArt, p seaPlan, x seaExtra) {
	f, pt := h.font(), &h.paint
	pt.fill(dst, p.box, rgb(clrSeaBack, 0.85))
	top := p.water - seaTopH*p.u
	pt.fill(dst, rect{p.box.X, top, p.box.W, p.front - top + p.u}, rgb(clrSeaSky, 1))
	pt.text(dst, f, v.title, p.box.X+p.pad, p.box.Y+p.pad, p.b, yellow)
	for i := range v.rows {
		pt.text(dst, f, v.rows[i].title, p.x0+(float64(i)+0.08)*seaColW*p.u, p.box.Y+p.pad+1.4*p.b, 0.8*p.b, white)
	}
	for lane := range p.lanes {
		layer := a.mid
		if lane == 0 {
			layer = a.back
		}
		wy := p.waterOf(lane)
		h.drawSeaStrip(dst, layer, p.box.X, wy-seaWaveUp*p.u, p.box.W, p.u)
		for i := range v.rows {
			if nodes := v.rows[i].nodes; lane < len(nodes) {
				h.drawSeaIsland(dst, a, &nodes[lane], p.nodeX(i, lane, len(nodes)), wy, p.u)
			}
		}
		if x.ship && x.lane == lane {
			h.drawSeaSprite(dst, a.ship, x.x, x.y, p.u, white)
		}
	}
	if x.front != nil { // breiter als die Tafel, damit der Versatz keine Lücke am Rand lässt
		over := math.Abs(x.shift) + p.u
		h.drawSeaStrip(x.front, a.front, p.box.X-over+x.shift, p.front, p.box.W+2*over, p.u)
	} else {
		h.drawSeaStrip(dst, a.front, p.box.X, p.front, p.box.W, p.u)
	}
	focus := h.drawSeaFrames(dst, v, p)
	y := p.front + seaWaveUp*p.u
	if focus != nil {
		pt.text(dst, f, focus.name, p.box.X+p.pad, y, p.b, yellow)
	}
	pt.text(dst, f, v.hint, p.box.X+p.pad, y+1.2*p.b, 0.8*p.b, white)
}

// drawSeaStrip kachelt die Papierschicht s ab (x, y) über die Breite w: so viele ganze Kacheln, wie am besten passen,
// leicht gestreckt statt beschnitten, je 1 Bildpunkt überlappt (keine Naht).
func (h *hud) drawSeaStrip(dst *ebiten.Image, s *art.Sheet, x, y, w, u float64) {
	n := max(1, int(math.Round(w/(float64(s.FW)*u))))
	tw := w / float64(n)
	for i := range n {
		over := 1.0
		if i == n-1 {
			over = 0
		}
		h.paint.stamp(dst, s.Frame(0), x+float64(i)*tw, y, tw+over, float64(s.FH)*u, white)
	}
}

// drawSeaSprite zeichnet Bild 0 von s im Maßstab sc mit dem Fußpunkt bei (x, y), eingefärbt mit clr.
func (h *hud) drawSeaSprite(dst *ebiten.Image, s *art.Sheet, x, y, sc float64, clr color.RGBA) {
	w, ht := float64(s.FW)*sc, float64(s.FH)*sc
	h.paint.stamp(dst, s.Frame(0), x-s.OriginX*w, y-s.OriginY*ht, w, ht, clr)
}

// seaIslandRect ist die Fläche der Miniatur s der Insel nd mit Mitte cx und Wasserlinie wy.
func seaIslandRect(s *art.Sheet, nd *seaNode, cx, wy, u float64) rect {
	w, ht := float64(s.FW)*u*nd.look.scale, float64(s.FH)*u*nd.look.scale
	return rect{cx - s.OriginX*w, wy - s.OriginY*ht, w, ht}
}

// seaBerth ist der Fußpunkt des Schiffs am Liegeplatz der Insel mit Fläche r, Mitte cx und Wasserlinie wy: links vor
// der Miniatur.
func seaBerth(r rect, cx, wy, u float64) (x, y float64) { return cx - 0.46*r.W, wy + 10*u }

// drawSeaIsland zeichnet die Insel nd mit der Mitte cx auf die Wasserlinie wy: Schichten, Miniatur, Aufsatz, Zeichen
// und, liegt das Schiff an, das Schiff. Gesperrte Inseln sind blass.
func (h *hud) drawSeaIsland(dst *ebiten.Image, a seaArt, nd *seaNode, cx, wy, u float64) {
	mini, top := h.seaSheet(h.seaMap.Island(nd.look.world)), h.seaSheet(h.seaMap.Topper(nd.look.seed))
	if mini == nil || top == nil {
		return // seaArtOf hat geprüft; ohne Sheet nichts
	}
	sc, alpha := u*nd.look.scale, 1.0
	if nd.kind == seaLocked {
		alpha = 0.5
	}
	r := seaIslandRect(mini, nd, cx, wy, u)
	for i := range nd.look.layers {
		clr, w := uint32(clrSeaSlabA), r.W*(0.66-0.06*float64(i))
		if i%2 == 1 {
			clr = clrSeaSlabB
		}
		h.paint.fill(dst, rect{cx - w/2, r.Y + r.H - sc + float64(i)*seaSlabH*sc, w, seaSlabH * sc}, rgb(clr, alpha))
	}
	tint := rgb(0xffffff, alpha)
	h.drawSeaSprite(dst, mini, cx, wy, sc, tint)
	h.drawSeaSprite(dst, top, cx+0.42*r.W, wy+3*sc, sc, tint)
	switch nd.kind {
	case seaDone:
		h.drawSeaSprite(dst, a.flag, cx+0.05*r.W, wy-2*sc, sc, white)
	case seaLocked:
		h.drawSeaSprite(dst, a.fog, cx, wy-0.2*r.H, sc, rgb(0xffffff, 0.92))
	}
	if nd.current {
		x, y := seaBerth(r, cx, wy, u)
		h.drawSeaSprite(dst, a.ship, x, y, u, white)
	}
}

// drawSeaFrames zeichnet die Rahmen über alle Schichten: weiß um das gewählte Ziel, gelb innen um den Cursor. Rückgabe:
// die Insel, deren Name unten steht (Cursor, sonst die Wahl; nil ohne beides).
func (h *hud) drawSeaFrames(dst *ebiten.Image, v *seaView, p seaPlan) *seaNode {
	var focus *seaNode
	for i := range v.rows {
		nodes := v.rows[i].nodes
		for j := range nodes {
			nd := &nodes[j]
			if !nd.chosen && !nd.cursor {
				continue
			}
			mini := h.seaSheet(h.seaMap.Island(nd.look.world))
			if mini == nil {
				continue
			}
			r := seaIslandRect(mini, nd, p.nodeX(i, j, len(nodes)), p.waterOf(j), p.u)
			lw := max(2, p.b*0.08)
			if nd.chosen {
				h.paint.frame(dst, rect{r.X - 2*lw, r.Y - 2*lw, r.W + 4*lw, r.H + 4*lw}, lw, white)
			}
			if nd.cursor {
				h.paint.frame(dst, rect{r.X - lw, r.Y - lw, r.W + 2*lw, r.H + 2*lw}, max(3, p.b*0.12), yellow)
			}
			if nd.cursor || focus == nil {
				focus = nd
			}
		}
	}
	return focus
}
