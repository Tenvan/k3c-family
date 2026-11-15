package props

// Zeichenliste der Kampfregeln (KS5, B-934, docs/rules/kampf.md): HP-Balken über jedem stehenden Portal, gefallene Kronen
// (fliegend im Bogen, liegend am Boden) und die Zahlstellen der Kampfvariante (Schild, Auslöser, Banner) mit Preis. Kronen auf
// Köpfen (Monarch, Gegner, Mitspieler) zeichnet actors. Krone und Zahlstellen sind Papierschnitt-Icons (B-940, Gruppe
// icons); nur der Portal-Balken bleibt gezeichnet. Die Zahlstellen tragen keinen Text, nur Preis-Münzen.

import (
	"image/color"
	"math"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// spotsEvery: so selten (Sekunden Spielzeit) liest die Anzeige die Zahlstellen neu (sim.CombatSpots legt eine Liste an).
const spotsEvery = 0.5

// Farben der Zahlstellen und Kronen.
var (
	portalRed   = color.RGBA{230, 57, 70, 255}
	portalBarW  = px(80)
	portalAbove = px(14) // Abstand des Balkens über dem Portal-Bild
)

// Bilder der Krone und der Zahlstellen (data/paper.json › objekte), einmal angelegt.
var (
	crownLook   = look{"icons/icon-krone", idle}
	shieldLook  = look{"icons/zahlstelle-schild", idle}
	triggerLook = look{"icons/zahlstelle-ausloeser", idle}
	bannerLook  = look{"icons/zahlstelle-banner", idle}
)

// combatAssets sind die Icons der Kampfregeln, die props in jedem Level anmeldet (levelAssets).
var combatAssets = []string{crownLook.asset, shieldLook.asset, triggerLook.asset, bannerLook.asset}

// combat legt Portal-Balken, Kronen am Boden bzw. in der Luft und Zahlstellen an.
func (p *planner) combat(w *sim.World, info *stageInfo, k view.Camera) {
	p.portalBars(w, k)
	for _, o := range w.Players {
		if o != nil && o.Crown != nil && (o.Crown.State == sim.CrownFlying || o.Crown.State == sim.CrownGround) &&
			visible(k, o.Crown.X, 2) {
			p.fallenCrown(o.Crown.X, o.Crown.Z)
		}
	}
	if info == nil {
		return
	}
	for _, s := range info.spots {
		if visible(k, s.X, 3) {
			p.spot(w, s)
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

// fallenCrown: Krone-Icon mit Fuß bei (x, z), z = Höhe in Units; liegt über den Figuren (top).
func (p *planner) fallenCrown(x, z float64) {
	if _, _, ok := p.size(crownLook); ok {
		p.top = append(p.top, item{shape: shapeSprite, look: crownLook, x: x, y: z})
	}
}

// spot: Icon der Zahlstelle (Schild an der Burg, Auslöser und Banner am Tor) und, steht ein Monarch mit Krone in Reichweite,
// der Preis als Münz-Plätze (gefüllt = bezahlt).
func (p *planner) spot(w *sim.World, s sim.CombatSpot) {
	l := bannerLook
	switch s.Kind {
	case "shield":
		l = shieldLook
	case "trigger":
		l = triggerLook
	}
	p.sprite(l, s.X, 0, 0)
	v := p.viewer // Preis und Zahlstand nur für den Monarchen dieser Split-Hälfte
	if v == nil || v.Crown != nil || !s.NearOf(w, v) { // erst die billige Entfernung, StateFor alloziert
		return
	}
	if st := s.StateFor(w, v); st.Payable {
		p.price(s.X, px(86), st.Price, st.Paid, "", true)
	}
}
