package sim

import (
	"encoding/json"
	"errors"
	"fmt"
	"slices"
	"sort"

	"k3c/engine/level"
)

// Spielstand der Insel (Version 9, B-100, S1.4, S2.2, K2.3b, K3.2a, W7.2, WZ18.3, WZ27.1). Die Campaign schreibt weiter Version 1 (save.go); der Raum benutzt bis zur
// Umstellung (B-133) nur Version 1. Gespeichert wird wie dort nur, was sich nicht aus dem Seed ergibt: Hubs, Truppen,
// Vorrat, Gold und Entferntes. Flüchtiges (Gegner, Münzen am Boden, Geschosse, Zufallsstand) geht beim Laden verloren.

// IslandSaveVersion ist die Version des Insel-Spielstands.
const IslandSaveVersion = 9

// IslandSave ist ein Spielstand einer Insel.
type IslandSave struct {
	Version    int                `json:"version"`
	CampaignID string             `json:"campaignId"`
	World      string             `json:"world"`    // Welt-ID (Version 8, welten.md § 5; ältere Stände: standard)
	IslandID   string             `json:"islandId"` // Insel-ID der aktuellen Insel in der Welt (Version 8)
	SavedAt    string             `json:"savedAt"`
	Day        int                `json:"day,omitempty"`   // Tag der Insel beim Speichern (Version 4, nur Anzeige)
	Phase      string             `json:"phase,omitempty"` // day, dusk, night, dawn beim Speichern (Version 4, nur Anzeige)
	Seed       string             `json:"seed"`
	Time       float64            `json:"time"`
	Stock      Stock              `json:"stock"`     // Vorrat der Insel (alle Stufen teilen ihn)
	Options    IslandOptions      `json:"options"`   // fehlt im Stand: Standard (Normal, Endboss, Stufenverlust)
	Stages     []HubSave          `json:"stages"`    // je Stufe der Hub, nach Tiefe aufsteigend
	SkillPool  int                `json:"skillPool"` // Fund-Pool der Insel (Version 2: Summe der Zähler je Stufe)
	Players    []IslandPlayerSave `json:"players"`
	// Version 5 (island_save_v5.go): besiegte Bosse, Münzzähler und ob der Sieg schon gemeldet ist; die aktuelle Insel
	// stand bis Version 7 als Index in `island` (Migration: island_save_v8.go).
	DefeatedBosses []string `json:"defeatedBosses,omitempty"`
	GoldCollected  int      `json:"goldCollected,omitempty"`
	Won            bool     `json:"won,omitempty"`
	// Version 6 (B-373): Besuche des Händlers und der anwesende Händler (Material, Abreisetag, HP).
	MerchantVisits int           `json:"merchantVisits,omitempty"`
	Merchant       *MerchantSave `json:"merchant,omitempty"`
	// Version 7 (save_economy.go): Rüstung, Berufe, Krieger, Elite, Schwerter und Plantage stehen in den Stufen (HubSave).
	// Version 9 (enemies_guard.go): geräumte Gegnerlager stehen in den Stufen (HubSave.GuardsCleared).
}

// IslandPlayerSave ist ein Spieler der Insel: inselweiter Index, Gold, die Tiefe seiner Stufe und seine Verteilung
// (gelernte Skills in Lernreihenfolge, Slots 1 bis 4).
type IslandPlayerSave struct {
	Index  int      `json:"index"`
	Gold   int      `json:"gold"`
	Depth  int      `json:"depth"`
	Skills []string `json:"skills,omitempty"`
	Slots  []string `json:"slots,omitempty"`
	// SkillBase: mitgebrachte Skills einer früheren Insel (Version 5), zählen nicht gegen den Pool.
	SkillBase int `json:"skillBase,omitempty"`
}

