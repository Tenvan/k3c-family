package sim

// Wirtschaftsstand einer Stufe für das Protokoll (B-323, W9.1): Lager-Maximum, Hub-Ausbau und Wartegrund „Gefahr“.
// Alles wird beim Lesen aus den bestehenden Regeln abgelesen (capacity, nextLevel, isDangerous), nichts gespeichert;
// die Welt-JSON und die Golden-Daten bleiben gleich. W5.1 übernimmt die Felder in `stateOf` (engine/net).

// Economy ist der abgeleitete Wirtschaftsstand einer Stufe.
type Economy struct {
	// StockMax ist das Maximum je Rohstoff der Insel; fehlt ohne Insel (unbegrenzt, island_storage.go).
	StockMax int `json:"stockMax,omitempty"`
	// HubLevel ist die Hub-Stufe (1 bis 5).
	HubLevel int `json:"hubLevel"`
	// HubUpgrade ist der Ausbau auf die nächste Hub-Stufe; fehlt auf der letzten Stufe (keine Kosten).
	HubUpgrade *HubUpgrade `json:"hubUpgrade,omitempty"`
	// Danger: Nacht oder Gegner da, Bau und Ausbau warten (`waitingWorker` heißt dann Gefahr, nicht „kein Bauer“).
	Danger bool `json:"danger,omitempty"`
}

// HubUpgrade sind Kosten und Stand des Ausbaus auf die nächste Hub-Stufe (hub_level.go).
type HubUpgrade struct {
	Gold     int    `json:"gold"`            // Goldkosten der nächsten Stufe
	Material Stock  `json:"material"`        // Materialkosten der nächsten Stufe
	Paid     int    `json:"paid"`            // schon bezahltes Gold
	State    string `json:"state,omitempty"` // waitingMaterial, waitingWorker; fehlt, solange Gold offen ist
}

// EconomyOf liest den Wirtschaftsstand der Stufe w ab.
func EconomyOf(w *World) Economy {
	e := Economy{HubLevel: w.HubLevel, Danger: isDangerous(w)}
	if limit, ok := capacity(w); ok {
		e.StockMax = limit
	}
	if h := w.hubSite; h != nil {
		if next := nextLevel(w, h); next != nil {
			e.HubUpgrade = &HubUpgrade{Gold: next.Cost.Gold, Material: materialOf(next.Cost), Paid: h.UpgradePaid, State: h.Upgrade}
		}
	}
	return e
}

// materialOf sind die Materialkosten ohne Gold.
func materialOf(c Cost) Stock {
	return Stock{Wood: c.Wood, Stone: c.Stone, Copper: c.Copper, Iron: c.Iron, Crystal: c.Crystal}
}
