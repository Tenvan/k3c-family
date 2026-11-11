package hud

// Lage der HUD-Elemente je Zelle (Spiegel von src/scenes/hud/hudLayout.ts, B-337): jedes Element hat einen Anker in
// der Zelle und einen Rang; layout legt sie ohne Überschneidung untereinander und mit den Freiflächen ab. Was nicht
// passt, fällt nach Rang weg, Rang 0 (Pflicht) nie. Reine Rechnung ohne Ebitengine und ohne Allokation.
//
// Abweichung vom Browser: Es gibt keine Leiste über die ganze Breite; die Zeilen der Leiste (Uhr, Kampf, Vorrat,
// Spielerkarte, Meldung) stehen oben in jeder Zelle, jede Zelle zeigt die Werte ihrer eigenen Stufe.

import "math"

// rect ist ein Rechteck in Fenster-Pixeln.
type rect struct{ X, Y, W, H float64 }

func (a rect) overlaps(b rect) bool {
	return a.X < b.X+b.W && b.X < a.X+a.W && a.Y < b.Y+b.H && b.Y < a.Y+a.H
}

func (a rect) inside(b rect) bool {
	return a.X >= b.X && a.Y >= b.Y && a.X+a.W <= b.X+b.W && a.Y+a.H <= b.Y+b.H
}

// align ist die Bündigkeit eines Ankers (waagrecht: Anfang = links, senkrecht: Anfang = oben).
type align int

const (
	alignStart align = iota
	alignCenter
	alignEnd
)

// itemID benennt ein HUD-Element einer Zelle; die Reihenfolge gilt bei gleichem Rang.
type itemID int

const (
	itemPlayer itemID = iota
	itemBanner
	itemSkills
	itemClock
	itemFight
	itemShared
	itemPlace
	itemCitizens
	itemBoss
	itemEvents
	itemTravel
	itemJoin
	itemRadar
	itemCount
)

// Ränge laut 🧑 (2026-10-10): Pflicht Spielerkarte, Meldung, Skill-Leiste; dann die Zeilen der Leiste, Reise,
// Beitritt, Radar.
const (
	rankMust   = 0
	rankShared = 1
	rankTravel = 2
	rankJoin   = 3
	rankRadar  = 4
	rankMax    = rankRadar
)

// gap ist der Abstand zwischen Elementen und zu Freiflächen (px bei 1080, mit dem Maßstab skaliert).
const gap = 8.0

// anchor ist Anker und Rang eines Elements (Abstände in px bei 1080).
type anchor struct {
	h, v   align
	dx, dy float64
	rank   int
}

// rowH ist der Zeilenabstand der oberen Zeilen (Browser: 42 px je Zeile der Leiste).
const rowH = 42.0

// anchors der Elemente: drei Zeilen oben wie die Leiste des Browsers (Zeile 0 Spielerkarte, Uhr, Kampf; Zeile 1
// Vorrat, Meldung, Ort; Zeile 2 Hub und Bürger), darunter Boss, Events und Radar, unten Skill-Leiste, Reise, Beitritt.
var anchors = [itemCount]anchor{
	itemPlayer:   {alignStart, alignStart, 24, 4, rankMust},
	itemBanner:   {alignCenter, alignStart, 0, 4 + rowH, rankMust},
	itemSkills:   {alignStart, alignEnd, 24, 24, rankMust},
	itemClock:    {alignCenter, alignStart, 0, 4, rankShared},
	itemFight:    {alignEnd, alignStart, 24, 4, rankShared},
	itemShared:   {alignStart, alignStart, 24, 4 + rowH, rankShared},
	itemPlace:    {alignEnd, alignStart, 24, 4 + rowH, rankShared},
	itemCitizens: {alignEnd, alignStart, 24, 4 + 2*rowH, rankShared},
	itemBoss:     {alignCenter, alignStart, 0, 4 + 3*rowH, rankShared},
	itemEvents:   {alignCenter, alignStart, 0, 4 + 4*rowH, rankShared},
	itemTravel:   {alignCenter, alignEnd, 0, 150, rankTravel},
	itemJoin:     {alignCenter, alignEnd, 0, 85, rankJoin},
	itemRadar:    {alignCenter, alignStart, 0, 4 + 3*rowH, rankRadar},
}

