package sim

import (
	"math"
	"testing"
)

// W4.3b (B-122, B-014; Sprint W4 AC-04, AC-06): Elite-Upgrade in der Schmiede und Rüstung in der Rüstkammer, Kosten
// und Werte aus data/troops.json und data/buildings.json.

// eliteSpot ist der Ort des Elite-Krieger-Angebots an der Schmiede s und sein Index in buildings.smithy.offers.
func eliteSpot(t *testing.T, s *Site) (float64, int) {
	t.Helper()
	for i, o := range buildings[s.Kind].Offers {
		if o.Kind == "eliteWarrior" {
			return s.X + o.DX, i
		}
	}
	t.Fatal("Schmiede ohne Elite-Krieger-Angebot")
	return 0, 0
}

// fighterAt ist ein Kämpfer der Figur kind bei x auf der Seite von x.
func fighterAt(w *World, kind string, x float64) *Troop {
	tr := spawnVagrant(w, x, x)
	makeFighter(w, tr, kind)
	tr.X, tr.AnchorX = x, w.HubX+math.Copysign(1, x-w.HubX)
	return tr
}

// wantElite: Ein Spieler zahlt am Zahlziel at der Schmiede das Gold für kind, erst danach kommt das Material in den Vorrat. Der nächste
// Kämpfer der Ausgangsfigur zur Schmiede wird nach craftSeconds Elite, der fernere nicht; Kosten und HP laut Daten.
func wantElite(t *testing.T, kind string, at func(*Site) float64) (*World, *Troop) {
	t.Helper()
	w := quietWorld(t)
	p := AddPlayer(w)
	smithy := buildSite(t, w, "smithy")
	from, cost := troops[kind].UpgradeFrom, troops[kind].Cost
	near, far := fighterAt(w, from, smithy.X+30), fighterAt(w, from, w.HubX-60)
	gold := p.Gold
	payAt(w, []*Player{p}, at(smithy), 1.5*float64(cost.Gold)*economy.PayIntervalSeconds, func() bool { return false })
	if gold-p.Gold != cost.Gold {
		t.Fatalf("%s: Spieler zahlte %d Gold, laut Daten %d", kind, gold-p.Gold, cost.Gold)
	}
	*w.Stock = Stock{Stone: 1000, Copper: 1000}
	took := runUntil(w, 60, func() bool { return near.Kind == kind })
	if want := buildings["smithy"].CraftSeconds[kind]; math.Abs(took-want) > 2*dt {
		t.Errorf("%s: Upgrade nach %.2f s, laut Daten %v s", kind, took, want)
	}
	if w.Stock.Stone != 1000-cost.Stone || w.Stock.Copper != 1000-cost.Copper {
		t.Errorf("%s: Vorrat %+v, laut Daten %+v abgebucht", kind, *w.Stock, cost)
	}
	if far.Kind != from || near.HP != troops[kind].HP || near.MaxHP != troops[kind].HP {
		t.Errorf("%s: fernerer ist %s, Elite-HP %v/%v, laut Daten %v", kind, far.Kind, near.HP, near.MaxHP, troops[kind].HP)
	}
	w.Troops = []*Troop{near}
	return w, near
}

func TestUpgradeEliteBogenschuetze(t *testing.T) {
	w, _ := wantElite(t, "eliteArcher", func(s *Site) float64 { return s.X })
	spawnEnemy(w, "skeleton", w.HubX+40)
	for tm := 0.0; tm < 20 && len(w.Projectiles) == 0; tm += dt {
		Step(w, nil, dt)
	}
	if len(w.Projectiles) == 0 || w.Projectiles[0].Damage != troops["eliteArcher"].Damage {
		t.Errorf("Elite-Bogenschütze schießt %+v, laut Daten Schaden %v", w.Projectiles, troops["eliteArcher"].Damage)
	}
}

func TestUpgradeEliteKrieger(t *testing.T) {
	w, tr := wantElite(t, "eliteWarrior", func(s *Site) float64 {
		spot, _ := eliteSpot(t, s)
		return spot
	})
	wall := sturdy(lineSite(t, w, "wall", 1, 1))
	if x := warriorPostOf(w, tr); math.Abs(x-(wall.X-troops["warrior"].PostUnits)) > arrive {
		t.Fatalf("Elite-Krieger bei %v, Posten hinter der Mauer %v erwartet", x, wall.X)
	}
	if got := firstHit(w, wall.X+8); got != troops["eliteWarrior"].Damage {
		t.Errorf("Gegner an der Mauer nimmt %v Schaden, laut Daten %v", got, troops["eliteWarrior"].Damage)
	}
}

