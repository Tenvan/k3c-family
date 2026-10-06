package sim

import (
	"math"
	"strconv"
	"strings"
)

// Monarchen, Münzen und alles, was man bezahlt (Port von src/world/sim/economy.ts). A halten gibt im Takt Münzen an
// das nächste bezahlbare Ziel in Reichweite. Ohne Ziel fällt die Münze auf den Boden (so gibt man anderen Gold).

// payTarget ist genau eins von site, troop (Landstreicher), node, offer (Beruf) oder merchant ("buy", "sell").
type payTarget struct {
	site     *Site
	troop    *Troop
	node     *ResourceNode
	offer    *offerTarget
	merchant string
	sword    *Site // Schwert-Zahlziel der Werkstatt (warrior.go)
}

func stepPlayers(w *World, commands []PlayerCommand, dt float64) {
	stepRevive(w, commands, dt)
	for _, p := range w.Players {
		var cmd PlayerCommand
		if p.Index < len(commands) {
			cmd = commands[p.Index]
		}
		p.PayCooldown = math.Max(0, p.PayCooldown-dt)
		if !isAlive(p) {
			p.Paying = false
			refundPending(w, p)
			p.RespawnIn -= dt
			if p.RespawnIn <= 0 {
				respawn(w, p)
				emit(w, "revive", Event{"player": p.Index, "x": unitX(p.X)})
			}
			continue
		}
		movePlayer(w, p, cmd, dt)
		stepAttack(w, p, cmd, dt)
		stepSkills(w, p, cmd, dt)
		p.Paying = cmd.Pay
		if cmd.Pay && !p.reviving && p.PayCooldown <= 0 && (p.Gold > 0 || merchantAt(w, p.X) == "sell") {
			payOneCoin(w, p)
			p.PayCooldown = economy.PayIntervalSeconds
		}
		// Aufgehört zu halten (oder das Ziel gewechselt), bevor der Betrag voll war: Münzen kommen zurück.
		if p.PayKey != nil && (!cmd.Pay || keyOf(findPayTarget(w, p)) != *p.PayKey) {
			refundPending(w, p)
		}
	}
	collectCoins(w)
	collectPickups(w)
}

func movePlayer(w *World, p *Player, cmd PlayerCommand, dt float64) {
	mult := 1.0
	if cmd.Sprint {
		mult = float64(monarch.SprintMultiplier * MountOf(p).SprintFactor) // Sprint auf dem Reittier
	}
	target := float64(cmd.MoveX * speedOf(w, p) * mult)
	p.VX += float64((target - p.VX) * (1 - math.Exp(-monarch.Acceleration*dt)))
	p.X = math.Min(w.WidthUnits, math.Max(0, p.X+float64(p.VX*dt)))
	if math.Abs(p.VX) > 0.05 {
		p.Facing = int(sign(p.VX))
	}
}

func respawn(w *World, p *Player) {
	p.RespawnIn, p.HP, p.VX, p.ReviveProgress = 0, p.MaxHP, 0, 0
	p.X = w.HubX + 3
	if p.Index%2 == 0 {
		p.X = w.HubX - 3
	}
}

// nearest ist das nächste Element in Reichweite r von x, für das ok gilt; bei Gleichstand das frühere.
func nearest[T any](items []*T, pos func(*T) float64, x, r float64, ok func(*T) bool) *T {
	var best *T
	for _, it := range items {
		if math.Abs(pos(it)-x) > r || !ok(it) {
			continue
		}
		if best == nil || math.Abs(pos(it)-x) < math.Abs(pos(best)-x) {
			best = it
		}
	}
	return best
}

// findPayTarget: Was würde eine Münze gerade bezahlen? Bauplatz vor Landstreicher vor Ressource.
func findPayTarget(w *World, p *Player) *payTarget {
	r := economy.PayRangeUnits
	s := nearest(w.Sites, func(s *Site) float64 { return s.X }, p.X, r, func(s *Site) bool { return sitePayable(w, s) })
	if h := w.hubSite; h != nil && math.Abs(h.X-p.X) <= r && sitePayable(w, h) && (s == nil || math.Abs(h.X-p.X) < math.Abs(s.X-p.X)) {
		s = h
	}
	if s != nil {
		return &payTarget{site: s}
	}
	if o := findOffer(w, p.X); o != nil {
		return &payTarget{offer: o}
	}
	if s := swordAt(w, p.X, true); s != nil {
		return &payTarget{sword: s}
	}
	if side := merchantAt(w, p.X); side != "" {
		return &payTarget{merchant: side}
	}
	if t := nearest(w.Troops, func(t *Troop) float64 { return t.X }, p.X, r, func(t *Troop) bool { return t.Kind == "vagrant" }); t != nil {
		return &payTarget{troop: t}
	}
	markable := func(n *ResourceNode) bool {
		_, ok := gatherOf(n.Kind)
		return !n.Marked && ok
	}
	if n := nearest(w.Nodes, func(n *ResourceNode) float64 { return n.X }, p.X, r, markable); n != nil {
		return &payTarget{node: n}
	}
	return nil
}

