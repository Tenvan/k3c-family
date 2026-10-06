package sim

import "math"

// Elite und Rüstung (docs/rules/buerger.md §§ 1–2, B-122, B-014; Q34, Q35, Q37):
//   - Schmiede: Am Platz zahlt man den Elite-Bogenschützen, am Angebot eliteWarrior (dx aus data/buildings.json) den
//     Elite-Krieger; Gold und Material aus troops.json › cost. Nach craftSeconds wird der nächste Bogenschütze bzw.
//     Krieger zum Gebäude Elite (HP, Schaden, Reichweite aus troops.json); ohne ihn wartet das fertige Upgrade.
//   - Rüstkammer: Am Platz zahlt man die nächste Rüstungsstufe (buildings.json › armory.armor, ab ihrer Hub-Stufe).
//     Nach craftSeconds.armor bekommen alle Kämpfer des Hubs sofort den Zuschlag auf die Basis-HP auf MaxHP und HP,
//     ohne Vollheilung; neue Kämpfer starten damit (makeFighter). Bauern und Landstreicher bleiben ohne.
// Gold am Platz liegt in Site.BowPaidGold, die Arbeit in Site.CraftDone (wie der Bogen der Werkstatt).

// placePayable: Nimmt der gebaute Platz s Münzen für sein Produkt am Platz (Bogen, Elite-Bogenschütze, Rüstung)?
func placePayable(w *World, s *Site) bool {
	switch s.Kind {
	case "workshop":
		return s.Bows+bowsCrafting(s) < buildings["workshop"].BowRack && s.BowPaidGold < troops["archer"].Cost.Gold
	case "smithy":
		return s.BowPaidGold < troops["eliteArcher"].Cost.Gold
	case "armory":
		stage, ok := armorStage(w)
		return ok && s.CraftDone == 0 && w.HubLevel >= stage.HubLevel && s.BowPaidGold < stage.Cost.Gold
	}
	return false
}

// offerAt: das Berufs-Angebot (findOffer), sonst ein bezahlbares Elite-Angebot in Zahl-Reichweite von x.
func offerAt(w *World, x float64) *offerTarget {
	if o := findOffer(w, x); o != nil {
		return o
	}
	return eliteOfferAt(w, x, true)
}

// eliteOfferAt ist das Elite-Angebot eines gebauten Gebäudes in Zahl-Reichweite von x; mit payable nur, solange sein
// Preis nicht voll bezahlt ist.
func eliteOfferAt(w *World, x float64, payable bool) *offerTarget {
	for _, s := range w.Sites {
		for i, of := range buildings[s.Kind].Offers {
			o := &offerTarget{s, i}
			near := isFighter(of.Kind) && s.State == "built" && math.Abs(o.x()-x) <= economy.PayRangeUnits
			if near && (!payable || o.paid() < troops[of.Kind].Cost.Gold) {
				return o
			}
		}
	}
	return nil
}

// payAnyOffer zahlt eine Münze an ein Angebot. payOffer schließt die Zahlung an einem Elite-Angebot nach jeder Münze
// ab (es ist kein Beruf); bis der Preis voll ist, bleibt sie offen, damit Loslassen erstattet.
func payAnyOffer(w *World, p *Player, o *offerTarget) {
	key, amount := p.PayKey, p.PayAmount
	payOffer(w, p, o)
	if isFighter(o.kind()) && o.paid() < troops[o.kind()].Cost.Gold {
		p.PayKey, p.PayAmount = key, amount
	}
}

// stepCrafts: Elite-Upgrades der Schmiede und Rüstungsstufen der Rüstkammer herstellen und anwenden.
func stepCrafts(w *World) {
	for _, s := range w.Sites {
		if s.State != "built" {
			continue
		}
		switch s.Kind {
		case "smithy":
			stepElite(w, s, "eliteArcher", &s.BowPaidGold, &s.CraftDone)
			for i, o := range buildings[s.Kind].Offers {
				if isFighter(o.Kind) && i < len(s.OfferPaid) {
					stepElite(w, s, o.Kind, &s.OfferPaid[i], &s.OfferDone)
				}
			}
		case "armory":
			stepArmor(w, s)
		}
	}
}

// craft: Voll bezahltes Gold geht mit dem Material aus dem Vorrat in Arbeit, wenn gerade nichts entsteht; true, sobald
// das Stück fertig ist (done bleibt gesetzt, bis der Aufrufer es anwendet und auf 0 setzt).
func craft(w *World, s *Site, product string, cost Cost, paid *int, done *float64) bool {
	if *done == 0 && *paid >= cost.Gold && canAfford(*w.Stock, cost) {
		spend(w.Stock, cost)
		*paid, *done = 0, w.Time+craftSeconds(w, s, product)
	}
	return *done > 0 && w.Time >= *done
}

// stepElite wertet nach der Herstellung den nächsten Kämpfer der Ausgangsfigur (troops.json › upgradeFrom) zum Platz
// zur Figur kind auf: volle HP der neuen Figur mit Rüstung, Schaden und Reichweite liest sein Verhalten aus kind.
func stepElite(w *World, s *Site, kind string, paid *int, done *float64) {
	if !craft(w, s, kind, troops[kind].Cost, paid, done) {
		return
	}
	from := troops[kind].UpgradeFrom
	t := nearest(w.Troops, func(t *Troop) float64 { return t.X }, s.X, math.Inf(1), func(t *Troop) bool { return t.Kind == from })
	if t == nil {
		return
	}
	t.Kind, t.HP, t.MaxHP = kind, armoredHP(w, kind), armoredHP(w, kind)
	*done = 0
}

// stepArmor: Nach der Herstellung steigt die Rüstungsstufe des Hubs; alle Kämpfer bekommen den Zuschlag auf MaxHP
// und HP (keine Vollheilung).
func stepArmor(w *World, s *Site) {
	stage, ok := armorStage(w)
	if !ok || !craft(w, s, "armor", stage.Cost, &s.BowPaidGold, &s.CraftDone) {
		return
	}
	s.CraftDone = 0
	before := armorBonus(w)
	w.ArmorLevel++
	for _, t := range w.Troops {
		if isFighter(t.Kind) {
			gain := troops[t.Kind].HP * (armorBonus(w) - before)
			t.MaxHP += gain
			t.HP += gain
		}
	}
}

// armorStage ist die nächste Rüstungsstufe; false, wenn alle erreicht sind.
func armorStage(w *World) (ArmorStage, bool) {
	stages := buildings["armory"].Armor
	if w.ArmorLevel >= len(stages) {
		return ArmorStage{}, false
	}
	return stages[w.ArmorLevel], true
}

// armorBonus ist der Zuschlag der erreichten Rüstungsstufe als Anteil der Basis-HP (0 ohne Rüstung).
func armorBonus(w *World) float64 {
	if w.ArmorLevel == 0 {
		return 0
	}
	return buildings["armory"].Armor[w.ArmorLevel-1].HPBonus
}

// armoredHP sind die vollen HP der Kämpfer-Figur kind mit der Rüstung des Hubs.
func armoredHP(w *World, kind string) float64 {
	return troops[kind].HP * (1 + armorBonus(w))
}
