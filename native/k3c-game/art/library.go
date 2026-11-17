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
	Scales                      map[string]float64 // Anzeige-Maßstab je Asset `gruppe/id` (DataScales); nil = überall 1
}

// DefaultConfig liefert die Pfade im Repo.
func DefaultConfig() Config {
	return Config{GrafikDir: DefaultGrafikDir, RigDir: DefaultRigDir, FontPath: DefaultFontPath, StrictFont: true,
		Scales: DataScales()}
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
	ld, err := l.BeginLevels(want)
	if err != nil {
		return err
	}
	_, err = ld.Step(len(ld.todo))
	return err
}

// LevelLoad ist ein SetLevels in Schritten (B-949): Step lädt einige Sheets, zwischen den Schritten kann die Schleife
// einen Fortschritt zeichnen. Erst der letzte Schritt gibt frei und stellt den neuen Stand ein; bis dahin bleibt der
// alte unverändert.
type LevelLoad struct {
	l     *Library
	want  map[string][]string
	plans []*Plan
	todo  []SheetSpec
	free  []sheetKey
	got   map[sheetKey]*Sheet
	done  bool
}

// BeginLevels plant das Laden der Level in want wie SetLevels, lädt aber noch nichts. Fehlt Grafik, kommt der Fehler
// (ErrMissing) schon hier; ein nil-Library nimmt nur Level ohne Assets an.
func (l *Library) BeginLevels(want map[string][]string) (*LevelLoad, error) {
	if l == nil {
		for lv, assets := range want {
			if len(assets) > 0 {
				return nil, fmt.Errorf("%w: Level %s braucht %d Assets, aber es gibt keine Grafik-Bibliothek", ErrMissing, lv, len(assets))
			}
		}
		return &LevelLoad{done: true}, nil
	}
	plans, err := planLevels(l.cfg.GrafikDir, want)
	if err != nil {
		return nil, err
	}
	load, free := diffSheets(l.sheets, plans)
	return &LevelLoad{l: l, want: want, plans: plans, todo: load, free: free, got: make(map[sheetKey]*Sheet, len(load))}, nil
}

// Step lädt bis zu n weitere Sheets und meldet, ob alles geladen und eingestellt ist. Bei einem Fehler sind die schon
// geladenen Sheets wieder frei und der alte Stand bleibt.
func (ld *LevelLoad) Step(n int) (bool, error) {
	if ld.done {
		return true, nil
	}
	for ; n > 0 && len(ld.got) < len(ld.todo); n-- {
		spec := ld.todo[len(ld.got)]
		sh, err := uploadOne(spec)
		if err != nil {
			freeSheets(ld.got)
			clear(ld.got) // sonst gibt ein folgendes Abort dieselben Bilder ein zweites Mal frei
			return false, err
		}
		ld.got[sheetKey{spec.Asset, spec.Anim}] = sh
	}
	if len(ld.got) < len(ld.todo) {
		return false, nil
	}
	l := ld.l
	for _, k := range ld.free {
		l.sheets[k].img.Deallocate()
		delete(l.sheets, k)
	}
	applyScales(ld.got, l.cfg.Scales)
	maps.Copy(l.sheets, ld.got)
	l.levels, l.budget = maps.Clone(ld.want), budgetOf(ld.plans)
	slog.Info("📂 Grafik geladen", "level", l.Levels(), "neu", len(ld.todo), "frei", len(ld.free),
		"sheets", l.budget.String(), "rigs", fmt.Sprintf("%.1f MiB", l.RigMiB()))
	ld.done = true
	return true, nil
}

// Abort bricht ein laufendes Laden ab und gibt die schon geladenen Sheets frei; der alte Stand bleibt. Nach Ende oder
// Abbruch ohne Wirkung.
func (ld *LevelLoad) Abort() {
	if ld == nil || ld.done {
		return
	}
	freeSheets(ld.got)
	clear(ld.got)
	ld.done = true
}

// Progress ist der Anteil der geladenen Sheets, 0 … 1 (ohne Sheets: 1).
func (ld *LevelLoad) Progress() float64 {
	if ld.done || len(ld.todo) == 0 {
		return 1
	}
	return float64(len(ld.got)) / float64(len(ld.todo))
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

// uploadOne liest das PNG des Sheets und lädt es hoch.
func uploadOne(spec SheetSpec) (*Sheet, error) {
	rgba, err := paint.ReadPNG(spec.Path)
	if err != nil {
		return nil, fmt.Errorf("%w: Sheet %s nicht lesbar (%v)", ErrMissing, spec.Path, err)
	}
	return upload(spec, rgba.Pix), nil
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
