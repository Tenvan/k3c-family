package sim

import (
	"math"
	"strconv"
)

// Aktiver Angriff (docs/rules/kampf.md § 3, B-932, B-933). Variante groups: Schild an der Burg (Zahlziel `shield`) macht
// einen Soldaten zum Gruppenführer, ein Auslöser am äußersten Tor (`trigger`, je Seite) schickt alle Gruppen der Seite los.
// Variante general: ein Banner je Monarch an jedem Tor (`banner`), ihm folgt ein Trupp. Angreifende Truppen setzen
// Troop.attacking und beschädigen so Portale (portal.go). Zahlziele laufen über input.pay (economy.go › findPayTarget).
// Kein rng; Reihenfolge der Truppen und Spieler bestimmt jede Auswahl.

// CombatSpot ist ein Zahlziel der Kampfvariante: Kind `shield` (Burg), `trigger` (Auslöser) oder `banner`; Side -1 links,
// 1 rechts, 0 an der Burg.
type CombatSpot struct {
	Kind string  `json:"kind"`
	Side int     `json:"side"`
	X    float64 `json:"x"`
}

// combatRuntime: bezahlter Betrag je Zahlziel und bezahlte, noch nicht vergebene Schilde (nicht im Spielstand).
type combatRuntime struct {
	paid    map[string]int
	shields int
}

func (s CombatSpot) key() string { return s.Kind + ":" + strconv.Itoa(s.Side) }

// CombatSpots sind die Zahlziele der Kampfvariante der Stufe (Auslöser und Banner nur an einem gebauten Tor).
func CombatSpots(w *World) []CombatSpot {
	c := w.values().Economy.Combat
	var out []CombatSpot
	kind := "trigger"
	if combatMode(w) == CombatGeneral {
		kind = "banner"
	} else {
		out = append(out, CombatSpot{Kind: "shield", X: w.HubX + c.ShieldOffsetUnits})
	}
	for _, side := range []float64{-1, 1} {
		if wall := outerWall(w, side); wall != nil {
			out = append(out, CombatSpot{Kind: kind, Side: int(side), X: wall.X - side*c.GateInsetUnits})
		}
	}
	return out
}

// combatSpotNear: Das Zahlziel in Zahl-Reichweite von x, auch wenn es gerade nichts nimmt.
func combatSpotNear(w *World, x float64) *CombatSpot {
	var best *CombatSpot
	for _, s := range CombatSpots(w) {
		if math.Abs(s.X-x) <= w.values().Economy.PayRangeUnits && (best == nil || math.Abs(s.X-x) < math.Abs(best.X-x)) {
			best = &s
		}
	}
	return best
}

// combatTarget: Das Zahlziel, das für den Monarchen p gerade Münzen nimmt, sonst nil.
func combatTarget(w *World, p *Player) *CombatSpot {
	var best *CombatSpot
	for _, s := range CombatSpots(w) {
		if math.Abs(s.X-p.X) > w.values().Economy.PayRangeUnits || !combatPayable(w, p, s) {
			continue
		}
		if best == nil || math.Abs(s.X-p.X) < math.Abs(best.X-p.X) {
			best = &s
		}
	}
	return best
}

// combatPayable: Schild nur, solange ein freier Soldat da ist; Auslöser nur mit einer Gruppe der Seite; Banner nur ohne
// Banner.
func combatPayable(w *World, p *Player, s CombatSpot) bool {
	switch s.Kind {
	case "shield":
		return w.combat.shields+countFetching(w) < len(freeWarriors(w))
	case "trigger":
		for _, l := range leadersOf(w, float64(s.Side)) {
			if l.sortie == 0 { // nur Gruppen am Tor lassen sich losschicken
				return true
			}
		}
		return false
	}
	return p.Banner == 0
}

func combatCost(w *World, kind string) int {
	c := w.values().Economy.Combat
	switch kind {
	case "shield":
		return c.ShieldCost
	case "trigger":
		return c.TriggerCost
	}
	return c.BannerCost
}

// payCombat nimmt eine Münze am Zahlziel s; ist der Preis voll, wirkt es (Schild vergeben, Gruppen los, Banner aufnehmen).
func payCombat(w *World, p *Player, s *CombatSpot) {
	if w.combat.paid == nil {
		w.combat.paid = map[string]int{}
	}
	key := s.key()
	w.combat.paid[key]++
	if w.combat.paid[key] < combatCost(w, s.Kind) {
		return
	}
	delete(w.combat.paid, key)
	clearPending(p)
	switch s.Kind {
	case "shield":
		w.combat.shields++
	case "trigger":
		sendGroups(w, float64(s.Side))
	default:
		takeBanner(w, p, s.Side)
	}
}

