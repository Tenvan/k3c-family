package sim

import (
	"bytes"
	"slices"
	"strings"
	"testing"
)

// Spielstand Version 7 (W7.2, B-201, AC-04): Plantage, Berufe, Krieger, Elite, Rüstung und Schwerter überleben
// Speichern und Laden; eine unbekannte Truppenart ist ein Fehler.

// setEconomy stellt die Wirtschaft her: Stufe 0 mit Farm samt Plantage, Werkstatt, Schmiede, Rüstkammer (Stufe 1 der
// Rüstung), Bergmann, Handwerker, Bogenschütze, Krieger und beiden Elite-Figuren; Stufe 1 mit Krieger und Baumeister.
func setEconomy(t *testing.T, isl *Island) {
	t.Helper()
	w, w1 := isl.Stages[0], isl.Stages[1]
	farm, shop := builtSite(t, w, "farm"), builtSite(t, w, "workshop")
	smithy, armory := builtSite(t, w, "smithy"), builtSite(t, w, "armory")
	stepPlantations(w, 31) // alle Plätze tragen einen Baum
	cut := farm.plots[0].tree
	w.Nodes = slices.DeleteFunc(w.Nodes, func(n *ResourceNode) bool { return n == cut })
	farm.plots[0] = plot{growIn: 12.5} // einer abgebaut, der nächste wächst
	shop.Swords, shop.SwordPaidGold, shop.SwordDone = 2, 1, 20.5
	smithy.CraftDone, smithy.OfferPaid, smithy.OfferDone = 40.5, []int{3}, 50.5
	armory.BowPaidGold, armory.CraftDone = 70, 60.5
	w.ArmorLevel = 1
	peasant := func(w *World, prof string, site *Site) {
		tr := spawnVagrant(w, w.HubX, w.HubX+1)
		tr.Kind, tr.HP, tr.MaxHP, tr.Profession = "peasant", troops["peasant"].HP, troops["peasant"].HP, prof
		if site != nil {
			tr.WorkSite = site.ID
		}
	}
	peasant(w, "miner", nil)
	peasant(w, "craftsman", shop)
	for _, kind := range []string{"archer", "warrior", "eliteArcher", "eliteWarrior"} {
		makeFighter(w, spawnVagrant(w, w.HubX, w.HubX+2), kind)
	}
	peasant(w1, "builder", nil)
	makeFighter(w1, spawnVagrant(w1, w1.HubX, w1.HubX+2), "warrior")
}

func economyIsland(t *testing.T) *Island {
	t.Helper()
	isl := mustIsland(t, "familie", []int{0, 1})
	AddIslandPlayer(isl, 0).Gold, AddIslandPlayer(isl, 1).Gold = 23, 8
	setEconomy(t, isl)
	return isl
}

// added sind die Truppen, die setEconomy zur Startbesetzung (Bauer, zwei Bogenschützen) dazugibt.
func added(w *World) []*Troop {
	out := []*Troop{}
	for _, tr := range w.Troops {
		if tr.Kind != "vagrant" {
			out = append(out, tr)
		}
	}
	return out[3:]
}

func troopKinds(w *World) []string {
	out := []string{}
	for _, tr := range added(w) {
		out = append(out, tr.Kind+":"+tr.Profession)
	}
	return out
}

// checkEconomy prüft den Zustand aus setEconomy an der geladenen Insel isl.
func checkEconomy(t *testing.T, isl *Island) {
	t.Helper()
	w, w1 := isl.Stages[0], isl.Stages[1]
	farm, shop := siteByKey(w, key("farm", findKind(t, w, "farm"))), siteByKey(w, key("workshop", findKind(t, w, "workshop")))
	smithy, armory := siteByKey(w, key("smithy", findKind(t, w, "smithy"))), siteByKey(w, key("armory", findKind(t, w, "armory")))
	checkPlots(t, farm)
	if shop.Swords != 2 || shop.SwordPaidGold != 1 || shop.SwordDone != 20.5 {
		t.Errorf("Schwerter: %d/%d/%v", shop.Swords, shop.SwordPaidGold, shop.SwordDone)
	}
	if smithy.CraftDone != 40.5 || !slices.Equal(smithy.OfferPaid, []int{3}) || smithy.OfferDone != 50.5 {
		t.Errorf("Elite: %v/%v/%v", smithy.CraftDone, smithy.OfferPaid, smithy.OfferDone)
	}
	if armory.BowPaidGold != 70 || armory.CraftDone != 60.5 || w.ArmorLevel != 1 || w1.ArmorLevel != 0 {
		t.Errorf("Rüstung: %d/%v, Stufe %d/%d", armory.BowPaidGold, armory.CraftDone, w.ArmorLevel, w1.ArmorLevel)
	}
	checkTroops(t, w, w1, shop)
}

