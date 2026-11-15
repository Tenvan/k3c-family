package sim

// Münzen am Ziel buchen (aus economy.go ausgelagert: Dateigrenze 400 Zeilen).

// payToTarget bucht die Münze beim Ziel (Angebot, Schwert, Kampfvariante, Bauplatz, Landstreicher oder Ressource).
func payToTarget(w *World, p *Player, target *payTarget) {
	switch {
	case target.offer != nil:
		payAnyOffer(w, p, target.offer)
	case target.sword != nil:
		paySword(w, p, target.sword)
	case target.combat != nil:
		payCombat(w, p, target.combat)
	case target.site != nil:
		paySite(w, p, target.site)
	case target.troop != nil:
		payVagrant(w, p, target.troop)
	default:
		n := target.node
		n.PaidGold++
		if g, _ := gatherOf(w.values(), n.Kind); n.PaidGold >= g.MarkCost {
			markNode(n, p)
			clearPending(p)
		}
	}
}

func payVagrant(w *World, p *Player, t *Troop) {
	t.PaidGold++
	// ponytail: `recruitCost?.gold ?? 1` in TS; ein ausdrückliches gold: 0 in data/troops.json würde hier 1 kosten.
	cost := w.values().Troops["vagrant"].RecruitCost.Gold
	if cost == 0 {
		cost = 1
	}
	if t.PaidGold < cost {
		return
	}
	hp := w.values().Troops["peasant"].HP
	t.Kind, t.HP, t.MaxHP = "peasant", hp, hp
	t.AnchorX, t.TargetX, t.Job, t.PaidGold = w.HubX, t.X, nil, 0
	w.Events = append(w.Events, Event{"type": "recruited", "player": p.Index})
	clearPending(p)
}
