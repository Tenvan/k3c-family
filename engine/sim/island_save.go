package sim

import (
	"encoding/json"
	"errors"
	"fmt"
	"sort"
)

// Spielstand der Insel (Version 2, B-100). Die Campaign schreibt weiter Version 1 (save.go); der Raum benutzt bis zur
// Umstellung (B-133) nur Version 1. Gespeichert wird wie dort nur, was sich nicht aus dem Seed ergibt: Hubs, Truppen,
// Vorrat, Gold und Entferntes. Flüchtiges (Gegner, Münzen am Boden, Geschosse, Zufallsstand) geht beim Laden verloren.

// IslandSaveVersion ist die Version des Insel-Spielstands.
const IslandSaveVersion = 2

// IslandSave ist ein Spielstand einer Insel.
type IslandSave struct {
	Version    int           `json:"version"`
	CampaignID string        `json:"campaignId"`
	SavedAt    string        `json:"savedAt"`
	Seed       string        `json:"seed"`
	Time       float64       `json:"time"`
	Stock      Stock         `json:"stock"`   // Vorrat der Insel (alle Stufen teilen ihn)
	Options    IslandOptions `json:"options"` // fehlt im Stand: Standard (Normal, Endboss, Stufenverlust)
	Stages     []HubSave     `json:"stages"`  // je Stufe der Hub, nach Tiefe aufsteigend
	// StageSkillPoints: Skill-Punkt-Zähler je Stufe (gleiche Reihenfolge); der Pool je Insel folgt mit B-118.
	StageSkillPoints []int              `json:"stageSkillPoints"`
	Players          []IslandPlayerSave `json:"players"`
}

// IslandPlayerSave ist ein Spieler der Insel: inselweiter Index, Gold und die Tiefe seiner Stufe.
type IslandPlayerSave struct {
	Index int `json:"index"`
	Gold  int `json:"gold"`
	Depth int `json:"depth"`
}

// ToSave schreibt den Spielstand. savedAt ist ein Zeitstempel (ISO 8601).
func (isl *Island) ToSave(savedAt string) IslandSave {
	s := IslandSave{
		Version: IslandSaveVersion, CampaignID: isl.ID, SavedAt: savedAt, Seed: isl.Seed, Time: isl.Stages[0].Time,
		Stock: *isl.Stock, Options: isl.Options, Stages: []HubSave{}, StageSkillPoints: []int{}, Players: []IslandPlayerSave{},
	}
	for _, w := range isl.Stages {
		s.Stages = append(s.Stages, hubSave(w, newWorld(w.Biome.Depth, isl.Seed, Options{})))
		s.StageSkillPoints = append(s.StageSkillPoints, w.SkillPoints)
	}
	for _, p := range isl.Players() {
		s.Players = append(s.Players, IslandPlayerSave{Index: p.Index, Gold: p.Gold, Depth: isl.Stages[isl.StageOf(p.Index)].Biome.Depth})
	}
	return s
}

// ParseIslandSave liest einen Spielstand der Version 2 oder, überführt, der Version 1 (Campaign).
func ParseIslandSave(raw []byte) (IslandSave, error) {
	var head struct {
		Version int `json:"version"`
	}
	if err := json.Unmarshal(raw, &head); err != nil {
		return IslandSave{}, fmt.Errorf("spielstand: %w", err)
	}
	switch head.Version {
	case SaveVersion:
		old, err := ParseSave(raw)
		if err != nil {
			return IslandSave{}, err
		}
		return islandFromV1(old), nil
	case IslandSaveVersion:
		return parseIslandV2(raw)
	}
	return IslandSave{}, fmt.Errorf("spielstand: Version %d, erwartet %d oder %d", head.Version, SaveVersion, IslandSaveVersion)
}

func parseIslandV2(raw []byte) (IslandSave, error) {
	var check map[string]json.RawMessage
	if err := json.Unmarshal(raw, &check); err != nil {
		return IslandSave{}, fmt.Errorf("spielstand: %w", err)
	}
	for _, f := range []string{"campaignId", "seed", "time", "stock", "stages", "stageSkillPoints", "players"} {
		if v, ok := check[f]; !ok || string(v) == "null" {
			return IslandSave{}, fmt.Errorf("spielstand: Feld %s fehlt", f)
		}
	}
	var s IslandSave
	s.Options = DefaultOptions() // Felder, die im Stand fehlen, behalten den Standard
	if err := json.Unmarshal(raw, &s); err != nil {
		return IslandSave{}, fmt.Errorf("spielstand: %w", err)
	}
	return s, validateIslandSave(s)
}

