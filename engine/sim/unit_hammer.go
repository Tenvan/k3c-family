package sim

// Handwerker und Hammer (docs/rules/buerger.md § 1, B-950, B-951): Nur ein Bauer mit Hammer baut, baut aus und repariert.
//   - Hammer (Troop.Hammer): Die Werkstatt (Handwerkerbedarf) stellt sie her (Preis buildings.json › workshop.hammerCost,
//     Regal hammerRack); ein Bauer ohne Hammer holt sie dort (fetchHammer) oder vom Boden (Drop hammer), mit demselben
//     Gegner-Abstand wie Waffen (weaponToFetch, B-929).
//   - Ein Handwerker holt weder Waffe noch zweiten Hammer (weaponToFetch); fällt er (Verlust), bleibt der Hammer als Drop
//     am Boden.
//   - Start: hub.json › startTroops.craftsman Handwerker zusätzlich zu den Bauern (je Welt übersteuerbar, Standard 1).
//   - Bögen stellt der Bogner her (Gebäude bowyer, Regal bowRack); Werkstatt und Bogner teilen den Mechanismus „Produkt
//     am Platz“ (Site.BowPaidGold, Site.CraftDone, building_upgrades.go).

// hammerKind ist die Art des Hammers als Drop und als Ausrüstung.
const hammerKind = "hammer"

// placeProductInfo beschreibt das Produkt am Platz eines Herstellers: Name in craftSeconds, Preis, Regalgröße und der
// Zähler im Regal (Hämmer der Werkstatt, Bögen des Bogners).
type placeProductInfo struct {
	name string
	cost Cost
	rack int
	have *int
}

func placeProduct(w *World, s *Site) placeProductInfo {
	b := w.values().Buildings[s.Kind]
	if s.Kind == "workshop" {
		return placeProductInfo{hammerKind, b.HammerCost, b.HammerRack, &s.Hammers}
	}
	return placeProductInfo{"bow", w.values().Troops["archer"].Cost, b.BowRack, &s.Bows}
}

// dropHammer lässt den Hammer von t am Boden liegen (nichts, wenn t keinen trägt).
func dropHammer(w *World, t *Troop) {
	if !t.Hammer {
		return
	}
	t.Hammer = false
	w.Drops = append(w.Drops, &Drop{ID: w.newID(), Kind: hammerKind, X: t.X})
}

// spawnStartPeasants stellt die Start-Bauern hin: startTroops.craftsman Handwerker mit Hammer, dazu startTroops.peasant
// Bauern ohne (die Waffen holen).
func spawnStartPeasants(w *World, hubX float64) {
	st := w.values().Hub.StartTroops
	hp := w.values().Troops["peasant"].HP
	for i := range st.Craftsman + st.Peasant {
		t := spawnVagrant(w, hubX, hubX+2+float64(i))
		t.Kind, t.HP, t.MaxHP = "peasant", hp, hp
		t.Hammer = i < st.Craftsman
	}
}
