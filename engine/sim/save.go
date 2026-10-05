package sim

import (
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"sort"
	"strconv"
)

// Spielstand (Port von src/world/sim/campaign.ts). Gespeichert wird nur, was sich nicht aus dem Seed ergibt: Hubs,
// Truppen, Vorräte, Gold und was aus der Welt schon entfernt wurde. Das Format ist ein Vertrag mit Version.

// SaveVersion ist die einzige Version, die Go liest und schreibt.
const SaveVersion = 1

// SaveGame ist ein Spielstand, dieselbe JSON-Form wie in TS.
type SaveGame struct {
	Version       int          `json:"version"`
	CampaignID    string       `json:"campaignId"` // gleiche ID = gleiches Spiel
	SavedAt       string       `json:"savedAt"`
	Seed          string       `json:"seed"`
	Depth         int          `json:"depth"`
	UnlockedDepth int          `json:"unlockedDepth"`
	Time          float64      `json:"time"`
	SkillPoints   int          `json:"skillPoints"`
	Players       []PlayerSave `json:"players"`
	Hubs          []HubSave    `json:"hubs"`
}

// HubSave ist der Hub einer Stufe.
type HubSave struct {
	Depth      int        `json:"depth"`
	CastleHP   float64    `json:"castleHp"`
	Stock      Stock      `json:"stock"`
	Wave       int        `json:"wave"`
	Aggression *float64   `json:"aggression"`
	Sites      []SiteSave `json:"sites"`
	// Hub-Stufe (W1.3, 0 = 1) und ein laufender Hub-Ausbau an der Burg (hub_level.go), beide optional.
	HubLevel   int       `json:"hubLevel,omitempty"`
	HubUpgrade *SiteSave `json:"hubUpgrade,omitempty"`
	// Nur Bauern und Bogenschützen. Landstreicher kommen von allein aus den Camps.
	Troops       []TroopSave `json:"troops"`
	NodesGone    []string    `json:"nodesGone"` // Schlüssel `kind@x` der abgebauten Bäume/Felsen
	NodesMarked  []string    `json:"nodesMarked"`
	PickupsTaken []string    `json:"pickupsTaken"`
}

// PlayerSave ist das Gold eines Spielerplatzes.
type PlayerSave struct {
	Gold int `json:"gold"`
}

// TroopSave ist ein gespeicherter Bauer oder Bogenschütze.
type TroopSave struct {
	Kind    string  `json:"kind"`
	X       float64 `json:"x"`
	AnchorX float64 `json:"anchorX"`
}

// SiteSave ist ein gespeicherter Bauplatz.
type SiteSave struct {
	Kind          string  `json:"kind"`
	X             float64 `json:"x"`
	State         string  `json:"state"`
	PaidGold      int     `json:"paidGold"`
	BuildProgress float64 `json:"buildProgress"`
	HP            float64 `json:"hp"`
	Bows          int     `json:"bows"`
	BowPaidGold   int     `json:"bowPaidGold"`
	// Ausbau-Stufe (0 = 1) und laufender Ausbau (W1.3, hub_level.go), optional.
	Level       int    `json:"level,omitempty"`
	Upgrade     string `json:"upgrade,omitempty"`
	UpgradePaid int    `json:"upgradePaid,omitempty"`
}

// key ist `kind@x` mit x wie `x.toFixed(2)` in JS: Gleichstände (nur bei x = n/8) runden weg von 0, Go `FormatFloat`
// würde zur geraden Ziffer runden.
func key(kind string, x float64) string {
	if t := x * 8; t == math.Trunc(t) && math.Mod(t, 2) != 0 {
		x += math.Copysign(0.001, x)
	}
	return kind + "@" + strconv.FormatFloat(x, 'f', 2, 64)
}

func keys[T any](items []*T, k func(*T) (string, bool)) []string {
	out := []string{}
	for _, it := range items {
		if s, ok := k(it); ok {
			out = append(out, s)
		}
	}
	return out
}

func without(all, now []string) []string {
	have := map[string]bool{}
	for _, k := range now {
		have[k] = true
	}
	out := []string{}
	for _, k := range all {
		if !have[k] {
			out = append(out, k)
		}
	}
	return out
}

func nodeKey(n *ResourceNode) (string, bool) { return key(n.Kind, n.X), true }
func pickupKey(p *Pickup) (string, bool)     { return key(p.Kind, p.X), true }

