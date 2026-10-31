package sim

import (
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"maps"
	"reflect"
	"slices"
	"strings"
	"testing/fstest"
)

// Welt-Werte (docs/rules/welten.md § 1, B-680): Eine Welt erbt die Basis-Werte aus data/ und übersteuert mit `overrides`
// nur einzelne Felder; mit `extends` erbt sie von einer anderen Welt (Kette Basis → A → B, höchstens maxWorldChain
// Welten). ResolveWorldValues löst eine Welt zu ihrem Wertesatz auf, ohne globalen Zustand zu ändern; die Sim rechnet
// bis BAL8.2 weiter mit den Paket-Variablen aus data.go. Merge und Prüfung je Feld: world_values_merge.go.

// maxWorldChain ist die größte Zahl Welten in einer Kette über der Basis (Basis → A → B → C → D).
const maxWorldChain = 4

// overrideFiles sind die Basis-Dateien, die eine Welt übersteuern darf (Schlüssel = Dateiname ohne .json), mit dem
// Lese-Typ, gegen den jedes Override geprüft wird.
var overrideFiles = map[string]reflect.Type{
	"buildings": reflect.TypeFor[map[string]BuildingData](),
	"ship":      reflect.TypeFor[shipFile](),
	"troops":    reflect.TypeFor[map[string]TroopData](),
	"economy":   reflect.TypeFor[economyData](),
	"hub":       reflect.TypeFor[HubData](),
	"monarch":   reflect.TypeFor[monarchData](),
	"enemies":   reflect.TypeFor[map[string]EnemyData](),
	"waves":     reflect.TypeFor[wavesData](),
	"goals":     reflect.TypeFor[Goals](),
}

// WorldValues ist der aufgelöste Wertesatz einer Welt, gleich gebaut wie die Paket-Variablen buildings, troops, economy,
// hub, monarch, enemyData, waves und goals (data.go).
type WorldValues struct {
	Buildings map[string]BuildingData // mit dem Platz `ship` aus ship.json (withShip)
	Troops    map[string]TroopData
	Economy   economyData
	Hub       HubData
	Monarch   monarchData
	Enemies   map[string]EnemyData
	Waves     wavesData
	Goals     Goals
}

// economyData ist der Lese-Typ von data/economy.json, gleich der Variablen economy in data.go (der Test
// TestWorldValuesOhneOverrides vergleicht beide; BAL8.2 stellt economy auf diesen Typ um).
type economyData struct {
	Purse                                               struct{ StartGold, MaxGold int }
	PayRangeUnits, PayIntervalSeconds, PickupRangeUnits float64
	DropPickupDelaySeconds                              float64
	DawnGoldPerPlayer                                   int
	DawnGoldPerDepth                                    int
	ChestGold                                           [2]int
	EnemyResourceDrop                                   struct {
		Chance float64
		Amount int
	}
	Gatherables map[string]Gatherable
	Veins       map[string]VeinData
	Storage     struct{ BasePerHub, PerStorage int }
	RecruitCamp struct {
		MaxVagrants                 int
		RespawnSeconds, WanderUnits float64
	}
	Merchant MerchantData
}

// ResolveWorldValues löst die Welt id zu ihrem Wertesatz auf: Basis-Dateien aus base (Aufbau wie data/), darüber die
// Overrides der Kette von der ältesten Eltern-Welt bis zur Welt selbst. lookup liefert eine Welt nach ID (z. B.
// LoadWorld). Jede Welt mit Overrides wird nach ihrem Schritt mit den Basis-Regeln geprüft; ein Fehler nennt die Welt,
// deren Override ihn auslöst. Ohne Overrides ist das Ergebnis gleich dem Laden der Basis.
func ResolveWorldValues(base fs.FS, lookup func(id string) (WorldDef, error), id string) (WorldValues, error) {
	chain, err := worldChain(lookup, id)
	if err != nil {
		return WorldValues{}, err
	}
	files := fstest.MapFS{}
	for _, w := range chain {
		if len(w.Overrides) == 0 {
			continue
		}
		if err := applyOverrides(base, files, w.Overrides); err != nil {
			return WorldValues{}, fmt.Errorf("%s: %w", worldPath(w.ID), err)
		}
		if _, err := valuesFrom(overlay(base, files)); err != nil {
			return WorldValues{}, fmt.Errorf("%s: overrides: %w", worldPath(w.ID), err)
		}
	}
	return valuesFrom(overlay(base, files))
}

// worldChain liefert die Kette der Welt id mit ihren Eltern, die älteste zuerst. Abgelehnt: unbekannte Eltern-Welt,
// Zyklus, mehr als maxWorldChain Welten.
func worldChain(lookup func(id string) (WorldDef, error), id string) ([]WorldDef, error) {
	w, err := lookup(id)
	if err != nil {
		return nil, err
	}
	chain := []WorldDef{w}
	for w.Extends != "" {
		parent := w.Extends
		if slices.ContainsFunc(chain, func(c WorldDef) bool { return c.ID == parent }) {
			return nil, fmt.Errorf("%s: extends: Zyklus %s → %s", worldPath(id), chainIDs(chain), parent)
		}
		if len(chain) == maxWorldChain {
			return nil, fmt.Errorf("%s: extends: Kette zu lang (%s → %s, höchstens %d Welten)", worldPath(id), chainIDs(chain), parent, maxWorldChain)
		}
		p, err := lookup(parent)
		if err != nil {
			return nil, fmt.Errorf("%s: extends: Eltern-Welt %q unbekannt: %w", worldPath(w.ID), parent, err)
		}
		w = p
		chain = append(chain, w)
	}
	slices.Reverse(chain)
	return chain, nil
}