// refundCombat zieht bis zu amount Münzen vom Zahlziel `key` ("trigger:-1") zurück und liefert die Anzahl.
func refundCombat(w *World, key string, amount int) int {
	back := min(amount, w.combat.paid[key])
	if back > 0 {
		w.combat.paid[key] -= back
	}
	return back
}

// stepCombat läuft je Tick vor stepTroops: Zuordnungen pflegen, Gruppen bzw. Banner und Trupps führen, Angriffsflag setzen.
func stepCombat(w *World) {
	if combatMode(w) == CombatGeneral {
		stepBanners(w)
	} else {
		stepGroups(w)
	}
	for _, t := range w.Troops {
		if t.Group != 0 {
			t.attacking = squadActive(w, t)
		}
	}
}

// playerByID: der Monarch mit der ID id, sonst nil.
func playerByID(w *World, id int) *Player {
	for _, p := range w.Players {
		if p.ID == id {
			return p
		}
	}
	return nil
}

// isSoldier: Krieger oder Elite-Krieger (Gruppen nehmen nur Soldaten).
func isSoldier(t *Troop) bool { return t.Kind == "warrior" || t.Kind == "eliteWarrior" }

// troopSide: Seite (-1 links, 1 rechts) der Truppe, nach ihrem Anker.
func troopSide(w *World, t *Troop) float64 {
	if t.AnchorX < w.HubX {
		return -1
	}
	return 1
}

// leaveSquad löst t aus Gruppe oder Trupp.
func leaveSquad(w *World, t *Troop) {
	if t.shieldFetch {
		w.combat.shields++ // das Schild ging nicht verloren: ein anderer Soldat holt es
	}
	t.Leader, t.Group, t.sortie, t.shieldFetch, t.attacking = false, 0, 0, false, false
}

// squadActive: Ist die Gruppe bzw. der Trupp von t gerade im Angriff (Führer mit Auftrag, Monarch mit Banner)?
func squadActive(w *World, t *Troop) bool {
	if combatMode(w) == CombatGeneral {
		p := playerByID(w, t.Group)
		return p != nil && p.Banner != 0
	}
	l := troopByID(w, t.Group)
	return l != nil && l.Leader && l.sortie != 0
}

// squadTarget: Ort des nächsten Ziels vor `anchor` in Richtung dir: der nächste Gegner innerhalb der Anspähweite, sonst das
// nächste Portal (mit allPortals unbegrenzt, sonst ebenfalls innerhalb der Anspähweite).
func squadTarget(w *World, anchor, dir float64, allPortals bool) (float64, bool) {
	reach := w.values().Economy.Combat.EngageUnits
	best, found := 0.0, false
	for _, e := range w.Enemies {
		if d := (e.X - anchor) * dir; d >= -1 && d <= reach && (!found || d < (best-anchor)*dir) {
			best, found = e.X, true
		}
	}
	if found {
		return best, true
	}
	for _, x := range w.Portals {
		if d := (x - anchor) * dir; d >= -1 && (allPortals || d <= reach) && (!found || d < (best-anchor)*dir) {
			best, found = x, true
		}
	}
	return best, found
}

// squadPost: Ort, zu dem t im Angriff geht (Posten der Gruppe bzw. des Trupps); false = normaler Posten am Tor.
func squadPost(w *World, t *Troop) (float64, bool) {
	if t.shieldFetch {
		return w.HubX, true
	}
	if t.Group == 0 || !squadActive(w, t) {
		return 0, false
	}
	anchor, dir, all := 0.0, 0.0, true
	if combatMode(w) == CombatGeneral {
		p := playerByID(w, t.Group)
		anchor, dir, all = p.X, float64(p.Banner), false
	} else {
		l := troopByID(w, t.Group)
		anchor, dir = l.X, l.sortie
	}
	reach := math.Max(w.values().Troops[t.Kind].Range*0.8, 1)
	if x, ok := squadTarget(w, anchor, dir, all); ok {
		return x - dir*reach, true
	}
	if combatMode(w) == CombatGeneral { // ohne Ziel folgt der Trupp dem Monarchen: Soldaten voraus, Schützen dahinter
		if isSoldier(t) {
			return anchor + dir*1.5, true
		}
		return anchor - dir*1.5, true
	}
	return 0, false
}
