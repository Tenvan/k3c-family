package balance

import (
	"bytes"
	"encoding/json"
	"fmt"
	"regexp"
	"slices"
	"strings"

	"k3c/data"
)

// Richtwerte der Kennzahlen und Hebel je Ziel (B-632): data/balance-hints.json, Startwerte nach
// docs/rules/zielkorridore.md § 5. Die Auswertung (insights.go) liest sie; die Simulation nicht.

// Band ist [von, bis] mit null = offen.
type Band [2]*float64

// FigureHint ist der Richtwert einer Kennzahl: grün im Band Green, gelb im Band Yellow, sonst rot. AllGrades: auch für
// die Grade außer normal bewerten (sonst nur normal, z. B. fällt das Überleben bei hard und ultra gewollt stärker ab).
type FigureHint struct {
	Name      string `json:"name"`
	Unit      string `json:"unit"`
	Green     Band   `json:"green"`
	Yellow    Band   `json:"yellow"`
	AllGrades bool   `json:"allGrades,omitempty"`
}

// Lever ist ein Hebel: Wert in data/ (datei.json:a.b.c), Richtung „+“ oder „-“ und relative Schrittweite (0.1 = 10 %).
type Lever struct {
	Path      string  `json:"path"`
	Direction string  `json:"direction"`
	Step      float64 `json:"step"`
	Label     string  `json:"label"`
}

// Levers sind die Hebel eines Ziels: Low bei einem Wert unter der Untergrenze, High über der Obergrenze.
type Levers struct {
	Low  []Lever `json:"low"`
	High []Lever `json:"high"`
}

// Hints ist der Inhalt von data/balance-hints.json.
type Hints struct {
	Version int                   `json:"version"`
	Notes   string                `json:"notes"`
	Figures map[string]FigureHint `json:"figures"`
	Levers  map[string]Levers     `json:"levers"`
}

// figureKeys sind die Kennzahlen, die die Auswertung aus Figures berechnet (insights_figures.go).
var figureKeys = []string{"firstWallTick", "firstWorkshopTick", "idleShare", "survivalDrop", "goldBalance", "woodBalance", "stoneBalance"}

// leverPath: datei.json:a.b.c wie bei Varianten und Tuning (ohne Platzhalter).
var leverPath = regexp.MustCompile(`^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_-]+(\.[A-Za-z0-9_-]+)*$`)

// DefaultHints lädt die eingebettete data/balance-hints.json.
func DefaultHints() (*Hints, error) {
	raw, err := data.Files.ReadFile("balance-hints.json")
	if err != nil {
		return nil, err
	}
	return LoadHints(raw)
}

// LoadHints liest und prüft eine Richtwert-Datei; unbekannte Felder, Kennzahlen und Ziele sind ein Fehler.
func LoadHints(raw []byte) (*Hints, error) {
	var h Hints
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.DisallowUnknownFields()
	if err := dec.Decode(&h); err != nil {
		return nil, fmt.Errorf("balance-hints: %w", err)
	}
	if err := h.validate(); err != nil {
		return nil, fmt.Errorf("balance-hints: %w", err)
	}
	return &h, nil
}

func (h *Hints) validate() error {
	if h.Version != 1 {
		return fmt.Errorf("version %d unbekannt, erwartet 1", h.Version)
	}
	for key, f := range h.Figures {
		if !slices.Contains(figureKeys, key) {
			return fmt.Errorf("kennzahl %q unbekannt (bekannt: %s)", key, strings.Join(figureKeys, ", "))
		}
		if err := f.validate(); err != nil {
			return fmt.Errorf("kennzahl %s: %w", key, err)
		}
	}
	for key, l := range h.Levers {
		if _, ok := measures[key]; !ok {
			return fmt.Errorf("hebel: ziel-kennzahl %q unbekannt", key)
		}
		for _, lv := range slices.Concat(l.Low, l.High) {
			if err := lv.validate(); err != nil {
				return fmt.Errorf("hebel %s: %w", key, err)
			}
		}
	}
	return nil
}

func (f FigureHint) validate() error {
	switch {
	case f.Name == "":
		return fmt.Errorf("name ist Pflicht")
	case !f.Green.valid() || !f.Yellow.valid():
		return fmt.Errorf("band: von ≤ bis nötig")
	case !f.Yellow.covers(f.Green):
		return fmt.Errorf("yellow muss green umfassen")
	}
	return nil
}

func (l Lever) validate() error {
	switch {
	case !leverPath.MatchString(l.Path):
		return fmt.Errorf("pfad %q: erwartet datei.json:a.b.c", l.Path)
	case l.Direction != "+" && l.Direction != "-":
		return fmt.Errorf("pfad %s: direction + oder -", l.Path)
	case l.Step <= 0 || l.Step > 1:
		return fmt.Errorf("pfad %s: step über 0 bis 1", l.Path)
	case l.Label == "":
		return fmt.Errorf("pfad %s: label ist Pflicht", l.Path)
	}
	return nil
}

func (b Band) valid() bool { return b[0] == nil || b[1] == nil || *b[0] <= *b[1] }

// covers: b enthält inner (offene Enden von b umfassen alles).
func (b Band) covers(inner Band) bool {
	lo := b[0] == nil || (inner[0] != nil && *inner[0] >= *b[0])
	hi := b[1] == nil || (inner[1] != nil && *inner[1] <= *b[1])
	return lo && hi
}

func (b Band) contains(v float64) bool {
	return (b[0] == nil || v >= *b[0]) && (b[1] == nil || v <= *b[1])
}

// Light ist die Ampel eines Werts nach dem Richtwert.
func (f FigureHint) Light(v float64) string {
	switch {
	case f.Green.contains(v):
		return LightGreen
	case f.Yellow.contains(v):
		return LightYellow
	}
	return LightRed
}

// leversFor: Hebel eines Ziels in die nötige Richtung (unter der Untergrenze Low, sonst High).
func (h *Hints) leversFor(e Entry) []Lever {
	l := h.Levers[e.Measure]
	if e.Lower != nil && e.Value < *e.Lower {
		return l.Low
	}
	if e.Upper != nil && e.Value > *e.Upper {
		return l.High
	}
	if e.Lower != nil && (e.Upper == nil || e.Value-*e.Lower < *e.Upper-e.Value) {
		return l.Low // knapp an der Untergrenze
	}
	return l.High
}
