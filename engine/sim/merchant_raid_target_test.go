package sim

import (
	"encoding/json"
	"testing"
)

// K3.2a (B-373, Sprint K3 AC-04, AC-05): Der Händler ist eine angreifbare Figur, die Insel zählt seine Besuche,
// beides übersteht Speichern und Laden.

// merchantArrives stellt die Stufe auf den dawn von Tag day und lässt den Händler nach Rhythmus kommen oder gehen.
func merchantArrives(w *World, day int) {
	w.Cycle = CycleInfo{Phase: "day", Day: day}
	merchantDawn(w)
}

// (a) Jede Ankunft zählt einen Besuch, ohne und mit Taverne.
func TestHaendlerBesucheZaehlen(t *testing.T) {
	isl := mustIsland(t, "haendler", []int{0, 1})
	w := isl.Stages[0]
	for day := 1; day <= 3*economy.Merchant.EveryDays; day++ {
		merchantArrives(w, day)
	}
	if isl.MerchantVisits != 3 {
		t.Fatalf("drei Ankünfte ohne Taverne erwartet, %d", isl.MerchantVisits)
	}
	buildSite(t, w, "tavern")
	for day := 3*economy.Merchant.EveryDays + 1; isl.MerchantVisits < 5 && day < 100; day++ {
		merchantArrives(w, day)
	}
	if isl.MerchantVisits != 5 || w.Merchant == nil || w.Merchant.HP != economy.Merchant.HP {
		t.Fatalf("mit Taverne fünf Besuche und Händler mit %v HP erwartet: %d, %+v", economy.Merchant.HP, isl.MerchantVisits, w.Merchant)
	}
}

// (b) Ein Gegner in Reichweite greift den Händler an; ohne Händler ist er kein Ziel.
func TestHaendlerWirdAngegriffen(t *testing.T) {
	w := merchantWorld(0)
	merchantArrives(w, economy.Merchant.EveryDays)
	x := merchantX(w, "buy")
	e := spawnEnemy(w, "wolf", x)
	if c := chooseTarget(w, e, 1, nil); c == nil || c.kind != "merchant" {
		t.Fatalf("Händler als Ziel erwartet, %+v", c)
	}
	attack(w, e, chooseTarget(w, e, 1, nil))
	if w.Merchant.HP != economy.Merchant.HP-e.Damage {
		t.Fatalf("Händler-HP %v, erwartet %v", w.Merchant.HP, economy.Merchant.HP-e.Damage)
	}
	w.Merchant = nil
	if c := chooseTarget(w, e, 1, nil); c != nil && c.kind == "merchant" {
		t.Fatal("ohne Händler kein Händler-Ziel")
	}
}

// (c) Bei 0 HP flieht der Händler (merchantFled), ein halber Kauf fällt zu Boden; der nächste Besuch kommt normal.
func TestHaendlerFliehtBeiNullHP(t *testing.T) {
	isl := mustIsland(t, "haendler", []int{0})
	w := isl.Stages[0]
	day := economy.Merchant.EveryDays
	merchantArrives(w, day)
	w.Merchant.BuyPaid = 2
	coins := len(w.Coins)
	w.Events = []Event{}
	applyDamageBy(w, merchantID, economy.Merchant.HP, "wolf")
	if w.Merchant != nil || count(w.Events, "merchantFled") != 1 || len(w.Coins) != coins+2 {
		t.Fatalf("Flucht mit zwei Münzen erwartet: Händler %+v, Ereignisse %v", w.Merchant, w.Events)
	}
	merchantArrives(w, 2*day)
	if w.Merchant == nil || w.Merchant.HP != economy.Merchant.HP || isl.MerchantVisits != 2 {
		t.Fatalf("nächster Besuch mit vollen HP erwartet: %+v, Besuche %d", w.Merchant, isl.MerchantVisits)
	}
}

// (d) Besuchszähler und anwesender Händler mit HP überstehen Speichern und Laden; Version 5 lädt mit 0 ohne Händler.
func TestHaendlerImSpielstand(t *testing.T) {
	isl := mustIsland(t, "haendler", []int{0, 1})
	AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	merchantArrives(w, economy.Merchant.EveryDays)
	w.Merchant.HP = 40
	raw, err := json.Marshal(isl.ToSave("2026-10-09T20:00:00Z"))
	if err != nil {
		t.Fatal(err)
	}
	loaded := loadIsland(t, raw)
	m := loaded.Stages[0].Merchant
	if loaded.MerchantVisits != 1 || m == nil || m.HP != 40 || *m != *w.Merchant {
		t.Fatalf("Besuche %d, Händler %+v, erwartet 1 und %+v", loaded.MerchantVisits, m, w.Merchant)
	}
	if loaded.Stages[1].Merchant != nil {
		t.Fatal("Händler nur in seiner Tiefe")
	}
	old := loadIsland(t, readFixture(t, 5))
	if old.MerchantVisits != 0 || old.Stages[0].Merchant != nil {
		t.Fatalf("v5: Besuche %d, Händler %+v", old.MerchantVisits, old.Stages[0].Merchant)
	}
	v6 := loadIsland(t, readFixture(t, 6))
	if v6.MerchantVisits != 3 || v6.Stages[0].Merchant == nil || v6.Stages[0].Merchant.HP != 60 {
		t.Fatalf("v6-Fixture: Besuche %d, Händler %+v", v6.MerchantVisits, v6.Stages[0].Merchant)
	}
}

// Ungültige Händler-Werte im Spielstand werden abgelehnt.
func TestHaendlerSpielstandPrueft(t *testing.T) {
	s, err := ParseIslandSave(readFixture(t, 6))
	if err != nil {
		t.Fatal(err)
	}
	s.Merchant.HP = 0
	if validateIslandSave(s) == nil {
		t.Error("Händler mit 0 HP im Stand wird abgelehnt")
	}
	s.Merchant, s.MerchantVisits = nil, -1
	if validateIslandSave(s) == nil {
		t.Error("negativer Besuchszähler wird abgelehnt")
	}
}
