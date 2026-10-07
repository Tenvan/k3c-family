package sim

import (
	"fmt"
	"io/fs"
	"slices"

	"k3c/data"
)

// Boss-Daten (data/bosses.json, docs/rules/bosse.md § 1): eine Liste, damit der Sim-Code nie über eine Map iteriert.

// BossData ist ein Boss: Werte relativ zum Standardgegner `Base`, eine Fähigkeit und die Belohnung.
type BossData struct {
	ID, Name, Kind         string // Kind: mini oder end
	Depth, Wave            int    // Stufe; Welle, mit der ein Miniboss kommt
	Base                   string // Standardgegner der Stufe (enemies.json)
	HPFactor, DamageFactor float64
	Speed, Range           float64
	Ability                BossAbility
	Reward                 struct{ Gold, Material int }
}

// BossAbility: `summon` ruft Count Gegner Enemy, `aoe` trifft alles im Radius um den Boss; beides alle IntervalSeconds.
type BossAbility struct {
	Type, Enemy             string
	Count                   int
	Radius, IntervalSeconds float64
}

var bosses = loadBossData()

func loadBossData() []BossData {
	var v []BossData
	if err := loadBosses(data.Files, &v); err != nil {
		panic(err)
	}
	return v
}

// loadBosses liest bosses.json aus fsys und prüft jeden Eintrag; bei einem Fehler bleibt *dst unverändert.
func loadBosses(fsys fs.FS, dst *[]BossData) error {
	var v struct{ Bosses []BossData }
	if err := loadFrom(fsys, "bosses.json", &v); err != nil {
		return err
	}
	var ids []string
	for _, b := range v.Bosses {
		if err := checkBoss(b); err != nil {
			return fmt.Errorf("data/bosses.json: %s: %w", b.ID, err)
		}
		if slices.Contains(ids, b.ID) {
			return fmt.Errorf("data/bosses.json: %s doppelt", b.ID)
		}
		ids = append(ids, b.ID)
	}
	*dst = v.Bosses
	return nil
}

func checkBoss(b BossData) error {
	_, base := enemyData[b.Base]
	switch {
	case b.ID == "":
		return fmt.Errorf("ohne id")
	case b.Kind != "mini" && b.Kind != "end":
		return fmt.Errorf("unbekannte Art %q", b.Kind)
	case !hasDepth(b.Depth):
		return fmt.Errorf("unbekannte Stufe %d", b.Depth)
	case !base:
		return fmt.Errorf("unbekannter Bezug %q", b.Base)
	case b.Kind == "mini" && b.Wave <= 0:
		return fmt.Errorf("ohne Welle (Miniboss)")
	case b.HPFactor <= 0 || b.DamageFactor < 0 || b.Speed < 0 || b.Range <= 0:
		return fmt.Errorf("ungültige Werte")
	case b.Reward.Gold < 0 || b.Reward.Material < 0:
		return fmt.Errorf("negative Belohnung")
	}
	return checkAbility(b.Ability)
}

func checkAbility(a BossAbility) error {
	_, known := enemyData[a.Enemy]
	switch {
	case a.IntervalSeconds <= 0:
		return fmt.Errorf("ohne Intervall der Fähigkeit")
	case a.Type == "summon" && (!known || a.Count <= 0):
		return fmt.Errorf("summon braucht bekannten Gegner und Anzahl > 0")
	case a.Type == "aoe" && a.Radius <= 0:
		return fmt.Errorf("aoe ohne Radius")
	case a.Type != "summon" && a.Type != "aoe":
		return fmt.Errorf("unbekannte Fähigkeit %q", a.Type)
	}
	return nil
}

// bossByID: der Boss mit dieser ID (nil: keiner).
func bossByID(id string) *BossData {
	for i := range bosses {
		if bosses[i].ID == id {
			return &bosses[i]
		}
	}
	return nil
}

// miniBoss: der Miniboss der Stufe (nil: keiner).
func miniBoss(depth int) *BossData {
	for i := range bosses {
		if bosses[i].Kind == "mini" && bosses[i].Depth == depth {
			return &bosses[i]
		}
	}
	return nil
}
