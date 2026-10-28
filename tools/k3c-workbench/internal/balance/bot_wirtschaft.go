package balance

import (
	"math"

	"k3c/engine/sim"
)

// Profil „Wirtschaft“ (`wirtschaft`, BAL6, B-347): der Bot des Standardszenarios. Er spielt die Wirtschaft so, dass
// jeder Korridor aus docs/rules/zielkorridore.md › BR1 einen Messwert hat: Mauern, Turm, Farm und Werkstatt bauen, den
// Hub ausbauen (Material markieren oder beim Händler kaufen), Holz nachmarkieren und Kämpfer schaffen (Bögen,
// Landstreicher). Ob ein Platz Münzen nimmt und wie viel noch fehlt, liest er über sim.SiteGoldLeft; er zahlt nur, wenn
// das eigene Gold den Rest deckt, sonst kämen die Münzen beim Loslassen zurück.

// buildOrder: unbezahlte Plätze in dieser Reihenfolge, je Art der nächste bezahlbare.
var buildOrder = []string{"wall", "tower", "farm", "workshop"}

const (
	woodWant     = 150 // Holz, das er im Vorrat halten will (Korridor ≥ 100 plus ein Bogen); darunter markiert er Bäume
	peasantLimit = 4   // so viele Bauern wirbt er höchstens an (Arbeiter und Träger der Bögen); ab 6 fehlt das Gold für Mauern und Bögen, Nacht 5 vernichtet alles (B-645, Hebel 3)
	fighterWant  = 8   // so viele Kämpfer will er (Korridor Tag 6 ≥ 8); darüber spart er das Gold für den Hub
)

// wirtschaft: nachts an der Burg; tags in dieser Reihenfolge: Bauten aus buildOrder, deren Material im Vorrat liegt;
// der Hub-Ausbau; Holz nachmarkieren; Bögen an der Werkstatt; Landstreicher anwerben. Liegt das Material des
// Hub-Ausbaus bereit, spart er dessen Gold: alles danach nur mit Gold über dieser Rücklage.
func wirtschaft(w *sim.World, p *sim.Player) sim.PlayerCommand {
	if w.Cycle.Phase == "night" {
		return goTo(p, w.HubX, false)
	}
	if peasants(w) == 0 && nearestVagrant(w, p) != nil { // ohne Arbeiter bleibt jeder bezahlte Bau liegen (waitingWorker): erst anwerben
		return recruitOrHub(w, p)
	}
	x, reserve := hubStep(w, p)
	if x == &w.Castle.X { // Material und Gold für den Hub-Ausbau liegen bereit: zuerst zahlen, vor neuen Bauten
		return goTo(p, *x, true)
	}
	if x := fighterStep(w, p, reserve); x != nil { // Kämpfer vor Wiederaufbau: Unter Tage räumt jede Welle Mauern und Türme ab, die Armee trägt
		return goTo(p, *x, true)
	}
	if bx := buildX(w, p, reserve); bx != nil {
		return goTo(p, *bx, true)
	}
	if x == nil && reserve == 0 {
		if n := nodeFor(w, p, sim.Cost{Wood: woodWant}); n != nil {
			x = &n.X
		}
	}
	if x == nil {
		x = fighterStep(w, p, reserve)
	}
	if x != nil {
		return goTo(p, *x, true)
	}
	if p.Gold >= prices.recruitGold+reserve && peasants(w) < peasantLimit {
		return recruitOrHub(w, p)
	}
	return goTo(p, w.HubX, false)
}

// buildX: der Ort des nächsten Baus (nextBuild) oder, wenn nichts bezahlbar ist und keine Rücklage nötig, des nächsten Baums
// oder Felsens, der einen Mauer- oder Turm-Platz sperrt (B-652); nil sonst.
func buildX(w *sim.World, p *sim.Player, reserve int) *float64 {
	if s := nextBuild(w, p, reserve); s != nil {
		return &s.X
	}
	if reserve == 0 {
		if n := clearNode(w, p); n != nil {
			return &n.X
		}
	}
	return nil
}

// nextBuild: der nächste Bau aus buildOrder, dessen Material im Vorrat liegt und dessen Rest plus Rücklage das Gold deckt.
func nextBuild(w *sim.World, p *sim.Player, reserve int) *sim.Site {
	if waitingMaterial(w) {
		return nil
	}
	for _, kind := range buildOrder {
		if s := nearestPayable(w, p, kind, "unpaid", reserve); s != nil && covers(*w.Stock, prices.cost[kind]) {
			return s
		}
	}
	return nil
}

