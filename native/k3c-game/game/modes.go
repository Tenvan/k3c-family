package game

// Schalter der Abnahme-Modi (NG9): Ablauf ab dem Hauptmenü (--flow), Aufnahme zu festen Takten (--at), Messmodus
// (--measure) und die Liste der Grafik-Ordner (--assets).

import (
	"errors"
	"flag"
	"fmt"
	"slices"
	"strconv"
	"strings"

	"k3c/native/k3c-game/view"
)

// Measure sind die Schalter des Messmodus (measure.go): Fenster in Echtzeit, Aufwärmen, dann Messen, Bericht als JSON.
type Measure struct {
	Seconds float64 // Messdauer in Sekunden; 0 = kein Messmodus
	Warmup  float64 // Sekunden Aufwärmen vor der Messung
	Report  string  // Pfad des Berichts; leer = Standardausgabe
	NoVsync bool    // VSync aus (Bildrate unbegrenzt)
}

// Prüf-Schalter für Fehler-, Lade- und Leistungsanzeige (NG14): --grafik (anderer Grafik-Ordner, z. B. ein leerer, um
// die Fehlermeldung zu sehen), --river und --stats (wie F3 und F2, gespeichert in den Einstellungen des Ordners
// --saves), --stall (nur mit --shots: dauerhaft die Ladeanzeige).

// addModeFlags bindet die Schalter der Abnahme-Modi.
func addModeFlags(fs *flag.FlagSet, a *Args, r *rawArgs) {
	fs.BoolVar(&a.Flow, "flow", false, "")
	fs.StringVar(&r.at, "at", "", "")
	fs.Float64Var(&a.Measure.Seconds, "measure", 0, "")
	fs.Float64Var(&a.Measure.Warmup, "warmup", 5, "")
	fs.StringVar(&a.Measure.Report, "report", "", "")
	fs.BoolVar(&a.Measure.NoVsync, "novsync", false, "")
	fs.BoolVar(&a.Assets, "assets", false, "")
	fs.StringVar(&a.Grafik, "grafik", "", "")
	fs.StringVar(&a.River, "river", "", "")
	fs.BoolVar(&a.Stats, "stats", false, "")
	fs.BoolVar(&a.Stall, "stall", false, "")
}

// applyAt liest „t,t,…“ (streng aufsteigend, ab 0): der erste Takt ist der Vorlauf, die Zahl der Takte die der Bilder.
func (a *Args) applyAt(s string) error {
	if s == "" {
		return nil
	}
	for part := range strings.SplitSeq(s, ",") {
		t, err := strconv.Atoi(part)
		if err != nil || t < 0 || t > maxTick || (len(a.At) > 0 && t <= a.At[len(a.At)-1]) {
			return fmt.Errorf("🚫 --at %q: Takte 0 bis %d, streng aufsteigend, durch Komma", s, maxTick)
		}
		a.At = append(a.At, t)
	}
	a.Scene.Tick, a.Count = a.At[0], len(a.At)
	return nil
}

// checkModes prüft, dass sich die Modi vertragen.
func (a Args) checkModes() error {
	switch {
	case a.Flow && (a.Scene.Direct() || a.Scene.Menu != ""):
		return errors.New("🚫 --flow startet im Hauptmenü: ohne --seed, --load und --menu")
	case len(a.At) > 0 && a.Shots == "":
		return errors.New("🚫 --at nur mit --shots")
	case a.Measure.Seconds < 0 || a.Measure.Warmup < 0:
		return errors.New("🚫 --measure und --warmup ab 0 Sekunden")
	case a.Measure.Seconds > 0 && a.Shots != "":
		return errors.New("🚫 --measure und --shots schließen sich aus")
	case a.River != "" && !slices.Contains([]string{view.RiverShader, view.RiverTiles}, a.River):
		return fmt.Errorf("🚫 --river %q: %s oder %s", a.River, view.RiverShader, view.RiverTiles)
	case a.Stall && a.Shots == "":
		return errors.New("🚫 --stall nur mit --shots")
	}
	return nil
}
