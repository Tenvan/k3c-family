package world

// Licht bei Tag und Nacht (B-795/AC-03), Regeln wie src/scenes/render/nightLight.ts und viewRules.ts › daylight:
// Helligkeit aus der Phase des Zyklus (sim.World.Cycle) mit den Untergrenzen aus data/light.json, Lichtquellen
// (Lagerfeuer an der Burg auf Hub-Stufe 1, Fackeln an gebauter Mauer und gebautem Turm) heben sie um ihre Mitte an.
// Reine Rechnung, ohne Allokation je Aufruf.

import (
	"encoding/json"
	"image/color"
	"log/slog"
	"math"

	"k3c/data"
	"k3c/engine/sim"
)

// white ist die Rückfall-Farbe des Lichtscheins.
var white = color.RGBA{255, 255, 255, 255}

// lightSource sind die Werte einer Lichtquellen-Art: Reichweite in Units und Stärke in der Mitte.
type lightSource struct {
	Radius   float64 `json:"radius"`
	Strength float64 `json:"strength"`
}

// lightData sind die Werte aus data/light.json.
type lightData struct {
	Night   float64                `json:"nightBrightness"`
	Back    float64                `json:"backgroundBrightness"`
	Glow    string                 `json:"glowColor"`
	Sources map[string]lightSource `json:"sources"`
	Torches map[string][]float64   `json:"torches"`

	glow     [3]float64 // Farbe des Lichtscheins 0 … 1
	campfire lightSource
	torch    lightSource
}

// loadLight liest data/light.json; ein Fehler steht im Log, dann gibt es weder Nacht noch Licht.
func loadLight() lightData {
	d := lightData{Night: 1, Back: 1}
	raw, err := data.Files.ReadFile("light.json")
	if err == nil {
		err = json.Unmarshal(raw, &d)
	}
	if err != nil {
		slog.Error("❌ data/light.json nicht lesbar, Nacht bleibt hell", "err", err)
		d = lightData{Night: 1, Back: 1}
	}
	d.prepare()
	return d
}

// prepare legt die abgeleiteten Werte an (Farbe, Quellen-Arten).
func (d *lightData) prepare() {
	g := parseHex(d.Glow, white)
	d.glow = [3]float64{float64(g.R) / 255, float64(g.G) / 255, float64(g.B) / 255}
	d.campfire, d.torch = d.Sources["campfire"], d.Sources["torch"]
}

// daylight ist die Helligkeit der Phase: 1 = Tag, 0 = tiefste Nacht; Dämmerung fällt, Morgengrauen steigt linear,
// unbekannte Phasen gelten als Nacht.
func daylight(c sim.CycleInfo) float64 {
	switch c.Phase {
	case "day":
		return 1
	case "dusk":
		return 1 - c.Progress
	case "dawn":
		return c.Progress
	}
	return 0
}

// brightnessOf ist die Helligkeit mit Nacht-Untergrenze night: night + (1 − night)·daylight.
func brightnessOf(c sim.CycleInfo, night float64) float64 {
	return night + (1-night)*daylight(c)
}

// front ist die Helligkeit des Vordergrunds außerhalb des Lichts.
func (d *lightData) front(c sim.CycleInfo) float64 { return brightnessOf(c, d.Night) }

// backdrop ist die Helligkeit von Himmel und Kulisse (nachts dunkler als der Vordergrund).
func (d *lightData) backdrop(c sim.CycleInfo) float64 { return brightnessOf(c, d.Back) }

// glowAlpha ist die Deckkraft eines Lichtscheins der Stärke strength: am Tag 0, nachts die Stärke (0 … 1).
func glowAlpha(strength float64, c sim.CycleInfo) float64 {
	return min(1, max(0, strength)) * (1 - daylight(c))
}

// falloff ist der Anteil eines Lichts mit Mitte sx und Reichweite radius am Ort x: 1 in der Mitte, linear 0 am Rand.
func falloff(x, sx, radius float64) float64 {
	if radius <= 0 {
		return 0
	}
	return max(0, 1-math.Abs(x-sx)/radius)
}

// valid meldet, ob die Quelle Licht gibt (Werte in data/ vorhanden).
func (s lightSource) valid() bool { return s.Radius > 0 && s.Strength > 0 }

// hasCampfire meldet, ob die Burg der Welt das Lagerfeuer ist (Hub-Stufe 1, src/scenes/render/hubGrowth.ts).
func hasCampfire(w *sim.World) bool { return w.HubLevel <= 1 }

// lift ist die Summe der Lichtscheine am Ort x (vor der Begrenzung auf 1): je Quelle glowAlpha × falloff.
func (d *lightData) lift(x float64, w *sim.World) float64 {
	add := 0.0
	if d.campfire.valid() && hasCampfire(w) {
		add += glowAlpha(d.campfire.Strength, w.Cycle) * falloff(x, w.Castle.X, d.campfire.Radius)
	}
	if !d.torch.valid() {
		return add
	}
	a := glowAlpha(d.torch.Strength, w.Cycle)
	if a <= 0 {
		return add
	}
	for _, s := range w.Sites {
		if s.State != "built" {
			continue
		}
		side := torchSide(s.X, w.HubX)
		for _, o := range d.Torches[s.Kind] {
			add += a * falloff(x, s.X+side*o, d.torch.Radius)
		}
	}
	return add
}

// torchSide ist die Richtung der Fackel-Versätze: von der Hub-Mitte weg, in der Mitte nach rechts (engine/sim/torches.go).
func torchSide(x, hubX float64) float64 {
	if x < hubX {
		return -1
	}
	return 1
}

// levelAt ist die Helligkeit des Vordergrunds am Ort x (grau, 0 … 1) wie lightLevelAt im Browser.
func (d *lightData) levelAt(x float64, w *sim.World) float64 {
	base := d.front(w.Cycle)
	return min(1, base+(1-base)*d.lift(x, w))
}

// colorAt ist die Lichtfarbe am Ort x je Kanal (Multiplikator 0 … 1): Grundhelligkeit plus Schein in glowColor.
func (d *lightData) colorAt(x float64, w *sim.World) (r, g, b float64) {
	base := d.front(w.Cycle)
	l := (1 - base) * d.lift(x, w)
	return min(1, base+l*d.glow[0]), min(1, base+l*d.glow[1]), min(1, base+l*d.glow[2])
}
