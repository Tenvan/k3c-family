package props

// Zeichenliste der Kampfregeln (KS5, B-934, docs/rules/kampf.md): HP-Balken über jedem stehenden Portal, gefallene Kronen
// (fliegend im Bogen, liegend am Boden) und die Zahlstellen der Kampfvariante (Schild, Auslöser, Banner) mit Preis. Kronen auf
// Köpfen (Monarch, Gegner, Mitspieler) zeichnet actors. Bis es Papierschnitt-Assets gibt (assets_*, 005), entstehen alle
// Formen aus Rechtecken und Dreiecken (vorläufig); die Zahlstellen tragen keinen Text, nur Preis-Münzen.

import (
	"image/color"
	"math"
	"strings"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// combatPrices sind die Preise der Zahlstellen und die Zahl-Reichweite aus data/economy.json (Spiegel von sim.CombatSpot-Preisen).
var combatPrices = readJSON[struct {
	PayRangeUnits float64
	Combat        struct{ ShieldCost, TriggerCost, BannerCost int }
}]("economy.json")

// spotsEvery: so selten (Sekunden Spielzeit) liest die Anzeige die Zahlstellen neu (sim.CombatSpots legt eine Liste an).
const spotsEvery = 0.5

// Farben der Zahlstellen und Kronen.
var (
	portalRed   = color.RGBA{230, 57, 70, 255}
	crownGold   = color.RGBA{255, 209, 102, 255}
	crownRim    = color.RGBA{156, 102, 68, 255}
	poleBrown   = color.RGBA{111, 69, 24, 255}
	shieldGrey  = color.RGBA{141, 153, 174, 255}
	triggerRed  = color.RGBA{193, 18, 31, 255}
	portalBarW  = px(80)
	portalAbove = px(14) // Abstand des Balkens über dem Portal-Bild
)

// combat legt Portal-Balken, Kronen am Boden bzw. in der Luft und Zahlstellen an.
func (p *planner) combat(w *sim.World, info *stageInfo, k view.Camera) {
	p.portalBars(w, k)
	for _, o := range w.Players {
		if o != nil && o.Crown != nil && (o.Crown.State == view.CrownFlying || o.Crown.State == view.CrownGround) &&
			visible(k, o.Crown.X, 2) {
			p.fallenCrown(o.Crown.X, o.Crown.Z)
		}
	}
	if info == nil {
		return
	}
	for _, s := range info.spots {
		if visible(k, s.X, 3) {
			p.spot(s)
		}
	}
}

// portalBars: HP jedes stehenden Portals (w.PortalHP läuft mit w.Portals, Index für Index).
func (p *planner) portalBars(w *sim.World, k view.Camera) {
	if w.PortalMaxHP <= 0 {
		return
	}
	_, h, ok := p.size(object("portal"))
	if !ok {
		h = px(220)
	}
	for i, x := range w.Portals {
		if i < len(w.PortalHP) && visible(k, x, 4) {
			p.bar(x, h+portalAbove, portalBarW, w.PortalHP[i]/w.PortalMaxHP, portalRed)
		}
	}
}

// portalDown: Das Portal an x ist zerstört (w.PortalsDown), sein Bild verschwindet.
func portalDown(w *sim.World, x float64) bool {
	for _, d := range w.PortalsDown {
		if math.Abs(d-x) < 0.01 {
			return true
		}
	}
	return false
}

// fallenCrown: Krone mit Fuß bei (x, z), z = Höhe in Units; Form view.CrownParts mit Rand.
func (p *planner) fallenCrown(x, z float64) {
	lift := view.CrownParts[0][3] / 2
	for _, q := range view.CrownParts {
		p.rect(&p.top, x+px(q[0]), z+px(lift+q[1]-q[3]/2-1), px(q[2]+2), px(q[3]+2), crownRim)
	}
	for _, q := range view.CrownParts {
		p.rect(&p.top, x+px(q[0]), z+px(lift+q[1]-q[3]/2), px(q[2]), px(q[3]), crownGold)
	}
}

// spot: Zeichen der Zahlstelle (Schild an der Burg, Auslöser und Banner am Tor) und, steht ein Monarch mit Krone in Reichweite,
// der Preis als Münz-Plätze (gefüllt = bezahlt).
func (p *planner) spot(s sim.CombatSpot) {
	p.rect(&p.body, s.X, 0, px(4), px(64), poleBrown)
	switch s.Kind {
	case "shield":
		p.rect(&p.body, s.X, px(30), px(24), px(30), dark)
		p.rect(&p.body, s.X, px(32), px(20), px(26), shieldGrey)
	case "trigger":
		p.body = append(p.body, item{shape: shapeTri, x: s.X + px(10), y: px(40), w: px(20), h: px(22), clr: triggerRed})
	default:
		p.rect(&p.body, s.X+px(9), px(46), px(18), px(14), crownRim)
		p.rect(&p.body, s.X+px(9), px(47), px(16), px(12), crownGold)
	}
	v := p.viewer // Preis und Zahlstand nur für den Monarchen dieser Split-Hälfte
	if v == nil || v.Crown != nil || math.Abs(v.X-s.X) > combatPrices.PayRangeUnits+1 {
		return
	}
	paid := 0
	if v.PayKey != nil && payKeyIs(*v.PayKey, s, v.Index) {
		paid = v.PayAmount
	}
	p.price(s.X, px(86), spotCost(s.Kind), paid, "", true)
}

// spotCost ist der Preis einer Zahlstelle in Münzen.
func spotCost(kind string) int {
	switch kind {
	case "shield":
		return combatPrices.Combat.ShieldCost
	case "trigger":
		return combatPrices.Combat.TriggerCost
	}
	return combatPrices.Combat.BannerCost
}

// payKeyIs: Der Zahl-Schlüssel eines Monarchen ("combat:<art>:<seite>", Banner "combat:banner:<monarch>:<seite>", Spiegel von
// sim.CombatSpot.key) gehört zur Zahlstelle s; ohne Allokation.
func payKeyIs(key string, s sim.CombatSpot, player int) bool {
	rest, ok := strings.CutPrefix(key, "combat:")
	if !ok {
		return false
	}
	rest, ok = strings.CutPrefix(rest, s.Kind+":")
	if !ok {
		return false
	}
	if s.Kind == "banner" {
		if len(rest) < 2 || rest[0] != byte('0'+player) || rest[1] != ':' {
			return false
		}
		rest = rest[2:]
	}
	switch s.Side {
	case -1:
		return rest == "-1"
	case 1:
		return rest == "1"
	}
	return rest == "0"
}
