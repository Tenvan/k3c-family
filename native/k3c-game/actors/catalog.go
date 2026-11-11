package actors

// Katalog der Figuren: welche Spielrolle welches Papier-Asset zeigt, wie hoch, wie weit über dem Boden und ob als Rig
// oder Sheet (data/paper.json, data/monarch.json, data/bosses.json). Reine Datei-Logik ohne Grafik.

import (
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"slices"
	"strings"

	"k3c/data"
)

// unitPx sind Spiel-Pixel je Unit (UNIT_PX des Browser-Clients); Höhen in data/paper.json stehen in Spiel-Pixeln.
const unitPx = 32

// bossHeightPx ist die Spielhöhe der Bosse in Spiel-Pixeln. data/paper.json nennt keine Bosse (der Browser-Client
// zeichnet sie als Goblin); bis dort ein Eintrag steht, gilt diese Höhe (anderthalb Truppen).
const bossHeightPx = 150

// bossAssets ordnet die Boss-IDs aus data/bosses.json ihren Papier-Assets zu (data/paper.json kennt keine Bosse).
var bossAssets = map[string]string{
	"goblinLeader":       "bosse/goblin-anfuehrer",
	"trollKing":          "bosse/troll-koenig",
	"ratQueen":           "bosse/ratten-koenigin",
	"lavaGolem":          "bosse/lava-golem",
	"shardTitan":         "bosse/splitter-titan",
	"crystalHeartWarden": "bosse/kristallherz-waechter",
}

// sheetAssets sind die Figuren mit Rig-Typ `requisite` (ohne Teile-Baum, native/k3c-rig/RIG.md): Sie bleiben Sheets.
// catalog_test.go gleicht die Liste mit den Specs unter public/grafik ab.
var sheetAssets = map[string]bool{
	"gegner/fledermaus": true, "gegner/rattenschwarm": true, "gegner/grubengeist": true, "gegner/lavaschleim": true,
	"gegner/eisenkaefer": true, "gegner/feuergeist": true, "gegner/kristallspinne": true, "gegner/splitterling": true,
	"bosse/kristallherz-waechter": true,
}

// Look sagt, wie eine Figurenart aussieht.
type Look struct {
	Asset   string  // Papier-Asset `gruppe/id` (Rig-Ordner bzw. Sheet)
	Variant string  // Farbvariante, z. B. "spieler2" ("" = Standard)
	Height  float64 // Spielhöhe in Units (Rahmenhöhe des Assets)
	Lift    float64 // Höhe des Fußpunkts über dem Boden in Units (Fledermaus)
	Alpha   float64 // Deckkraft 0..1
	Sheet   bool    // aus Sheets statt als Rig
}

// Catalog sind die Looks aller Figurenarten.
type Catalog struct {
	Players  []Look          // je Spieler-Index (zyklisch)
	Mount    string          // Reittier des Monarchen `reittiere/<id>`
	MountRef float64         // Referenz-Tempo des Reittiers (Units/s) für die Gangart
	Troops   map[string]Look // nach Troop.Kind
	Enemies  map[string]Look // nach Enemy.Kind
	Bosses   map[string]Look // nach Boss-ID (Enemy.Kind bei Enemy.Boss)

	depthKinds  map[int][]string // Gegner-Arten je Tiefe (enemies.json › depth, dazu Beschwörungen der Bosse)
	depthBosses map[int][]string // Boss-IDs je Tiefe (bosses.json › depth)
}

type figureSpec struct {
	Height float64  `json:"hoehe"`
	Y      float64  `json:"y"`
	Alpha  *float64 `json:"alpha"`
}

type paperFile struct {
	Players []string          `json:"spieler"`
	Mounts  map[string]string `json:"reittiere"`
	Troops  map[string]string `json:"truppen"`
	Enemies map[string]string `json:"gegner"`
	Figures struct {
		Players []figureSpec          `json:"spieler"`
		Troops  map[string]figureSpec `json:"truppen"`
		Enemies map[string]figureSpec `json:"gegner"`
	} `json:"figuren"`
}

type monarchFile struct {
	Base  struct{ Speed float64 } `json:"base"`
	Mount struct {
		Sprite      string  `json:"sprite"`
		SpeedFactor float64 `json:"speedFactor"`
	} `json:"mount"`
}

// bossAbility ist der Teil einer Boss-Fähigkeit, der eine Figur ins Level bringt (summon).
type bossAbility struct {
	Enemy string `json:"enemy"`
}

type bossEntry struct {
	ID      string       `json:"id"`
	Depth   int          `json:"depth"`
	Ability *bossAbility `json:"ability"`
	Phases  []struct {
		Ability *bossAbility `json:"ability"`
	} `json:"phases"`
}

type bossesFile struct {
	Bosses    []bossEntry `json:"bosses"`
	EndBosses []bossEntry `json:"endBosses"`
}

