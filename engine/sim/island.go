package sim

import (
	"fmt"
	"strconv"

	"k3c/engine/level"
)

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
	// island_victory.go). Won: Die Insel hat ihr Ziel erreicht, `victory` kam schon. hadFinite: Es stand ein endliches
	// Ressourcenobjekt (mineAll). Nur im Speicher, der Spielstand folgt mit K2.3b.
	GoldCollected int
	Won           bool
	hadFinite     bool
	// Number: Index der Insel in der Standard-Welt (data/worlds/standard.json) (0 = Insel 1); Scaling: ihre Tabelle der Gegner-Skalierung.
	// GateOpen: Wechselpunkt offen, SwitchReady: alle stehen dort, der Raum tauscht die Insel (island_switch.go).
	Number         int
	Scaling        struct{ HP, Damage, Speed float64 }
	GateOpen       bool
	SwitchReady    bool
	switchProgress float64
	// Over: Niederlage-Modus „Komplett verloren“ (island_defeat.go); die Insel ist zu Ende, StepIsland ändert nichts mehr.
	Over bool
	// Dev: Schalter der Dev-Aktionen (dev.go), nur im Speicher; der Raum trägt sie über den Inselwechsel.
	Dev DevFlags
	// MerchantVisits: Ankünfte des Händlers auf der Insel (merchant.go), Rhythmus des Händler-Überfalls (K3.2).
	MerchantVisits int
	nextPlayer     int
	// travel: Reisefortschritt je Spielerindex (island_travel.go); nur über die Stufen- und Spielerlisten iterieren.
	travel map[int]*islandTravel
}

// CreateOptions sind die Parameter der Erzeugung einer Insel (B-461, B-462). Sie gelten nur für die Erzeugung und werden
// NICHT gespeichert (Spielstand-Format und IslandSaveVersion bleiben); getrennt von IslandOptions (Grad, Ziel, Niederlage).
// Laden eines Spielstands und Inselwechsel nutzen die eingebetteten Daten und die Planung der Insel.
type CreateOptions struct {
	// Planned sind die geplanten Chunks je Tiefe (docs/rules/geplante-chunks.md). nil = die Planung der eingebetteten
	// ersten Insel der Standard-Welt (data/worlds/standard.json); eine leere Map schaltet die Planung ab.
	Planned map[int][]level.PlannedChunk
	// Candidate: Kandidaten-hub.json (Gebäude-Planer, B-462); nil = eingebettete data/hub.json. Plätze, Hub-Stufen,
	// Mauerlinien, Tor-Stufe, Händler und Startvorrat kommen dann aus ihm. CreateIsland prüft ihn vorher mit
	// ValidateHubData und lehnt ihn bei Befunden ab (fail closed).
	Candidate *HubData
	// ChunkCounts sind die Chunk-Anzahlen je Tiefe (Inselgrößen, geplante-chunks.md § 7). nil = die der eingebetteten
	// ersten Insel; ohne Eintrag gilt die Länge des Bioms. CreateIsland lehnt eine Anzahl ab, die keine Inselgröße ist.
	ChunkCounts map[int]int
}

// plannedFor liefert die geplanten Chunks der Tiefe.
func (o CreateOptions) plannedFor(depth int) []level.PlannedChunk {
	if o.Planned != nil {
		return o.Planned[depth]
	}
	return standardWorld.Islands[0].plannedAt(depth)
}

// countFor liefert die Chunk-Anzahl der Tiefe (0 = Länge des Bioms).
func (o CreateOptions) countFor(depth int) int {
	if o.ChunkCounts != nil {
		return o.ChunkCounts[depth]
	}
	return standardWorld.Islands[0].countAt(depth)
}

// plannedAt liefert die geplanten Chunks der Insel für eine Tiefe (leer = reiner Zufall).
func (d IslandDef) plannedAt(depth int) []level.PlannedChunk { return d.PlannedChunks[strconv.Itoa(depth)] }

// CreateOptions liefert die Erzeugungs-Parameter dieser Insel aus den Daten (ihre eigene Planung und Chunk-Anzahl).
func (d IslandDef) CreateOptions() CreateOptions {
	planned, counts := map[int][]level.PlannedChunk{}, map[int]int{}
	for _, depth := range d.Depths {
		if plan := d.plannedAt(depth); len(plan) > 0 {
			planned[depth] = plan
		}
		if n := d.countAt(depth); n > 0 {
			counts[depth] = n
		}
	}
	return CreateOptions{Planned: planned, ChunkCounts: counts}
}

// CreateIsland baut die Stufen für die angegebenen Tiefen (Reihenfolge = Stufenindex). CycleSpeed 0 bedeutet 1.
// Eine neue Insel startet mit dem Vorrat aus hub.json › islandStartStock; Spielstände setzen ihren eigenen (FromIslandSave).
func CreateIsland(seed string, depths []int, cycleSpeed float64, opts CreateOptions) (*Island, error) {
	h := &hub
	if opts.Candidate != nil {
		if r := ValidateHubData(opts.Candidate); len(r.Findings) > 0 {
			return nil, fmt.Errorf("insel: Kandidaten-hub.json abgelehnt: %s", r.Findings[0])
		}
		h = cloneHub(opts.Candidate) // geprüft wird die Kopie, die die Welt behält: spätere Änderungen des Aufrufers wirken nicht
		opts.Candidate = h
	}
	isl, err := createIsland(seed, depths, cycleSpeed, 0, opts)
	if err != nil {
		return nil, err
	}
	*isl.Stock = h.IslandStartStock
	return isl, nil
}

// createIsland wie CreateIsland, mit Startzeit (Laden eines Spielstands).
func createIsland(seed string, depths []int, cycleSpeed, startTime float64, opts CreateOptions) (*Island, error) {
	if len(depths) == 0 {
		return nil, fmt.Errorf("insel: keine Stufen")
	}
	isl := &Island{ID: seed, Seed: seed, CycleSpeed: cycleSpeed, Stock: &Stock{}, Options: DefaultOptions(), travel: map[int]*islandTravel{}}
	isl.Scaling = standardWorld.Islands[0].DepthScaling
	for _, d := range depths {
		if !hasDepth(d) {
			return nil, fmt.Errorf("insel: unbekannte Tiefe %d", d)
		}
		count := opts.countFor(d)
		if count != 0 {
			if err := checkChunkCount(count); err != nil {
				return nil, fmt.Errorf("insel: Stufe mit Tiefe %d: %w", d, err)
			}
		}
		w, err := CreateWorld(biomeForDepth(d), seed, Options{CycleSpeed: cycleSpeed, Time: startTime, Planned: opts.plannedFor(d), ChunkCount: count, hub: opts.Candidate})
		if err != nil {
			return nil, fmt.Errorf("insel: Stufe mit Tiefe %d: %w", d, err)
		}
		w.Stock = isl.Stock
		w.island = isl
		for _, s := range hubOf(w).IslandSites {
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
		mirrorSwitch(isl)
		checkVictory(isl)
	}
	for _, w := range isl.Stages {
		mirrorState(w)
	}
	for i, w := range isl.Stages { // nach dem Wechsel, damit auch `arrived` seine Stufe trägt
		for _, ev := range w.Events {
			ev["stage"] = i
		}
	}
}
