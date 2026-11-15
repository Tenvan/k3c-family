package hud

// Zeichnen der HUD-Elemente einer Zelle: Größen messen, Lage über layout (gemerkt, nur bei Änderung neu gerechnet),
// dann Fläche und Inhalt je Element.

import (
	"image"
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// Stil der Elemente laut 🧑 (2026-10-10): Schwarz mit 55 % Deckkraft, Innenabstand 8/4 px, Breite in Stufen von
// 32 px, damit wechselnde Texte (Uhr, Abklingzeit) das Layout nicht jedes Bild neu anstoßen.
const (
	boxAlpha  = 0.55
	boxPadX   = 8.0
	boxPadY   = 4.0
	widthStep = 32.0
)

// Maße des Boss-Balkens in px bei 1080 (HudScene.ts).
const (
	bossBarW = 640.0
	bossBarH = 22.0
)

// textItem ist ein Element aus einem Text des Spielers.
type textItem struct {
	id   itemID
	slot textSlot
	role fontRole
	get  func(*seatState) string
}

var textItems = [...]textItem{
	{itemBanner, slotBanner, roleBanner, func(s *seatState) string { return s.banner.showing }},
	{itemSkills, slotSkillBar, rolePlayer, func(s *seatState) string { return s.barS }},
	{itemClock, slotClock, roleShared, func(s *seatState) string { return s.clockS }},
	{itemFight, slotFight, roleFight, func(s *seatState) string { return s.fightS }},
	{itemPlace, slotPlace, roleRoomInfo, func(s *seatState) string { return s.placeS }},
	{itemCitizens, slotCit, roleShared, func(s *seatState) string { return s.citS }},
	{itemEvents, slotEvents, roleBanner, func(s *seatState) string { return s.evS }},
	{itemTravel, slotTravel, roleTravel, func(s *seatState) string { return s.travelS }},
}

// layoutCache merkt das Layout einer Zelle zu seinen Eingaben.
type layoutCache struct {
	in   items
	area rect
	free rect
	k    float64
	out  placed
	set  bool
}

func rectOf(r image.Rectangle) rect {
	return rect{float64(r.Min.X), float64(r.Min.Y), float64(r.Dx()), float64(r.Dy())}
}

// boxSize ist die Größe der Fläche um einen Text der Größe (w, h) beim Maßstab k.
func boxSize(w, h, k float64) (float64, float64) {
	if w <= 0 {
		return 0, 0
	}
	step := widthStep * k
	return math.Ceil((w+2*boxPadX*k)/step) * step, h + 2*boxPadY*k
}

// drawItems misst, legt ab und zeichnet die Elemente der Zelle c.
func (h *hud) drawItems(dst *ebiten.Image, st *seatState, c view.Cell, k float64) {
	f, p := h.font(), c.Player
	var in items
	var px [len(textItems)]float64
	for i, t := range textItems {
		px[i] = fontPx(t.role, h.cells) * k
		w, ht := h.paint.measureText(f, p, t.slot, t.get(st), px[i])
		if t.id == itemBanner {
			px[i] = fitPx(px[i], w+2*boxPadX*k, float64(c.Rect.Dx())-2*gap*k)
			w, ht = h.paint.measureText(f, p, t.slot, t.get(st), px[i])
		}
		in[t.id].W, in[t.id].H = boxSize(w, ht, k)
	}
	if c.Index == h.cells-1 {
		w, ht := h.paint.measureText(f, p, slotJoin, h.joinS, fontPx(roleJoin, h.cells)*k)
		in[itemJoin].W, in[itemJoin].H = boxSize(w, ht, k)
	}
	in[itemPlayer].W, in[itemPlayer].H = h.cardSize(st, f, p, k)
	sharedPx := fontPx(roleShared, h.cells) * k
	in[itemShared].W, in[itemShared].H = h.stockSize(st, f, p, sharedPx, k)
	in[itemBoss].W, in[itemBoss].H = h.bossSize(st, f, p, k, float64(c.Rect.Dx()))
	h.storySize(st, f, p, k, float64(c.Rect.Dx()), &in)
	if st.widthUnits > 0 {
		in[itemRadar].W, in[itemRadar].H = radarSize(float64(c.Rect.Dx()), k)
	}
	pl := h.layoutFor(c, &in, k)
	for i, t := range textItems {
		if pl.ok[t.id] {
			h.textBox(dst, f, pl.at[t.id], t.get(st), px[i], t.role.clr, k)
		}
	}
	if pl.ok[itemJoin] {
		h.textBox(dst, f, pl.at[itemJoin], h.joinS, fontPx(roleJoin, h.cells)*k, roleJoin.clr, k)
	}
	if pl.ok[itemPlayer] {
		h.drawCard(dst, st, f, p, pl.at[itemPlayer], k)
	}
	if pl.ok[itemShared] {
		h.drawStock(dst, st, f, p, pl.at[itemShared], sharedPx, k)
	}
	if pl.ok[itemBoss] {
		h.drawBoss(dst, st, f, p, pl.at[itemBoss], k)
	}
	if pl.ok[itemStory] {
		h.drawStory(dst, st, f, p, pl.at[itemStory], k, float64(c.Rect.Dx()))
	}
	if pl.ok[itemRadar] {
		h.drawRadar(dst, st, c, pl.at[itemRadar], k)
	}
}

// layoutFor liefert das Layout der Zelle aus dem Cache oder rechnet es neu. Die oberste Zelle lässt die große Anzeige
// oben rechts frei (view.StatsRect; Zelle 0 hat Oberkante und Breite des Fensters).
func (h *hud) layoutFor(c view.Cell, in *items, k float64) placed {
	area := rectOf(c.Rect)
	var free rect
	if c.Index == 0 {
		free = rectOf(view.StatsRect(c.Rect))
	}
	lc := &h.lay[c.Player]
	if !lc.set || lc.in != *in || lc.area != area || lc.free != free || lc.k != k {
		frees := []rect{free}
		if free.W == 0 {
			frees = nil
		}
		lc.in, lc.area, lc.free, lc.k, lc.set = *in, area, free, k, true
		lc.out = layout(in, area, frees, k)
	}
	return lc.out
}

// textBox zeichnet einen Text der Größe size auf seiner Fläche, bündig links.
func (h *hud) textBox(dst *ebiten.Image, f *art.Font, r rect, s string, size float64, clr color.RGBA, k float64) {
	h.paint.fill(dst, r, rgb(0, boxAlpha))
	h.paint.text(dst, f, s, r.X+boxPadX*k, r.Y+boxPadY*k, size, clr)
}

// fitPx verkleinert die Schriftgröße px im Verhältnis, wenn ein Text der Breite w nicht in maxW passt (Meldung über
// die Breite der Zelle, B-813); sonst bleibt sie.
func fitPx(px, w, maxW float64) float64 {
	if w <= maxW || w <= 0 || maxW <= 0 {
		return px
	}
	// Stufen von 2 px wie menuSize: jede Größe ist ein eigenes Face samt Glyph-Bildern (art.Font.faces).
	return max(2, math.Floor(px*maxW/w/2)*2)
}

// spaceW ist die Breite eines Leerzeichens in der Größe size (Abstand der Teile der Spielerkarte).
func spaceW(size float64) float64 { return size * 0.3 }

// cardSize: Geldsack (Kante doppelt so groß wie ein Ressourcen-Icon) und Gold nebeneinander, ohne Lebensbalken und
// Spielermarke (B-919).
func (h *hud) cardSize(st *seatState, f *art.Font, p int, k float64) (float64, float64) {
	size := rolePlayer.px * k
	side := purseSide(fontPx(roleShared, h.cells) * k)
	gw, th := h.paint.measureText(f, p, slotGold, st.goldS, size)
	w := side + gw + 2*spaceW(size) + 2*boxPadX*k
	return math.Ceil(w/(widthStep*k)) * (widthStep * k), max(th, side) + 2*boxPadY*k
}

// drawCard zeichnet die Spielerkarte: Geldsack in der Größe des Spielers (hervorgehoben nach einer Änderung des
// Goldes, B-398) und „Gold/Maximum“.
func (h *hud) drawCard(dst *ebiten.Image, st *seatState, f *art.Font, player int, r rect, k float64) {
	pt, size := &h.paint, rolePlayer.px*k
	pt.fill(dst, r, rgb(0, boxAlpha))
	side := purseSide(fontPx(roleShared, h.cells) * k)
	x, y := r.X+boxPadX*k, r.Y+boxPadY*k
	inner := r.H - 2*boxPadY*k
	h.drawPurse(dst, rect{x, y + (inner-side)/2, side, side}, st.purse, st.flash, k)
	x += side + spaceW(size)
	_, th := pt.measureText(f, player, slotGold, st.goldS, size)
	pt.text(dst, f, st.goldS, x, y+(inner-th)/2, size, white)
}

// bossSize: Titel und Leiste untereinander; ohne Boss 0.
func (h *hud) bossSize(st *seatState, f *art.Font, p int, k, cellW float64) (float64, float64) {
	if !st.boss.ok {
		return 0, 0
	}
	tw, th := h.paint.measureText(f, p, slotBoss, st.bossS, roleBoss.px*k)
	w := min(max(tw, bossBarW*k)+2*boxPadX*k, cellW-48*k)
	return w, th + 4*k + bossBarH*k + 2*boxPadY*k
}

// drawBoss zeichnet Name und Phase, darunter die Lebensleiste (kampfView.ts › drawBar).
func (h *hud) drawBoss(dst *ebiten.Image, st *seatState, f *art.Font, player int, r rect, k float64) {
	pt := &h.paint
	pt.fill(dst, r, rgb(0, boxAlpha))
	tw, th := pt.measureText(f, player, slotBoss, st.bossS, roleBoss.px*k)
	pt.text(dst, f, st.bossS, r.X+(r.W-tw)/2, r.Y+boxPadY*k, roleBoss.px*k, roleBoss.clr)
	bar := rect{r.X + boxPadX*k, r.Y + boxPadY*k + th + 4*k, r.W - 2*boxPadX*k, bossBarH * k}
	pt.fill(dst, bar, rgb(0, 0.7))
	pt.fill(dst, rect{bar.X + 3*k, bar.Y + 3*k, (bar.W - 6*k) * st.boss.share, bar.H - 6*k}, rgb(0xe63946, 1))
	pt.frame(dst, bar, 2*k, white)
}
