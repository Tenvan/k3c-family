package art

import (
	"cmp"
	"fmt"
	"log/slog"
	"maps"
	"slices"

	"k3c/native/k3c-rig/paint"
)

// Pfade relativ zur Repo-Wurzel (Aufruf über `task native:game`).
const (
	DefaultGrafikDir = "public/grafik"
	DefaultRigDir    = "native/k3c-rig/assets"
	DefaultFontPath  = "public/schrift/k3c.ttf"
)

// Config sagt, woher die Grafik kommt. RigDir leer = ohne Rig-Bibliothek (Tests). StrictFont: fehlende Schrift bricht ab
// (DefaultConfig: an, die Schrift liegt eingecheckt unter public/schrift).
type Config struct {
	GrafikDir, RigDir, FontPath string
	StrictFont                  bool
}

// DefaultConfig liefert die Pfade im Repo.
func DefaultConfig() Config {
	return Config{GrafikDir: DefaultGrafikDir, RigDir: DefaultRigDir, FontPath: DefaultFontPath, StrictFont: true}
}

type sheetKey struct{ asset, anim string }

// Library hält die Grafik: die Schrift und die Rigs dauerhaft, die Sheets der Level, die gerade gebraucht werden
// (mehrere zugleich: zwei Spieler können auf Stufen verschiedener Level stehen; ein Sheet, das mehrere Level nennen,
// liegt einmal auf der GPU). Nicht nebenläufig: Laden und Zeichnen laufen im Hauptfaden. Ein nil-Library ist erlaubt
// (Platzhalter ohne Grafik); die Abfragen liefern dann nil.
type Library struct {
	Font *Font

	cfg    Config
	rigs   *paint.Assets
	levels map[string][]string // geladenes Level → seine Assets
	sheets map[sheetKey]*Sheet
	budget Budget
}

// New öffnet die Bibliothek: lädt Rig-Teile und Schrift. Sheets kommen erst mit SetLevels.
func New(cfg Config) (*Library, error) {
	l := &Library{cfg: cfg, sheets: map[sheetKey]*Sheet{}, levels: map[string][]string{}}
	if cfg.RigDir != "" {
		a, err := paint.LoadAssets(cfg.RigDir)
		if err != nil {
			return nil, fmt.Errorf("%w: Rig-Bibliothek %s: %v", ErrMissing, cfg.RigDir, err)
		}
		l.rigs = a
	}
	font, err := LoadFont(cfg.FontPath, cfg.StrictFont)
	if err != nil {
		return nil, err
	}
	l.Font = font
	return l, nil
}

// SetLevels stellt die geladenen Sheets auf genau die Level in want (Level → Assets `gruppe/id`): lädt, was fehlt, und
// gibt frei, was kein Level aus want mehr braucht. Fehlt Grafik, bricht es vor jedem Hochladen ab (Fehler mit allen
// Pfaden, ErrMissing) und der alte Stand bleibt. Das Texturbudget des neuen Stands steht im Log. Ein nil-Library
// nimmt nur Level ohne Assets an.
func (l *Library) SetLevels(want map[string][]string) error {
	if l == nil {
		for lv, assets := range want {
			if len(assets) > 0 {
				return fmt.Errorf("%w: Level %s braucht %d Assets, aber es gibt keine Grafik-Bibliothek", ErrMissing, lv, len(assets))
			}
		}
		return nil
	}
	plans, err := planLevels(l.cfg.GrafikDir, want)
	if err != nil {
		return err
	}
	load, free := diffSheets(l.sheets, plans)
	loaded, err := uploadAll(load)
	if err != nil {
		return err
	}
	for _, k := range free {
		l.sheets[k].img.Deallocate()
		delete(l.sheets, k)
	}
	maps.Copy(l.sheets, loaded)
	l.levels, l.budget = maps.Clone(want), budgetOf(plans)
	slog.Info("📂 Grafik geladen", "level", l.Levels(), "neu", len(load), "frei", len(free),
		"sheets", l.budget.String(), "rigs", fmt.Sprintf("%.1f MiB", l.RigMiB()))
	return nil
}