// LoadCatalog liest den Katalog aus den Daten; alle Lücken (Rolle ohne Höhe, Boss ohne Asset) kommen gesammelt.
func LoadCatalog(fsys data.FS) (*Catalog, error) {
	var p paperFile
	var m monarchFile
	var b bossesFile
	var e map[string]json.RawMessage
	if err := errors.Join(readJSON(fsys, "paper.json", &p), readJSON(fsys, "monarch.json", &m),
		readJSON(fsys, "bosses.json", &b), readJSON(fsys, "enemies.json", &e)); err != nil {
		return nil, err
	}
	c := &Catalog{MountRef: m.Base.Speed * m.Mount.SpeedFactor, Troops: map[string]Look{}, Enemies: map[string]Look{},
		Bosses: map[string]Look{}}
	var errs []error
	mount, ok := p.Mounts[m.Mount.Sprite]
	if !ok {
		errs = append(errs, fmt.Errorf("paper.json › reittiere: kein Eintrag für monarch.json › mount.sprite %q", m.Mount.Sprite))
	}
	c.Mount = "reittiere/" + mount
	for i, ref := range p.Players {
		if i >= len(p.Figures.Players) {
			errs = append(errs, fmt.Errorf("paper.json › figuren.spieler: Höhe für Spieler %d fehlt", i+1))
			continue
		}
		c.Players = append(c.Players, look(ref, p.Figures.Players[i]))
	}
	errs = append(errs, fill(c.Troops, p.Troops, p.Figures.Troops, "truppen"), fill(c.Enemies, p.Enemies, p.Figures.Enemies, "gegner"))
	for _, x := range slices.Concat(b.Bosses, b.EndBosses) {
		asset, ok := bossAssets[x.ID]
		if !ok {
			errs = append(errs, fmt.Errorf("bosses.json: Boss %q hat kein Asset (actors › bossAssets)", x.ID))
			continue
		}
		c.Bosses[x.ID] = look(asset, figureSpec{Height: bossHeightPx})
	}
	if len(c.Players) == 0 {
		errs = append(errs, errors.New("paper.json › spieler ist leer"))
	}
	c.depthKinds, c.depthBosses = byDepth(e, slices.Concat(b.Bosses, b.EndBosses))
	if err := errors.Join(errs...); err != nil {
		return nil, err
	}
	return c, nil
}

// fill legt je Rolle aus refs (Rolle → Asset) den Look mit seiner Höhe aus specs an.
func fill(out map[string]Look, refs map[string]string, specs map[string]figureSpec, group string) error {
	var errs []error
	for _, kind := range slices.Sorted(maps.Keys(refs)) {
		s, ok := specs[kind]
		if !ok || s.Height <= 0 {
			errs = append(errs, fmt.Errorf("paper.json › figuren.%s.%s: Höhe fehlt", group, kind))
			continue
		}
		out[kind] = look(refs[kind], s)
	}
	return errors.Join(errs...)
}

// look baut einen Look aus `gruppe/id[@variante]` und den Maßen in Spiel-Pixeln.
func look(ref string, s figureSpec) Look {
	asset, variant, _ := strings.Cut(ref, "@")
	l := Look{Asset: asset, Variant: variant, Height: s.Height / unitPx, Lift: s.Y / unitPx, Alpha: 1, Sheet: sheetAssets[asset]}
	if s.Alpha != nil {
		l.Alpha = *s.Alpha
	}
	return l
}

func readJSON(fsys data.FS, name string, v any) error {
	raw, err := fsys.ReadFile(name)
	if err != nil {
		return fmt.Errorf("data/%s: %w", name, err)
	}
	if err := json.Unmarshal(raw, v); err != nil {
		return fmt.Errorf("data/%s: %w", name, err)
	}
	return nil
}

// Player liefert den Look des Monarchen mit Spieler-Index i.
func (c *Catalog) Player(i int) Look {
	return c.Players[((i%len(c.Players))+len(c.Players))%len(c.Players)]
}

// Enemy liefert den Look eines Gegners bzw. Bosses (ok = false: unbekannte Art).
func (c *Catalog) Enemy(kind string, boss bool) (Look, bool) {
	if boss {
		l, ok := c.Bosses[kind]
		return l, ok
	}
	l, ok := c.Enemies[kind]
	return l, ok
}

// SheetAssets nennt die Assets, die als Sheet gezeichnet werden, sortiert und je einmal.
func (c *Catalog) SheetAssets() []string {
	var out []string
	for _, m := range []map[string]Look{c.Troops, c.Enemies, c.Bosses} {
		for _, l := range m {
			if l.Sheet {
				out = append(out, l.Asset)
			}
		}
	}
	slices.Sort(out)
	return slices.Compact(out)
}
