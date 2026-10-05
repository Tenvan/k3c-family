package sim

import (
	"fmt"
	"math"
	"slices"
)

// Raum-Optionen der Insel (B-101): Schwierigkeitsgrad, Ziel und Niederlage-Modus. Ziel und Niederlage-Modus sind hier
// nur Werte; ihre Wirkung folgt mit B-102. Die Regeln gelten nur für Inseln (World.island != nil).

// IslandOptions sind die Optionen einer Insel. Grad: dev, easy, normal, hard, ultra.
type IslandOptions struct {
	Grade  string `json:"grade"`
	Goal   string `json:"goal"`   // endboss, gold, days, mineAll, buildAll
	Defeat string `json:"defeat"` // resources, stage, lost
}

// Grade sind die Faktoren eines Schwierigkeitsgrads aus data/difficulty.json.
type Grade struct {
	WaveSize, EnemyHP, EnemyDamage float64
	DefaultDefeat                  string
	ProtectedNights                int // die ersten n Nächte ohne Verlust (protectedNight), fehlt = 0
}

var (
	gradeNames  = []string{"dev", "easy", "normal", "hard", "ultra"}
	goalNames   = []string{"endboss", "gold", "days", "mineAll", "buildAll"}
	defeatNames = []string{"resources", "stage", "lost"}
	difficulty  = loadDifficulty()
)

func loadDifficulty() map[string]Grade {
	d := load[struct{ Grades map[string]Grade }]("difficulty.json").Grades
	for _, g := range gradeNames {
		if _, ok := d[g]; !ok {
			panic(fmt.Sprintf("data/difficulty.json: Grad %q fehlt", g))
		}
	}
	return d
}

// DefaultOptionsFor liefert die Standard-Optionen eines Grads (für den Anlegen-Dialog). Unbekannter Grad: Normal.
func DefaultOptionsFor(grade string) IslandOptions {
	g, ok := difficulty[grade]
	if !ok {
		grade, g = "normal", difficulty["normal"]
	}
	return IslandOptions{Grade: grade, Goal: "endboss", Defeat: g.DefaultDefeat}
}

// DefaultOptions: Normal, Endboss, Stufenverlust.
func DefaultOptions() IslandOptions { return DefaultOptionsFor("normal") }

func validateOptions(o IslandOptions) error {
	for _, c := range []struct {
		name, v string
		allowed []string
	}{{"Grad", o.Grade, gradeNames}, {"Ziel", o.Goal, goalNames}, {"Niederlage-Modus", o.Defeat, defeatNames}} {
		if !slices.Contains(c.allowed, c.v) {
			return fmt.Errorf("insel: unbekannter Wert %q für %s", c.v, c.name)
		}
	}
	return nil
}

// SetOptions setzt die Optionen der Insel; Grad dev nur im Dev-Mode. Die Faktoren gelten ab der nächsten Welle.
func SetOptions(isl *Island, o IslandOptions, devMode bool) error {
	if err := validateOptions(o); err != nil {
		return err
	}
	if o.Grade == "dev" && !devMode {
		return fmt.Errorf("insel: Grad dev nur im Dev-Mode")
	}
	isl.Options = o
	return nil
}

// SetGrade wechselt nur den Grad; Ziel und Niederlage-Modus bleiben (Standards: DefaultOptionsFor).
func SetGrade(isl *Island, grade string, devMode bool) error {
	o := isl.Options
	o.Grade = grade
	return SetOptions(isl, o, devMode)
}

// waveFactors: Wellengröße (Spieleranzahl der Insel × Grad), HP- und Schadensfaktor zum Zeitpunkt des Wellenstarts.
func (isl *Island) waveFactors() (size, hp, damage float64) {
	g := difficulty[isl.Options.Grade]
	extra := float64(max(len(isl.Players()), 1) - 1)
	return float64((1 + waves.PerExtraPlayer*extra) * g.WaveSize), g.EnemyHP, g.EnemyDamage
}

// protectedNight: Die Stufe gehört zu einer Insel, deren Grad die laufende Nacht schützt (protectedNights, S6.1):
// Gegner zerstören kein Gebäude, rauben kein Gold, ein Burgfall kostet nichts. Kosten und Einkommen bleiben.
func (w *World) protectedNight() bool {
	return w.island != nil && w.Cycle.Phase == "night" && w.Cycle.Day <= difficulty[w.island.Options.Grade].ProtectedNights
}

// scaled skaliert eine Gegnerzahl: gerundet, mindestens 1 bei Zahl > 0.
func scaled(n int, size float64) int {
	if n <= 0 || size == 1 {
		return n
	}
	return max(1, int(math.Round(float64(n)*size)))
}
