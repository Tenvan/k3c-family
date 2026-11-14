package hud

// Hinweise über der Welt (Spiegel von src/scenes/hints/actionHints.ts, B-319, W11.4, und guideHints.ts, S6.3, B-148):
// je Spieler höchstens ein Aktions-Hinweis über seinem Monarchen (Ausbilden, Tauschen, Bau des Endbosses; Tasten-
// Aktionen stehen im Pause-Menü › Tasten, B-741, B-807) und der nächste ungesehene Hinweis der geführten ersten Nacht.

import (
	"math"

	"k3c/engine/sim"
)

// Höhen über dem Boden in Units (Browser: Welt-Pixel / 32): Aktions-Hinweis über dem Monarchen, Hinweis über einem
// Objekt bzw. über dem Aktions-Hinweis.
const (
	hintAboveUnits      = 280.0 / 32
	guideObjectUnits    = 160.0 / 32
	guidePlayerUnits    = 370.0 / 32
	priceTagRangeUnits  = 6.0 // viewRules.ts › PRICE_TAG_RANGE
	hintTopFreeRefPixel = 110.0
)

// hintKind ist die Art des Aktions-Hinweises.
type hintKind int

const (
	hintNone hintKind = iota
	hintTrain
	hintBuy
	hintSell
	hintLair
)

// hintKey ist der gewählte Aktions-Hinweis mit seinem Namen (Beruf bzw. Rohstoff).
type hintKey struct {
	lang string
	kind hintKind
	name string
}

// chooseHint wählt den Hinweis (playerHint): gefallen keiner; Ausbilden und Tauschen (Gewicht wie Bezahlen) auch am
// Preisschild; der Bau des Endbosses (Gewicht 1) nur abseits eines Preisschilds.
func chooseHint(down bool, train, side, resource string, atTag, atLair bool) (hintKind, string) {
	switch {
	case down:
		return hintNone, ""
	case train != "":
		return hintTrain, train
	case side == "sell":
		return hintSell, resource
	case side != "":
		return hintBuy, resource
	case atLair && !atTag:
		return hintLair, ""
	}
	return hintNone, ""
}

// actionHint liest die Aktionen des Monarchen p an seinem Ort (sim.TrainAt, sim.TradeAt, sim.AtLair wie die
// Aktionsliste des Servers, engine/net/actions.go).
func actionHint(lang string, w *sim.World, p *sim.Player) hintKey {
	if p.RespawnIn > 0 {
		return hintKey{lang: lang}
	}
	side, res := sim.TradeAt(w, p)
	kind, name := chooseHint(false, sim.TrainAt(w, p), side, res, atPriceTag(w, p.X), sim.AtLair(w, p))
	return hintKey{lang, kind, name}
}

// hintText ist der Text des Aktions-Hinweises ohne Tastennamen ("" = keiner).
func hintText(k hintKey) string {
	switch k.kind {
	case hintTrain:
		return tf(k.lang, "hint.train", "name", professionText(k.lang, k.name, false))
	case hintBuy:
		return tf(k.lang, "hint.buy", "name", resName(k.lang, k.name))
	case hintSell:
		return tf(k.lang, "hint.sell", "name", resName(k.lang, k.name))
	case hintLair:
		return tr(k.lang, "hint.lair")
	}
	return ""
}

// atPriceTag: Steht bei x ein Preisschild in Reichweite (unbezahlter, bezahlbarer Bauplatz oder Werkstatt mit freiem
// Bogen-Regal, priceTagRules.ts › offersPrice)?
func atPriceTag(w *sim.World, x float64) bool {
	for _, s := range w.Sites {
		if s != nil && math.Abs(s.X-x) < priceTagRangeUnits && offersPrice(w, s) {
			return true
		}
	}
	return false
}

func offersPrice(w *sim.World, s *sim.Site) bool {
	if s.State == "unpaid" {
		return sim.SitePayable(w, s)
	}
	return s.State == "built" && s.Kind == "workshop" && s.Bows < buildingInfo["workshop"].BowRack
}

// guideID ist ein Hinweis der geführten ersten Nacht; am Bauplatz steht keiner mehr (🧑, 2026-11-05).
type guideID int

const (
	guideNone guideID = iota
	guideCoin
	guideDusk
	guideRecruit
)

// guideOrder ist die Reihenfolge, wenn mehrere passen.
var guideOrder = [...]guideID{guideCoin, guideDusk, guideRecruit}

