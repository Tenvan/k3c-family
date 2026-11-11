package art

import (
	"fmt"
	"log/slog"

	"k3c/native/k3c-rig/paint"
)

// Pfade relativ zur Repo-Wurzel (Aufruf über `task native:game`).
const (
	DefaultGrafikDir = "public/grafik"
	DefaultRigDir    = "native/k3c-rig/assets"
	DefaultFontPath  = "public/schrift/k3c.ttf"
)

// Config sagt, woher die Grafik kommt. RigDir leer = ohne Rig-Bibliothek (Tests). StrictFont: fehlende Schrift bricht ab.
type Config struct {
	GrafikDir, RigDir, FontPath string
	StrictFont                  bool
}

// DefaultConfig liefert die Pfade im Repo.
func DefaultConfig() Config {
	return Config{GrafikDir: DefaultGrafikDir, RigDir: DefaultRigDir, FontPath: DefaultFontPath}
}

type sheetKey struct{ asset, anim string }

// Library hält die Grafik: die Schrift und die Rigs dauerhaft, die Sheets je Level (ein Level zugleich). Nicht
// nebenläufig: Laden und Zeichnen laufen im Hauptfaden. Ein nil-Library ist erlaubt (Platzhalter ohne Grafik); die
// Abfragen liefern dann nil.
type Library struct {
	Font *Font

	cfg    Config
	rigs   *paint.Assets
	level  string
	sheets map[sheetKey]*Sheet
	budget Budget
}

// New öffnet die Bibliothek: lädt Rig-Teile und Schrift. Sheets kommen erst mit LoadLevel.
func New(cfg Config) (*Library, error) {
	l := &Library{cfg: cfg, sheets: map[sheetKey]*Sheet{}}
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

// LoadLevel lädt alle Animationen der Assets (`gruppe/id`) als Sheets und gibt die des vorigen Levels frei. Fehlt
// Grafik, bricht es vor dem Hochladen ab (Fehler mit allen Pfaden, ErrMissing); der alte Stand bleibt dann erhalten.
// Das Texturbudget steht einmal je Level im Log.
func (l *Library) LoadLevel(level string, assets []string) error {
	plan, err := PlanLevel(l.cfg.GrafikDir, level, assets)
	if err != nil {
		return err
	}
	loaded := make(map[sheetKey]*Sheet, len(plan.Sheets))
	for _, spec := range plan.Sheets {
		rgba, err := paint.ReadPNG(spec.Path)
		if err != nil {
			freeSheets(loaded)
			return fmt.Errorf("%w: Sheet %s nicht lesbar (%v)", ErrMissing, spec.Path, err)
		}
		loaded[sheetKey{spec.Asset, spec.Anim}] = upload(spec, rgba.Pix)
	}
	freeSheets(l.sheets)
	l.sheets, l.level, l.budget = loaded, level, plan.Budget
	slog.Info("📂 Grafik geladen", "level", level, "sheets", l.budget.String(), "rigs", fmt.Sprintf("%.1f MiB", l.RigMiB()))
	return nil
}

// Sheet liefert die Animation `anim` des Assets `gruppe/id` aus dem geladenen Level, sonst nil. Ohne Allokation.
func (l *Library) Sheet(asset, anim string) *Sheet {
	if l == nil {
		return nil
	}
	return l.sheets[sheetKey{asset, anim}]
}

// Level nennt das geladene Level ("" = keins).
func (l *Library) Level() string {
	if l == nil {
		return ""
	}
	return l.level
}

// Budget liefert das Texturbudget der Sheets des geladenen Levels.
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
