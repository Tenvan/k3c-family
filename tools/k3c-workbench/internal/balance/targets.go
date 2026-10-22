package balance

import (
	"bytes"
	"encoding/json"
	"fmt"
	"slices"
	"strings"

	"k3c/data"
)

// Zielkorridore (B-157, BAL2.1): Die Grenzen stehen in data/balance-targets.json, die Zahlen sind die von 🧑
// bestätigten aus docs/rules/zielkorridore.md. Jedes Ziel nennt eine Kennzahl (measures) und ein Szenario; ein Ziel
// ohne bekannte Kennzahl ist ein Ladefehler. Ziele, deren Kennzahl der Tester noch nicht liefert, stehen nicht in der Datei.

// Art eines Ziels: Anteil der Läufe mit erfüllter Bedingung (in %, 0–100) oder Median über alle Messwerte.
const (
	KindShare  = "share"
	KindMedian = "median"
)

// measure liest die Messwerte eines Laufs. Bei einem Anteil ist es genau ein Wert (1 = Bedingung erfüllt, 0 = nicht),
// bei einem Median beliebig viele (z. B. einer je Welle).
type measure struct {
	kind   string
	values func(*Metrics) []float64
}

// measures sind die Kennzahl-IDs, die Ziele benutzen dürfen.
var measures = map[string]measure{
	// Kein castleFallen bis zum Morgen nach dem letzten Tag des Szenarios.
	"castleHeld": {KindShare, func(m *Metrics) []float64 { return []float64{b2f(m.CastleFallTick == nil)} }},
	// Erste Mauer spätestens zu Dämmerungsbeginn von Tag 1 (Ende der hellen Phase).
	"firstWallBeforeDusk1": {KindShare, func(m *Metrics) []float64 {
		ok := len(m.Days) > 0 && m.Days[0].DuskTick != nil && m.FirstWallTick != nil && *m.FirstWallTick <= *m.Days[0].DuskTick
		return []float64{b2f(ok)}
	}},
	// Zerstörte Gebäude je Welle 1–5.
	"destroyedPerWave": {KindMedian, func(m *Metrics) []float64 {
		var v []float64
		for _, w := range m.Waves {
			if w.Wave <= 5 {
				v = append(v, float64(w.BuildingsDestroyed))
			}
		}
		return v
	}},
}

func b2f(b bool) float64 {
	if b {
		return 1
	}
	return 0
}

// Targets ist der Inhalt von data/balance-targets.json.
type Targets struct {
	Version int `json:"version"`
	// Margin ist die Breite von „knapp“ (Standardwert, je Ziel mit Target.Margin überschreibbar).
	Margin struct {
		SharePp        float64 `json:"sharePp"`        // Prozentpunkte bei Anteilen
		MedianFraction float64 `json:"medianFraction"` // Anteil der Korridorbreite bei Medianen
	} `json:"margin"`
	Seeds   []string `json:"seeds"` // die festen Seeds, keine Zufallsauswahl
	Targets []Target `json:"targets"`
}

// Target ist ein Ziel: Kennzahl, Szenario (ohne Seed) und Korridor. Eine fehlende Grenze (null) heißt: keine in diese Richtung.
type Target struct {
	Measure string   `json:"measure"`
	Name    string   `json:"name"`
	Kind    string   `json:"kind"`
	Players int      `json:"players"`
	Bot     string   `json:"bot"`
	Depth   int      `json:"depth"`
	Days    int      `json:"days"`
	Lower   *float64 `json:"lower"`
	Upper   *float64 `json:"upper"`
	Margin  *float64 `json:"margin,omitempty"` // Breite von „knapp“ statt des Standardwerts der Datei
	Rule    string   `json:"rule"`             // Regel-Datei und Abschnitt der Zahlen
}

// DefaultTargets lädt die eingebettete data/balance-targets.json.
func DefaultTargets() (*Targets, error) {
	raw, err := data.Files.ReadFile("balance-targets.json")
	if err != nil {
		return nil, err
	}
	return LoadTargets(raw)
}

// LoadTargets liest und prüft eine Korridor-Datei.
func LoadTargets(raw []byte) (*Targets, error) {
	var t Targets
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.DisallowUnknownFields()
	if err := dec.Decode(&t); err != nil {
		return nil, fmt.Errorf("balance-targets: %w", err)
	}
	if err := t.validate(); err != nil {
		return nil, fmt.Errorf("balance-targets: %w", err)
	}
	return &t, nil
}

func (t *Targets) validate() error {
	if t.Version != 1 {
		return fmt.Errorf("version %d unbekannt, erwartet 1", t.Version)
	}
	if t.Margin.SharePp < 0 || t.Margin.MedianFraction < 0 {
		return fmt.Errorf("margin darf nicht negativ sein")
	}
	if len(t.Seeds) == 0 {
		return fmt.Errorf("seeds: mindestens ein Seed nötig")
	}
	seen := map[string]bool{}
	for _, s := range t.Seeds {
		if s == "" || seen[s] {
			return fmt.Errorf("seeds: %q ist leer oder doppelt", s)
		}
		seen[s] = true
	}
	for i, g := range t.Targets {
		if err := g.validate(); err != nil {
			return fmt.Errorf("ziel %d (%q): %w", i+1, g.Name, err)
		}
	}
	return nil
}

func (g Target) validate() error {
	m, ok := measures[g.Measure]
	switch {
	case !ok:
		names := make([]string, 0, len(measures))
		for n := range measures {
			names = append(names, n)
		}
		slices.Sort(names)
		return fmt.Errorf("kennzahl %q unbekannt (bekannt: %s)", g.Measure, strings.Join(names, ", "))
	case g.Name == "" || g.Rule == "":
		return fmt.Errorf("name und rule sind Pflicht")
	case g.Kind != m.kind:
		return fmt.Errorf("art %q passt nicht zur Kennzahl %s (%s)", g.Kind, g.Measure, m.kind)
	case bots[g.Bot] == nil:
		return fmt.Errorf("unbekannter Bot %q", g.Bot)
	case g.Players < 1 || g.Players > 4 || g.Days < 1:
		return fmt.Errorf("players 1 bis 4 und days ab 1 nötig, war %d und %d", g.Players, g.Days)
	}
	if err := checkPlayers(g.Bot, g.Players); err != nil {
		return err
	}
	return g.validateBounds()
}

func (g Target) validateBounds() error {
	switch {
	case g.Lower == nil && g.Upper == nil:
		return fmt.Errorf("mindestens eine Grenze nötig")
	case g.Lower != nil && g.Upper != nil && *g.Lower > *g.Upper:
		return fmt.Errorf("untergrenze %v über Obergrenze %v", *g.Lower, *g.Upper)
	case g.Margin != nil && *g.Margin < 0:
		return fmt.Errorf("margin darf nicht negativ sein")
	}
	for _, b := range []*float64{g.Lower, g.Upper} {
		if g.Kind == KindShare && b != nil && (*b < 0 || *b > 100) {
			return fmt.Errorf("grenze %v eines Anteils muss in 0 bis 100 liegen", *b)
		}
	}
	return nil
}