// ToSave schreibt den Spielstand. savedAt ist ein Zeitstempel (ISO 8601).
func (isl *Island) ToSave(savedAt string) IslandSave {
	s := IslandSave{
		Version: IslandSaveVersion, CampaignID: isl.ID, World: isl.world.ID, IslandID: isl.islandID, SavedAt: savedAt, Seed: isl.Seed, Time: isl.Stages[0].Time,
		Day: isl.Stages[0].Cycle.Day, Phase: isl.Stages[0].Cycle.Phase,
		Stock: *isl.Stock, Options: isl.Options, Stages: []HubSave{}, SkillPool: isl.SkillPool, Players: []IslandPlayerSave{},
		DefeatedBosses: slices.Clone(isl.DefeatedBosses), GoldCollected: isl.GoldCollected, Won: isl.Won,
		MerchantVisits: isl.MerchantVisits, Merchant: savedMerchant(isl),
	}
	for _, w := range isl.Stages {
		s.Stages = append(s.Stages, hubSave(w, newWorld(w.Biome.Depth, isl.Seed, Options{})))
	}
	for _, p := range isl.Players() {
		s.Players = append(s.Players, IslandPlayerSave{
			Index: p.Index, Gold: p.Gold, Depth: isl.Stages[isl.StageOf(p.Index)].Biome.Depth,
			Skills: slices.Clone(p.Skills), Slots: slices.Clone(p.Slots), SkillBase: p.SkillBase,
		})
	}
	return s
}

// ParseIslandSave liest einen Spielstand der Version 9 oder, überführt, der Version 8 (ohne geräumte Lager), 7 (Insel als Index), 6 (ohne
// Wirtschaft), 5 (ohne Händler), 4 (ohne Bosse und Insel), 3 (ohne Tag und Phase), 2 oder 1 (Campaign). Stände bis
// Version 7 kommen in die Standard-Welt (migrateWorld). Tag und Phase wirken beim Laden nicht, die Zeit kommt aus `time`.
func ParseIslandSave(raw []byte) (IslandSave, error) {
	var head struct {
		Version int `json:"version"`
		Island  int `json:"island"` // bis Version 7: Index der Insel in der Standard-Welt
	}
	if err := json.Unmarshal(raw, &head); err != nil {
		return IslandSave{}, fmt.Errorf("spielstand: %w", err)
	}
	var s IslandSave
	var err error
	switch head.Version {
	case IslandSaveVersion, 8:
		return parseIslandV3(raw, "world", "islandId")
	case SaveVersion:
		var old SaveGame
		if old, err = ParseSave(raw); err == nil {
			s = islandFromV1(old)
		}
	case 2:
		s, err = parseIslandV2(raw)
	case 3, 4, 5, 6, 7:
		s, err = parseIslandV3(raw)
	default:
		return IslandSave{}, fmt.Errorf("spielstand: Version %d, erwartet %d, 2, 3, 4, 5, 6, 7, 8 oder %d", head.Version, SaveVersion, IslandSaveVersion)
	}
	if err != nil {
		return IslandSave{}, err
	}
	return migrateWorld(s, head.Island)
}

func validateIslandSave(s IslandSave) error {
	if err := validateOptions(s.Options); err != nil {
		return err
	}
	if len(s.Stages) == 0 {
		return errors.New("spielstand: Stufen fehlen")
	}
	depths := map[int]bool{}
	for _, h := range s.Stages {
		if !hasDepth(h.Depth) || depths[h.Depth] {
			return fmt.Errorf("spielstand: unbekannte oder doppelte Tiefe %d", h.Depth)
		}
		depths[h.Depth] = true
		if err := validateEconomy(h); err != nil {
			return err
		}
	}
	seen := map[int]bool{}
	for _, p := range s.Players {
		if !depths[p.Depth] || p.Index < 0 || seen[p.Index] {
			return fmt.Errorf("spielstand: Spieler %d ungültig", p.Index)
		}
		seen[p.Index] = true
	}
	if err := validateProgress(s); err != nil {
		return err
	}
	return validateSkills(s)
}

