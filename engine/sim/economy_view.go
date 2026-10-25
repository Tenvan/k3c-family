package sim

// Wirtschaftsstand einer Stufe für das Protokoll (B-323, W9.1): Lager-Maximum, Hub-Ausbau und Wartegrund „Gefahr“.
// Alles wird beim Lesen aus den bestehenden Regeln abgelesen (capacity, nextLevel, isDangerous), nichts gespeichert;
// die Welt-JSON und die Golden-Daten bleiben gleich. W5.1 übernimmt die Felder in `stateOf` (engine/net).

// Economy ist der abgeleitete Wirtschaftsstand einer Stufe.
type Economy struct {
	// StockMax ist das Maximum je Rohstoff der Insel; fehlt ohne Insel (unbegrenzt, island_storage.go).
	StockMax int `json:"stockMax,omitempty"`
	// PurseMax ist das Gold-Maximum je Spieler; fehlt ohne Dev-Bonus (dann gilt economy.purse.maxGold aus data/).
	PurseMax int `json:"purseMax,omitempty"`
	// HubLevel ist die Hub-Stufe (1 bis Anzahl der `levels` in data/hub.json).
	HubLevel int `json:"hubLevel"`
	// HubStep ist der Ausbauschritt, den HubUpgrade nennt (1 bis HubUpgrade.Steps, B-592); fehlt mit HubUpgrade.
	HubStep int `json:"hubStep,omitempty"`
	// HubUpgrade ist der Ausbau auf die nächste Hub-Stufe; fehlt auf der letzten Stufe (keine Kosten).
	HubUpgrade *HubUpgrade `json:"hubUpgrade,omitempty"`
	// Danger: Nacht oder Gegner da, Bau und Ausbau warten (`waitingWorker` heißt dann Gefahr, nicht „kein Bauer“).
	Danger bool `json:"danger,omitempty"`
	// Fighters ist die Zahl der Kämpfer der Stufe, TroopLimit ihr Truppen-Limit (building_barracks.go; B-332, W10.1, W10.2);
	// ohne omitempty, 0 Kämpfer ist ein Wert.
	Fighters   int `json:"fighters"`
	TroopLimit int `json:"troopLimit"`
}

// HubUpgrade sind Kosten und Stand des laufenden Ausbauschritts auf die nächste Hub-Stufe (hub_level.go).
type HubUpgrade struct {
	Gold     int    `json:"gold"`            // Goldkosten des Schritts
	Material Stock  `json:"material"`        // Materialkosten des Schritts samt Material-Tor
	Paid     int    `json:"paid"`            // schon bezahltes Gold
	State    string `json:"state,omitempty"` // waitingMaterial, waitingNextIsland, waitingWorker; fehlt, solange Gold offen ist
	Steps    int    `json:"steps"`           // Zahl der Ausbauschritte auf die nächste Hub-Stufe (B-592)
}

// WaitingNextIsland ist der Wartegrund „Material der nächsten Insel“ (B-592): Gold ist bezahlt, aber das Material-Tor
// fehlt im Vorrat. Die Simulation wartet wie bei waitingMaterial (stepUpgrade, Spielstand unverändert); nur der
// Zustand nennt den eigenen Grund, weil der Spieler dafür in die nächste Inselstufe muss statt hier abzubauen.
const WaitingNextIsland = "waitingNextIsland"

// EconomyOf liest den Wirtschaftsstand der Stufe w ab.
func EconomyOf(w *World) Economy {
	e := Economy{HubLevel: w.HubLevel, Danger: isDangerous(w), Fighters: fighters(w), TroopLimit: troopLimit(w)}
	if limit, ok := capacity(w); ok {
		e.StockMax = limit
	}
	if w.island != nil && w.island.Dev.StockBonus > 0 {
		e.PurseMax = purseMax(w)
	}
	if h := w.hubSite; h != nil {
		if next := nextLevel(w, h); next != nil {
			e.HubStep = max(1, w.HubStep)
			e.HubUpgrade = &HubUpgrade{Gold: next.Cost.Gold, Material: materialOf(next.Cost), Paid: h.UpgradePaid,
				State: hubWaitState(w, h, next.Cost), Steps: hubSteps(w)}
		}
	}
	return e
}

// hubWaitState ist der Wartegrund des Hub-Ausbaus: waitingNextIsland, solange mit bezahltem Gold ein Material des
// Material-Tors im Vorrat fehlt (need sind die Kosten des Schritts samt Tor), sonst der Stand des Platzes.
func hubWaitState(w *World, h *Site, need Cost) string {
	if h.Upgrade != "waitingMaterial" {
		return h.Upgrade
	}
	gate, have := hubGate(w), *w.Stock
	// je Material: Tor, Vorrat, Bedarf
	for _, m := range [][3]int{{gate.Wood, have.Wood, need.Wood}, {gate.Stone, have.Stone, need.Stone},
		{gate.Copper, have.Copper, need.Copper}, {gate.Iron, have.Iron, need.Iron}, {gate.Crystal, have.Crystal, need.Crystal}} {
		if m[0] > 0 && m[1] < m[2] {
			return WaitingNextIsland
		}
	}
	return h.Upgrade
}

// SitePayable: Nimmt der Bauplatz gerade Münzen für Bau oder Ausbau? Dieselbe Regel wie für `input.pay` (sitePayable),
// für `sites[].payable` im Zustand (B-587); nur gelesen, nichts gespeichert.
func SitePayable(w *World, s *Site) bool { return sitePayable(w, s) }

// NodeMarkable: Nimmt der Knoten gerade eine Markier-Münze? Dieselbe Regel wie für `input.pay` (nodeMarkable), für
// `nodes[].minable` im Zustand (B-588).
func NodeMarkable(n *ResourceNode) bool { return nodeMarkable(n) }

// materialOf sind die Materialkosten ohne Gold.
func materialOf(c Cost) Stock {
	return Stock{Wood: c.Wood, Stone: c.Stone, Copper: c.Copper, Iron: c.Iron, Crystal: c.Crystal}
}

// SiteGoldLeft ist das Gold, das der Platz bis zum vollen Preis noch nimmt (Bau, Ausbau oder Produkt wie in paySite),
// und ob er gerade Münzen nimmt (sitePayable). Nur gelesen, für die Bots des Balancing-Testers (BAL6), die so die
// Zahl-Regeln nicht nachbilden müssen.
func SiteGoldLeft(w *World, s *Site) (int, bool) {
	if !sitePayable(w, s) {
		return 0, false
	}
	switch {
	case s.State == "unpaid":
		return cheap(w, buildings[s.Kind].Cost).Gold - s.PaidGold, true
	case upgradePayable(w, s):
		return nextLevel(w, s).Cost.Gold - s.UpgradePaid, true
	}
	price := 0
	switch s.Kind {
	case "workshop":
		price = cheap(w, troops["archer"].Cost).Gold
	case "smithy":
		price = cheap(w, troops["eliteArcher"].Cost).Gold
	case "armory":
		stage, _ := armorStage(w)
		price = stage.Cost.Gold
	}
	return price - s.BowPaidGold, true
}