func chainIDs(chain []WorldDef) string {
	ids := make([]string, len(chain))
	for i, w := range chain {
		ids[i] = w.ID
	}
	return strings.Join(ids, " → ")
}

// applyOverrides legt die Overrides einer Welt über den bisherigen Stand der Dateien (files, sonst base), Datei für
// Datei in fester Reihenfolge, und schreibt das Ergebnis nach files.
func applyOverrides(base fs.FS, files fstest.MapFS, overrides map[string]json.RawMessage) error {
	for _, name := range slices.Sorted(maps.Keys(overrides)) {
		t, ok := overrideFiles[name]
		if !ok {
			return unknownOverrideFile(name)
		}
		file := name + ".json"
		raw, err := fs.ReadFile(overlay(base, files), file)
		if err != nil {
			return fmt.Errorf("data/%s: %w", file, err)
		}
		merged, err := mergeFile(raw, overrides[name], t)
		if err != nil {
			return fmt.Errorf("overrides › data/%s › %w", file, err)
		}
		files[file] = &fstest.MapFile{Data: merged}
	}
	return nil
}

// overlay: die übersteuerten Dateien aus files, alle anderen aus base.
func overlay(base fs.FS, files fstest.MapFS) fs.FS {
	if len(files) == 0 {
		return base
	}
	return overlayFS{base, files}
}

type overlayFS struct {
	base  fs.FS
	files fstest.MapFS
}

func (o overlayFS) Open(name string) (fs.File, error) {
	if _, ok := o.files[name]; ok {
		return o.files.Open(name)
	}
	return o.base.Open(name)
}

// valuesFrom liest den Wertesatz aus fsys wie UseData und prüft die Basis-Regeln: Traits (loadEnemies), Presets,
// Wellen und die Gegner-Pools der Biome.
func valuesFrom(fsys fs.FS) (WorldValues, error) {
	var v WorldValues
	var ship shipFile
	err := errors.Join(loadFrom(fsys, "buildings.json", &v.Buildings), loadFrom(fsys, "ship.json", &ship),
		loadFrom(fsys, "troops.json", &v.Troops), loadFrom(fsys, "economy.json", &v.Economy), loadFrom(fsys, "hub.json", &v.Hub),
		loadFrom(fsys, "monarch.json", &v.Monarch), loadEnemies(fsys, &v.Enemies), loadFrom(fsys, "waves.json", &v.Waves),
		loadFrom(fsys, "goals.json", &v.Goals))
	if err == nil {
		err = errors.Join(checkPresets(v.Monarch), checkWaves(v.Waves), checkPools(v.Enemies))
	}
	if err == nil && len(ship.Stages) == 0 {
		err = errors.New("data/ship.json: keine Abschnitte")
	}
	if err != nil {
		return WorldValues{}, err
	}
	v.Buildings = withShip(v.Buildings, ship)
	return v, nil
}

// checkPools prüft die Gegner-Pools aller Biome gegen enemies (checkPool); die Biome sind für alle Welten gleich.
func checkPools(enemies map[string]EnemyData) error {
	for _, b := range biomes {
		if err := checkPool(b, enemies); err != nil {
			return err
		}
	}
	return nil
}

// checkWorldLinks prüft beim Lesen der Welt-Datei `extends` (Welt-ID, nicht die eigene) und die Dateinamen in
// `overrides`; Felder und Werte prüft ResolveWorldValues.
func checkWorldLinks(id string, extends *string, overrides map[string]json.RawMessage) error {
	switch {
	case extends == nil:
	case !idPattern.MatchString(*extends):
		return fmt.Errorf("extends: Welt-ID %q ungültig (a-z, 0-9, -, höchstens 32 Zeichen)", *extends)
	case *extends == id:
		return errors.New("extends: Welt erbt von sich selbst")
	}
	for _, name := range slices.Sorted(maps.Keys(overrides)) {
		if _, ok := overrideFiles[name]; !ok {
			return unknownOverrideFile(name)
		}
	}
	return nil
}

func unknownOverrideFile(name string) error {
	return fmt.Errorf("overrides: unbekannte Datei %q (erlaubt: %s)", name, strings.Join(slices.Sorted(maps.Keys(overrideFiles)), ", "))
}

// WorldChildren liefert die IDs der Welten aus all, die mit `extends` direkt von id erben, sortiert.
func WorldChildren(all []WorldDef, id string) []string {
	var out []string
	for _, w := range all {
		if w.Extends == id {
			out = append(out, w.ID)
		}
	}
	slices.Sort(out)
	return out
}

// CheckWorldRemovable: Löschen oder Umbenennen der Welt id ist nur erlaubt, wenn keine Welt aus all von ihr erbt; die
// Meldung nennt die Kind-Welten.
func CheckWorldRemovable(all []WorldDef, id string) error {
	if kids := WorldChildren(all, id); len(kids) > 0 {
		return fmt.Errorf("%s: Welt hat Kind-Welten (%s), Löschen und Umbenennen abgelehnt", worldPath(id), strings.Join(kids, ", "))
	}
	return nil
}