func hubSave(w, base *World) HubSave {
	h := HubSave{Depth: w.Biome.Depth, CastleHP: w.Castle.HP, Stock: *w.Stock, Wave: w.Wave, Sites: []SiteSave{}}
	if w.Aggression != nil {
		a := *w.Aggression
		h.Aggression = &a
	}
	for _, s := range w.Sites {
		h.Sites = append(h.Sites, siteSave(s))
	}
	if w.HubLevel > 1 {
		h.HubLevel = w.HubLevel
	}
	if u := w.hubSite; u != nil && (u.Upgrade != "" || u.UpgradePaid > 0) {
		saved := siteSave(u)
		h.HubUpgrade = &saved
	}
	h.Troops = []TroopSave{}
	for _, t := range w.Troops {
		if t.Kind != "vagrant" {
			h.Troops = append(h.Troops, TroopSave{t.Kind, t.X, t.AnchorX})
		}
	}
	h.NodesGone = without(keys(base.Nodes, nodeKey), keys(w.Nodes, nodeKey))
	h.NodesMarked = keys(w.Nodes, func(n *ResourceNode) (string, bool) { return key(n.Kind, n.X), n.Marked })
	h.PickupsTaken = without(keys(base.Pickups, pickupKey), keys(w.Pickups, pickupKey))
	return h
}

func siteSave(s *Site) SiteSave {
	return SiteSave{s.Kind, s.X, s.State, s.PaidGold, s.BuildProgress, s.HP, s.Bows, s.BowPaidGold, s.Level, s.Upgrade, s.UpgradePaid}
}

// ToSave schreibt den Spielstand. savedAt ist ein Zeitstempel (ISO 8601).
func (c *Campaign) ToSave(savedAt string) SaveGame {
	w := c.CurrentWorld()
	hubs := []HubSave{}
	for _, h := range c.pendingHubs {
		hubs = append(hubs, h)
	}
	for depth, world := range c.worlds {
		// Vergleichswelt frisch aus dem Seed: was dort ist, hier aber fehlt, wurde entfernt.
		hubs = append(hubs, hubSave(world, newWorld(depth, c.Seed, Options{})))
	}
	// Die Tiefen sind eindeutig, die Reihenfolge der Maps spielt nach dem Sortieren keine Rolle.
	sort.SliceStable(hubs, func(i, j int) bool { return hubs[i].Depth < hubs[j].Depth })
	gold := append([]int{}, c.playerGold...)
	for _, p := range w.Players {
		for len(gold) <= p.Index {
			gold = append(gold, 0)
		}
		gold[p.Index] = p.Gold
	}
	s := SaveGame{
		Version: SaveVersion, CampaignID: c.ID, SavedAt: savedAt, Seed: c.Seed, Depth: c.Depth,
		UnlockedDepth: c.UnlockedDepth, Time: w.Time, SkillPoints: w.SkillPoints, Hubs: hubs,
	}
	s.Players = make([]PlayerSave, len(gold))
	for i, g := range gold {
		s.Players[i].Gold = g
	}
	return s
}

// ParseSave liest einen Spielstand. Andere Versionen und fehlende Pflichtfelder sind ein Fehler (wie `isSaveGame`).
func ParseSave(raw []byte) (SaveGame, error) {
	var check map[string]json.RawMessage
	if err := json.Unmarshal(raw, &check); err != nil {
		return SaveGame{}, fmt.Errorf("spielstand: %w", err)
	}
	for _, f := range []string{"version", "campaignId", "seed", "depth", "time", "hubs", "players"} {
		if v, ok := check[f]; !ok || string(v) == "null" {
			return SaveGame{}, fmt.Errorf("spielstand: Feld %s fehlt", f)
		}
	}
	var s SaveGame
	if err := json.Unmarshal(raw, &s); err != nil {
		return SaveGame{}, fmt.Errorf("spielstand: %w", err)
	}
	if s.Version != SaveVersion {
		return SaveGame{}, fmt.Errorf("spielstand: Version %d, erwartet %d", s.Version, SaveVersion)
	}
	if !hasDepth(s.Depth) {
		return SaveGame{}, errors.New("spielstand: unbekannte Tiefe")
	}
	return s, nil
}