// items sind die Größen der Elemente einer Zelle in px; W == 0 heißt: nicht gezeigt.
type items [itemCount]struct{ W, H float64 }

// placed ist das Ergebnis: Lage je Element, ok false = ausgeblendet oder nicht gezeigt.
type placed struct {
	at [itemCount]rect
	ok [itemCount]bool
}

// maxFree ist die Höchstzahl der Freiflächen.
const maxFree = 4

// anchorRect ist die Wunschlage des Elements id mit Größe (w, h) in der Fläche area beim Maßstab k.
func anchorRect(id itemID, area rect, w, h, k float64) rect {
	a := anchors[id]
	return rect{X: pos(a.h, area.X, area.W, w, a.dx*k), Y: pos(a.v, area.Y, area.H, h, a.dy*k), W: w, H: h}
}

func pos(al align, start, size, w, d float64) float64 {
	switch al {
	case alignEnd:
		return start + size - w - d
	case alignCenter:
		return start + (size-w)/2 + d
	}
	return start + d
}

// layout legt die Elemente in area ab: höherer Rang (kleinere Zahl) zuerst, bei Gleichstand nach itemID; free sind
// Flächen, die frei bleiben (höchstens maxFree). Ein Pflicht-Element, das nirgends passt, steht an seinem Anker.
func layout(in *items, area rect, free []rect, k float64) placed {
	var out placed
	var taken [itemCount + maxFree]rect
	n := copy(taken[:], free[:min(len(free), maxFree)])
	for rank := 0; rank <= rankMax; rank++ {
		for id := range itemCount {
			if anchors[id].rank != rank || in[id].W <= 0 {
				continue
			}
			r, ok := place(id, area, in[id].W, in[id].H, k, taken[:n])
			if !ok && rank == rankMust {
				r, ok = anchorRect(id, area, in[id].W, in[id].H, k), true
			}
			if ok {
				out.at[id], out.ok[id] = r, true
				taken[n] = r
				n++
			}
		}
	}
	return out
}

// place sucht die nächste freie Lage zur Wunschlage: Kandidaten sind die Wunschlage und die Lagen direkt neben jedem
// Hindernis (rechts, links, darunter, darüber); gewählt wird die mit dem kürzesten Weg.
func place(id itemID, area rect, w, h, k float64, obstacles []rect) (rect, bool) {
	base := anchorRect(id, area, w, h, k)
	g := gap * k
	best, bestD, found := rect{}, math.Inf(1), false
	for i := -1; i < 2*len(obstacles); i++ {
		x := candidate(i, base.X, w, g, obstacles, true)
		for j := -1; j < 2*len(obstacles); j++ {
			y := candidate(j, base.Y, h, g, obstacles, false)
			r := rect{x, y, w, h}
			d := math.Abs(x-base.X) + math.Abs(y-base.Y)
			if d >= bestD || !r.inside(area) || hits(r, obstacles) {
				continue
			}
			best, bestD, found = r, d, true
		}
	}
	return best, found
}

// candidate ist der Kandidat i einer Achse: -1 = Wunschlage, 2o = hinter Hindernis o, 2o+1 = davor.
func candidate(i int, base, size, g float64, obstacles []rect, horizontal bool) float64 {
	if i < 0 {
		return base
	}
	o := obstacles[i/2]
	start, extent := o.Y, o.H
	if horizontal {
		start, extent = o.X, o.W
	}
	if i%2 == 0 {
		return start + extent + g
	}
	return start - g - size
}

func hits(r rect, obstacles []rect) bool {
	for _, o := range obstacles {
		if r.overlaps(o) {
			return true
		}
	}
	return false
}
