package sim

import (
	"encoding/json"
	"testing"
)

// Händler-Überfall (K3.2, B-131/AC-01, AC-02).

// raidIsland ist eine Insel (Oberwelt und Höhle) mit zwei Spielern und leerem Vorrat.
func raidIsland(t *testing.T) *Island {
	t.Helper()
	isl := mustIsland(t, "ueberfall", []int{0, 1})
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	return isl
}

// nextVisit tickt dawns bis zur nächsten Ankunft des Händlers und liefert die Ereignisse dieses dawn.
func nextVisit(t *testing.T, w *World, day *int) []Event {
	t.Helper()
	for range 100 {
		*day++
		w.Events = []Event{}
		merchantArrives(w, *day)
		if count(w.Events, "merchantArrived") == 1 {
			return w.Events
		}
	}
	t.Fatal("kein Händler-Besuch")
	return nil
}

func raidStarted(events []Event) bool {
	for _, e := range events {
		if e["type"] == "eventStarted" && e["event"] == "merchantRaid" {
			return true
		}
	}
	return false
}

// (a) Besuch 1 bis 3 ohne Überfall, Besuch 4 mit, Besuch 8 wieder; der Rhythmus kommt aus den Daten.
func TestHaendlerUeberfallJederVierteBesuch(t *testing.T) {
	isl := raidIsland(t)
	w, day := isl.Stages[0], 0
	every := merchantRaid().EveryVisits
	for visit := 1; visit <= 2*every; visit++ {
		queued := len(w.SpawnQueue)
		events := nextVisit(t, w, &day)
		want := visit%every == 0
		if raidStarted(events) != want || w.Merchant.Raid != want || (len(w.SpawnQueue) > queued) != want {
			t.Errorf("Besuch %d: Überfall %v erwartet (Ereignis %v, Raid %v, Welle %d → %d)", visit, want,
				raidStarted(events), w.Merchant.Raid, queued, len(w.SpawnQueue))
		}
	}
}

// (b) Ohne Händler-Besuch kein Überfall: nicht in der Höhle, nicht ohne Insel, nicht an Tagen ohne Ankunft.
func TestHaendlerUeberfallNurBeiBesuch(t *testing.T) {
	isl := raidIsland(t)
	isl.MerchantVisits = merchantRaid().EveryVisits - 1
	cave := isl.Stages[1]
	merchantArrives(cave, economy.Merchant.EveryDays)
	if raidStarted(cave.Events) || cave.Merchant != nil {
		t.Error("in der Höhle kein Händler und kein Überfall")
	}
	w := isl.Stages[0]
	w.Events = []Event{}
	merchantArrives(w, economy.Merchant.EveryDays+1) // kein Ankunftstag
	if raidStarted(w.Events) {
		t.Error("ohne Ankunft kein Überfall")
	}
	single := merchantWorld(0)
	for day := 1; day <= 4*economy.Merchant.EveryDays; day++ {
		merchantArrives(single, day)
		if raidStarted(single.Events) {
			t.Fatal("ohne Insel kein Überfall (Golden-Läufe bleiben gleich)")
		}
	}
}

// startRaid bringt die Insel bis zur Ankunft des überfallenen Besuchs und liefert den Tag.
func startRaid(t *testing.T, isl *Island) int {
	t.Helper()
	isl.MerchantVisits = merchantRaid().EveryVisits - 1
	day := 0
	nextVisit(t, isl.Stages[0], &day)
	if !isl.Stages[0].Merchant.Raid {
		t.Fatal("Überfall erwartet")
	}
	return day
}

// (c) Der Händler überlebt bis zur Abreise: rewardMaterial seines Materials im Vorrat, eventEnded mit protected.
func TestHaendlerUeberfallBelohnungBeiSchutz(t *testing.T) {
	isl := raidIsland(t)
	day := startRaid(t, isl)
	w := isl.Stages[0]
	res := w.Merchant.Resource
	before := *stockField(isl.Stock, res)
	w.Events = []Event{}
	merchantArrives(w, day+economy.Merchant.StayDays)
	limit, capped := capacity(w)
	want := merchantRaid().RewardMaterial
	if capped {
		want = min(want, limit-before)
	}
	if got := *stockField(isl.Stock, res) - before; got != want || want == 0 {
		t.Fatalf("Belohnung %d %s erwartet, %d", want, res, got)
	}
	for _, e := range w.Events {
		if e["type"] == "eventEnded" && e["event"] == "merchantRaid" && e["protected"] == true && e["amount"] == want {
			return
		}
	}
	t.Fatalf("eventEnded mit protected und amount %d fehlt: %v", want, w.Events)
}

// (d) Der Händler stirbt: keine Belohnung, eventEnded ohne Schutz.
func TestHaendlerUeberfallTodOhneBelohnung(t *testing.T) {
	isl := raidIsland(t)
	startRaid(t, isl)
	w := isl.Stages[0]
	stock := *isl.Stock
	w.Events = []Event{}
	applyDamageBy(w, merchantID, w.Merchant.HP, "wolf")
	if *isl.Stock != stock || w.Merchant != nil {
		t.Fatalf("keine Belohnung und Flucht erwartet: Vorrat %+v → %+v", stock, *isl.Stock)
	}
	for _, e := range w.Events {
		if e["type"] == "eventEnded" && e["protected"] == false {
			return
		}
	}
	t.Fatalf("eventEnded ohne Schutz fehlt: %v", w.Events)
}

// Gegner greifen im Überfall zuerst den Händler an, auch wenn ein Spieler näher steht.
func TestHaendlerUeberfallGegnerWaehlenHaendler(t *testing.T) {
	isl := raidIsland(t)
	startRaid(t, isl)
	w := isl.Stages[0]
	x := merchantX(w, "buy")
	w.Players[0].X = x + 0.5
	e := spawnEnemy(w, "goblin", x+0.5)
	if c := chooseTarget(w, e, 1, nil); c == nil || c.kind != "merchant" {
		t.Fatalf("Händler als Ziel im Überfall erwartet, %+v", c)
	}
}

// (e) Zwei Spieler, gleicher Seed: der Überfall läuft gleich ab.
func TestHaendlerUeberfallDeterministisch(t *testing.T) {
	run := func() string {
		isl := raidIsland(t)
		startRaid(t, isl)
		runIsland(isl, nil, 30)
		raw, _ := json.Marshal(isl.Stages[0])
		return string(raw)
	}
	if first, second := run(), run(); first != second {
		t.Fatal("gleicher Seed, anderer Verlauf")
	}
}

// (f) Der laufende Überfall übersteht Speichern und Laden.
func TestHaendlerUeberfallImSpielstand(t *testing.T) {
	isl := raidIsland(t)
	startRaid(t, isl)
	raw, err := json.Marshal(isl.ToSave("2026-10-09T21:00:00Z"))
	if err != nil {
		t.Fatal(err)
	}
	loaded := loadIsland(t, raw)
	if m := loaded.Stages[0].Merchant; m == nil || !m.Raid || loaded.MerchantVisits != merchantRaid().EveryVisits {
		t.Fatalf("Überfall nach dem Laden: %+v, Besuche %d", m, loaded.MerchantVisits)
	}
}
