package sim

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"path"
	"sort"
	"strings"

	"k3c/data"
	"k3c/engine/level"
)

// Balancing-Daten aus data/ (Port von src/world/sim/data.ts). encoding/json ordnet Felder ohne Rücksicht auf
// Groß-/Kleinschreibung zu, deshalb tragen die Lese-Typen keine Tags. Fehlende Werte sind 0 wie `?? 0` in TS.

// Cost sind Kosten in Gold und Baumaterial.
type Cost struct {
	Gold, Wood, Stone, Copper, Iron, Crystal int
}

// BuildingData ist ein Eintrag aus data/buildings.json.
type BuildingData struct {
	HP, BuildSeconds                 float64
	Cost                             Cost
	ArcherSlots, RangeBonus, BowRack int
	Offers                           []Offer     // Angebots-Zahlziele am Gebäude (Q52), Mechanik ab W4
	Levels                           []LevelData // Ausbau-Stufen am Platz (Mauer, Turm; hub_level.go)
	Plantation                       Plantation  // Farm: Bäume wachsen nach (plantation.go)
}

// Offer ist ein Angebots-Zahlziel mit festem Abstand DX zum Platz seines Gebäudes.
type Offer struct {
	Kind string
	DX   float64
}

// TroopData ist ein Eintrag aus data/troops.json.
type TroopData struct {
	HP, Speed, Damage, Range, AttacksPerSecond float64
	Cost, RecruitCost                          Cost
}

// Gatherable ist eine Ressource, die Bauern holen (Baum, Fels, Kupfererz).
type Gatherable struct {
	Resource    string
	Amount      int
	WorkSeconds float64
	MarkCost    int
}

// EnemyData ist ein Eintrag aus data/enemies.json.
type EnemyData struct {
	Tier                     string // standard, elite
	HP, Damage, Speed, Range float64
	Gold                     [2]int
	Traits                   []string
}

// HubSite ist ein Bauplatz relativ zur Hub-Mitte.
type HubSite struct {
	Kind        string
	OffsetUnits float64
	FromDepth   int
	NeedsDeeper bool
	HubLevel    int // ab dieser Hub-Stufe bezahlbar (§ 3 der Regeln)
}

// WallLineData sind die Mauerlinien je Seite (Q49, Q50): Linie k (ab 1) steht bei WallUnits[k-1] ab Hub-Mitte,
// die ersten FixedLines fest, die übrigen je Seed 0 bis JitterOutwardUnits weiter außen.
type WallLineData struct {
	WallUnits                        []float64
	FixedLines                       int
	JitterOutwardUnits               float64
	TowerInsetUnits, GateOutsetUnits float64
	GateHubLevel                     int
}

var (
	buildings = load[map[string]BuildingData]("buildings.json")
	troops    = load[map[string]TroopData]("troops.json")
	economy   = load[struct {
		Purse                                               struct{ StartGold, MaxGold int }
		PayRangeUnits, PayIntervalSeconds, PickupRangeUnits float64
		DropPickupDelaySeconds                              float64
		DawnGoldPerPlayer                                   int
		ChestGold                                           [2]int
		EnemyResourceDrop                                   struct {
			Chance float64
			Amount int
		}
		Gatherables map[string]Gatherable
		Veins       map[string]VeinData // Adern (vein.go), nicht in Gatherables
		Storage     struct{ BasePerHub, PerStorage int }
		RecruitCamp struct {
			MaxVagrants                 int
			RespawnSeconds, WanderUnits float64
		}
	}]("economy.json")
	hub = load[struct {
		Levels           []LevelData // Hub-Stufen (hub_level.go)
		Sites            []HubSite
		IslandSites      []HubSite // nur Insel-Stufen (island_storage.go)
		IslandStartStock Stock     // Vorrat einer neuen Insel (B-177)
		WallLines        WallLineData
		Merchant         struct {
			BuyOffsetUnits, SellOffsetUnits float64
			OnlyDepth                       int
		}
		CastleRadiusUnits, HomeRadiusUnits float64
		StartTroops                        struct{ Peasant, Archer int }
		Travel                             struct{ RangeUnits, Seconds float64 }
	}]("hub.json")
	monarch   = load[monarchData]("monarch.json")
	enemyData = load[map[string]EnemyData]("enemies.json")
	waves     = load[struct {
		Table []struct {
			FromWave        int
			Standard, Elite [2]int
		}
		SpawnSpreadSeconds float64
		PerExtraPlayer     float64 // Wellenfaktor je Zusatzspieler einer Insel
		DepthScaling       struct{ HP, Damage, Speed float64 }
		AttacksPerSecond   float64
		StealGold          int
	}]("waves.json")
	// biomes in der Reihenfolge von `BIOMES` in TS (Oberwelt zuerst, dann nach Tiefe).
	biomes         = loadBiomes()
	globalDayNight = firstDayNight()
)

func load[T any](name string) T {
	var v T
	raw, err := data.Files.ReadFile(name)
	if err == nil {
		err = json.Unmarshal(raw, &v)
	}
	if err != nil {
		panic(fmt.Sprintf("data/%s: %v", name, err))
	}
	return v
}

func loadBiomes() []level.Biome {
	files, err := fs.Glob(data.Files, "biomes/*.json")
	if err != nil {
		panic(err)
	}
	out := make([]level.Biome, 0, len(files))
	for _, f := range files {
		b, err := level.LoadBiome(strings.TrimSuffix(path.Base(f), ".json"))
		if err != nil {
			panic(err)
		}
		out = append(out, b)
	}
	sort.SliceStable(out, func(i, j int) bool { return out[i].Depth < out[j].Depth })
	return out
}

// firstDayNight ist der globale Zyklus: der des ersten Tag/Nacht-Bioms (Oberwelt), wie `globalDayNight()` in TS.
func firstDayNight() level.Cycle {
	for _, b := range biomes {
		if b.Cycle.Type == "dayNight" {
			return b.Cycle
		}
	}
	panic("Kein Biom mit Tag/Nacht-Zyklus")
}

// hasDepth: Gibt es ein Biom dieser Tiefe? (Port von `hasDepth` aus travel.ts.)
func hasDepth(depth int) bool {
	for _, b := range biomes {
		if b.Depth == depth {
			return true
		}
	}
	return false
}