// (d) Zwei Spieler zahlen gleichzeitig am Schmiede-Platz (Elite-Bogenschütze) und am Elite-Krieger-Angebot; wer vor
// dem vollen Preis loslässt, bekommt die Münzen zurück.
func TestUpgradeZweiSpielerAnBeidenSchmiedeZielen(t *testing.T) {
	w := quietWorld(t)
	p1, p2 := AddPlayer(w), AddPlayer(w)
	s := buildSite(t, w, "smithy")
	spot, idx := eliteSpot(t, s)
	g1, g2 := p1.Gold, p2.Gold
	for tm := 0.0; tm < 20; tm += dt {
		p1.X, p2.X = s.X, spot
		Step(w, []PlayerCommand{{Pay: true}, {Pay: true}}, dt)
	}
	archer, warrior := troops["eliteArcher"].Cost.Gold, troops["eliteWarrior"].Cost.Gold
	if s.BowPaidGold != archer || s.OfferPaid[idx] != warrior || g1-p1.Gold != archer || g2-p2.Gold != warrior {
		t.Fatalf("Elite-Bogenschütze %d/%d (Spieler 1 zahlte %d), Elite-Krieger %d/%d (Spieler 2 zahlte %d)",
			s.BowPaidGold, archer, g1-p1.Gold, s.OfferPaid[idx], warrior, g2-p2.Gold)
	}
	s.OfferPaid[idx], g2 = 0, p2.Gold
	paid := 0
	payAt(w, []*Player{p2}, spot, 3*economy.PayIntervalSeconds, func() bool {
		paid = max(paid, s.OfferPaid[idx])
		return false
	})
	Step(w, nil, dt)
	if paid == 0 || s.OfferPaid[idx] != 0 || p2.Gold != g2 {
		t.Errorf("gezahlt %d, nach dem Loslassen %d Gold im Angebot, Spieler hat %d statt %d", paid, s.OfferPaid[idx], p2.Gold, g2)
	}
}

// armorWorld: gebaute Rüstkammer bei Hub-Stufe level, ein Spieler mit Gold, Eisen im Vorrat.
func armorWorld(t *testing.T, level int) (*World, *Player, *Site) {
	t.Helper()
	w := quietWorld(t)
	w.HubLevel = level
	*w.Stock = Stock{Iron: 1000}
	return w, AddPlayer(w), buildSite(t, w, "armory")
}

// buyArmor zahlt die nächste Rüstungsstufe und wartet die Herstellung ab; false, wenn sie nicht zustande kommt.
func buyArmor(w *World, p *Player, s *Site) bool {
	level := w.ArmorLevel
	p.Gold = economy.Purse.MaxGold
	payAt(w, []*Player{p}, s.X, 60, func() bool { return s.CraftDone > 0 })
	return runUntil(w, 2*buildings["armory"].CraftSeconds["armor"], func() bool { return w.ArmorLevel > level }) > 0
}

// (b) Stufe 1 ab Hub-Stufe 4, Stufe 2 ab 5; Zuschlag auf MaxHP und HP ohne Vollheilung, nur Kämpfer, auch neue.
func TestArmorZweiStufenNurKaempfer(t *testing.T) {
	stages := buildings["armory"].Armor
	if len(stages) != 2 {
		t.Fatalf("%d Rüstungsstufen in den Daten, erwartet 2", len(stages))
	}
	w, p, s := armorWorld(t, stages[0].HubLevel-1)
	if sitePayable(w, s) {
		t.Errorf("Rüstung bei Hub-Stufe %d bezahlbar", w.HubLevel)
	}
	w.HubLevel = stages[0].HubLevel
	warrior := fighterAt(w, "warrior", w.HubX)
	warrior.HP = 50
	peasant := spawnVagrant(w, w.HubX, w.HubX)
	peasant.Kind, peasant.HP, peasant.MaxHP = "peasant", troops["peasant"].HP, troops["peasant"].HP
	if !sitePayable(w, s) {
		t.Errorf("Rüstung bei Hub-Stufe %d nicht bezahlbar", w.HubLevel)
	}
	iron := w.Stock.Iron
	if !buyArmor(w, p, s) {
		t.Fatalf("Stufe 1 nicht hergestellt: %+v", s)
	}
	if spent := economy.Purse.MaxGold - p.Gold; spent != stages[0].Cost.Gold || iron-w.Stock.Iron != stages[0].Cost.Iron {
		t.Errorf("Stufe 1 kostete %d Gold, %d Eisen, laut Daten %+v", spent, iron-w.Stock.Iron, stages[0].Cost)
	}
	base := troops["warrior"].HP
	gain := base * stages[0].HPBonus
	if warrior.MaxHP != base+gain || warrior.HP != 50+gain {
		t.Errorf("Krieger %v/%v, erwartet %v/%v (ohne Vollheilung)", warrior.HP, warrior.MaxHP, 50+gain, base+gain)
	}
	if peasant.MaxHP != troops["peasant"].HP {
		t.Errorf("Bauer mit Rüstung: MaxHP %v", peasant.MaxHP)
	}
	if fresh := fighterAt(w, "archer", w.HubX); fresh.MaxHP != troops["archer"].HP*(1+stages[0].HPBonus) {
		t.Errorf("neuer Bogenschütze startet mit %v HP, erwartet mit Rüstung", fresh.MaxHP)
	}
	if sitePayable(w, s) {
		t.Errorf("Stufe 2 bei Hub-Stufe %d bezahlbar", w.HubLevel)
	}
	w.HubLevel = stages[1].HubLevel
	if !buyArmor(w, p, s) || warrior.MaxHP != base*(1+stages[1].HPBonus) || sitePayable(w, s) {
		t.Errorf("Stufe 2: Stufe %d, Krieger MaxHP %v, erwartet %v, danach bezahlbar %v",
			w.ArmorLevel, warrior.MaxHP, base*(1+stages[1].HPBonus), sitePayable(w, s))
	}
}
