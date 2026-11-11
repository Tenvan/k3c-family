package props

// Zeichenliste der Bauplätze (siteView.ts, scaffoldView.ts, buildSignView.ts, damageView.ts) und der
// Hintergrundmauer.

import (
	"image/color"
	"math"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// siteSize: Platzhalter-Größe je Platzart in Spiel-Pixeln (siteView.ts › SITE_SIZE).
var siteSize = map[string][2]float64{
	"wall": {36, 150}, "tower": {70, 260}, "workshop": {150, 120}, "storage": {130, 100}, "stairsUp": {110, 110},
	"stairsDown": {110, 110}, "nursery": {140, 90}, "barracks": {150, 130}, "tavern": {130, 120}, "healer": {100, 100},
	"smithy": {120, 110}, "armory": {130, 120}, "ship": {190, 160},
}

// siteColor: Farbe der Platzhalter-Form (siteView.ts › SITE_COLOR).
var siteColor = map[string]color.RGBA{
	"wall": {141, 153, 174, 255}, "tower": {156, 102, 68, 255}, "workshop": {188, 108, 37, 255},
	"storage": {127, 85, 57, 255}, "stairsUp": {108, 117, 125, 255}, "stairsDown": {52, 58, 64, 255},
	"nursery": {167, 201, 87, 255}, "barracks": {108, 88, 76, 255}, "tavern": {176, 137, 104, 255},
	"healer": {229, 229, 229, 255}, "smithy": {73, 80, 87, 255}, "armory": {92, 103, 125, 255}, "ship": {74, 111, 165, 255},
}

// placeholder liefert Breite und Höhe der Platzhalter-Form (Units).
func placeholder(kind string) (w, h float64) {
	s, ok := siteSize[kind]
	if !ok {
		s = [2]float64{100, 100}
	}
	return px(s[0]), px(s[1])
}

// backWalls: gekachelte Hintergrundmauer hinter den Bauplätzen; ohne Grafik bleibt der Abschnitt leer.
func (p *planner) backWalls(w *sim.World, k view.Camera) {
	p.backs, p.walls = backWalls(p.backs[:0], p.walls, w.Sites, w.HubX)
	for _, b := range p.backs {
		l := backWallLook(b.stage)
		if _, _, ok := p.size(l); ok && visible(k, (b.from+b.to)/2, (b.to-b.from)/2+1) {
			p.body = append(p.body, item{shape: shapeTiles, look: l, x: b.from, x2: b.to})
		}
	}
}

// sites: alle sichtbaren Bauplätze in der Reihenfolge der Welt.
func (p *planner) sites(w *sim.World, info *stageInfo, k view.Camera) {
	for i, s := range w.Sites {
		fl := info.flag(i)
		if !fl.visible || !visible(k, s.X, 5) {
			continue
		}
		if s.State == "built" {
			p.builtSite(w, s, info, fl)
		} else {
			p.openSite(w, s, info, fl)
		}
	}
}

// builtSite: Gebäude der Stufe mit Schaden, Ruine, Treppen-Schild, Lebensbalken, Bögen und Bogen-Preis, Wartegrund.
func (p *planner) builtSite(w *sim.World, s *sim.Site, info *stageInfo, fl siteFlags) {
	hubLevel := max(1, w.HubLevel)
	l := siteLook(s.Kind, s.State, siteStage(s.Kind, s.Level, hubLevel))
	if _, _, ok := p.size(l); !ok {
		l = siteLook(s.Kind, s.State, 1) // Stufe nicht geladen: Grundform statt leerer Fläche
	}
	stage := damageStage(s.State, s.HP, s.MaxHP)
	full, h, ok := p.size(l)
	crack := full * 0.6 // Risse auf dem Bild nur in der Mitte, auf der Form über die ganze Breite
	if ok {
		p.body = append(p.body, item{shape: shapeSprite, look: l, x: s.X, shade: damageShade[stage]})
	} else {
		full, h = placeholder(s.Kind)
		crack = full
		p.builtShape(s.Kind, s.X, full, h)
		if stage > 0 {
			p.rect(&p.body, s.X, 0, full, h, color.RGBA{0, 0, 0, uint8(255 * damageShade[stage])})
		}
	}
	p.cracks(s.X, crack, h, stage)
	if showRubble(s.Ruin, s.State, s.HP, s.MaxHP) {
		p.rubble(s.X, full)
	}
	labeled := p.builtSigns(s, info, fl, h)
	if r, st := siteWait(s.State, s.Upgrade, s.BuildProgress, info != nil && info.danger); r != waitNone && !labeled {
		p.siteStatus(s, r, st, s.Level, s.X, h+px(20))
	}
}

// builtShape: Platzhalter eines gebauten Gebäudes ohne Grafik (Turm mit Dach, Treppe mit Stufen).
func (p *planner) builtShape(kind string, x, w, h float64) {
	p.rect(&p.body, x, 0, w, h, siteColor[kind])
	p.body = append(p.body, item{shape: shapeFrame, x: x, w: w, h: h, size: px(4), clr: dark})
	if kind == "tower" {
		p.rect(&p.body, x, h-px(4), w+px(30), px(14), color.RGBA{111, 69, 24, 255})
	}
	if kind == "stairsUp" || kind == "stairsDown" {
		for i := range 5 {
			p.rect(&p.body, x-w/2+(float64(i)+0.5)*w/5, 0, w/5, float64(i+1)*h/5, color.RGBA{73, 80, 87, 255})
		}
	}
}

// builtSigns: Treppen-Schild, Lebensbalken, Bögen im Regal und Bogen-Preis; true, wenn ein Text am Platz steht.
func (p *planner) builtSigns(s *sim.Site, info *stageInfo, fl siteFlags, h float64) bool {
	labeled := false
	if s.Kind == "stairsUp" || s.Kind == "stairsDown" {
		p.label(s.X, h+px(12), siteName(s.Kind), gold)
		labeled = true
	}
	if s.MaxHP > 0 && s.HP < s.MaxHP {
		p.bar(s.X, h+px(22), px(80), s.HP/s.MaxHP, hpGreen)
	}
	if s.Kind != "workshop" {
		return labeled
	}
	for i := range s.Bows {
		p.bow(s.X - px(40) + float64(i)*px(30))
	}
	if info.tagged(s.ID) && s.Bows < buildings["workshop"].BowRack {
		total := archerCost["gold"]
		if fl.payable {
			total = fl.goldLeft + s.BowPaidGold
		}
		text := p.texts.get(textKey{what: textBow}, siteText)
		p.price(s.X, h+px(20), total, s.BowPaidGold, text, canAfford(info.stock, archerCost))
		labeled = true
	}
	return labeled
}

// bow: Bogen im Regal der Werkstatt als Halbkreis (rechts offen) aus Linien, Mitte 60 Spiel-Pixel hoch.
func (p *planner) bow(x float64) {
	const n = 6
	r, y := px(14), px(60)
	for i := range n {
		a0 := -math.Pi/2 + math.Pi*float64(i)/n
		a1 := -math.Pi/2 + math.Pi*float64(i+1)/n
		p.top = append(p.top, item{shape: shapeLine, x: x + r*math.Cos(a0), y: y + r*math.Sin(a0),
			x2: x + r*math.Cos(a1), y2: y + r*math.Sin(a1), size: px(4), clr: gold})
	}
}

// openSite: Bauschild oder Gerüst, Fortschritt, Preis bzw. „erst roden“ am nächsten Platz, Wartegrund.
func (p *planner) openSite(w *sim.World, s *sim.Site, info *stageInfo, fl siteFlags) {
	bw, h := placeholder(s.Kind)
	top := h
	switch siteMarker(s.State, fl.payable) {
	case markSign:
		p.buildSign(s.X, signVariant(s.Kind))
	case markScaffold:
		top = p.scaffold(s.Kind, s.X, bw, h)
	}
	if s.BuildProgress > 0 {
		p.bar(s.X, top+px(4), px(80), s.BuildProgress, color.RGBA{221, 161, 94, 255})
	}
	if s.State == "unpaid" && info.tagged(s.ID) {
		p.sitePrice(w, s, info, fl, h)
	}
	if r, st := siteWait(s.State, s.Upgrade, s.BuildProgress, info != nil && info.danger); r != waitNone {
		p.siteStatus(s, r, st, w.Biome.Depth, s.X, top+px(20))
	}
}

// scaffold: Gerüst der Größenklasse, ohne Grafik Stangen und Laufbretter; liefert die Höhe für Balken darüber.
func (p *planner) scaffold(kind string, x, w, h float64) float64 {
	l := scaffoldLook(kind)
	if _, sh, ok := p.size(l); ok {
		p.sprite(l, x, 0, 0)
		return sh
	}
	pole := color.RGBA{102, 57, 49, 255}
	for _, dx := range [3]float64{-w/2 + px(2), 0, w/2 - px(2)} {
		p.rect(&p.body, x+dx, 0, px(4), h, pole)
	}
	for y := px(8); y < h; y += 1 {
		p.rect(&p.body, x, h-y-px(4), w, px(4), color.RGBA{143, 86, 59, 255})
	}
	return h
}

// signBoard: Farben des Bauschilds je Variante (Holz, Zierleiste).
var signBoard = [3][2]color.RGBA{
	{{176, 137, 104, 255}, {244, 213, 141, 255}},
	{{156, 102, 68, 255}, {193, 18, 31, 255}},
	{{141, 153, 174, 255}, {52, 58, 64, 255}},
}

// buildSign: Brett am Stock ohne Text (B-584), Fuß bei x.
func (p *planner) buildSign(x float64, variant int) {
	c := signBoard[variant]
	stick, bw, bh := px(17), px(34), px(22)
	p.rect(&p.body, x, 0, px(8), stick, color.RGBA{111, 69, 24, 255})
	p.rect(&p.body, x, stick, bw, bh, c[0])
	p.body = append(p.body, item{shape: shapeFrame, x: x, y: stick, w: bw, h: bh, size: px(2), clr: color.RGBA{61, 43, 31, 255}})
	p.rect(&p.body, x, stick+bh-px(7), bw-px(6), px(4), c[1])
}

// sitePrice: Preis am unbezahlten Platz; stehen Bäume oder Felsen im Freiraum, „erst roden“ (B-636, B-652).
func (p *planner) sitePrice(w *sim.World, s *sim.Site, info *stageInfo, fl siteFlags, h float64) {
	if fl.clearing {
		p.label(s.X, h+px(20), p.texts.get(textKey{what: textClearing, kind: s.Kind}, siteText), gold)
		return
	}
	c := buildCost(s.Kind, w.Biome.Depth)
	total := c["gold"]
	if fl.payable {
		total = fl.goldLeft + s.PaidGold
	}
	text := p.texts.get(textKey{what: textPrice, kind: s.Kind, n: w.Biome.Depth}, siteText)
	p.price(s.X, h+px(20), total, s.PaidGold, text, canAfford(info.stock, c))
}

// price: Münz-Plätze in Reihen zu zehn (gefüllt = bezahlt), darüber Name und Material; rot, wenn Material fehlt.
func (p *planner) price(x, y float64, total, paid int, text string, enough bool) {
	const perRow = 10
	step, r := px(22), px(8)
	for i := range total {
		row := i / perRow
		inRow := min(perRow, total-row*perRow)
		cx := x + float64(i%perRow)*step - float64(inRow-1)*step/2
		cy := y + float64(row)*step
		if i < paid {
			p.top = append(p.top, item{shape: shapeCircle, x: cx, y: cy, w: r, clr: gold})
		}
		p.top = append(p.top, item{shape: shapeRing, x: cx, y: cy, w: r, size: px(2), clr: gold})
	}
	clr := white
	if !enough {
		clr = poorRed
	}
	rows := (total + perRow - 1) / perRow
	p.label(x, y+float64(rows)*step, text, clr)
}

// waitColor: Farbe von Symbol und Text je Wartegrund (siteStatusView.ts › COLOR).
var waitColor = [4][2]color.RGBA{
	{},
	{{76, 201, 240, 255}, white},
	{{255, 143, 163, 255}, {255, 143, 163, 255}},
	{{230, 57, 70, 255}, {255, 107, 107, 255}},
}

// siteStatus: Wartegrund eines Platzes (Text „Name: Grund“, Symbol darüber); n = Tiefe bzw. Stufe für das Material.
func (p *planner) siteStatus(s *sim.Site, r waitReason, state string, n int, x, y float64) {
	if s.State == "built" {
		n = max(1, s.Level)
	}
	text := p.texts.get(textKey{what: textStatus, kind: s.Kind, state: state, n: n, m: int(r)}, siteText)
	p.status(x, y, r, text)
}

// status: Text mit Unterkante y, darüber das Symbol (Kreis Bauer, Quadrat Material, Dreieck Gefahr).
func (p *planner) status(x, y float64, r waitReason, text string) {
	c := waitColor[r]
	p.label(x, y, text, c[1])
	sy := y + 1.25 + px(18)
	switch r {
	case waitWorker:
		p.top = append(p.top, item{shape: shapeCircle, x: x, y: sy, w: px(12), clr: c[0]},
			item{shape: shapeRing, x: x, y: sy, w: px(12), size: px(3), clr: color.RGBA{0, 0, 0, 255}})
	case waitMaterial:
		p.rect(&p.top, x, sy-px(12), px(24), px(24), c[0])
		p.top = append(p.top, item{shape: shapeFrame, x: x, y: sy - px(12), w: px(24), h: px(24), size: px(3), clr: color.RGBA{0, 0, 0, 255}})
	default:
		p.top = append(p.top, item{shape: shapeTri, x: x, y: sy - px(12), w: px(28), h: px(26), clr: c[0]})
	}
}

// crackLines: Risse als Linienzüge in Anteilen der Breite (−0,5 … 0,5) und Höhe (0 … 1); je Stufe mehr.
var crackLines = [5][][2]float64{
	{{-0.2, 0.95}, {-0.12, 0.75}, {-0.22, 0.6}, {-0.15, 0.45}},
	{{0.25, 0.85}, {0.18, 0.65}, {0.28, 0.5}},
	{{-0.38, 0.4}, {-0.3, 0.28}, {-0.36, 0.15}},
	{{0.05, 0.55}, {0.12, 0.4}, {0.04, 0.25}, {0.1, 0.1}},
	{{0.36, 0.35}, {0.3, 0.2}},
}

// cracksPerStage: Zahl der Risse je Schadensstufe.
var cracksPerStage = [4]int{0, 2, 4, 5}

// cracks: Risse über einem Gebäude der Breite w und Höhe h (damageView.ts › drawDamage).
func (p *planner) cracks(x, w, h float64, stage int) {
	width := px(3)
	if stage == 3 {
		width = px(4)
	}
	clr := color.RGBA{23, 23, 23, 217}
	for _, line := range crackLines[:cracksPerStage[stage]] {
		for i := 1; i < len(line); i++ {
			a, b := line[i-1], line[i]
			p.top = append(p.top, item{shape: shapeLine, x: x + a[0]*w, y: a[1] * h, x2: x + b[0]*w, y2: b[1] * h, size: width, clr: clr})
		}
	}
}

// rubbleStones: Steine des Trümmerhaufens (Mitte als Anteil der Breite, Breite und Höhe in Spiel-Pixeln).
var rubbleStones = [6][3]float64{{-0.42, 22, 12}, {-0.3, 18, 16}, {-0.18, 26, 10}, {0.32, 20, 14}, {0.44, 16, 10}, {0.2, 24, 9}}

// rubble: Trümmer am Fuß einer Ruine (damageView.ts › drawRubble).
func (p *planner) rubble(x, w float64) {
	for _, s := range rubbleStones {
		p.rect(&p.top, x+s[0]*w, 0, px(s[1]), px(s[2]), stoneGray)
		p.top = append(p.top, item{shape: shapeFrame, x: x + s[0]*w, w: px(s[1]), h: px(s[2]), size: px(2), clr: dark})
	}
}
