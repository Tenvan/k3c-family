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
	// world: die Welt der Insel (welten.md, B-488); islandID: ihre Insel-ID darin (Spielstand, welten.md § 5).
	// Number: Index der Insel in ihrer Welt (0 = Insel 1); Scaling: ihre Tabelle der Gegner-Skalierung.
	// GateOpen: Wechselpunkt offen, SwitchReady: alle stehen dort, der Raum tauscht die Insel (island_switch.go).
	world          WorldDef
	islandID       string
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
// nicht als Ganzes gespeichert; der Spielstand hält je Stufe nur die verwendete Planung und Chunk-Anzahl (HubSave, B-489) und
// die Welt-ID (welten.md § 5); getrennt von IslandOptions (Grad, Ziel, Niederlage). Das Laden baut die Stufen mit dem
// Gespeicherten (savedCreateOptions), ein Inselwechsel nutzt die Planung der neuen Insel.
type CreateOptions struct {
	// Planned sind die geplanten Chunks je Tiefe (docs/rules/geplante-chunks.md). nil = die Planung der ersten Insel
	// der Welt (ohne Welt: Standard-Welt, data/worlds/standard.json); eine leere Map schaltet die Planung ab.
	Planned map[int][]level.PlannedChunk
	// Candidate: Kandidaten-hub.json (Gebäude-Planer, B-462); nil = eingebettete data/hub.json. Plätze, Hub-Stufen,
	// Mauerlinien, Tor-Stufe, Händler und Startvorrat kommen dann aus ihm. CreateIsland prüft ihn vorher mit
	// ValidateHubData und lehnt ihn bei Befunden ab (fail closed).
	Candidate *HubData
	// ChunkCounts sind die Chunk-Anzahlen je Tiefe (Inselgrößen, geplante-chunks.md § 7). nil = die der ersten Insel
	// der Welt; ohne Eintrag gilt die Länge des Bioms. CreateIsland lehnt eine Anzahl ab, die keine Inselgröße ist.
	ChunkCounts map[int]int
}

// plannedFor liefert die geplanten Chunks der Tiefe; ohne Planned die der Insel def.
func (o CreateOptions) plannedFor(depth int, def IslandDef) []level.PlannedChunk {
	if o.Planned != nil {
		return o.Planned[depth]
	}
	return def.plannedAt(depth)
}

// countFor liefert die Chunk-Anzahl der Tiefe (0 = Länge des Bioms); ohne ChunkCounts die der Insel def.
func (o CreateOptions) countFor(depth int, def IslandDef) int {
	if o.ChunkCounts != nil {
		return o.ChunkCounts[depth]
	}
	return def.countAt(depth)
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

// CreateIsland baut die erste Insel der Standard-Welt (CreateIslandIn).
func CreateIsland(seed string, depths []int, cycleSpeed float64, opts CreateOptions) (*Island, error) {
	return CreateIslandIn(standardWorld, seed, depths, cycleSpeed, opts)
}

// CreateIslandIn baut die erste Insel der Welt world mit den Stufen für die angegebenen Tiefen (Reihenfolge =
// Stufenindex). CycleSpeed 0 bedeutet 1. Eine neue Insel startet mit dem Vorrat aus hub.json › islandStartStock;
// Spielstände setzen ihren eigenen (FromIslandSaveIn).
func CreateIslandIn(world WorldDef, seed string, depths []int, cycleSpeed float64, opts CreateOptions) (*Island, error) {
	h := &hub
	if opts.Candidate != nil {
		if r := ValidateHubDataIn(opts.Candidate, world); len(r.Findings) > 0 {
			return nil, fmt.Errorf("insel: Kandidaten-hub.json abgelehnt: %s", r.Findings[0])
		}
		h = cloneHub(opts.Candidate) // geprüft wird die Kopie, die die Welt behält: spätere Änderungen des Aufrufers wirken nicht
		opts.Candidate = h
	}
	isl, err := createIsland(seed, depths, cycleSpeed, 0, opts, world)
	if err != nil {
		return nil, err
	}
	*isl.Stock = h.IslandStartStock
	return isl, nil
}

// createIsland wie CreateIslandIn, mit Startzeit (Laden eines Spielstands). Die Insel ist zunächst die erste der Welt;
// Laden und Inselwechsel setzen danach Number, islandID und Scaling.
func createIsland(seed string, depths []int, cycleSpeed, startTime float64, opts CreateOptions, world WorldDef) (*Island, error) {
	switch {
	case len(depths) == 0:
		return nil, fmt.Errorf("insel: keine Stufen")
	case len(world.Islands) == 0:
		return nil, fmt.Errorf("insel: Welt %q ohne Insel", world.ID)
	}
	first := world.Islands[0]
	isl := &Island{ID: seed, Seed: seed, CycleSpeed: cycleSpeed, Stock: &Stock{}, Options: DefaultOptions(), travel: map[int]*islandTravel{}}
	isl.world, isl.islandID, isl.Scaling = world, first.ID, first.DepthScaling
	for _, d := range depths {
		if !hasDepth(d) {
			return nil, fmt.Errorf("insel: unbekannte Tiefe %d", d)
		}
		count := opts.countFor(d, first)
		if count != 0 {
			if err := CheckChunkCount(count); err != nil {
				return nil, fmt.Errorf("insel: Stufe mit Tiefe %d: %w", d, err)
			}
		}
		w, err := CreateWorld(biomeForDepth(d), seed, Options{CycleSpeed: cycleSpeed, Time: startTime, Planned: opts.plannedFor(d, first), ChunkCount: count, hub: opts.Candidate})
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

// WorldID ist die ID der Welt der Insel (Spielstand `world`, welten.md § 5).
func (isl *Island) WorldID() string { return isl.world.ID }

// IslandID ist die ID der Insel in ihrer Welt (Spielstand `islandId`).
func (isl *Island) IslandID() string { return isl.islandID }

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
