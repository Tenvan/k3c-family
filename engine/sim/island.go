package sim

import "fmt"

// Insel (Entscheidung 003, B-100): Ein Level mit mehreren Stufen. Alle Stufen laufen im selben Takt weiter, auch ohne
// Spieler; jeder Spieler ist an genau eine Stufe gebunden. Das Baumaterial gehört der Insel (ein Vorrat für alle Stufen),
// Gold gehört dem Spieler. Die Insel steht neben der Campaign (campaign.go), die der Raum bis zur Umstellung (B-133)
// weiter benutzt.

// Island ist eine Insel mit ihren Stufen.
type Island struct {
	ID         string // gleiche ID = gleiches Spiel (Spielstand)
	Seed       string
	CycleSpeed float64
	// Stages: Stufe i ist die Welt der i-ten Tiefe aus CreateIsland.
	Stages []*World
	// Stock ist der Vorrat der Insel; alle Stufen zeigen auf denselben Wert.
	Stock *Stock
	// Options: Grad, Ziel, Niederlage-Modus (island_options.go).
	Options IslandOptions
	// SkillPool: Fund-Pool der Insel (monarch.md § 3), alle Stufen spiegeln ihn nach World.SkillPoints (addPoolPoints).
	SkillPool int
	// ChestsOpened zählt die geöffneten Truhen aller Stufen (jede n-te gibt einen Pool-Punkt).
	ChestsOpened int
	// DefeatedBosses: IDs der besiegten Bosse in Reihenfolge des Siegs (boss.go); ein besiegter Boss kommt nie wieder.
	DefeatedBosses []string
	// EndbossDefeated: Der Endboss der Insel ist besiegt (Siegvariante, Inselwechsel); endboss ist der laufende Kampf
	// gegen ihn (boss_endboss.go), nil solange er wartet oder nach dem Sieg.
	EndbossDefeated bool
	endboss         *endbossFight
	// GoldCollected: Münzen, die Spieler auf der Insel aufgehoben haben, jede Aufnahme zählt (Siegvariante gold,
	// victory.go). Won: Die Insel hat ihr Ziel erreicht, `victory` kam schon. hadFinite: Es stand ein endliches
	// Ressourcenobjekt (mineAll). Nur im Speicher, der Spielstand folgt mit K2.3b.
	GoldCollected int
	Won           bool
	hadFinite     bool
	// Number: Index der Insel in data/islands.json (0 = Insel 1); Scaling: ihre Tabelle der Gegner-Skalierung.
	// GateOpen: Wechselpunkt offen, SwitchReady: alle stehen dort, der Raum tauscht die Insel (island_switch.go).
	Number         int
	Scaling        struct{ HP, Damage, Speed float64 }
	GateOpen       bool
	SwitchReady    bool
	switchProgress float64
	// Over: Niederlage-Modus „Komplett verloren“ (defeat.go); die Insel ist zu Ende, StepIsland ändert nichts mehr.
	Over bool
	// MerchantVisits: Ankünfte des Händlers auf der Insel (merchant.go), Rhythmus des Händler-Überfalls (K3.2).
	MerchantVisits int
	nextPlayer     int
	// travel: Reisefortschritt je Spielerindex (island_travel.go); nur über die Stufen- und Spielerlisten iterieren.
	travel map[int]*islandTravel
}

// CreateIsland baut die Stufen für die angegebenen Tiefen (Reihenfolge = Stufenindex). CycleSpeed 0 bedeutet 1.
// Eine neue Insel startet mit dem Vorrat aus hub.json › islandStartStock; Spielstände setzen ihren eigenen (FromIslandSave).
func CreateIsland(seed string, depths []int, cycleSpeed float64) (*Island, error) {
	isl, err := createIsland(seed, depths, cycleSpeed, 0)
	if err != nil {
		return nil, err
	}
	*isl.Stock = hub.IslandStartStock
	return isl, nil
}

// createIsland wie CreateIsland, mit Startzeit (Laden eines Spielstands).
func createIsland(seed string, depths []int, cycleSpeed, startTime float64) (*Island, error) {
	if len(depths) == 0 {
		return nil, fmt.Errorf("insel: keine Stufen")
	}
	isl := &Island{ID: seed, Seed: seed, CycleSpeed: cycleSpeed, Stock: &Stock{}, Options: DefaultOptions(), travel: map[int]*islandTravel{}}
	isl.Scaling = islandDefs[0].DepthScaling
	for _, d := range depths {
		if !hasDepth(d) {
			return nil, fmt.Errorf("insel: unbekannte Tiefe %d", d)
		}
		w, err := CreateWorld(biomeForDepth(d), seed, Options{CycleSpeed: cycleSpeed, Time: startTime})
		if err != nil {
			return nil, fmt.Errorf("insel: Stufe mit Tiefe %d: %w", d, err)
		}
		w.Stock = isl.Stock
		w.island = isl
		for _, s := range hub.IslandSites {
			w.Sites = append(w.Sites, emptySite(w, s.Kind, w.HubX+s.OffsetUnits))
		}
		w.noTravel = true
		isl.Stages = append(isl.Stages, w)
	}
	return isl, nil
}

// AddIslandPlayer stellt einen neuen Monarchen an die Burg der Stufe `stage`. Der Index zählt inselweit hoch und
// bleibt auch nach einem Stufenwechsel derselbe (Eingaben werden über ihn zugeordnet).
func AddIslandPlayer(isl *Island, stage int) *Player {
	p := addPlayerAt(isl.Stages[stage], isl.nextPlayer)
	isl.nextPlayer++
	return p
}

// StepIsland rechnet einen Tick in allen Stufen, von Stufe 0 aufwärts. commands wird nach dem Spielerindex der Insel
// gelesen (`commands[p.Index]`); jede Stufe bekommt dasselbe Feld und nimmt sich ihre Spieler heraus.
func StepIsland(isl *Island, commands []PlayerCommand, dt float64) {
	if isl.Over { // Komplett verloren: alle Stufen stehen, keine Ereignisse mehr
		for _, w := range isl.Stages {
			w.Events = []Event{}
		}
		return
	}
	for _, w := range isl.Stages {
		Step(w, commands, dt)
	}
	if !isl.Over {
		stepIslandTravel(isl, dt)
		stepIslandSwitch(isl, dt)
		checkVictory(isl)
	}
	for i, w := range isl.Stages { // nach dem Wechsel, damit auch `arrived` seine Stufe trägt
		for _, ev := range w.Events {
			ev["stage"] = i
		}
	}
}
