package sim

import (
	"slices"
	"testing"
)

// W4.2 (B-121, Sprint W4 AC-03): Händler nach Rhythmus, Tausch in beide Richtungen, Abreise nach einem Tag.
// Alle Werte aus data/economy.json › merchant und data/hub.json › merchant.

// merchantDays tickt n dawns weiter und liefert je Tag, ob der Händler da ist (Index = Tag).
func merchantDays(t *testing.T, w *World, n int) map[int]bool {
	t.Helper()
	present := map[int]bool{}
	for range n {
		toDawn(t, w)
		present[w.Cycle.Day] = w.Merchant != nil
	}
	return present
}

func merchantWorld(depth int) *World {
	w := newWorld(depth, "haendler", Options{})
	w.Troops, w.Camps, w.Pickups, w.Portals = []*Troop{}, []*Camp{}, []*Pickup{}, []float64{}
	return w
}

// (d) Ankunft am Tag laut Rhythmus, nicht davor, Abreise beim nächsten dawn; mit Taverne früher; nur in Tiefe 0.
func TestMerchantRhythmusUndAbreise(t *testing.T) {
	m := economy.Merchant
	w := merchantWorld(0)
	present := merchantDays(t, w, 2*m.EveryDays)
	for day := 2; day <= 2*m.EveryDays+1; day++ {
		want := day%m.EveryDays == 0 // bleibt StayDays = 1 Tag
		if present[day] != want {
			t.Errorf("ohne Taverne Tag %d: da=%v, erwartet %v", day, present[day], want)
		}
	}

	w = merchantWorld(0)
	buildSite(t, w, "tavern")
	present = merchantDays(t, w, m.EveryDaysWithTavern)
	if present[m.EveryDaysWithTavern-1] || !present[m.EveryDaysWithTavern] {
		t.Errorf("mit Taverne: %v, erwartet Ankunft an Tag %d", present, m.EveryDaysWithTavern)
	}

	w = merchantWorld(1)
	for day, da := range merchantDays(t, w, m.EveryDays) {
		if da {
			t.Errorf("Händler in Tiefe 1 an Tag %d", day)
		}
	}
}

// (d) Gleicher Seed → gleiches Material, nur aus den freigeschalteten.
func TestMerchantMaterialAusRngUndFreigeschaltet(t *testing.T) {
	m := economy.Merchant
	pick := func(level int) string {
		w := merchantWorld(0)
		w.HubLevel = level
		merchantDays(t, w, m.EveryDays-1) // bis zum Ankunftstag
		if w.Merchant == nil {
			t.Fatal("kein Händler")
		}
		return w.Merchant.Resource
	}
	if got := pick(1); got != m.Materials[0] {
		t.Errorf("Hub-Stufe 1: %s, erwartet %s", got, m.Materials[0])
	}
	a, b := pick(len(m.Materials)), pick(len(m.Materials))
	if a != b || !slices.Contains(m.Materials, a) {
		t.Errorf("gleicher Seed: %s und %s", a, b)
	}
}

// tradeWorld: Händler mit Material res, Spieler 0 am Zahlziel side0, Spieler 1 (falls side1 != "") an side1.
func tradeWorld(res, side0, side1 string) (*World, []*Player) {
	w := merchantWorld(0)
	w.Merchant = &Merchant{Resource: res, Leaves: 99}
	ps := []*Player{AddPlayer(w)}
	ps[0].X = merchantX(w, side0)
	if side1 != "" {
		ps = append(ps, AddPlayer(w))
		ps[1].X = merchantX(w, side1)
	}
	return w, ps
}

// (d) Kaufen: 10 × gold Münzen → 10 × material; Verkaufen umgekehrt; andere Materialien bleiben.
func TestMerchantTauschtInBeideRichtungen(t *testing.T) {
	m := economy.Merchant
	trades := 10
	w, ps := tradeWorld("stone", "buy", "")
	ps[0].Gold = trades * m.Gold
	steps(w, []PlayerCommand{{Pay: true}}, 30)
	if ps[0].Gold != 0 || w.Stock.Stone != trades*m.Material || w.Stock.Wood != 0 || w.Merchant.BuyPaid != 0 {
		t.Errorf("Kaufen: Gold %d, Stein %d, Holz %d, offen %d", ps[0].Gold, w.Stock.Stone, w.Stock.Wood, w.Merchant.BuyPaid)
	}

	w, ps = tradeWorld("stone", "sell", "")
	*w.Stock = Stock{Stone: trades * m.Material, Wood: 50}
	ps[0].Gold = 0
	steps(w, []PlayerCommand{{Pay: true}}, 30)
	if ps[0].Gold != trades*m.Gold || w.Stock.Stone != 0 || w.Stock.Wood != 50 || len(w.Coins) != 0 {
		t.Errorf("Verkaufen: Gold %d, Stein %d, Holz %d, Münzen %d", ps[0].Gold, w.Stock.Stone, w.Stock.Wood, len(w.Coins))
	}
}

// (d) Nach einem Tag ist der Händler weg; ein halber Kauf fällt als Münzen zu Boden.
func TestMerchantReistAbHalberKaufZuBoden(t *testing.T) {
	w, _ := tradeWorld("wood", "buy", "")
	w.Players = nil // niemand hebt die Münzen auf
	w.Merchant.Leaves = w.Cycle.Day + economy.Merchant.StayDays
	w.Merchant.BuyPaid = economy.Merchant.Gold - 1
	paid := w.Merchant.BuyPaid
	toDawn(t, w)
	if w.Merchant != nil || paid == 0 || len(w.Coins) != paid {
		t.Errorf("Abreise: Händler %v, offen %d, Münzen %d", w.Merchant, paid, len(w.Coins))
	}
}

// (e) Zwei Spieler tauschen gleichzeitig (einer kauft, einer verkauft); das Ergebnis hängt nicht von der Reihenfolge ab.
func TestMerchantZweiSpielerReihenfolgeEgal(t *testing.T) {
	m := economy.Merchant
	run := func(swap bool) (Stock, int) {
		sides := []string{"buy", "sell"}
		if swap {
			sides = []string{"sell", "buy"}
		}
		w, ps := tradeWorld("stone", sides[0], sides[1])
		*w.Stock = Stock{Stone: 5 * m.Material}
		for i, p := range ps {
			p.Gold = 0
			if sides[i] == "buy" {
				p.Gold = 5 * m.Gold
			}
		}
		steps(w, []PlayerCommand{{Pay: true}, {Pay: true}}, 20)
		return *w.Stock, ps[0].Gold + ps[1].Gold
	}
	s1, g1 := run(false)
	s2, g2 := run(true)
	// Am Ende ist alles verkauft: 5 Käufe (+5 × material) und 10 Verkäufe, zusammen 10 × gold in den Börsen.
	if s1 != s2 || g1 != g2 || g1 != 10*m.Gold || s1.Stone != 0 {
		t.Errorf("Reihenfolge: Vorrat %v/%v, Gold %d/%d", s1, s2, g1, g2)
	}
}