// FromSave baut die Kampagne aus einem Spielstand (aus ParseSave).
func FromSave(s SaveGame, cycleSpeed float64) *Campaign {
	if cycleSpeed == 0 {
		cycleSpeed = 1
	}
	c := &Campaign{
		ID: s.CampaignID, Seed: s.Seed, CycleSpeed: cycleSpeed, Depth: s.Depth, UnlockedDepth: s.UnlockedDepth,
		worlds: map[int]*World{}, pendingHubs: map[int]HubSave{},
	}
	for _, h := range s.Hubs {
		c.pendingHubs[h.Depth] = h
	}
	for _, p := range s.Players {
		c.playerGold = append(c.playerGold, p.Gold)
	}
	c.worldFor(s.Depth, s.Time, s.SkillPoints)
	return c
}

// applyHub legt einen gespeicherten Hub auf eine frisch erzeugte Stufe.
func applyHub(w *World, h HubSave) {
	w.Castle.HP = h.CastleHP
	*w.Stock = h.Stock
	w.Wave = h.Wave
	if w.Aggression != nil && h.Aggression != nil {
		*w.Aggression = *h.Aggression
	}
	for _, saved := range h.Sites {
		applySite(w, saved)
	}
	applyHubLevel(w, h)
	// Start-Truppen durch die gespeicherten ersetzen
	vagrants := []*Troop{}
	for _, t := range w.Troops {
		if t.Kind == "vagrant" {
			vagrants = append(vagrants, t)
		}
	}
	w.Troops = vagrants
	for _, t := range h.Troops {
		troop := spawnVagrant(w, w.HubX, t.X)
		switch t.Kind {
		case "archer":
			makeArcher(w, troop)
			troop.AnchorX = t.AnchorX
		case "peasant":
			troop.Kind, troop.HP, troop.MaxHP, troop.AnchorX = "peasant", troops["peasant"].HP, troops["peasant"].HP, w.HubX
		}
	}
	gone, marked, taken := set(h.NodesGone), set(h.NodesMarked), set(h.PickupsTaken)
	nodes := []*ResourceNode{}
	for _, n := range w.Nodes {
		if !gone[key(n.Kind, n.X)] {
			n.Marked = n.Marked || marked[key(n.Kind, n.X)]
			nodes = append(nodes, n)
		}
	}
	w.Nodes = nodes
	pickups := []*Pickup{}
	for _, p := range w.Pickups {
		if !taken[key(p.Kind, p.X)] {
			pickups = append(pickups, p)
		}
	}
	w.Pickups = pickups
}

func applySite(w *World, saved SiteSave) {
	for _, s := range w.Sites {
		if key(s.Kind, s.X) != key(saved.Kind, saved.X) {
			continue
		}
		state := "unpaid"
		if saved.State == "waitingWorker" || saved.State == "waitingMaterial" || saved.State == "built" {
			state = saved.State
		}
		s.State, s.PaidGold, s.BuildProgress, s.HP = state, saved.PaidGold, saved.BuildProgress, saved.HP
		s.Bows, s.BowPaidGold, s.WorkerID = saved.Bows, saved.BowPaidGold, nil
		if levels := buildings[s.Kind].Levels; state == "built" && saved.Level > 1 && saved.Level <= len(levels) {
			s.Level, s.MaxHP = saved.Level, levels[saved.Level-1].HP
		}
		if state == "built" {
			applyUpgrade(s, saved)
		}
		return
	}
	// Bauplatz gibt es in dieser Version nicht mehr
}

// applyHubLevel übernimmt Hub-Stufe und laufenden Hub-Ausbau.
func applyHubLevel(w *World, h HubSave) {
	w.HubLevel = max(1, h.HubLevel)
	if u := h.HubUpgrade; u != nil && w.hubSite != nil {
		applyUpgrade(w.hubSite, *u)
	}
}

// applyUpgrade übernimmt einen laufenden Ausbau eines gebauten Platzes.
func applyUpgrade(s *Site, saved SiteSave) {
	if saved.Upgrade == "waitingMaterial" || saved.Upgrade == "waitingWorker" {
		s.Upgrade, s.BuildProgress = saved.Upgrade, saved.BuildProgress
	}
	s.UpgradePaid = saved.UpgradePaid
}

func set(items []string) map[string]bool {
	out := map[string]bool{}
	for _, k := range items {
		out[k] = true
	}
	return out
}