// hubStep: Ort zum Zahlen für den Hub-Ausbau (Burg, Knoten oder Händler) oder die Rücklage, solange das Gold fehlt.
func hubStep(w *sim.World, p *sim.Player) (*float64, int) {
	up := sim.EconomyOf(w).HubUpgrade
	if up == nil || up.State != "" {
		return nil, 0
	}
	need := sim.Cost{Wood: up.Material.Wood, Stone: up.Material.Stone, Copper: up.Material.Copper, Iron: up.Material.Iron,
		Crystal: up.Material.Crystal}
	left := up.Gold - up.Paid
	switch {
	case covers(*w.Stock, need) && p.Gold >= left:
		return &w.Castle.X, 0
	case covers(*w.Stock, need):
		return nil, left
	}
	if n := nodeFor(w, p, need); n != nil {
		return &n.X, 0
	}
	if m := w.Merchant; m != nil && amountOf(*w.Stock, m.Resource) < costOf(need, m.Resource) &&
		p.Gold >= prices.merchantGold-m.BuyPaid {
		x := w.HubX + prices.merchantBuyOffset
		return &x, 0
	}
	return nil, 0
}

// fighterStep: die nächste Werkstatt, die einen Bogen nimmt (Gold für den Rest plus Rücklage), solange ein Bauer ihn
// holen kann und weniger als fighterWant Kämpfer da sind; nil sonst.
func fighterStep(w *sim.World, p *sim.Player, reserve int) *float64 {
	if peasants(w) == 0 || sim.EconomyOf(w).Fighters >= fighterWant {
		return nil
	}
	if s := nearestPayable(w, p, "workshop", "built", reserve); s != nil {
		return &s.X
	}
	return nil
}

// nearestPayable: der nächste Platz der Art im Zustand state, der Münzen nimmt und dessen Rest plus Rücklage das Gold deckt.
func nearestPayable(w *sim.World, p *sim.Player, kind, state string, reserve int) *sim.Site {
	var best *sim.Site
	for _, s := range w.Sites {
		if s.Kind != kind || s.State != state {
			continue
		}
		if left, ok := sim.SiteGoldLeft(w, s); ok && left > 0 && p.Gold >= left+reserve && (best == nil || closer(p.X, s.X, best.X)) {
			best = s
		}
	}
	return best
}

// waitingMaterial: Ein bezahlter Bau wartet noch auf Material; dann zahlt er keinen weiteren, damit das Material nicht
// auf viele Plätze verteilt liegen bleibt.
func waitingMaterial(w *sim.World) bool {
	for _, s := range w.Sites {
		if s.State == "waitingMaterial" {
			return true
		}
	}
	return false
}

func peasants(w *sim.World) int {
	n := 0
	for _, t := range w.Troops {
		if t.Kind == "peasant" {
			n++
		}
	}
	return n
}

// nodeFor: der nächste unmarkierte Knoten eines Rohstoffs, der für need fehlt (Vorrat plus markierte Knoten reichen
// nicht), mit bezahlbarem Rest und ohne zahlbaren Platz in Reichweite (der ginge beim Zahlen vor).
func nodeFor(w *sim.World, p *sim.Player, need sim.Cost) *sim.ResourceNode {
	pending := map[string]int{}
	for _, n := range w.Nodes {
		if np, ok := prices.nodes[n.Kind]; ok && n.Marked {
			pending[np.Resource] += np.Amount
		}
	}
	blocked := payableSitesX(w)
	var best *sim.ResourceNode
	for _, n := range w.Nodes {
		np, ok := prices.nodes[n.Kind]
		if !ok || n.Marked || p.Gold < np.MarkCost-n.PaidGold || nearAny(n.X, blocked) ||
			amountOf(*w.Stock, np.Resource)+pending[np.Resource] >= costOf(need, np.Resource) {
			continue
		}
		if best == nil || closer(p.X, n.X, best.X) {
			best = n
		}
	}
	return best
}

// payableSitesX: Orte aller Plätze, die gerade Münzen nehmen.
func payableSitesX(w *sim.World) []float64 {
	var xs []float64
	for _, s := range w.Sites {
		if _, ok := sim.SiteGoldLeft(w, s); ok {
			xs = append(xs, s.X)
		}
	}
	return xs
}

// nearAny: x liegt in Zahl-Reichweite eines der Orte.
func nearAny(x float64, xs []float64) bool {
	for _, o := range xs {
		if math.Abs(o-x) <= prices.payRange {
			return true
		}
	}
	return false
}

// covers: Der Vorrat deckt das Material der Kosten (Gold zählt nicht).
func covers(have sim.Stock, c sim.Cost) bool {
	return have.Wood >= c.Wood && have.Stone >= c.Stone && have.Copper >= c.Copper && have.Iron >= c.Iron && have.Crystal >= c.Crystal
}

// amountOf ist die Menge des Rohstoffs r (Name wie in economy.json) im Vorrat, costOf in den Kosten.
func amountOf(s sim.Stock, r string) int {
	return costOf(sim.Cost{Wood: s.Wood, Stone: s.Stone, Copper: s.Copper, Iron: s.Iron, Crystal: s.Crystal}, r)
}

func costOf(c sim.Cost, r string) int {
	switch r {
	case "wood":
		return c.Wood
	case "stone":
		return c.Stone
	case "copper":
		return c.Copper
	case "iron":
		return c.Iron
	case "crystal":
		return c.Crystal
	}
	return 0
}
