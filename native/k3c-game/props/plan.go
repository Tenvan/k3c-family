package props

// Zeichenliste einer Zelle: aus Welt, abgeleitetem Zustand und Kamera entsteht eine Liste von Bildern und Formen in
// Welt-Maßen (Units, Höhe über dem Boden), hinten nach vorn. Rein, ohne Ebitengine; paint.go zeichnet sie.
// Reihenfolge wie die Tiefen im Browser (worldRenderer.ts): Burg, feste Objekte und Natur (0), Hintergrundmauer (1),
// Bauplätze (2), Plantage und Truhen (3), Münzen (6); Schilder, Balken und Texte danach (top).

import (
	"image/color"
	"math"
	"strconv"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// unitPx: Spiel-Pixel je Unit; Papier-Rahmen sind in Spiel-Pixeln (32 je Kachel).
const unitPx = 32.0

// px rechnet Spiel-Pixel des Browsers in Units um.
func px(v float64) float64 { return v / unitPx }

// shape ist die Art eines Eintrags der Zeichenliste.
type shape uint8

const (
	shapeSprite shape = iota // Bild look, Fuß bei (x, y), Startframe phase, vertikal sy
	shapeTiles               // Bild look gekachelt von x bis x2, Fuß auf y
	shapeRect                // Rechteck Mitte x, Unterkante y, Größe w × h
	shapeFrame               // Rechteck-Rahmen wie shapeRect, Linienbreite size
	shapeCircle              // Kreis um (x, y) mit Radius w
	shapeRing                // Ring wie shapeCircle, Linienbreite size
	shapeLine                // Linie (x, y) → (x2, y2), Breite size
	shapeTri                 // Dreieck (Spitze oben) Mitte x, Unterkante y, Größe w × h
	shapeText                // Text Mitte x, Unterkante y, Höhe size
)

// item ist ein Eintrag der Zeichenliste; Maße in Units, y = Höhe über dem Boden.
type item struct {
	shape  shape
	look   look
	phase  int
	x, y   float64
	x2, y2 float64
	w, h   float64
	size   float64
	sy     float64 // vertikaler Maßstab des Bilds (0 = 1)
	shade  float32 // Abdunklung 0 … 1 (Schaden)
	alpha  float32 // Deckkraft (0 = voll)
	clr    color.RGBA
	text   string
}

// sheets liefert die geladenen Sheets (art.Library; nil-Library = keine, dann Platzhalter).
type sheets interface {
	Sheet(asset, anim string) *art.Sheet
}

// planner baut die Zeichenliste; die Puffer bleiben über die Bilder erhalten.
type planner struct {
	lib       sheets
	body, top []item
	walls     []*sim.Site
	backs     []backWall
	roots     []float64
	texts     texts
}

func newPlanner(lib sheets) *planner { return &planner{lib: lib, texts: texts{}} }

// size liefert Breite und Höhe über dem Fuß (Units) des Bilds l; ok = Sheet geladen.
func (p *planner) size(l look) (w, h float64, ok bool) {
	if !l.ok() || p.lib == nil {
		return 0, 0, false
	}
	s := p.lib.Sheet(l.asset, l.anim)
	if s == nil {
		return 0, 0, false
	}
	k := s.UnitsPerPx / unitPx
	return float64(s.FW) * k, float64(s.FH) * s.OriginY * k, true
}

// sprite legt ein Bild an; ohne geladenes Sheet false.
func (p *planner) sprite(l look, x, y float64, phase int) bool {
	if _, _, ok := p.size(l); !ok {
		return false
	}
	p.body = append(p.body, item{shape: shapeSprite, look: l, x: x, y: y, phase: phase})
	return true
}

func (p *planner) rect(buf *[]item, x, y, w, h float64, clr color.RGBA) {
	*buf = append(*buf, item{shape: shapeRect, x: x, y: y, w: w, h: h, clr: clr})
}

// bar zeichnet einen Balken (Hintergrund schwarz, Anteil frac in clr), Unterkante y.
func (p *planner) bar(x, y, w, frac float64, clr color.RGBA) {
	h := px(8)
	p.rect(&p.top, x, y, w, h, color.RGBA{0, 0, 0, 153})
	f := min(max(frac, 0), 1)
	p.rect(&p.top, x-w/2+w*f/2, y, w*f, h, clr)
}

// label legt einen Text an (Mitte x, Unterkante y; Größe 32 Spiel-Pixel wie fontRules › priceTag).
func (p *planner) label(x, y float64, s string, clr color.RGBA) {
	if s != "" {
		p.top = append(p.top, item{shape: shapeText, x: x, y: y, size: 1, text: s, clr: clr})
	}
}

// visible: ragt ein Objekt bei x mit halber Breite half in die Zelle (half großzügig für Schilder darüber).
func visible(k view.Camera, x, half float64) bool { return k.Scale == 0 || k.Visible(x, half) }

// plan baut die Zeichenliste der Welt w (info = Zustand aus Listen, darf nil sein) für Kamera k.
func (p *planner) plan(w *sim.World, info *stageInfo, k view.Camera, clock float64) {
	p.body, p.top = p.body[:0], p.top[:0]
	if w == nil {
		return
	}
	biome := w.Biome.ID
	p.castle(w, info, k)
	p.entities(w, k, biome)
	p.nodes(w, k, biome)
	p.backWalls(w, k)
	p.sites(w, info, k)
	p.plantation(w, k)
	p.pickups(w, info, k, clock)
	p.coins(w, k)
}

var (
	white     = color.RGBA{255, 255, 255, 255}
	gold      = color.RGBA{255, 209, 102, 255}
	hpGreen   = color.RGBA{82, 183, 136, 255}
	dark      = color.RGBA{52, 58, 64, 255}
	stoneGray = color.RGBA{108, 117, 125, 255}
	poorRed   = color.RGBA{255, 143, 163, 255}
)

// castle: Look der Wachstumsreihe, Gerüst davor, Schmuck der Ausbauschritte; Balken und Ausbau-Schild darüber.
func (p *planner) castle(w *sim.World, info *stageInfo, k view.Camera) {
	x := w.Castle.X
	if !visible(k, x, 6) {
		return
	}
	lv, step, state := max(1, w.HubLevel), max(1, w.HubStep), ""
	if info != nil {
		state = info.hub.state
	}
	hl := hubLookOf(lv, step, state)
	main := hl.main
	if _, _, ok := p.size(main); !ok {
		main = look{buildingGroup + "burg", stageAnim(lv)} // fehlt der Look, die Burg der Hub-Stufe
	}
	_, h, ok := p.size(main)
	if ok {
		p.sprite(main, x, 0, 0)
	} else {
		h = px(290)
		p.castleShape(x)
	}
	if hl.scaffold {
		p.sprite(hubScaffold, x, 0, 0)
	}
	if ok && hl.overlay.ok() {
		p.sprite(hl.overlay, x, 0, 0)
	}
	if c := w.Castle; c.MaxHP > 0 && c.HP < c.MaxHP {
		p.bar(x, px(310-4), px(240), c.HP/c.MaxHP, hpGreen)
	}
	if info != nil {
		p.castleSign(w, info, x, h+px(20))
	}
}

// castleShape: Platzhalter-Burg ohne Grafik (siteView.ts › drawCastle).
func (p *planner) castleShape(x float64) {
	p.rect(&p.body, x, 0, px(300), px(260), stoneGray)
	p.body = append(p.body, item{shape: shapeFrame, x: x, w: px(300), h: px(260), size: px(4), clr: dark})
	p.rect(&p.body, x, 0, px(60), px(80), dark)
	for _, dx := range [4]float64{-120, -40, 40, 120} {
		p.rect(&p.body, x+px(dx), px(260), px(40), px(30), stoneGray)
	}
}

// castleSign: läuft der Ausbau, der Wartegrund; sonst am nächsten Preisschild das Angebot (B-591).
func (p *planner) castleSign(w *sim.World, info *stageInfo, x, y float64) {
	hub := info.hub
	if !hub.has {
		return
	}
	if r := waitOf(hub.state, info.danger); r != waitNone {
		need := hub.material
		if m := missingArr(hub.material, info.stock); hub.state == sim.WaitingNextIsland && bitsArr(m) != 0 {
			need = m // Material-Tor: nur nennen, was fehlt (sonst alle)
		}
		key := textKey{what: textHubStatus, state: hub.state, n: info.hubStep*100 + hub.steps, m: int(r)*100 + bitsArr(need)}
		text, ok := p.texts[key]
		if !ok {
			text = hubTitle(info.hubStep, hub.steps) + ": " + waitText(r, hub.state, resList(hubMaterial(need)))
			p.texts[key] = text
		}
		p.status(x, y, r, text)
		return
	}
	if !info.tagged(w.Castle.ID) || !offersHubUpgrade(hub.has, hub.state, hub.gold, hub.paid) {
		return
	}
	key := textKey{what: textHubPrice, n: info.hubStep*100 + hub.steps, m: info.hubLevel}
	text, ok := p.texts[key]
	if !ok {
		text = priceLabel(hubTitle(info.hubStep, hub.steps), hubMaterial(hub.material))
		p.texts[key] = text
	}
	p.price(x, y, hub.gold, hub.paid, text, coversArr(info.stock, hub.material))
}

// bitsArr kennzeichnet, welche Materialien eine Liste nennt (Schlüssel der Texte).
func bitsArr(c [5]int) int {
	n := 0
	for i, v := range c {
		if v > 0 {
			n |= 1 << i
		}
	}
	return n
}

// entities: feste Objekte des Levels (Portal, Ausgang mit Schild, Busch, Hafen-Steg, Rekrutierungslager).
func (p *planner) entities(w *sim.World, k view.Camera, biome string) {
	for _, e := range w.Level.Entities {
		l, dx, ok := entityLook(e.Kind, biome)
		x := e.X + dx
		if !ok || !visible(k, x, 4) {
			continue
		}
		if !p.sprite(l, x, 0, animPhase(x, 8)) {
			p.entityShape(e.Kind, x)
		}
		if e.Kind == "exit" {
			p.exitSign(w, l, x)
		}
	}
}

// exitSign: „Tiefe n / alle hierher“ über dem Ausgang, ohne tiefere Stufe „verschüttet“.
func (p *planner) exitSign(w *sim.World, l look, x float64) {
	_, h, ok := p.size(l)
	if !ok {
		h = px(180)
	}
	n := w.Biome.Depth + 1
	if !depths[n] {
		n = -1
	}
	y := h + px(24)
	if n < 0 {
		p.label(x, y, "verschüttet", white)
		return
	}
	line := p.texts.get(textKey{what: textExit, n: n}, func(k textKey) string { return "Tiefe " + strconv.Itoa(k.n) })
	p.label(x, y+1.1, line, white)
	p.label(x, y, "alle hierher", white)
}

// entityShape: Platzhalter-Formen ohne Grafik (worldRenderer.ts › drawStatic).
func (p *planner) entityShape(kind string, x float64) {
	switch kind {
	case "portal":
		p.rect(&p.body, x, 0, px(110), px(220), color.RGBA{123, 44, 191, 255})
	case "exit":
		p.rect(&p.body, x, 0, px(160), px(180), color.RGBA{17, 17, 17, 255})
	case "harbor":
		p.rect(&p.body, x, 0, 6, px(24), color.RGBA{139, 90, 43, 255})
	case "recruitCamp":
		p.body = append(p.body, item{shape: shapeTri, x: x, w: px(120), h: px(90), clr: color.RGBA{233, 196, 106, 255}})
	default:
		p.body = append(p.body, item{shape: shapeCircle, x: x, y: px(14), w: px(18), clr: color.RGBA{64, 145, 108, 255}})
	}
}

// pennantMax: höchste Lage von Balken über Knoten (Baumhöhe, pennant.ts › PENNANT_MAX_HEIGHT).
var pennantMax = px(360)

// nodes: Bäume, Felsen, Erz, Büsche und Adern mit Deko-Wurzeln, Abbau-Schild und Fortschritt; Plantage-Bäume später.
func (p *planner) nodes(w *sim.World, k view.Camera, biome string) {
	for _, n := range w.Nodes {
		if !visible(k, n.X, 3) || p.plantationTree(w, n) {
			continue
		}
		if n.Kind == "tree" {
			p.roots = decoRoots(p.roots[:0], n.X, biome, rootsByBiom[biome])
			for _, dx := range p.roots {
				p.sprite(object("wurzel"), n.X+dx, 0, 0)
			}
		}
		l := nodeLook(n.Kind, n.X, biome)
		_, h, ok := p.size(l)
		if ok {
			p.body = append(p.body, item{shape: shapeSprite, look: l, x: n.X, phase: animPhase(n.X, 8), sy: fellShrink(n.Progress)})
		} else {
			h = px(158)
			p.nodeShape(n.Kind, n.X)
		}
		if n.Marked && !p.sprite(mineSignLook(n.Kind), n.X-px(24), 0, animPhase(n.X, 4)) {
			p.rect(&p.body, n.X-px(24), px(14), px(40), px(40), color.RGBA{176, 137, 104, 255})
		}
		if n.Progress > 0 {
			p.bar(n.X, min(h, pennantMax)+px(28), px(50), n.Progress, gold)
		}
	}
}

// nodeShape: Platzhalter ohne Grafik (Baum aus Stamm und Krone, Fels, Erz, Busch).
func (p *planner) nodeShape(kind string, x float64) {
	switch kind {
	case "tree":
		p.rect(&p.body, x, 0, px(14), px(80), color.RGBA{107, 66, 38, 255})
		p.body = append(p.body, item{shape: shapeTri, x: x, y: px(65), w: px(80), h: px(90), clr: color.RGBA{45, 106, 79, 255}})
	case "rock":
		p.body = append(p.body, item{shape: shapeCircle, x: x, y: px(16), w: px(22), clr: color.RGBA{141, 153, 174, 255}})
	case "copperOre":
		p.body = append(p.body, item{shape: shapeCircle, x: x, y: px(16), w: px(20), clr: color.RGBA{184, 115, 51, 255}})
	default:
		p.body = append(p.body, item{shape: shapeCircle, x: x, y: px(14), w: px(18), clr: color.RGBA{64, 145, 108, 255}})
	}
}

// plantationTree meldet, ob der Knoten ein Baum auf einem Platz einer gebauten Baumschule ist (plantation.ts).
func (p *planner) plantationTree(w *sim.World, n *sim.ResourceNode) bool {
	if n.Kind != "tree" {
		return false
	}
	pl := buildings["nursery"].Plantation
	for _, s := range w.Sites {
		if s.Kind == "nursery" && s.State == "built" && math.Abs(n.X-s.X) <= pl.StripUnits+1 &&
			onPlot(n.X, s.X, pl.Slots, pl.StripUnits) {
			return true
		}
	}
	return false
}