// islandFromV1 überführt einen Stand der Campaign: die Hubs werden Stufen, alle Spieler stehen in der gespeicherten
// Tiefe, die Vorräte der Hubs zählen zusammen als Vorrat der Insel (Annahme), die Skill-Punkte werden der Pool.
func islandFromV1(old SaveGame) IslandSave {
	hubs := append([]HubSave{}, old.Hubs...)
	sort.SliceStable(hubs, func(i, j int) bool { return hubs[i].Depth < hubs[j].Depth })
	s := IslandSave{
		Version: IslandSaveVersion, CampaignID: old.CampaignID, SavedAt: old.SavedAt, Seed: old.Seed, Time: old.Time,
		Options: DefaultOptions(), Stages: []HubSave{}, SkillPool: old.SkillPoints, Players: []IslandPlayerSave{},
	}
	present := false
	for _, h := range hubs {
		present = present || h.Depth == old.Depth
		s.Stock.Wood += h.Stock.Wood
		s.Stock.Stone += h.Stock.Stone
		s.Stock.Copper += h.Stock.Copper
	}
	// Stufen ohne Hub im Stand (die aktuelle und die noch unbesuchten Tiefen 0..2) sind unverändert und kommen leer dazu,
	// sonst säße ein alter Stand auf seinen Stufen fest (moveToStage findet kein Ziel).
	for d := 0; d <= 2; d++ {
		if d != old.Depth && !hasHub(hubs, d) {
			hubs = append(hubs, HubSave{Depth: d, Sites: []SiteSave{}, Troops: []TroopSave{}})
		}
	}
	if !present {
		hubs = append(hubs, HubSave{Depth: old.Depth, Sites: []SiteSave{}, Troops: []TroopSave{}})
	}
	sort.SliceStable(hubs, func(i, j int) bool { return hubs[i].Depth < hubs[j].Depth })
	s.Stages = append(s.Stages, hubs...)
	for i, p := range old.Players {
		s.Players = append(s.Players, IslandPlayerSave{Index: i, Gold: p.Gold, Depth: old.Depth})
	}
	return s
}

// FromIslandSave baut die Insel aus einem Spielstand der Standard-Welt (FromIslandSaveIn).
func FromIslandSave(s IslandSave, cycleSpeed float64) (*Island, error) {
	return FromIslandSaveIn(s, cycleSpeed, standardWorld)
}

// FromIslandSaveIn baut die Insel aus einem Spielstand (aus ParseIslandSave) in der Welt world; Welt-ID und Insel-ID des
// Stands müssen zu ihr passen (islandIn). Gespeicherte Spieler stehen wieder in ihrer Stufe, mit ihrer gespeicherten
// Verteilung (das Beitritts-Preset gilt beim Laden nicht).
func FromIslandSaveIn(s IslandSave, cycleSpeed float64, world WorldDef) (*Island, error) {
	if err := validateIslandSave(s); err != nil {
		return nil, err
	}
	number, err := islandIn(world, s)
	if err != nil {
		return nil, err
	}
	if cycleSpeed == 0 {
		cycleSpeed = 1
	}
	depths := make([]int, len(s.Stages))
	for i, h := range s.Stages {
		depths[i] = h.Depth
	}
	def := world.Islands[number]
	isl, err := createIsland(s.Seed, depths, cycleSpeed, s.Time, savedCreateOptions(s), world)
	if err != nil {
		return nil, err
	}
	isl.ID, isl.Options = s.CampaignID, s.Options
	isl.Number, isl.islandID, isl.Scaling = number, def.ID, def.DepthScaling
	for i, h := range s.Stages {
		applyHub(isl.Stages[i], h)
	}
	*isl.Stock = s.Stock // applyHub setzt den Vorrat je Hub; maßgeblich ist der der Insel
	restorePool(isl, s)
	restoreProgress(isl, s)
	for _, p := range s.Players {
		q := addPlayerAt(isl.Stages[stageByDepth(isl, p.Depth)], p.Index)
		q.Gold = p.Gold
		q.Skills, q.Slots = slices.Clone(p.Skills), slices.Clone(p.Slots) // schon geprüft (validateSkills)
		q.SkillBase = p.SkillBase
		isl.nextPlayer = max(isl.nextPlayer, p.Index+1)
	}
	for _, w := range isl.Stages { // stabile Reihenfolge nach Index
		sort.SliceStable(w.Players, func(i, j int) bool { return w.Players[i].Index < w.Players[j].Index })
	}
	return isl, nil
}

// savedCreateOptions sind die Erzeugungs-Parameter aus dem Stand: je Stufe die Planung und Chunk-Anzahl, mit denen sie
// gebaut wurde (B-489). Spätere Änderungen von data/worlds/*.json betreffen nur neu erzeugte Stufen.
func savedCreateOptions(s IslandSave) CreateOptions {
	planned, counts := map[int][]level.PlannedChunk{}, map[int]int{}
	for _, h := range s.Stages {
		if len(h.Planned) > 0 {
			planned[h.Depth] = h.Planned
		}
		if h.ChunkCount > 0 {
			counts[h.Depth] = h.ChunkCount
		}
	}
	return CreateOptions{Planned: planned, ChunkCounts: counts}
}

func hasHub(hubs []HubSave, depth int) bool {
	for _, h := range hubs {
		if h.Depth == depth {
			return true
		}
	}
	return false
}