// sitePayable: Nimmt der Platz gerade Münzen? Unbezahlte Plätze erst ab ihrer Hub-Stufe, Linien-Plätze nach der
// Linien-Regel (lines.go); gebaute für den Ausbau auf die nächste Stufe (hub_level.go).
func sitePayable(w *World, s *Site) bool {
	if s.State == "unpaid" {
		return w.HubLevel >= siteHubLevel(w, s) && lineOpen(w, s)
	}
	if upgradePayable(w, s) {
		return true
	}
	if s.Kind == "workshop" && s.State == "built" {
		return s.Bows+bowsCrafting(s) < buildings["workshop"].BowRack && s.BowPaidGold < troops["archer"].Cost.Gold
	}
	return false
}

// keyOf ist der Schlüssel eines Ziels ("site:3"), "" für kein Ziel.
func keyOf(t *payTarget) string {
	switch {
	case t == nil:
		return ""
	case t.offer != nil:
		return t.offer.key()
	case t.merchant != "":
		return "merchant:" + t.merchant
	case t.sword != nil:
		return "sword:" + strconv.Itoa(t.sword.ID)
	case t.site != nil:
		return "site:" + strconv.Itoa(t.site.ID)
	case t.troop != nil:
		return "vagrant:" + strconv.Itoa(t.troop.ID)
	}
	return "node:" + strconv.Itoa(t.node.ID)
}

// refundPending wirft gezahltes, aber nicht vollendetes Gold zurück auf den Boden und zieht es beim Ziel ab.
func refundPending(w *World, p *Player) {
	key, amount := p.PayKey, p.PayAmount
	clearPending(p)
	if key == nil || amount <= 0 {
		return
	}
	kind, idText, _ := strings.Cut(*key, ":")
	id, _ := strconv.Atoi(idText)
	back := 0
	switch kind {
	case "site":
		back = refundSite(w, id, amount)
	case "offer":
		back = refundOffer(w, idText, amount)
	case "merchant":
		back = refundMerchant(w, amount)
	case "sword":
		back = refundSword(w, id, amount)
	case "vagrant":
		if t := troopByID(w, id); t != nil && t.Kind == "vagrant" {
			back = min(amount, t.PaidGold)
			t.PaidGold -= back
		}
	default:
		if n := nodeByID(w, id); n != nil && !n.Marked {
			back = min(amount, n.PaidGold)
			n.PaidGold -= back
		}
	}
	for range back {
		w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: p.X})
	}
}

// refundSite zieht bis zu amount Münzen aus Bau, Ausbau oder Bogen des Platzes id zurück und liefert die Anzahl.
func refundSite(w *World, id, amount int) int {
	s := siteByID(w, id)
	if s == nil || !sitePayable(w, s) {
		return 0
	}
	paid := &s.BowPaidGold
	switch {
	case s.State == "unpaid":
		paid = &s.PaidGold
	case upgradePayable(w, s):
		paid = &s.UpgradePaid
	}
	back := min(amount, *paid)
	*paid -= back
	return back
}

func clearPending(p *Player) {
	p.PayKey, p.PayAmount = nil, 0
}

func payOneCoin(w *World, p *Player) {
	target := findPayTarget(w, p)
	// Am fertig bezahlten Bauplatz oder vollen Angebot nichts fallen lassen (sonst verliert man beim Festhalten Münzen).
	if target == nil && nearFullTarget(w, p.X) {
		return
	}
	if target != nil && target.merchant != "" {
		payMerchant(w, p, target.merchant)
		return
	}
	p.Gold--
	if target == nil {
		w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: p.X, BlockedPlayerID: intPtr(p.ID), BlockedUntil: w.Time + economy.DropPickupDelaySeconds})
		return
	}
	if target.offer == nil { // payOffer meldet selbst (to: offer)
		coinGiveEvent(w, p, target)
	}
	key := keyOf(target)
	if p.PayKey == nil || *p.PayKey != key {
		refundPending(w, p)
	}
	p.PayKey = &key
	p.PayAmount++
	switch {
	case target.offer != nil:
		payOffer(w, p, target.offer)
	case target.sword != nil:
		paySword(p, target.sword)
	case target.site != nil:
		paySite(w, p, target.site)
	case target.troop != nil:
		payVagrant(w, p, target.troop)
	default:
		n := target.node
		n.PaidGold++
		if g, _ := gatherOf(n.Kind); n.PaidGold >= g.MarkCost {
			n.Marked = true
			clearPending(p)
		}
	}
}

func paySite(w *World, p *Player, s *Site) {
	switch {
	case s.State == "unpaid":
		s.PaidGold++
		if s.PaidGold >= buildings[s.Kind].Cost.Gold {
			s.State = "waitingMaterial"
		}
	case upgradePayable(w, s):
		payUpgrade(w, s)
	default:
		s.BowPaidGold++
	}
	if !sitePayable(w, s) {
		clearPending(p)
	}
}