func checkPlots(t *testing.T, farm *Site) {
	t.Helper()
	trees := 0
	for i, p := range farm.plots {
		if p.tree != nil {
			trees++
		} else if i != 0 || p.growIn != 12.5 {
			t.Errorf("Plantage Platz %d: kein Baum, growIn %v", i, p.growIn)
		}
	}
	if trees != 5 || len(farm.plots) != 6 {
		t.Errorf("Plantage: %d Bäume auf %d Plätzen", trees, len(farm.plots))
	}
}

func checkTroops(t *testing.T, w, w1 *World, shop *Site) {
	t.Helper()
	want := []string{"peasant:miner", "peasant:craftsman", "archer:", "warrior:", "eliteArcher:", "eliteWarrior:"}
	if got := troopKinds(w); !slices.Equal(got, want) || !slices.Equal(troopKinds(w1), []string{"peasant:builder", "warrior:"}) {
		t.Errorf("Truppen %v / %v", got, troopKinds(w1))
	}
	if craftsman := added(w)[1]; craftsman.WorkSite != shop.ID || added(w)[0].WorkSite != 0 {
		t.Errorf("Arbeitsplatz des Handwerkers %d, Werkstatt %d", craftsman.WorkSite, shop.ID)
	}
	if elite := added(w)[5]; elite.MaxHP != armoredHP(w, "eliteWarrior") || elite.MaxHP <= troops["eliteWarrior"].HP {
		t.Errorf("Elite-Krieger ohne Rüstung: %v", elite.MaxHP)
	}
}

func findKind(t *testing.T, w *World, kind string) float64 {
	t.Helper()
	for _, s := range w.Sites {
		if s.Kind == kind {
			return s.X
		}
	}
	t.Fatalf("kein Platz %s", kind)
	return 0
}

// AC-04: Rundlauf mit zwei Spielern: Speichern, Laden, Speichern ist gleich, und die geladene Insel hat alle Zustände.
func TestSpielstandWirtschaftRundlauf(t *testing.T) {
	isl := economyIsland(t)
	first := saveJSON(t, isl.ToSave("2026-10-13T20:00:00Z"))
	loaded := loadIsland(t, first)
	if second := saveJSON(t, loaded.ToSave("2026-10-13T20:00:00Z")); !bytes.Equal(first, second) {
		t.Fatalf("Speichern, Laden, Speichern ergibt etwas anderes:\n%s\n%s", first, second)
	}
	if len(loaded.Players()) != 2 {
		t.Fatalf("%d Spieler", len(loaded.Players()))
	}
	checkEconomy(t, loaded)
}

// AC-04: Das Fixture v7 trägt die Wirtschaft und die Felder aus v5 und v6 (Boss, Händler).
func TestSpielstandWirtschaftFixture(t *testing.T) {
	loaded := loadIsland(t, readFixture(t, 7))
	checkEconomy(t, loaded)
	if loaded.MerchantVisits != 3 || loaded.Stages[0].Merchant == nil {
		t.Fatalf("Händler aus v6 fehlt: %d, %+v", loaded.MerchantVisits, loaded.Stages[0].Merchant)
	}
	for v := 1; v < 7; v++ { // ältere Stände laden ohne Wirtschaft: keine Rüstung, kein Beruf
		for _, w := range loadIsland(t, readFixture(t, v)).Stages {
			if w.ArmorLevel != 0 || slices.ContainsFunc(w.Troops, func(tr *Troop) bool { return tr.Profession != "" }) {
				t.Fatalf("v%d: Rüstung %d", v, w.ArmorLevel)
			}
		}
	}
}

// AC-04: Eine unbekannte Truppenart oder ein unbekannter Beruf ist ein Fehler, nie ein Landstreicher.
func TestSpielstandUnbekannteTruppenartFehler(t *testing.T) {
	cases := map[string]TroopSave{
		"Art":         {Kind: "dragon"},
		"Landstreich": {Kind: "vagrant"},
		"Beruf":       {Kind: "peasant", Profession: "wizard"},
		"Beruf Krieg": {Kind: "warrior", Profession: "miner"},
	}
	for name, ts := range cases {
		s, err := ParseIslandSave(readFixture(t, IslandSaveVersion))
		if err != nil {
			t.Fatal(err)
		}
		s.Stages[0].Troops = append(s.Stages[0].Troops, ts)
		if _, err := FromIslandSave(s, islandSpeed); err == nil || !strings.HasPrefix(err.Error(), "spielstand:") {
			t.Errorf("%s: Fehler %v", name, err)
		}
	}
	s, _ := ParseIslandSave(readFixture(t, IslandSaveVersion))
	s.Stages[0].ArmorLevel = 9
	if _, err := FromIslandSave(s, islandSpeed); err == nil {
		t.Error("Rüstungsstufe 9: kein Fehler")
	}
}