func validateIslandSave(s IslandSave) error {
	if err := validateOptions(s.Options); err != nil {
		return err
	}
	if len(s.Stages) == 0 || len(s.StageSkillPoints) != len(s.Stages) {
		return errors.New("spielstand: Stufen fehlen oder passen nicht zu den Skill-Punkten")
	}
	depths := map[int]bool{}
	for _, h := range s.Stages {
		if !hasDepth(h.Depth) || depths[h.Depth] {
			return fmt.Errorf("spielstand: unbekannte oder doppelte Tiefe %d", h.Depth)
		}
		depths[h.Depth] = true
	}
	seen := map[int]bool{}
	for _, p := range s.Players {
		if !depths[p.Depth] || p.Index < 0 || seen[p.Index] {
			return fmt.Errorf("spielstand: Spieler %d ungültig", p.Index)
		}
		seen[p.Index] = true
	}
	return nil
}

// islandFromV1 überführt einen Stand der Campaign: die Hubs werden Stufen, alle Spieler stehen in der gespeicherten
// Tiefe, die Vorräte der Hubs zählen zusammen als Vorrat der Insel (Annahme), die Skill-Punkte gehören der Stufe der Spieler.
func islandFromV1(old SaveGame) IslandSave {
	hubs := append([]HubSave{}, old.Hubs...)
	sort.SliceStable(hubs, func(i, j int) bool { return hubs[i].Depth < hubs[j].Depth })
	s := IslandSave{
		Version: IslandSaveVersion, CampaignID: old.CampaignID, SavedAt: old.SavedAt, Seed: old.Seed, Time: old.Time,
		Options: DefaultOptions(), Stages: []HubSave{}, StageSkillPoints: []int{}, Players: []IslandPlayerSave{},
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
	for _, h := range hubs {
		sp := 0
		if h.Depth == old.Depth {
			sp = old.SkillPoints
		}
		s.Stages = append(s.Stages, h)
		s.StageSkillPoints = append(s.StageSkillPoints, sp)
	}
	for i, p := range old.Players {
		s.Players = append(s.Players, IslandPlayerSave{Index: i, Gold: p.Gold, Depth: old.Depth})
	}
	return s
}

// FromIslandSave baut die Insel aus einem Spielstand (aus ParseIslandSave). Gespeicherte Spieler stehen wieder in ihrer Stufe.
func FromIslandSave(s IslandSave, cycleSpeed float64) (*Island, error) {
	if err := validateIslandSave(s); err != nil {
		return nil, err
	}
	if cycleSpeed == 0 {
		cycleSpeed = 1
	}
	depths := make([]int, len(s.Stages))
	for i, h := range s.Stages {
		depths[i] = h.Depth
	}
	isl, err := createIsland(s.Seed, depths, cycleSpeed, s.Time)
	if err != nil {
		return nil, err
	}
	isl.ID = s.CampaignID
	isl.Options = s.Options
	for i, h := range s.Stages {
		applyHub(isl.Stages[i], h)
		isl.Stages[i].SkillPoints = s.StageSkillPoints[i]
	}
	*isl.Stock = s.Stock // applyHub setzt den Vorrat je Hub; maßgeblich ist der der Insel
	for _, p := range s.Players {
		q := addPlayerAt(isl.Stages[stageByDepth(isl, p.Depth)], p.Index)
		q.Gold = p.Gold
		isl.nextPlayer = max(isl.nextPlayer, p.Index+1)
	}
	for _, w := range isl.Stages { // stabile Reihenfolge nach Index
		sort.SliceStable(w.Players, func(i, j int) bool { return w.Players[i].Index < w.Players[j].Index })
	}
	return isl, nil
}

func hasHub(hubs []HubSave, depth int) bool {
	for _, h := range hubs {
		if h.Depth == depth {
			return true
		}
	}
	return false
}
