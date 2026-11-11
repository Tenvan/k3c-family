package game

// Schalter der Abnahme-Modi (NG9): Ablauf ab dem Hauptmenü (--flow), Aufnahme zu festen Takten (--at), Messmodus
// (--measure) und die Liste der Grafik-Ordner (--assets).

import (
	"errors"
	"flag"
	"fmt"
	"strconv"
	"strings"
)

// Measure sind die Schalter des Messmodus (measure.go): Fenster in Echtzeit, Aufwärmen, dann Messen, Bericht als JSON.
type Measure struct {
	Seconds float64 // Messdauer in Sekunden; 0 = kein Messmodus
	Warmup  float64 // Sekunden Aufwärmen vor der Messung
	Report  string  // Pfad des Berichts; leer = Standardausgabe
	NoVsync bool    // VSync aus (Bildrate unbegrenzt)
}

// addModeFlags bindet die Schalter der Abnahme-Modi.
func addModeFlags(fs *flag.FlagSet, a *Args, r *rawArgs) {
	fs.BoolVar(&a.Flow, "flow", false, "")
	fs.StringVar(&r.at, "at", "", "")
	fs.Float64Var(&a.Measure.Seconds, "measure", 0, "")
	fs.Float64Var(&a.Measure.Warmup, "warmup", 5, "")
	fs.StringVar(&a.Measure.Report, "report", "", "")
	fs.BoolVar(&a.Measure.NoVsync, "novsync", false, "")
	fs.BoolVar(&a.Assets, "assets", false, "")
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
	}
	return nil
}