// planLevels plant alle Level in fester Reihenfolge (nach Namen); alle Fehler gesammelt.
func planLevels(dir string, want map[string][]string) ([]*Plan, error) {
	out := make([]*Plan, 0, len(want))
	for _, lv := range slices.Sorted(maps.Keys(want)) {
		p, err := PlanLevel(dir, lv, want[lv])
		if err != nil {
			return nil, err
		}
		out = append(out, p)
	}
	return out, nil
}

// diffSheets vergleicht den geladenen Stand have mit den Plänen: load sind die geplanten Sheets, die fehlen (jedes
// einmal, in Plan-Reihenfolge), free die geladenen, die kein Plan mehr nennt (sortiert).
func diffSheets(have map[sheetKey]*Sheet, plans []*Plan) (load []SheetSpec, free []sheetKey) {
	need := map[sheetKey]bool{}
	for _, p := range plans {
		for _, s := range p.Sheets {
			k := sheetKey{s.Asset, s.Anim}
			if _, ok := have[k]; !ok && !need[k] {
				load = append(load, s)
			}
			need[k] = true
		}
	}
	for k := range have {
		if !need[k] {
			free = append(free, k)
		}
	}
	slices.SortFunc(free, func(a, b sheetKey) int {
		return cmp.Or(cmp.Compare(a.asset, b.asset), cmp.Compare(a.anim, b.anim))
	})
	return load, free
}

// budgetOf zählt jedes Sheet der Pläne einmal (gleiche Sheets mehrerer Level liegen nur einmal auf der GPU).
func budgetOf(plans []*Plan) Budget {
	var b Budget
	seen := map[sheetKey]bool{}
	for _, p := range plans {
		for _, s := range p.Sheets {
			if k := (sheetKey{s.Asset, s.Anim}); !seen[k] {
				seen[k] = true
				b.Add(s.W, s.H)
			}
		}
	}
	return b
}

// uploadAll liest und lädt die Sheets hoch; bei einem Fehler gibt es die schon geladenen wieder frei.
func uploadAll(specs []SheetSpec) (map[sheetKey]*Sheet, error) {
	loaded := make(map[sheetKey]*Sheet, len(specs))
	for _, spec := range specs {
		rgba, err := paint.ReadPNG(spec.Path)
		if err != nil {
			freeSheets(loaded)
			return nil, fmt.Errorf("%w: Sheet %s nicht lesbar (%v)", ErrMissing, spec.Path, err)
		}
		loaded[sheetKey{spec.Asset, spec.Anim}] = upload(spec, rgba.Pix)
	}
	return loaded, nil
}

// Sheet liefert die Animation `anim` des Assets `gruppe/id` aus den geladenen Leveln, sonst nil. Ohne Allokation.
func (l *Library) Sheet(asset, anim string) *Sheet {
	if l == nil {
		return nil
	}
	return l.sheets[sheetKey{asset, anim}]
}

// Levels nennt die geladenen Level sortiert (leer = keins).
func (l *Library) Levels() []string {
	if l == nil {
		return nil
	}
	return slices.Sorted(maps.Keys(l.levels))
}

// Budget liefert das Texturbudget der geladenen Sheets.
func (l *Library) Budget() Budget {
	if l == nil {
		return Budget{}
	}
	return l.budget
}

// RigMiB liefert den Speicher der bisher geladenen Rig-Bilder in Mebibyte.
func (l *Library) RigMiB() float64 {
	if l == nil || l.rigs == nil {
		return 0
	}
	return float64(l.rigs.Bytes) / (1 << 20)
}

// freeSheets gibt die GPU-Bilder frei.
func freeSheets(m map[sheetKey]*Sheet) {
	for _, s := range m {
		s.img.Deallocate()
	}
}
