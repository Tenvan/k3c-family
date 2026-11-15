package hud

// Zeichnen der Seekarte am Hafen (seekarte.go) aus gezeichneten Formen: dunkle Tafel, je Etappe eine Zeile mit
// Beschriftung links und den Inseln als Knoten (Marke: Haken = abgeschlossen, Ring = Ziel, Schloss = gesperrt). Gelber
// Rahmen: Cursor dieses Spielers; weißer Rahmen und hellere Fläche: gewähltes Ziel (gilt für alle).

import (
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const (
	clrSeaBack   = 0x0c1024
	clrSeaDone   = 0x1f3d2b
	clrSeaFree   = 0x2b3a67
	clrSeaPicked = 0x4a63b8
	clrSeaOff    = 0x2a2a2a
	clrSeaGreen  = 0x4caf50
	clrSeaGrey   = 0x8a8a8a
)

// seaFill ist die Flächenfarbe eines Knotens.
func seaFill(n seaNode) uint32 {
	switch {
	case n.chosen:
		return clrSeaPicked
	case n.kind == seaDone:
		return clrSeaDone
	case n.kind == seaTarget:
		return clrSeaFree
	}
	return clrSeaOff
}

// seaSizes sind die bei Schriftgröße b gemessenen Maße der Tafel.
type seaSizes struct {
	labW, nameW, titleW, hintW float64
	maxNodes                   int
}

// measureSea misst Beschriftungen, Namen, Titel und Hinweis bei Schriftgröße b.
func (h *hud) measureSea(v *seaView, b float64) seaSizes {
	f := h.font()
	var m seaSizes
	m.titleW, _ = f.Measure(v.title, b)
	m.hintW, _ = f.Measure(v.hint, b*0.8)
	for _, r := range v.rows {
		w, _ := f.Measure(r.title, b)
		m.labW, m.maxNodes = max(m.labW, w), max(m.maxNodes, len(r.nodes))
		for _, n := range r.nodes {
			w, _ = f.Measure(n.name, b)
			m.nameW = max(m.nameW, w)
		}
	}
	return m
}

// drawSea zeichnet die Tafel mittig und etwas hoch in die Zelle c; passt sie nicht, schrumpft sie samt Schrift (nie
// über die Zelle hinaus).
func (h *hud) drawSea(dst *ebiten.Image, st *seatState, c view.Cell, k float64) {
	f, v := h.font(), &st.sea
	if f == nil || len(v.rows) == 0 {
		return
	}
	b := 28 * k
	if v.sizesK != k {
		v.sizes, v.sizesK = h.measureSea(v, b), k // nur bei neuer Karte oder Skalierung messen
	}
	m := v.sizes
	pad, gap, nodeH := 0.6*b, 0.4*b, 1.6*b
	nodeW := m.nameW + nodeH + pad
	n := float64(len(v.rows))
	w := max(2*pad+m.labW+gap+float64(m.maxNodes)*(nodeW+gap), 2*pad+m.titleW, 2*pad+m.hintW)
	ht := 2*pad + 1.4*b + n*(nodeH+gap) + 1.1*b
	cell := rectOf(c.Rect)
	s := min(1, cell.W*0.96/w, cell.H*0.9/ht)
	b, pad, gap, nodeH, nodeW, w, ht = b*s, pad*s, gap*s, nodeH*s, nodeW*s, w*s, ht*s
	box := rect{W: w, H: ht}
	box.X, box.Y = cell.X+(cell.W-w)/2, max(cell.Y, cell.Y+(cell.H-ht)/2-40*k)
	pt := &h.paint
	pt.fill(dst, box, rgb(clrSeaBack, 0.85))
	x, y := box.X+pad, box.Y+pad
	pt.text(dst, f, v.title, x, y, b, yellow)
	y += 1.4 * b
	for _, r := range v.rows {
		pt.text(dst, f, r.title, x, y+(nodeH-b*1.2)/2, b, white)
		nx := x + m.labW*s + gap
		for _, nd := range r.nodes {
			h.drawSeaNode(dst, f, nd, rect{nx, y, nodeW, nodeH}, b)
			nx += nodeW + gap
		}
		y += nodeH + gap
	}
	pt.text(dst, f, v.hint, x, y, b*0.8, white)
}

// drawSeaNode zeichnet den Knoten nd in r: Fläche, Rahmen, Marke und Name.
func (h *hud) drawSeaNode(dst *ebiten.Image, f *art.Font, nd seaNode, r rect, b float64) {
	pt := &h.paint
	pt.fill(dst, r, rgb(seaFill(nd), 1))
	if nd.chosen {
		pt.frame(dst, r, max(2, b*0.08), white)
	}
	if nd.cursor {
		pt.frame(dst, r, max(3, b*0.12), yellow)
	}
	mr := r.H * 0.3
	mx, my := r.X+r.H*0.55, r.Y+r.H/2
	h.drawSeaMark(dst, nd.kind, mx, my, mr)
	clr := white
	if nd.kind == seaLocked {
		clr = rgb(clrSeaGrey, 1)
	}
	pt.text(dst, f, nd.name, r.X+r.H*1.0, r.Y+(r.H-b*1.2)/2, b, clr)
}

// drawSeaMark zeichnet die Marke eines Knotens um (cx, cy) mit Radius r.
func (h *hud) drawSeaMark(dst *ebiten.Image, kind seaKind, cx, cy, r float64) {
	pt := &h.paint
	switch kind {
	case seaDone:
		pt.circle(dst, cx, cy, r, rgb(clrSeaGreen, 1))
		lw := float32(max(2, r*0.22))
		vector.StrokeLine(dst, float32(cx-r*0.5), float32(cy), float32(cx-r*0.1), float32(cy+r*0.4), lw, white, true)
		vector.StrokeLine(dst, float32(cx-r*0.1), float32(cy+r*0.4), float32(cx+r*0.55), float32(cy-r*0.4), lw, white, true)
	case seaTarget:
		pt.ringAt(dst, cx, cy, r, white)
	default: // gesperrt: Schloss aus Bügel und Körper
		pt.ringAt(dst, cx, cy-r*0.25, r*0.5, rgb(clrSeaGrey, 1))
		pt.fill(dst, rect{cx - r*0.7, cy - r*0.1, r * 1.4, r * 0.95}, rgb(clrSeaGrey, 1))
	}
}