// guideKeys sind die Text-Schlüssel der Hinweise.
var guideKeys = [...]string{guideCoin: "guide.coin", guideDusk: "guide.dusk", guideRecruit: "guide.recruit"}

// guideIcon ist das Bild vor dem Hinweistext (guideHints.ts › guideImage, B-811): Münze, Mond oder keins. hud zeichnet
// beide aus eigenen Formen (Scheibe, Sichel), ohne Asset.
type guideIcon int

const (
	iconNone guideIcon = iota
	iconCoin
	iconMoon
)

// guideImage ist das Bild eines Hinweises; ready false (keine Vorlagen zum Zeichnen) = keins, der Text steht allein.
func guideImage(id guideID, ready bool) guideIcon {
	switch {
	case !ready:
		return iconNone
	case id == guideCoin:
		return iconCoin
	case id == guideDusk:
		return iconMoon
	}
	return iconNone
}

// guideHint ist ein gezeigter Hinweis: Ort in Units und das Objekt, an dem die Handlung erkannt wird (-1 = keins).
type guideHint struct {
	id     guideID
	x      float64
	target int
}

// guideSeen merkt die gesehenen Hinweise (Bit je guideID); gilt für alle Spieler dieses Geräts.
type guideSeen uint8

func (s guideSeen) has(id guideID) bool { return s&(1<<id) != 0 }

// nextGuide ist der nächste ungesehene Hinweis für den Monarchen bei x; gefallen keiner.
func nextGuide(w *sim.World, x float64, down bool, seen guideSeen) guideHint {
	if down {
		return guideHint{}
	}
	for _, id := range guideOrder {
		if seen.has(id) {
			continue
		}
		if h := guideCandidate(id, w, x); h.id != guideNone {
			return h
		}
	}
	return guideHint{}
}

func guideCandidate(id guideID, w *sim.World, x float64) guideHint {
	switch id {
	case guideCoin:
		return nearestCoin(w, x)
	case guideDusk:
		if w.Cycle.Phase == "dusk" {
			return guideHint{guideDusk, x, -1}
		}
	case guideRecruit:
		return nearestVagrant(w, x)
	}
	return guideHint{}
}

// nearestCoin ist die nächste Münze in Reichweite des Preisschilds.
func nearestCoin(w *sim.World, x float64) guideHint {
	best := guideHint{target: -1}
	for _, c := range w.Coins {
		if c != nil && math.Abs(c.X-x) < priceTagRangeUnits && (best.id == guideNone || math.Abs(c.X-x) < math.Abs(best.x-x)) {
			best = guideHint{guideCoin, c.X, c.ID}
		}
	}
	return best
}

// nearestVagrant ist der nächste Landstreicher in Reichweite des Preisschilds.
func nearestVagrant(w *sim.World, x float64) guideHint {
	best := guideHint{target: -1}
	for _, t := range w.Troops {
		if t != nil && t.Kind == "vagrant" && math.Abs(t.X-x) < priceTagRangeUnits && (best.id == guideNone || math.Abs(t.X-x) < math.Abs(best.x-x)) {
			best = guideHint{guideRecruit, t.X, t.ID}
		}
	}
	return best
}

// guideDone: Handlung erledigt (Münze aufgehoben, Nacht da, Landstreicher ist Bauer).
func guideDone(h guideHint, w *sim.World) bool {
	switch h.id {
	case guideCoin:
		for _, c := range w.Coins {
			if c != nil && c.ID == h.target {
				return false
			}
		}
		return true
	case guideDusk:
		return w.Cycle.Phase == "night"
	case guideRecruit:
		for _, t := range w.Troops {
			if t != nil && t.ID == h.target && t.Kind != "vagrant" {
				return true
			}
		}
	}
	return false
}

// stepGuide: Ist der zuletzt gezeigte Hinweis erledigt, gilt er als gesehen; dann kommt der nächste.
func stepGuide(prev guideHint, w *sim.World, x float64, down bool, seen *guideSeen) guideHint {
	if prev.id != guideNone && !seen.has(prev.id) && guideDone(prev, w) {
		*seen |= 1 << prev.id
	}
	return nextGuide(w, x, down, *seen)
}

// guideHeight ist die Höhe des Hinweises über dem Boden in Units.
func guideHeight(h guideHint) float64 {
	if h.target < 0 {
		return guidePlayerUnits
	}
	return guideObjectUnits
}
