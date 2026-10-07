package sim

import (
	"fmt"
	"io/fs"
	"slices"

	"k3c/data"
)

// Boss-Daten (data/bosses.json, docs/rules/bosse.md § 1): Listen der Minibosse und der Endbosse, damit der Sim-Code
// nie über eine Map iteriert.

// BossData ist ein Boss: Werte relativ zum Standardgegner `Base`, eine Fähigkeit und die Belohnung.
type BossData struct {
	ID, Name, Kind         string // Kind: mini oder end
	Depth, Wave            int    // Stufe; Welle, mit der ein Miniboss kommt
	Base                   string // Standardgegner der Stufe (enemies.json)
	HPFactor, DamageFactor float64
	Speed, Range           float64
	Ability                BossAbility // Miniboss; ein Endboss hat seine Fähigkeiten in Phases
	Reward                 struct{ Gold, Material int }
	// Endboss (boss_endboss.go): Bau mit Auslöse-Abstand und Phasen in Reihenfolge.
	Lair   struct{ TriggerUnits float64 }
	Phases []BossPhase
}

// BossPhase ist eine Phase des Endbosses: Sie beginnt, sobald seine HP unter BelowHPPercent fallen (die erste gilt ab
// dem Erscheinen), mit ihrer Fähigkeit (ohne: die der Phase davor bleibt) und Tempo als Faktor auf Takt von Angriff
// und Fähigkeit sowie auf die Laufgeschwindigkeit (0: unverändert).
type BossPhase struct {
	BelowHPPercent float64
	Ability        *BossAbility
	Tempo          float64
}

// BossAbility: `summon` ruft Count Gegner Enemy, `aoe` trifft alles im Radius um den Boss; beides alle IntervalSeconds.
// `flameTrail` legt alle IntervalSeconds eine Bodenfläche (Radius, DamagePerSecond, DurationSeconds) an den Ort des
// Bosses, `shardThrow` wirft alle IntervalSeconds ein Geschoss, dessen Einschlag alles im Radius trifft
// (boss_abilities.go).
type BossAbility struct {
	Type, Enemy                      string
	Count                            int
	Radius, IntervalSeconds          float64
	DamagePerSecond, DurationSeconds float64
}

var (
	bosses    = loadBossData(loadBosses)
	endBosses = loadBossData(loadEndBosses)
)

func loadBossData(load func(fs.FS, *[]BossData) error) []BossData {
	var v []BossData
	if err := load(data.Files, &v); err != nil {
		panic(err)
	}
	return v
}

// loadBosses liest die Minibosse (`bosses`) aus bosses.json in fsys und prüft jeden Eintrag; bei einem Fehler bleibt
// *dst unverändert.
func loadBosses(fsys fs.FS, dst *[]BossData) error {
	var v struct{ Bosses []BossData }
	if err := loadFrom(fsys, "bosses.json", &v); err != nil {
		return err
	}
	return checkBossList(v.Bosses, "mini", dst)
}

// loadEndBosses wie loadBosses für die Endbosse (`endBosses`).
func loadEndBosses(fsys fs.FS, dst *[]BossData) error {
	var v struct {
		EndBosses []BossData `json:"endBosses"`
	}
	if err := loadFrom(fsys, "bosses.json", &v); err != nil {
		return err
	}
	return checkBossList(v.EndBosses, "end", dst)
}

func checkBossList(list []BossData, kind string, dst *[]BossData) error {
	var ids []string
	for _, b := range list {
		if b.Kind != kind {
			return fmt.Errorf("data/bosses.json: %s: Art %q in der Liste für %q", b.ID, b.Kind, kind)
		}
		if err := checkBoss(b); err != nil {
			return fmt.Errorf("data/bosses.json: %s: %w", b.ID, err)
		}
		if slices.Contains(ids, b.ID) {
			return fmt.Errorf("data/bosses.json: %s doppelt", b.ID)
		}
		ids = append(ids, b.ID)
	}
	*dst = list
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
	case b.Kind == "end":
		return checkPhases(b)
	}
	return checkAbility(b.Ability)
}

// checkPhases: Ein Endboss braucht einen Bau mit Auslöse-Abstand und Phasen, deren erste eine Fähigkeit hat und deren
// HP-Schwellen streng fallen (zwischen 0 und 100 %).
func checkPhases(b BossData) error {
	if b.Lair.TriggerUnits <= 0 || len(b.Phases) == 0 || b.Phases[0].Ability == nil {
		return fmt.Errorf("endboss braucht lair.triggerUnits > 0 und eine erste Phase mit Fähigkeit")
	}
	below := 100.0
	for i, p := range b.Phases {
		if i > 0 && (p.BelowHPPercent <= 0 || p.BelowHPPercent >= below) {
			return fmt.Errorf("phase %d: belowHpPercent muss fallen und über 0 liegen", i+1)
		}
		if i > 0 {
			below = p.BelowHPPercent
		}
		if p.Tempo < 0 {
			return fmt.Errorf("phase %d: negatives Tempo", i+1)
		}
		if p.Ability != nil {
			if err := checkAbility(*p.Ability); err != nil {
				return fmt.Errorf("phase %d: %w", i+1, err)
			}
		}
	}
	return nil
}

func checkAbility(a BossAbility) error {
	_, known := enemyData[a.Enemy]
	switch {
	case a.IntervalSeconds <= 0:
		return fmt.Errorf("ohne Intervall der Fähigkeit")
	case a.Type == "summon" && (!known || a.Count <= 0):
		return fmt.Errorf("summon braucht bekannten Gegner und Anzahl > 0")
	case (a.Type == "aoe" || a.Type == "shardThrow") && a.Radius <= 0:
		return fmt.Errorf("%s ohne Radius", a.Type)
	case a.Type == "flameTrail" && (a.Radius <= 0 || a.DamagePerSecond <= 0 || a.DurationSeconds <= 0):
		return fmt.Errorf("flameTrail braucht Radius, Schaden und Dauer > 0")
	case !slices.Contains([]string{"summon", "aoe", "flameTrail", "shardThrow"}, a.Type):
		return fmt.Errorf("unbekannte Fähigkeit %q", a.Type)
	}
	return nil
}

// bossByID: der Miniboss oder Endboss mit dieser ID (nil: keiner).
func bossByID(id string) *BossData {
	for _, list := range [][]BossData{bosses, endBosses} {
		for i := range list {
			if list[i].ID == id {
				return &list[i]
			}
		}
	}
	return nil
}

// endBoss: der Endboss, dessen Bau in der Stufe liegt (nil: keiner).
func endBoss(depth int) *BossData {
	for i := range endBosses {
		if endBosses[i].Depth == depth {
			return &endBosses[i]
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