func payVagrant(w *World, p *Player, t *Troop) {
	t.PaidGold++
	// ponytail: `recruitCost?.gold ?? 1` in TS; ein ausdrückliches gold: 0 in data/troops.json würde hier 1 kosten.
	cost := troops["vagrant"].RecruitCost.Gold
	if cost == 0 {
		cost = 1
	}
	if t.PaidGold < cost {
		return
	}
	t.Kind, t.HP, t.MaxHP = "peasant", troops["peasant"].HP, troops["peasant"].HP
	t.AnchorX, t.TargetX, t.Job, t.PaidGold = w.HubX, t.X, nil, 0
	w.Events = append(w.Events, Event{"type": "recruited", "player": p.Index})
	clearPending(p)
}

func collectCoins(w *World) {
	kept := w.Coins[:0]
	for _, c := range w.Coins {
		var taker *Player
		for _, p := range w.Players {
			blocked := c.BlockedPlayerID != nil && *c.BlockedPlayerID == p.ID && w.Time < c.BlockedUntil
			if isAlive(p) && p.Gold < economy.Purse.MaxGold && math.Abs(p.X-c.X) <= economy.PickupRangeUnits && !blocked {
				taker = p
				break
			}
		}
		if taker == nil {
			kept = append(kept, c)
		} else {
			taker.Gold++
			emit(w, "coinPickup", Event{"player": taker.Index, "x": unitX(c.X)})
		}
	}
	w.Coins = kept
}

func collectPickups(w *World) {
	r := economy.PickupRangeUnits * 1.5
	kept := w.Pickups[:0]
	for _, pk := range w.Pickups {
		var p *Player
		for _, pl := range w.Players {
			if isAlive(pl) && math.Abs(pl.X-pk.X) <= r {
				p = pl
				break
			}
		}
		switch {
		case p == nil:
			kept = append(kept, pk)
		case pk.Kind == "chest":
			gold := w.rng.Int(economy.ChestGold[0], economy.ChestGold[1])
			giveGold(w, p, gold, pk.X)
			w.Events = append(w.Events, Event{"type": "chest", "player": p.Index, "gold": gold})
			chestSkillPoint(w, p)
		default:
			addPoolPoints(w, monarch.SkillPointSources.Hidden)
			w.Events = append(w.Events, Event{"type": "skillPoint", "player": p.Index, "total": w.SkillPoints})
		}
	}
	w.Pickups = kept
}

// giveGold füllt den Beutel, was nicht mehr passt, fällt bei x als Münzen auf den Boden.
func giveGold(w *World, p *Player, amount int, x float64) {
	fits := min(amount, economy.Purse.MaxGold-p.Gold)
	p.Gold += fits
	scatterCoins(w, x, amount-fits)
}

func scatterCoins(w *World, x float64, count int) {
	for range count {
		cx := math.Min(w.WidthUnits, math.Max(0, x+float64((w.rng.Next()-0.5)*3)))
		w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: cx})
	}
}

// addStock legt Baumaterial in den Vorrat (in einer Insel höchstens bis zur Kapazität); false, wenn resource kein
// Baumaterial ist.
func addStock(w *World, resource string, amount int) bool {
	_, ok := addStockCapped(w, resource, amount)
	return ok
}

func canAfford(s Stock, c Cost) bool {
	return s.Wood >= c.Wood && s.Stone >= c.Stone && s.Copper >= c.Copper && s.Iron >= c.Iron && s.Crystal >= c.Crystal
}

func spend(s *Stock, c Cost) {
	s.Wood -= c.Wood
	s.Stone -= c.Stone
	s.Copper -= c.Copper
	s.Iron -= c.Iron
	s.Crystal -= c.Crystal
}

// stepSites: Bezahlte Bauplätze ziehen das Material aus dem Hub-Vorrat, sobald genug da ist. Werkstatt fertigt Bögen
// (mit Herstellungszeit), voll bezahlte Angebote bilden Bauern aus (professions.go).
func stepSites(w *World) {
	for _, s := range w.Sites {
		if s.State == "waitingMaterial" && canAfford(*w.Stock, buildings[s.Kind].Cost) {
			spend(w.Stock, buildings[s.Kind].Cost)
			s.State = "waitingWorker"
		}
		if s.Kind == "workshop" && s.State == "built" {
			stepBowCraft(w, s)
			stepSwordCraft(w, s)
		}
	}
	eachSite(w, func(s *Site) { stepUpgrade(w, s) })
	stepOffers(w)
}

// payDawnIncome: morgens Steuern für jeden lebenden Monarchen.
func payDawnIncome(w *World) {
	for _, p := range w.Players {
		if isAlive(p) {
			giveGold(w, p, economy.DawnGoldPerPlayer, p.X)
		}
	}
}
